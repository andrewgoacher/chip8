use super::State;
use crate::memory::Memory;
use crate::opcode::{OpCode,LoadOp};

fn load_x_from_y(state: State, vx: u8, vy: u8, pc: u16) -> State {
    let mut registers = state.registers;
    registers[vx as usize] = registers[vy as usize];

    State {
        registers,
        pc,
        ..state
    }
}

fn load_delay_timer(state: State, vx: u8, pc: u16) -> State {
    let mut registers = state.registers;
    registers[vx as usize] = state.delay_timer;
    State {
        registers,
        pc,
        ..state
    }
}

fn set_i(state: State, pc: u16, kk: u16) -> State {
    State {
        i: kk,
        pc,
        ..state
    }
}

fn set_delay_timer(state: State, vx: u8, pc: u16) -> State {
    let delay = state.registers[vx as usize];
    State {
        delay_timer: delay,
        pc,
        ..state
    }
}

fn handle_load_key(state: State, vx: u8, pc: u16, keycode: Option<u8>, loadop: LoadOp) -> State {
    let mut registers = state.registers;
    let mut pc = pc-2;
    let next_opcode = match keycode {
      None => Some(OpCode::LD(loadop)),
      Some(key_press) => {
          registers[vx as usize] = key_press;
          pc += 2;
          None
      }  
    };

    State {
        opcode: next_opcode,
        pc,
        registers,
        ..state
    }
}

fn set_sound_timer(state: State, vx: u8, pc: u16) -> State {
    let sound_timer = state.registers[vx as usize];
    State {
        pc,
        sound_timer,
        ..state
    }
}

const BYTES_PER_SPRITE: u16 = 5;

fn load_sprite(state: State, vx: u8, pc: u16) -> State {
    let sprite = u16::from(state.registers[vx as usize]);
    let i = BYTES_PER_SPRITE * sprite;
    State {
        pc,
        i,
        ..state
    }
}

fn handle_bcd_representation(state: State, memory: &mut Memory, pc: u16, vx: u8) -> State {
    let val = state.registers[vx as usize];
    let units = val % 10;
    let tens = (val - units) % 100;
    let hundreds = val - tens - units;
    let i = state.i;

    memory.set(i as usize, hundreds);
    memory.set((i + 1) as usize, tens);
    memory.set((i + 2) as usize, units);

    State {
        pc,
        ..state
    }
}

fn load_from_registers(state: State, memory: &mut Memory, vx: u8, pc: u16) -> State {
    let registers = state.registers;
    let i = state.i;
    for v in 0..=u16::from(vx) {
        let val = registers[v as usize];
        let addr = i+v;
        memory.set(addr as usize, val);
    }
    
    State {
        pc,
        ..state
    }
}

fn set_register(state: State, pc: u16, vx: u8, kk: u8) -> State {
    let mut registers = state.registers;
    registers[vx as usize] = kk;
    State {
        registers,
        pc,
        ..state
    }
}

fn set_registers(state: State, pc: u16, vx: u8, memory: &Memory) -> State {
    let mut registers = state.registers;
    let i = state.i;

    for v in 0..=u16::from(vx) {
        let addr = i+v;
        let val = memory.read(addr);
        registers[v as usize] = val;
    }

    State {
        registers,
        pc,
        ..state
    }
}

pub fn handle_load_operands(state: State, load_op: LoadOp, pc: u16, memory: &mut Memory, keycode: Option<u8>) -> State {
    match load_op {
        LoadOp::LD(vx, kk) => set_register(state, pc, vx, kk),
        LoadOp::LDV0XI(vx) => set_registers(state, pc, vx, memory),
        LoadOp::LDIV0X(vx) => load_from_registers(state, memory, vx, pc),
        LoadOp::LDB(vx) => handle_bcd_representation(state, memory, pc, vx),
        LoadOp::LDF(vx) => load_sprite(state, vx, pc),
        LoadOp::LDSTVX(vx) => set_sound_timer(state, vx, pc),
        LoadOp::LDKEY(vx) => handle_load_key(state, vx, pc, keycode, load_op),
        LoadOp::LDDTVX(vx) => set_delay_timer(state, vx, pc),
        LoadOp::LDI(kk) => set_i(state, pc, kk),
        LoadOp::LDVXDT(vx) => load_delay_timer(state, vx, pc),
        LoadOp::LDXY(vx, vy) => load_x_from_y(state, vx, vy, pc)
    }
}
