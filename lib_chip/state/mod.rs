mod functions;
mod display;
mod loadops;
mod jumpops;
mod skipops;
mod addops;
mod shiftops;


use crate::memory::Memory;
use crate::opcode::OpCode;
use functions::get_opcode;
use rand::Rng;

use self::loadops::handle_load_operands;
use self::jumpops::handle_jump_ops;
use self::skipops::handle_skip_ops;
use self::addops::handle_add_op;
use self::shiftops::handle_shift_op;

#[derive(Debug)]
pub struct State {
    pub stack: [u16; 16],
    pub registers: [u8; 16],
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub pc: u16,
    pub stack_pointer: u16,
    pub i: u16,
    pub draw_flag: bool,
    pub run_flag: bool,
    pub clear_flag: bool,
    pub last_opcode: OpCode,
    pub opcode: Option<OpCode>,
    pub width: u32,
    pub height: u32
}

fn delay_timer(state: &State) -> u8 {
    if state.delay_timer > 0 {
        state.delay_timer -1
    } else {
        0
    }
}

fn sound_timer(state: &State) -> u8 {
    if state.sound_timer > 0 {
        state.sound_timer-1
    } else {
        0
    }
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
        ..state
    }
}

fn return_from_routine(state: State) -> State {
    let stack_pointer = state.stack_pointer-1;
    let pc = state.stack[stack_pointer as usize];
    State {
        pc,
        stack_pointer,
        ..state
    }
}

fn subtract_y_from_x(state: State, pc: u16, vx: u8, vy: u8) -> State {
    let mut registers = state.registers;
    let x = registers[vx as usize];
    let y = registers[vy as usize];

    let (result, borrows) = x.overflowing_sub(y);
    registers[vx as usize] = result;
    registers[0xF] = if borrows { 0 } else { 1 };

    State {
        registers,
        pc,
        ..state
    }
}

fn subtract_x_from_y(state: State, pc: u16, vx: u8, vy: u8) -> State {
    let mut registers = state.registers;
    let x = registers[vx as usize];
    let y = registers[vy as usize];

    let (result, borrows) = y.overflowing_sub(x);
    registers[vx as usize] = result;
    registers[0xF] = if borrows  { 0 } else { 1 };

    State {
        registers,
        pc,
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
        ..state
    }
}

fn handle_draw(state: State, pc: u16, vx: u8, vy: u8, n: u8, memory: &Memory, screen: &mut [u8]) -> State {
    let mut erased = 0;
    let row = vx;
    let col = vy;
    let width = state.width;
    let height = state.height;

    for byte_index in 0 .. n {
        let byte = memory.read(state.i + u16::from(byte_index));
        
        for bit_index in 0 .. 8 {
            let bit: u8 = (byte >> bit_index) & 0x1;

            let curr_x = u32::from(row + byte_index) % width;
            let curr_y = u32::from(col + (7-bit)) % height;
            let curr_idx = ((curr_y*width) + curr_x) as usize;
            let curr_pixel = screen[curr_idx];


            if bit == 1 && curr_pixel == 1 {
                erased = 1;
            }

            let pixel = curr_pixel ^ bit;
            screen[curr_idx] = pixel;
        }
    }

    let mut registers = state.registers;
    registers[0xF] = erased;

    State {
        registers,
        pc,
        draw_flag: true,
        ..state
    }
}

fn handle_logical(state: State, pc: u16, vx: u8, vy: u8, logical: Logical) -> State {
    let mut registers = state.registers;
    let x = registers[vx as usize];
    let y = registers[vy as usize];
    registers[vx as usize] = match logical {
        Logical::AND => x & y,
        Logical::OR => x | y,
        Logical::XOR => x ^ y
    };

    State {
        pc,
        registers,
        ..state
    }
}

enum Logical {
    AND,
    OR, 
    XOR
}

fn step_inner(state: State, memory: &mut Memory, keycode: Option<u8>, screen: &mut [u8], opcode: OpCode) -> State {
    let pc: u16 = state.pc+2;
    // todo: Not currently passing these in
    let _d = delay_timer(&state);
    let _s = sound_timer(&state);

    match opcode {
        OpCode::Unknown(c) => panic!("Unknown opcode: {:04X}", c),
        OpCode::CLS => State {clear_flag: true, pc, ..state},
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

impl State {
    pub fn new(w: u32, h: u32) -> State {
        State {
            stack: [0; 16],
            registers: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
            pc: 0x200,
            stack_pointer: 0,
            i: 0,
            draw_flag: true,
            run_flag: true,
            clear_flag: true,
            last_opcode: OpCode::Unknown(0),
            opcode: None,
            width: w,
            height: h
        }
    }

    pub fn step(self, memory: &mut Memory, keycode: Option<u8>, 
    screen: &mut Vec<u8>) -> State {
        let opcode = match self.opcode {
            None => get_opcode(&self, memory),
            Some(code) => code
        };

        step_inner(self, memory, keycode, &mut screen[..], opcode)
    }
}
