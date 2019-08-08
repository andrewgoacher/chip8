use super::State;
use crate::opcode::SkipOp;

fn handle_skip_if_equal(state: State, vx: u8, kk: u8, pc: u16) -> State {
    let x = state.registers[vx as usize];
    let mut pc = pc;
    if x == kk {
        pc +=2;
    }

    State {
        pc: pc,
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
        pc: pc,
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
        pc: pc,
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
        pc: pc,
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
        pc: pc,
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
        pc: pc,
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