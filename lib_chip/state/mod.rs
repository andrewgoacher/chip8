mod display;
mod assembler;


use crate::memory::Memory;
use crate::opcode::{OpCode, parser::parse_opcode};

use assembler::assemble;

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

fn get_opcode(state: &State, memory: &Memory) -> OpCode {
    let pc = state.pc;
    let high = memory.read(pc);
    let low = memory.read(pc + 1);
    parse_opcode(high, low)
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

        let d = delay_timer(&self);
        let s = sound_timer(&self);

        let state = State {
            delay_timer: d,
            sound_timer: s,
            ..self
        };

        assemble(state, memory, keycode, &mut screen[..], opcode)
    }
}
