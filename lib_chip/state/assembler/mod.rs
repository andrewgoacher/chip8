use super::State;
use crate::memory::Memory;
use crate::opcode::OpCode;

mod loadops;
mod jumpops;
mod skipops;
mod addops;
mod shiftops;

use self::loadops::handle_load_operands;
use self::jumpops::handle_jump_ops;
use self::skipops::handle_skip_ops;
use self::addops::handle_add_op;
use self::shiftops::handle_shift_op;

use rand::Rng;

enum Logical {
    AND,
    OR, 
    XOR
}

fn call_routine(location: u16, pc: u16, state: State) -> State {
    let mut stack = state.stack;
    let mut stack_pointer = state.stack_pointer;
    stack[stack_pointer as usize] = pc;
    stack_pointer += 1;

    State {
        pc: location,
        stack_pointer,
        stack,
        last_opcode: OpCode::CALL(location),
        ..state
    }
}

fn return_from_routine(state: State) -> State {
    let stack_pointer = state.stack_pointer-1;
    let pc = state.stack[stack_pointer as usize];
    State {
        pc,
        stack_pointer,
        last_opcode: OpCode::RET,
        ..state
    }
}

fn subtract_y_from_x(state: State, pc: u16, vx: u8, vy: u8) -> State {
    let mut registers = state.registers;
    let x = registers[vx as usize];
    let y = registers[vy as usize];

    let (result, _overflows) = x.overflowing_sub(y);
    registers[vx as usize] = result;
    registers[0xF] = if x > y { 1 } else { 0 };

    State {
        registers,
        pc,
        last_opcode: OpCode::SUB(vx,vy),
        ..state
    }
}

fn subtract_x_from_y(state: State, pc: u16, vx: u8, vy: u8) -> State {
    let mut registers = state.registers;
    let x = registers[vx as usize];
    let y = registers[vy as usize];

    let (result, _overflows) = y.overflowing_sub(x);
    registers[vx as usize] = result;
    registers[0xF] = if y > x  { 1 } else { 0 };

    State {
        registers,
        pc,
        last_opcode: OpCode::SUBN(vx,vy),
        ..state
    }
}

fn set_rnd(state: State, vx: u8, pc: u16, kk: u8) -> State {
    let mut rng = rand::thread_rng();
    let r:u8 = rng.gen();
    let val = r & kk;
    let mut registers = state.registers;
    registers[vx as usize] = val;

    State {
        registers,
        pc,
        last_opcode: OpCode::RND(vx, kk),
        ..state
    }
}

fn wrap(val: u8, max: u8) -> u8 {
    if val > max {
        val % max
    } else {
        val
    }
}

fn handle_draw(state: State, pc: u16, vx: u8, vy: u8, n: u8, memory: &Memory, screen: &mut [u8]) -> State {
    let mut erased = 0;
    let row = state.registers[vx as usize];
    let col = state.registers[vy as usize];
    let width = state.width;
    let height = state.height;

    for yline in 0..n {
        let sprite = memory.read(state.i + u16::from(yline));
        for xline in 0..8{
            if (sprite & (0x80 >> xline)) != 0 {
                let x = u32::from(wrap(row + xline, width as u8));
                let y = u32::from(wrap(yline  + col, height as u8));
                let idx = ((y*width) + x) as usize;
                let current_pixel = screen[idx];
                if current_pixel == 1 {
                    erased = 1;
                }

                screen[idx] ^=1;
            }
        }
    }

    let mut registers = state.registers;
    registers[0xF] = erased;

    State {
        registers,
        pc,
        draw_flag: true,
        last_opcode: OpCode::DRW(vx,vy,n),
        ..state
    }
}

fn handle_logical(state: State, pc: u16, vx: u8, vy: u8, logical: Logical) -> State {
    let mut registers = state.registers;
    let x = registers[vx as usize];
    let y = registers[vy as usize];
    let (r,o) = match logical {
        Logical::AND => (x & y, OpCode::AND(vx,vy)),
        Logical::OR => (x | y, OpCode::OR(vx,vy)),
        Logical::XOR => (x ^ y, OpCode::XOR(vx,vy))
    };

    registers[vx as usize] = r;

    State {
        pc,
        registers,
        last_opcode: o,
        ..state
    }
}

