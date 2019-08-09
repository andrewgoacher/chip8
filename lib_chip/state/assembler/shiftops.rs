use super::State;
use crate::opcode::{ShiftOp, OpCode};

fn handle_shift_left(state: State, pc: u16, vx: u8) -> State {
    let mut registers = state.registers;
    let x = registers[vx as usize] << 1;
    let msb = (x & 0xF0) >> 7;
    registers[0xF] = if msb == 1 { 1 } else { 0 };
    registers[vx as usize] = x;

    State {
        last_opcode: OpCode::SHIFT(ShiftOp::SHL(vx)),
        pc,
        registers,
        ..state
    }
}

fn handle_shift_right(state: State, pc: u16, vx: u8) -> State {
    let mut registers = state.registers;
    let x = registers[vx as usize] >> 1;
    let lsb = x & 0x01;
    registers[0xF] = if lsb == 1 { 1 } else { 0 };
    registers[vx as usize] = x;

    State {
        last_opcode: OpCode::SHIFT(ShiftOp::SHR(vx)),
        pc,
        registers,
        ..state
    }
}

/// Handles shift right and shift left operations
pub fn handle_shift_op(state: State, pc: u16, op: ShiftOp) -> State {
    match op {
        ShiftOp::SHL(vx) => handle_shift_left(state, pc, vx),
        ShiftOp::SHR(vx) => handle_shift_right(state, pc, vx)
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;
    use crate::opcode::ShiftOp;

    #[test]
    fn it_will_shift_left_msb_true() {
        const VX:u8 = 0xD;
        let mut registers = [0x0;16];
        registers[VX as usize] = 0xFF;

        let state = State {
            registers,
            ..Default::default()
        };

        let new_state = handle_shift_op(state, 0x200, ShiftOp::SHL(VX));

        let msb = new_state.registers[0xF];
        let vx = new_state.registers[VX as usize];

        assert_eq!(1, msb);
        assert_eq!(0xFE, vx);
    }

    #[test]
    fn it_will_shift_left_msb_false() {
        const VX:u8 = 0xD;
        let mut registers = [0x0;16];
        registers[VX as usize] = 0xBF;

        let state = State {
            registers,
            ..Default::default()
        };

        let new_state = handle_shift_op(state, 0x200, ShiftOp::SHL(VX));

        let msb = new_state.registers[0xF];
        let vx = new_state.registers[VX as usize];

        assert_eq!(0, msb);
        assert_eq!(0x7E, vx);
    }

    #[test]
    fn it_will_shift_right_lsb_true() {
        const VX:u8 = 0xD;
        let mut registers = [0x0;16];
        registers[VX as usize] = 0xFF;

        let state = State {
            registers,
            ..Default::default()
        };

        let new_state = handle_shift_op(state, 0x200, ShiftOp::SHR(VX));

        let lsb = new_state.registers[0xF];
        let vx = new_state.registers[VX as usize];

        assert_eq!(1, lsb);
        assert_eq!(0x7F, vx);
    }

    #[test]
    fn it_will_shift_right_lsb_false() {
        const VX:u8 = 0xD;
        let mut registers = [0x0;16];
        registers[VX as usize] = 0xFD;

        let state = State {
            registers,
            ..Default::default()
        };

        let new_state = handle_shift_op(state, 0x200, ShiftOp::SHR(VX));

        let lsb = new_state.registers[0xF];
        let vx = new_state.registers[VX as usize];

        assert_eq!(0, lsb);
        assert_eq!(0x7E, vx);
    }
}