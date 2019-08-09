use super::State;
use crate::opcode::{AddOp, OpCode};

/// Adds kk to register V[x]
fn add_to_vx(state: State, vx: u8, kk: u8, pc: u16) -> State {
    let mut registers = state.registers;
    let x = registers[vx as usize];
    let (val, _overflowed) = x.overflowing_add(kk);
    registers[vx as usize] = val;

    State {
        last_opcode: OpCode::ADD(AddOp::ADD(vx,kk)),
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
        last_opcode: OpCode::ADD(AddOp::ADDREG(vx,vy)),
        pc,
        registers,
        ..state
    }
}

fn add_vx_to_i(state: State, vx: u8, pc: u16) -> State {
    let x = state.registers[vx as usize];
    let i = state.i + u16::from(x);

    State {
        last_opcode: OpCode::ADD(AddOp::ADDI(vx)),
        pc,
        i,
        ..state
    }
}

/// Handles all operands that fall under the ADD category.
pub fn handle_add_op(state: State, op: AddOp, pc: u16) -> State {
    match op {
        AddOp::ADD(vx, kk) => add_to_vx(state, vx, kk, pc),
        AddOp::ADDREG(vx, vy) => add_vy_to_vx(state, vx, vy, pc),
        AddOp::ADDI(vx) => add_vx_to_i(state, vx, pc)
    }
}

#[cfg(test)]
mod tests {
    use super::super::State;
    use super::*;

    #[test]
    fn it_should_add_kk_to_register_vx() {
        const VX:u8 = 0x3;
        let mut registers = [0x0; 16];
        registers[VX as usize] = 0x10;

        let state: State = State {
            registers,
            ..Default::default()
        };


        let new_state = handle_add_op(state, AddOp::ADD(VX, 0xA1), 0x200);
        let registers = new_state.registers;

        assert_eq!(registers[VX as usize], 0xB1);
    }

    #[test]
    fn it_should_add_kk_to_register_vx_with_overflow() {
        const VX:u8 = 0x3;
        let mut registers = [0x0; 16];
        registers[VX as usize] = 0x10;

        let state: State = State {
            registers,
            ..Default::default()
        };

        let new_state = handle_add_op(state, AddOp::ADD(VX, 0xFF), 0x200);
        let registers = new_state.registers;

        assert_eq!(registers[VX as usize], 0x0F);   
    }

    #[test]
    fn it_should_add_vy_to_vx() {
        const VX:u8 = 0x3;
        const VY:u8 = 0x4;
        let mut registers = [0x0; 16];
        registers[VX as usize] = 0x10;
        registers[VY as usize] = 0x15;

        let state: State = State {
            registers,
            ..Default::default()
        };

        let new_state = handle_add_op(state, AddOp::ADDREG(VX,VY), 0x200);
        let registers = new_state.registers;

        assert_eq!(registers[VX as usize], 0x25);  
        assert_eq!(registers[0xF], 0x00);
    }

    #[test]
    fn it_should_add_vy_to_vx_with_overflow() {
        const VX:u8 = 0x3;
        const VY:u8 = 0x4;
        let mut registers = [0x0; 16];
        registers[VX as usize] = 0x10;
        registers[VY as usize] = 0xFF;

        let state: State = State {
            registers,
            ..Default::default()
        };


        let new_state = handle_add_op(state, AddOp::ADDREG(VX, VY), 0x200);
        let registers = new_state.registers;

        assert_eq!(registers[VX as usize], 0x0F);  
        assert_eq!(registers[0xF], 0x01);
    }

    #[test]
    fn it_should_add_vx_to_i() {
        const VX:u8 = 0x3;
        let mut registers = [0x0;16];
        registers[VX as usize] = 0x10;

        let state: State = State {
            registers,
            i: 0x15,
            ..Default::default()
        };

        let new_state = handle_add_op(state, AddOp::ADDI(VX), 0x200);
        assert_eq!(0x25, new_state.i)
    }
}