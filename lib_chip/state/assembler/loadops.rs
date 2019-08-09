use super::State;
use crate::memory::Memory;
use crate::opcode::{OpCode,LoadOp};

fn load_x_from_y(state: State, vx: u8, vy: u8, pc: u16) -> State {
    let mut registers = state.registers;
    registers[vx as usize] = registers[vy as usize];

    State {
        last_opcode: OpCode::LD(LoadOp::LDXY(vx,vy)),
        registers,
        pc,
        ..state
    }
}

fn load_delay_timer(state: State, vx: u8, pc: u16) -> State {
    let mut registers = state.registers;
    registers[vx as usize] = state.delay_timer;
    State {
        last_opcode: OpCode::LD(LoadOp::LDVXDT(vx)),
        registers,
        pc,
        ..state
    }
}

fn set_i(state: State, pc: u16, kk: u16) -> State {
    State {
        last_opcode: OpCode::LD(LoadOp::LDI(kk)),
        i: kk,
        pc,
        ..state
    }
}

fn set_delay_timer(state: State, vx: u8, pc: u16) -> State {
    let delay = state.registers[vx as usize];
    State {
        delay_timer: delay,
        last_opcode: OpCode::LD(LoadOp::LDDTVX(vx)),
        pc,
        ..state
    }
}

fn handle_load_key(state: State, vx: u8, pc: u16, keycode: Option<u8>, loadop: LoadOp) -> State {
    let mut registers = state.registers;
    let mut pc = pc-2;
    let next_opcode = match keycode {
      None => Some(OpCode::LD(loadop)),
      Some(key_press) => {
          registers[vx as usize] = key_press;
          pc += 2;
          None
      }  
    };

    State {
        opcode: next_opcode,
        last_opcode: OpCode::LD(LoadOp::LDKEY(vx)),
        pc,
        registers,
        ..state
    }
}

fn set_sound_timer(state: State, vx: u8, pc: u16) -> State {
    let sound_timer = state.registers[vx as usize];
    State {
        last_opcode: OpCode::LD(LoadOp::LDSTVX(vx)),
        pc,
        sound_timer,
        ..state
    }
}

const BYTES_PER_SPRITE: u16 = 5;

fn load_sprite(state: State, vx: u8, pc: u16) -> State {
    let sprite = u16::from(state.registers[vx as usize]);
    let i = BYTES_PER_SPRITE * sprite;
    State {
        last_opcode: OpCode::LD(LoadOp::LDF(vx)),
        pc,
        i,
        ..state
    }
}

fn handle_bcd_representation(state: State, memory: &mut Memory, pc: u16, vx: u8) -> State {
    let val = state.registers[vx as usize];
    let units = val % 10;
    let tens = (val - units) % 100;
    let hundreds = val - tens - units;
    let i = state.i;

    memory.set(i as usize, hundreds);
    memory.set((i + 1) as usize, tens);
    memory.set((i + 2) as usize, units);

    State {
        last_opcode: OpCode::LD(LoadOp::LDB(vx)),
        pc,
        ..state
    }
}

fn load_from_registers(state: State, memory: &mut Memory, vx: u8, pc: u16) -> State {
    let registers = state.registers;
    let i = state.i;
    for v in 0..=u16::from(vx) {
        let val = registers[v as usize];
        let addr = i+v;
        memory.set(addr as usize, val);
    }
    
    State {
        last_opcode: OpCode::LD(LoadOp::LDIV0X(vx)),
        pc,
        ..state
    }
}

fn set_register(state: State, pc: u16, vx: u8, kk: u8) -> State {
    let mut registers = state.registers;
    registers[vx as usize] = kk;
    State {
        last_opcode: OpCode::LD(LoadOp::LD(vx, kk)),
        registers,
        pc,
        ..state
    }
}

fn set_registers(state: State, pc: u16, vx: u8, memory: &Memory) -> State {
    let mut registers = state.registers;
    let i = state.i;

    for v in 0..=u16::from(vx) {
        let addr = i+v;
        let val = memory.read(addr);
        registers[v as usize] = val;
    }

    State {
        registers,
        pc,
        last_opcode: OpCode::LD(LoadOp::LDV0XI(vx)),
        ..state
    }
}

