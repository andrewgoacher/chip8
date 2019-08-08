use crate::opcode::parser::parse_opcode;
use crate::opcode::OpCode;
use crate::memory::Memory;
use super::State;

pub fn get_opcode(state: &State, memory: &Memory) -> OpCode {
    let pc = state.pc;
    let high = memory.read(pc);
    let low = memory.read(pc + 1);
    parse_opcode(high, low)
}