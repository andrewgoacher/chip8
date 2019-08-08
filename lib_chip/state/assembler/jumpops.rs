use crate::opcode::{JumpOp, OpCode};
use super::State;

fn handle_jump_from_v0(state: State, nnn: u16) -> State {
    let v0 = u16::from(state.registers[0x0]);
    State {
        last_opcode: OpCode::JP(JumpOp::JPV0(nnn)),
        pc: nnn+v0,
        ..state
    }
}

pub fn handle_jump_ops(state: State, op: JumpOp) -> State {
    match op {
        JumpOp::JP(nnn) => State {pc: nnn, last_opcode: OpCode::JP(JumpOp::JP(nnn)), ..state},
        JumpOp::JPV0(nnn) => handle_jump_from_v0(state, nnn)
    }
}