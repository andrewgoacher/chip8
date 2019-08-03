use super::memory::Memory;
use super::opcode::{parse_opcode, AddOp, JumpOp, LoadOp, OpCode, ShiftOp, SkipOp};
use rand::Rng;
use std::boxed::Box;
use std::{thread, time};

use super::keyboard::get_unmapped_key;

use std::fmt::{self, Formatter, Display};

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
    pub opcode: Option<OpCode>
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "state: ({:?})", self.last_opcode);
        match self.opcode {
            None => (),
            Some(x) => {writeln!(f, "stored: {:?}", x);()}
        };
        writeln!(f, "delay: {}, sound {}", self.delay_timer, self.sound_timer);
        writeln!(f, "pc: {} | stack pointer: {} | interrupt: {}", self.pc,
            self.stack_pointer, self.i);
        writeln!(f, "stack: 0x{:04X}", self.stack[self.stack_pointer as usize]);
        writeln!(f, "draw: {} | run: {} | clear: {} ", self.draw_flag, 
            self.run_flag, self.clear_flag)
    }
}

impl State {
    pub fn new() -> State {
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
            opcode: None
        }
    }

    fn get_opcode(&self, memory: &Memory) -> OpCode {
        let pc = self.pc;
        let high = memory.read(pc);
        let low = memory.read(pc + 1);

        parse_opcode(high, low)
    }

    pub fn step(&self, memory: &mut Memory, keycode: Option<u8>, 
    screen: &mut Vec<(i32,i32,u8)>) -> State {
        let mut running = self.run_flag;
        let mut rng = rand::thread_rng();
        let now = time::Instant::now();
        let elapsed = now.elapsed().as_nanos();

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
            None => self.get_opcode(memory),
            Some(code) => code
        };

        // println!("next state: {:?}", opcode);

        match opcode {
            OpCode::Unknown(c) => panic!("Unknown opcode: {:04X}", c),
            OpCode::CLS => {
                clear_flag = true;
                pc += 2;
                draw_flag = true;
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
                    println!("press any key");
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
                    let sprite = registers[vx as usize];
                    pc += 2;

                    let addr = match sprite {
                        0x0 => 0x0,
                        0x1 => 0x5,
                        0x2 => 0xA,
                        0x3 => 0xF,
                        0x4 => 0x14,
                        0x5 => 0x19,
                        0x6 => 0x1E,
                        0x7 => 0x23,
                        0x8 => 0x28,
                        0x9 => 0x2D,
                        0xA => 0x32,
                        0xB => 0x37,
                        0xC => 0x3C,
                        0xD => 0x41,
                        0xE => 0x46,
                        0xF => 0x4B,
                        _ => panic!("Unknown sprite value"),
                    };
                    i = addr;
                }
                    LoadOp::LDB(vx) => {
                        let val = registers[vx as usize];
                        let h = val - (val % 100);
                        let tmp_t = val - h;
                        let t = tmp_t - (tmp_t % 10);
                        let u = val - h - t;

                        memory.set(i as usize, h);
                        memory.set((i + 1) as usize, t);
                        memory.set((i + 2) as usize, u);

                        pc += 2;
                    }
                    LoadOp::LDIV0X(vx) => {
                        for v in 0..(vx + 1) {
                            let val = registers[v as usize];
                            let addr = i + v as u16;

                            memory.set(addr as usize, val);
                            pc += 2;
                        }
                    }
                    LoadOp::LDV0XI(vx) => {
                        for v in 0..(vx + 1) {
                            let addr = i + v as u16;
                            let val = memory.read(addr);

                            registers[v as usize] = val;
                            pc += 2;
                        }
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
                        match get_unmapped_key(Some(value)) {
                            None => panic!("Weird key on skip"),
                            Some(key) => println!("press {}", key)
                        };
                        println!("press: {:02X}", value);
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
                            match get_unmapped_key(Some(value)) {
                            None => panic!("Weird key on skip"),
                            Some(key) => println!("press any except {}", key)
                        };
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
                        let x = registers[vx as usize] as u16;
                        let y = registers[vy as usize] as u16;

                        let result = x + y;
                        let carry = if result > 255 { 1 } else { 0 };
                        let low = result & 0x00FF;
                        registers[0xF] = carry;
                        registers[vx as usize] = low as u8;
                        pc += 2;
                    }
                    AddOp::ADDI(vx) => {
                        let x = registers[vx as usize];
                        let new_i = i + x as u16;
                        i = new_i;
                        pc += 2;
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
                    let mut erased = false;
                    for d in 0..data {
                        let addr = i + d as u16;
                        let mem = memory.read(addr);

                        let y = vy as i32 + d as i32;
                        let e = false;
                        screen.push((vx as i32, y, mem));
                        draw_flag = true;
                        erased = erased | e;
                    }
                    registers[0xf] = if erased { 1 } else { 0 };
                    pc += 2;
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
            last_opcode: opcode
        }
    }
}
