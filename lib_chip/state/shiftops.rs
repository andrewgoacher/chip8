use super::State;
use crate::opcode::ShiftOp;

fn handle_shift_left(state: State, pc: u16, vx: u8) -> State {
    let mut registers = state.registers;
    let x = registers[vx as usize] << 1;
    let msb = (x & 0xF0) >> 1;
    registers[0xF] = if msb == 1 { 1 } else { 0 };
    registers[vx as usize] = x;

    State {
        pc: pc,
        registers: registers,
        ..state
    }
}

fn handle_shift_right(state: State, pc: u16, vx: u8) -> State {
    let mut registers = state.registers;
    let x = registers[vx as usize] >> 1;
    let lsb = x & 0x0F;
    registers[0xF] = if lsb == 1 { 1 } else { 0 };
    registers[vx as usize] = x;

    State {
        pc: pc,
        registers: registers,
        ..state
    }
}

pub fn handle_shift_op(state: State, pc: u16, op: ShiftOp) -> State {
    match op {
        ShiftOp::SHL(vx) => handle_shift_left(state, pc, vx),
        ShiftOp::SHR(vx) => handle_shift_right(state, pc, vx)
    }
}