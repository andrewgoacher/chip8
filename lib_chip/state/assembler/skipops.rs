use super::State;
use crate::opcode::{SkipOp, OpCode};

fn handle_skip_if_equal(state: State, vx: u8, kk: u8, pc: u16) -> State {
    let x = state.registers[vx as usize];
    let mut pc = pc;
    if x == kk {
        pc +=2;
    }

    State {
        last_opcode: OpCode::SKIP(SkipOp::SE(vx, kk)),
        pc,
        ..state
    }
}

fn handle_skip_if_not_equal(state: State, vx: u8, kk: u8, pc: u16) -> State {
    let x = state.registers[vx as usize];
    let mut pc = pc;
    if x != kk {
        pc +=2;
    }

    State {
        pc,
        last_opcode: OpCode::SKIP(SkipOp::SNE(vx, kk)),
        ..state
    }
}

fn handle_skip_if_registers_equal(state: State, vx: u8, vy: u8, pc: u16) -> State {
    let x = state.registers[vx as usize];
    let y = state.registers[vy as usize];
    let mut pc = pc;
    if x == y {
        pc +=2;
    }

    State {
        last_opcode: OpCode::SKIP(SkipOp::SEXY(vx,vy)),
        pc,
        ..state
    }
}

fn handle_skip_if_registers_not_equal(state: State, vx: u8, vy: u8, pc: u16) -> State {
    let x = state.registers[vx as usize];
    let y = state.registers[vy as usize];
    let mut pc = pc;
    if x != y {
        pc +=2;
    }

    State {
        last_opcode: OpCode::SKIP(SkipOp::SNEXY(vx,vy)),
        pc,
        ..state
    }
}

fn handle_skip_on_keyboard(state: State, keycode: Option<u8>, vx: u8, pc: u16) -> State {
    let value = state.registers[vx as usize];
    let mut pc = pc;
    match keycode {
        None => (),
        Some(code) => {
            if value == code {
                pc += 2;
            }
        }
    };

    State {
        last_opcode: OpCode::SKIP(SkipOp::SKP(vx)),
        pc,
        ..state
    }
}

fn handle_skip_on_keyboard_up(state: State, keycode: Option<u8>, vx: u8, pc: u16) -> State {
    let value = state.registers[vx as usize];
    let mut pc = pc;
    match keycode {
        None => (),
        Some(code) => {
            if value != code {
                pc += 2;
            }
        }
    };

    State {
        last_opcode: OpCode::SKIP(SkipOp::SKNP(vx)),
        pc,
        ..state
    }
}


