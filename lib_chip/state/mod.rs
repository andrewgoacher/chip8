pub mod functions;
pub mod display;

use crate::memory::Memory;
use crate::opcode::{AddOp, JumpOp, LoadOp, OpCode, ShiftOp, SkipOp};
use functions::{get_opcode, draw_sprite};
use rand::Rng;

pub struct State {
    pub stack: [u16; 16],
    pub registers: [u8; 16],
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub pc: u16,
    pub stack_pointer: u16,
    pub i: u16,
    pub draw_flag: bool,
    pub run_flag: bool,
    pub clear_flag: bool,
    pub last_opcode: OpCode,
    pub opcode: Option<OpCode>,
    pub width: u32,
    pub height: u32
}

impl State {
    pub fn new(w: u32, h: u32) -> State {
        State {
            stack: [0; 16],
            registers: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
            pc: 0x200,
            stack_pointer: 0,
            i: 0,
            draw_flag: true,
            run_flag: true,
            clear_flag: true,
            last_opcode: OpCode::Unknown(0),
            opcode: None,
            width: w,
            height: h
        }
    }

    pub fn step(&self, memory: &mut Memory, keycode: Option<u8>, 
    screen: &mut Vec<u8>) -> State {
        let running = self.run_flag;
        let mut rng = rand::thread_rng();

        let mut delay_timer = self.delay_timer;
        let mut draw_flag = self.draw_flag;
        let mut sound_timer = self.sound_timer;
        let mut clear_flag = self.clear_flag;
        let mut pc = self.pc;
        let mut stack_pointer = self.stack_pointer;
        let mut stack = self.stack;
        let mut registers = self.registers;
        let mut i = self.i;

        let mut next_opcode: Option<OpCode> = self.opcode;

        if self.delay_timer > 0 {
            delay_timer -= 1;
            if delay_timer == 0 {
                //draw_flag = true;
            }
        }

        if self.sound_timer > 0 {
            sound_timer -= 1;
            if sound_timer == 0 {
                // todo: do sound
            }
        }

        let opcode = match next_opcode {
            None => get_opcode(self, memory),
            Some(code) => code
        };

        match opcode {
            OpCode::Unknown(c) => panic!("Unknown opcode: {:04X}", c),
            OpCode::CLS => {
                clear_flag = true;
                draw_flag = true;
                pc += 2;
            }
            OpCode::RET => {
                stack_pointer -= 1;
                pc = stack[stack_pointer as usize];
            }
            OpCode::LD(op) => match op {
                LoadOp::LD(vx, data) => {
                    registers[vx as usize] = data;
                    pc += 2;
                }
                LoadOp::LDI(data) => {
                    i = data;
                    pc += 2;
                }
                LoadOp::LDXY(vx, vy) => {
                    registers[vx as usize] = registers[vy as usize];
                    pc += 2;
                }
                LoadOp::LDVXDT(vx) => {
                    registers[vx as usize] = delay_timer;
                    pc += 2;
                }
                LoadOp::LDDTVX(vx) => {
                    delay_timer = registers[vx as usize];
                    pc += 2;
                }
                LoadOp::LDKEY(vx) => {
                    match keycode {
                        None => {
                            next_opcode = Some(opcode);
                        },
                        Some(key_press) => {
                            next_opcode = None;
                            registers[vx as usize] = key_press;
                            pc += 2;
                        }
                    }
                }
                LoadOp::LDSTVX(vx) => {
                    sound_timer = registers[vx as usize];
                    pc += 2;
                }
                LoadOp::LDF(vx) => {
                    let sprite = registers[vx as usize] as u16;
                    pc += 2;
                    i = 5 * sprite;
                }
                    LoadOp::LDB(vx) => {
                        let val = registers[vx as usize] as u32;
                        let (ha, _) = val.overflowing_rem(1000); 
                        let (h, _) = ha.overflowing_div(100);

                        let (ta, _) = val.overflowing_rem(100);
                        let (t, _) = ta.overflowing_div(10);

                        let (u, _) = val.overflowing_rem(10);

                        memory.set(i as usize, h as u8);
                        memory.set((i + 1) as usize, t as u8);
                        memory.set((i + 2) as usize, u as u8);

                        pc += 2;
                    }
                    LoadOp::LDIV0X(vx) => {
                        for v in 0..(vx + 1) {
                            let val = registers[v as usize];
                            let addr = i + v as u16;

                            memory.set(addr as usize, val);
                        }
                        i += vx as u16 + 1;
                        pc += 2;
                    }
                    LoadOp::LDV0XI(vx) => {
                        for v in 0..(vx + 1) {
                            let addr = i + v as u16;
                            let val = memory.read(addr);

                            registers[v as usize] = val;
                        }
                        i += vx as u16 + 1;
                        pc += 2;
                    }
                },
                OpCode::JP(jp) => match jp {
                    JumpOp::JP(loc) => {
                        pc = loc;
                    }
                    JumpOp::JPV0(loc) => {
                        let v0 = registers[0x0] as u16;
                        pc = loc + v0;
                    }
                },
                OpCode::CALL(loc) => {
                    stack[stack_pointer as usize] = pc+2;
                    stack_pointer += 1;
                    pc = loc;
                }
                OpCode::SKIP(op) => match op {
                    SkipOp::SE(vx, data) => {
                        let x = registers[vx as usize];
                        pc += 2;
                        if x == data {
                            pc += 2;
                        }
                    }
                    SkipOp::SNE(vx, data) => {
                        let x = registers[vx as usize];
                        pc += 2;
                        if x != data {
                            pc += 2;
                        }
                    }
                    SkipOp::SEXY(vx, vy) => {
                        let x = registers[vx as usize];
                        let y = registers[vy as usize];
                        pc += 2;
                        if x == y {
                            pc += 2;
                        }
                    }
                    SkipOp::SNEXY(vx, vy) => {
                        let x = registers[vx as usize];
                        let y = registers[vy as usize];
                        pc += 2;
                        if x != y {
                            pc += 2;
                        }
                    }
                    SkipOp::SKP(vx) => {
                        let value = registers[vx as usize];
                        match keycode {
                            None => {
                                next_opcode = Some(opcode);
                            },
                            Some(key_code) => {
                                next_opcode = None;
                                pc +=2;
                                if value == key_code {
                                    pc +=2;
                                }
                            }
                        }
                    }
                    SkipOp::SKNP(vx) => {
                        let value = registers[vx as usize];
                        match keycode {
                            None => {
                                next_opcode= Some(opcode);
                            },
                            Some(key_press) => {
                                next_opcode = None;
                                pc += 2;
                                if !value == key_press {
                                    pc += 2;
                                }
                            }
                        }
                    }
                },
                OpCode::ADD(op) => match op {
                    AddOp::ADD(vx, data) => {
                        let x = registers[vx as usize];
                        let (val, _) = x.overflowing_add(data);
                        registers[vx as usize] = val;
                        pc += 2;
                    }
                    AddOp::ADDREG(vx, vy) => {
                        let x = registers[vx as usize];
                        let y = registers[vy as usize];

                        let (result, carry) = x.overflowing_add(y);
                        registers[0xF] = if carry { 1 } else { 0 };
                        registers[vx as usize] = result;
                        pc += 2;
                    }
                    AddOp::ADDI(vx) => {
                        let x = registers[vx as usize];
                        let new_i = i + x as u16;
                        i = new_i;
                        pc += 2;
                        registers[0xF] = if new_i > 0xFFF { 1 } else { 0 };
                    }
                },
                OpCode::SUB(vx, vy) => {
                    let x = registers[vx as usize];
                    let y = registers[vy as usize];

                    let new_x = x - y;
                    let not_borrow = if new_x > y { 1 } else { 0 };

                    registers[vx as usize] = new_x;
                    registers[0xF] = not_borrow;
                    pc += 2;
                }
                OpCode::SUBN(vx, vy) => {
                    let x = registers[vx as usize];
                    let y = registers[vy as usize];

                    let new_x = y - x;
                    let not_borrow = if new_x < y { 1 } else { 0 };

                    registers[vx as usize] = new_x;
                    registers[0xF] = not_borrow;
                    pc += 2;
                }
                OpCode::RND(vx, data) => {
                    let r: u8 = rng.gen();
                    let val = r & data;
                    registers[vx as usize] = val;
                    pc += 2;
                }
                OpCode::DRW(vx, vy, data) => {
                    pc += 2;
                    let erased = draw_sprite(self, memory, screen,
                     vx, vy, data);
                    registers[0xf] = if erased { 1 } else { 0 };
                    draw_flag = true;
                }
                OpCode::OR(vx, vy) => {
                    let x = registers[vx as usize];
                    let y = registers[vy as usize];

                    registers[vx as usize] = x | y;
                    pc += 2;
                }
                OpCode::AND(vx, vy) => {
                    let x = registers[vx as usize];
                    let y = registers[vy as usize];

                    registers[vx as usize] = x & y;
                    pc += 2;
                }
                OpCode::XOR(vx, vy) => {
                    let x = registers[vx as usize];
                    let y = registers[vy as usize];

                    registers[vx as usize] = x ^ y;
                    pc += 2;
                }
                OpCode::SHIFT(op) => match op {
                    ShiftOp::SHR(vx) => {
                        let x = registers[vx as usize];
                        registers[vx as usize] = x >> 1;
                        pc += 2;
                    }
                    ShiftOp::SHL(vx) => {
                        let x = registers[vx as usize];
                        registers[vx as usize] = x << 1;
                        pc += 2;
                    }
                },
        };

        State {
            stack: stack,
            registers: registers,
            delay_timer: delay_timer,
            sound_timer: sound_timer,
            pc: pc,
            stack_pointer: stack_pointer,
            i: i,
            draw_flag: draw_flag,
            run_flag: running,
            clear_flag: clear_flag,
            opcode: next_opcode,
            last_opcode: opcode,
            width: self.width,
            height: self.height
        }
    }
}
