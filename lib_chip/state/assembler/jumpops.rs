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

#[cfg(test)]
mod tests {
    use super::super::State;
    use super::*;
    use crate::opcode::JumpOp;

    #[test]
    fn it_should_jump_to_stated_location() {
        let state: State = Default::default();

        let new_state = handle_jump_ops(state, JumpOp::JP(0x0FFF));

        assert_eq!(0x0FFF, new_state.pc);
    }

    #[test]
    fn it_should_jump_to_stated_offset_from_v0() {
        let mut registers = [0x0;16];
        registers[0x0] = 0x11;
        let state = State {
            registers,
            ..Default::default()
        };

        let new_state = handle_jump_ops(state, JumpOp::JPV0(0x0FFF));

        assert_eq!(0x1010, new_state.pc);
    }
}