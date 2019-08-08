use super::State;
use crate::opcode::AddOp;

fn add_to_vx(state: State, vx: u8, kk: u8, pc: u16) -> State {
    let mut registers = state.registers;
    let x = registers[vx as usize];
    let (val, _overflowed) = x.overflowing_add(kk);
    registers[vx as usize] = val;

    State {
        registers,
        pc,
        ..state
    }
}

fn add_vy_to_vx(state: State, vx: u8, vy: u8, pc: u16) -> State {
    let mut registers = state.registers;
    let x = registers[vx as usize];
    let y = registers[vy as usize];

    let (result, carry) = x.overflowing_add(y);

    registers[0xF] = if carry { 1 } else { 0 };
    registers[vx as usize] = result;

    State {
        pc,
        registers,
        ..state
    }
}

fn add_vx_to_i(state: State, vx: u8, pc: u16) -> State {
    let x = state.registers[vx as usize];
    let i = state.i + u16::from(x);

    State {
        pc,
        i,
        ..state
    }
}

pub fn handle_add_op(state: State, op: AddOp, pc: u16) -> State {
    match op {
        AddOp::ADD(vx, kk) => add_to_vx(state, vx, kk, pc),
        AddOp::ADDREG(vx, vy) => add_vy_to_vx(state, vx, vy, pc),
        AddOp::ADDI(vx) => add_vx_to_i(state, vx, pc)
    }
}