pub fn handle_load_operands(state: State, load_op: LoadOp, pc: u16, memory: &mut Memory, keycode: Option<u8>) -> State {
    match load_op {
        LoadOp::LD(vx, kk) => set_register(state, pc, vx, kk),
        LoadOp::LDV0XI(vx) => set_registers(state, pc, vx, memory),
        LoadOp::LDIV0X(vx) => load_from_registers(state, memory, vx, pc),
        LoadOp::LDB(vx) => handle_bcd_representation(state, memory, pc, vx),
        LoadOp::LDF(vx) => load_sprite(state, vx, pc),
        LoadOp::LDSTVX(vx) => set_sound_timer(state, vx, pc),
        LoadOp::LDKEY(vx) => handle_load_key(state, vx, pc, keycode, load_op),
        LoadOp::LDDTVX(vx) => set_delay_timer(state, vx, pc),
        LoadOp::LDI(kk) => set_i(state, pc, kk),
        LoadOp::LDVXDT(vx) => load_delay_timer(state, vx, pc),
        LoadOp::LDXY(vx, vy) => load_x_from_y(state, vx, vy, pc)
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;
    use crate::opcode::{OpCode, LoadOp};
    use crate::memory::Memory;

    #[test]
    fn it_should_load_value_into_vx() {
        let state:State = Default::default();
        let mut memory = Memory::new();
        const VX:u8 = 0x4;
        const KK:u8 = 0xFF;

        let new_state = handle_load_operands(state, LoadOp::LD(VX,KK), 0x299, &mut memory, None);
        let actual = new_state.registers[VX as usize];

        assert_eq!(KK, actual);
    }

    #[test]
    fn it_should_load_memory_into_specified_number_of_registers() {
        const I:u16 = 0x200;
        let mut memory = Memory::new();
        let mem = &vec![0x1,0x2,0x3,0x4][..];
        memory.set_range(I as usize, mem);
        const VX:u8 = 0x03;

        let state = State {
            i: I,
            ..Default::default()
        };

        let new_state = handle_load_operands(state, LoadOp::LDV0XI(VX), 0x299, &mut memory, None);
        let registers = new_state.registers;
        let slice = &registers[..4];
        assert_eq!(mem, slice);
    }

    #[test]
    fn it_should_read_specified_registers_into_memory() {
        let mut registers = [0x0;16];
        const VX:u8 = 0x5;
        registers[0x0] = 0x1;
        registers[0x3] = 0x6;
        registers[0x5] = 0xF;

        let mut memory = Memory::new();
        const I:u16=0x200;

        let state = State {
            i: I,
            registers: registers,
            ..Default::default()
        };

        let new_state = handle_load_operands(state, LoadOp::LDIV0X(VX), 0x200, &mut memory, None);
        let registers = new_state.registers;
        let reg_slice = &registers[0..5];
        let mem = [memory.read(I), memory.read(I+1), memory.read(I+2),
        memory.read(I+3), memory.read(I+4)];

        assert_eq!(reg_slice, &mem[..]);
    }

    #[test]
    fn it_shuold_read_bcd_representation_into_memory() {
        let mut registers = [0x0;16];
        const VX:u8 = 0x6;
        registers[VX as usize] = 0xFE;
        let mut memory = Memory::new();
        const I:u16 = 0x200;

        let state = State {
            i: I,
            registers: registers,
            ..Default::default()
        };

        let new_state = handle_load_operands(state, LoadOp::LDB(VX), 0x200, &mut memory, None);

        let i = new_state.i;
        let (h,t,u) = (memory.read(i), memory.read(i+1), memory.read(i+2));

        assert_eq!(200, h);
        assert_eq!(50, t);
        assert_eq!(4, u);
    }

    #[test]
    fn it_should_load_sprite_into_memory() {
        let mut registers = [0x0;16];
        const VX:u8 = 0xE;
        const DATA:u8 = 0xD;
        registers[VX as usize] = DATA;
        const I:u16 = 0x202;
        let mut memory = Memory::new();

        let state = State {
            i:I,
            registers: registers,
            ..Default::default()
        };

        let new_state = handle_load_operands(state, LoadOp::LDF(VX), 0x200, &mut memory, None);

        assert_eq!(u16::from(DATA) * 5, new_state.i);
    }

    #[test]
    fn it_should_set_the_sound_timer() {
        let mut registers = [0x0;16];
        const VX:u8 = 0xD;
        registers[VX as usize] = 0x12;

        let mut memory = Memory::new();

        let state = State {
            registers,
            ..Default::default()
        };

        let new_state = handle_load_operands(state, LoadOp::LDSTVX(VX), 0x200, &mut memory, None);
        assert_eq!(0x12, new_state.sound_timer);
    }

    #[test]
    fn when_key_not_pressed_should_not_progress_pc() {
        let registers = [0x0;16];
        const VX:u8 = 0x01;

        let mut memory = Memory::new();

        let state = State {
            registers,
            ..Default::default()
        };

        let new_state = handle_load_operands(state, LoadOp::LDKEY(VX), 0x202, &mut memory, None);

        assert_eq!(0x200, new_state.pc);
        assert_eq!(Some(OpCode::LD(LoadOp::LDKEY(VX))), new_state.opcode);
    }

    #[test]
    fn when_key_pressed_should_progress() {
        let registers = [0x0;16];
        const VX:u8=0x01;
        const KEY:u8 = 0x12;

        let mut memory = Memory::new();

        let state = State {
            registers,
            ..Default::default()
        };

        let new_state = handle_load_operands(state, LoadOp::LDKEY(VX), 0x200, &mut memory, Some(KEY));

        assert_eq!(None, new_state.opcode);
        let registers = new_state.registers;

        assert_eq!(KEY, registers[VX as usize]);
    }

    #[test]
    fn it_should_load_delay_timer_in_vx() {
        const VX:u8 = 0x04;
        let registers = [0x0;16];
        const DT:u8 = 0xFF;
        let mut memory = Memory::new();

        let state = State {
            registers,
            delay_timer: DT,
            ..Default::default()
        };

        let new_state = handle_load_operands(state, LoadOp::LDVXDT(VX), 0x200, &mut memory, None);

        assert_eq!(0xFF, new_state.registers[VX as usize]);
    }

    #[test]
    fn it_should_load_vy_into_vx() {
        const VX:u8 = 0xF;
        const VY:u8 = 0xD;

        let mut registers = [0x0;16];
        registers[VY as usize] = 0xAE;
        registers[VX as usize] = 0x01;

        let mut memory = Memory::new();

        let state = State { registers, ..Default::default()};

        let new_state = handle_load_operands(state, LoadOp::LDXY(VX, VY), 0x200, &mut memory, None);

        assert_eq!(0xAE, new_state.registers[VX as usize]);
    }
}