pub fn assemble(state: State, memory: &mut Memory, keycode: &[u8], screen: &mut [u8], opcode: OpCode) -> State {
    let pc: u16 = state.pc+2;

    match opcode {
        OpCode::Unknown(c) => panic!("Unknown opcode: {:04X}", c),
        OpCode::CLS => State {clear_flag: true, pc, last_opcode: OpCode::CLS, ..state},
        OpCode::CALL(nnn) => call_routine(nnn, pc, state),
        OpCode::RET => return_from_routine(state),
        OpCode::LD(ld) => handle_load_operands(state, ld, pc, memory, keycode),
        OpCode::JP(jp) => handle_jump_ops(state, jp),
        OpCode::SKIP(sp) => handle_skip_ops(state, sp, pc, keycode),
        OpCode::ADD(op) => handle_add_op(state, op, pc),
        OpCode::SUB(vx, vy) => subtract_y_from_x(state, pc, vx, vy),
        OpCode::SUBN(vx, vy) => subtract_x_from_y(state, pc, vx, vy),
        OpCode::RND(vx, kk) => set_rnd(state, vx, pc, kk),
        OpCode::DRW(vx, vy, n) => handle_draw(state, pc, vx, vy, n, memory, screen),
        OpCode::OR(vx, vy) => handle_logical(state, pc, vx, vy, Logical::OR),
        OpCode::AND(vx, vy) => handle_logical(state, pc, vx, vy, Logical::AND),
        OpCode::XOR(vx, vy) => handle_logical(state, pc, vx, vy, Logical::XOR),
        OpCode::SHIFT(so) => handle_shift_op(state, pc, so)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::opcode::OpCode;
    use crate::memory::Memory;

    #[test]
    fn it_sets_the_clear_flag() {
        let state:State = Default::default();
        let mut screen = [0x0;200];
        let mut memory = Memory::new();

        let new_state = assemble(state, &mut memory, &Vec::new()[..], &mut screen[..], OpCode::CLS);
        assert!(new_state.clear_flag);
    }

    #[test]
    fn it_calls_the_new_routine() {
        let state:State = State { pc: 0x200, ..Default::default() };
        let mut screen = [0x0;200];
        let mut memory = Memory::new();

        let new_state = assemble(state, &mut memory, &Vec::new()[..], &mut screen[..], OpCode::CALL(0x0123));
        
        assert_eq!(0x0123, new_state.pc);

        let stack = new_state.stack;
        let addr = stack[(new_state.stack_pointer-1) as usize];
        assert_eq!(0x202, addr);
    }

    #[test]
    fn it_returns_from_routine() {
        let mut stack = [0x0000; 16];
        stack[0] = 0xF334;
        let stack_pointer = 1;
        let state = State {
            pc: 0x200,
            stack,
            stack_pointer,
            ..Default::default()
        };

        let mut screen = [0x0;200];
        let mut memory = Memory::new();

        let new_state = assemble(state, &mut memory, &Vec::new()[..], &mut screen[..], OpCode::RET);

        assert_eq!(0xF334, new_state.pc);
        assert_eq!(0, new_state.stack_pointer);
    }

    #[test]
    fn it_will_subtract_vy_from_vx() {
        let mut registers = [0x0;16];
        const VX:u8 = 0xC;
        const VY:u8 = 0xD;
        registers[VX as usize] = 0xFF;
        registers[VY as usize] = 0xF0;

        let mut memory = Memory::new();
        let mut screen = [0x0;200];

        let state = State {
            registers,
            ..Default::default()
        };

        let new_state = assemble(state, &mut memory, &Vec::new()[..], &mut screen[..], OpCode::SUB(VX, VY));

        let registers = new_state.registers;
        assert_eq!(0x0F, registers[VX as usize]);

        assert_eq!(0x01, registers[0xF]);
    }

    #[test]
    fn it_will_subtract_vy_from_vx_and_borrow() {
        let mut registers = [0x0;16];
        const VX:u8 = 0xC;
        const VY:u8 = 0xD;
        registers[VX as usize] = 0xF0;
        registers[VY as usize] = 0xFF;

        let mut memory = Memory::new();
        let mut screen = [0x0;200];

        let state = State {
            registers,
            ..Default::default()
        };

        let new_state = assemble(state, &mut memory, &Vec::new()[..], &mut screen[..], OpCode::SUB(VX, VY));

        let registers = new_state.registers;
        assert_eq!(0xF1, registers[VX as usize]);

        assert_eq!(0x0, registers[0xF]);
    }

    #[test]
    fn it_will_subtract_vx_from_vy() {
        let mut registers = [0x0;16];
        const VX:u8 = 0xC;
        const VY:u8 = 0xD;

        registers[VX as usize] = 0xF0;
        registers[VY as usize] = 0xFF;

        let mut memory = Memory::new();
        let mut screen = [0x0;200];

        let state = State {
            registers,
            ..Default::default()
        };

        let new_state = assemble(state, &mut memory, &Vec::new()[..], &mut screen[..], OpCode::SUBN(VX, VY));

        let registers = new_state.registers;

        assert_eq!(0x0F, registers[VX as usize]);
        assert_eq!(0x1, registers[0xF]);
    }

    #[test]
    fn it_will_subtract_vx_from_vy_with_no_borrow() {
        let mut registers = [0x0;16];
        const VX:u8 = 0xC;
        const VY:u8 = 0xD;

        registers[VX as usize] = 0xFF;
        registers[VY as usize] = 0xF0;

        let mut memory = Memory::new();
        let mut screen = [0x0;200];

        let state = State {
            registers,
            ..Default::default()
        };

        let new_state = assemble(state, &mut memory, &Vec::new()[..], &mut screen[..], OpCode::SUBN(VX, VY));

        let registers = new_state.registers;

        assert_eq!(0xF1, registers[VX as usize]);
        assert_eq!(0x0, registers[0xF]);  
    }

    #[test]
    #[ignore]
    fn it_will_set_random_number() {
        // note: This is a relatively fragile test because 
        // can't inject random (yet)

        let mut memory = Memory::new();
        let mut screen = [0x0;200];
        let state:State = Default::default();

        const VX:u8 = 0xD;
        const KK:u8 = 0x12;

        let new_state = assemble(state, &mut memory, &Vec::new()[..], &mut screen[..], OpCode::RND(VX, KK));
        let registers = new_state.registers;
        assert_ne!(0x0, registers[VX as usize]);
    }

    #[test]
    fn it_will_or_vx_and_vy(){
        let mut memory = Memory::new();
        let mut screen = [0x0;200];
        const VX:u8 = 0xD;
        const VY:u8 = 0x2;
        let mut registers = [0x0;16];
        registers[VX as usize] = 0xF0;
        registers[VY as usize] = 0x0F;


        let state = State { 
            registers,
            ..Default::default() 
        };

        let new_state = assemble(state, &mut memory, &Vec::new()[..],
         &mut screen[..], OpCode::OR(VX, VY));
        
        let registers = new_state.registers;
        assert_eq!(0xFF, registers[VX as usize]);
    }

    #[test]
    fn it_will_and_vx_and_vy(){
        let mut memory = Memory::new();
        let mut screen = [0x0;200];
        const VX:u8 = 0xD;
        const VY:u8 = 0x2;
        let mut registers = [0x0;16];
        registers[VX as usize] = 0xFF;
        registers[VY as usize] = 0x00;


        let state = State { 
            registers,
            ..Default::default() 
        };

        let new_state = assemble(state, &mut memory, &Vec::new()[..],
         &mut screen[..], OpCode::AND(VX, VY));
        
        let registers = new_state.registers;
        assert_eq!(0x00, registers[VX as usize]);
    }

    #[test]
    fn it_will_exclusive_or_vx_and_vy(){
        let mut memory = Memory::new();
        let mut screen = [0x0;200];
        const VX:u8 = 0xD;
        const VY:u8 = 0x2;
        let mut registers = [0x0;16];
        registers[VX as usize] = 0b01010011;
        registers[VY as usize] = 0b00100100;


        let state = State { 
            registers,
            ..Default::default() 
        };

        let new_state = assemble(state, &mut memory, &Vec::new()[..],
         &mut screen[..], OpCode::XOR(VX, VY));
        
        let registers = new_state.registers;
        assert_eq!(0b01110111, registers[VX as usize]);
    }
}