pub fn handle_skip_ops(state: State, op: SkipOp, pc: u16, keycode: Option<u8>) -> State {
    match op {
        SkipOp::SE(vx, kk) => handle_skip_if_equal(state, vx, kk, pc),
        SkipOp::SNE(vx, kk) => handle_skip_if_not_equal(state, vx, kk, pc),
        SkipOp::SEXY(vx, vy) => handle_skip_if_registers_equal(state, vx, vy, pc),
        SkipOp::SNEXY(vx, vy) => handle_skip_if_registers_not_equal(state, vx, vy, pc),
        // todo: Need to ensure these don't need wait for inputs
        SkipOp::SKP(vx) => handle_skip_on_keyboard(state, keycode, vx, pc),
        SkipOp::SKNP(vx) => handle_skip_on_keyboard_up(state, keycode, vx, pc)
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;
    use crate::opcode::{SkipOp};

    #[test]
    fn it_should_not_skip_if_kk_not_equal() {
        let mut registers = [0x0;16];
        const VX:u8 = 0xD;
        registers[VX as usize] = 0x3;
        const KK:u8 = 5u8;

        let state = State {
            registers,
            ..Default::default()
        };

        let new_state = handle_skip_ops(state, SkipOp::SE(VX, KK), 0x200, None);

        assert_eq!(0x200, new_state.pc);
    }

    #[test]
    fn it_should_skip_if_kk_is_equal() {
        let mut registers = [0x0;16];
        const VX:u8 = 0xD;
        registers[VX as usize] = 0x3;

        const KK:u8 = 0x3;

        let state = State {
            registers,
            ..Default::default()
        };

        let new_state = handle_skip_ops(state, SkipOp::SE(VX, KK), 0x200, None);

        assert_eq!(0x202, new_state.pc);
    }

    #[test]
    fn it_should_not_skip_if_kk_equal() {
        let mut registers = [0x0;16];
        const VX:u8 = 0xD;
        registers[VX as usize] = 0x3;
        const KK:u8 = 0x3;

        let state = State {
            registers,
            ..Default::default()
        };

        let new_state = handle_skip_ops(state, SkipOp::SNE(VX, KK), 0x200, None);

        assert_eq!(0x200, new_state.pc);
    }

    #[test]
    fn it_should_skip_if_kk_is_not_equal() {
        let mut registers = [0x0;16];
        const VX:u8 = 0xD;
        registers[VX as usize] = 0x3;

        const KK:u8 = 0x4;

        let state = State {
            registers,
            ..Default::default()
        };

        let new_state = handle_skip_ops(state, SkipOp::SNE(VX, KK), 0x200, None);

        assert_eq!(0x202, new_state.pc);
    }

    #[test]
    fn it_should_skip_if_vx_and_vy_are_equal() {
        let mut registers = [0x0; 16];
        const VX:u8 = 0xD;
        const VY:u8 = 0x4;
        const DATA:u8 = 0x5;

        registers[VX as usize] = DATA;
        registers[VY as usize] = DATA;

        let state = State {
            registers,
            ..Default::default()
        };

        let new_state = handle_skip_ops(state, SkipOp::SEXY(VX, VY), 0x200, None);

        assert_eq!(0x202, new_state.pc);
    }

    #[test]
    fn it_should_not_skip_if_vx_and_vy_are_not_equal() {
        let mut registers = [0x0; 16];
        const VX:u8 = 0xD;
        const VY:u8 = 0x4;

        registers[VX as usize] = 0x1;
        registers[VY as usize] = 0x2;

        let state = State {
            registers,
            ..Default::default()
        };

        let new_state = handle_skip_ops(state, SkipOp::SEXY(VX, VY), 0x200, None);

        assert_eq!(0x200, new_state.pc);
    }

    #[test]
    fn it_should_skip_if_vx_and_vy_are_not_equal() {
        let mut registers = [0x0; 16];
        const VX:u8 = 0xD;
        const VY:u8 = 0x4;

        registers[VX as usize] = 0x1;
        registers[VY as usize] = 0x2;

        let state = State {
            registers,
            ..Default::default()
        };

        let new_state = handle_skip_ops(state, SkipOp::SNEXY(VX, VY), 0x200, None);

        assert_eq!(0x202, new_state.pc);
    }

    #[test]
    fn it_should_not_skip_if_vx_and_vy_are_equal() {
        let mut registers = [0x0; 16];
        const VX:u8 = 0xD;
        const VY:u8 = 0x4;
        const DATA:u8 = 0x5;

        registers[VX as usize] = DATA;
        registers[VY as usize] = DATA;

        let state = State {
            registers,
            ..Default::default()
        };

        let new_state = handle_skip_ops(state, SkipOp::SNEXY(VX, VY), 0x200, None);

        assert_eq!(0x200, new_state.pc);
    }

    #[test]
    fn it_should_skip_if_key_pressed() {
        let mut registers = [0x0; 16];
        const VX:u8 = 0xD;
        const DATA:u8 = 0x5;

        registers[VX as usize] = DATA;

        let state = State {
            registers,
            ..Default::default()
        };

        let key = Some(5u8);

        let new_state = handle_skip_ops(state, SkipOp::SKP(VX), 0x200, key);

        assert_eq!(0x202, new_state.pc);   
    }

    #[test]
    fn it_should_not_skip_if_key_not_pressed() {
        let mut registers = [0x0; 16];
        const VX:u8 = 0xD;
        const DATA:u8 = 0x5;

        registers[VX as usize] = DATA;

        let state = State {
            registers,
            ..Default::default()
        };

        let key = Some(6u8);

        let new_state = handle_skip_ops(state, SkipOp::SKP(VX), 0x200, key);

        assert_eq!(0x200, new_state.pc);  
    }

    #[test]
    fn it_should_skip_if_key_not_pressed() {
        let mut registers = [0x0; 16];
        const VX:u8 = 0xD;
        const DATA:u8 = 0x5;

        registers[VX as usize] = DATA;

        let state = State {
            registers,
            ..Default::default()
        };

        let key = Some(6u8);

        let new_state = handle_skip_ops(state, SkipOp::SKNP(VX), 0x200, key);

        assert_eq!(0x202, new_state.pc);  
    }

    #[test]
    fn it_should_not_skip_if_key_is_pressed() {
        let mut registers = [0x0; 16];
        const VX:u8 = 0xD;
        const DATA:u8 = 0x5;

        registers[VX as usize] = DATA;

        let state = State {
            registers,
            ..Default::default()
        };

        let key = Some(5u8);

        let new_state = handle_skip_ops(state, SkipOp::SKNP(VX), 0x200, key);

        assert_eq!(0x200, new_state.pc);     
    }
}