use super::input::{ConsoleInput, Input};
use super::memory::Memory;
use super::opcodes::{parse_opcode, AddOp, JumpOp, LoadOp, OpCode, ShiftOp, SkipOp};
use super::rom::Rom;
use super::screen::Screen;

use rand::Rng;
use std::boxed::Box;
use std::{thread, time};

pub struct Chip8 {
    screen: Box<Screen>,
    stack: [u16; 16],
    memory: Memory,
    running: bool,
    rom: Option<Rom>,
    registers: [u8; 16],
    delay_timer: u8,
    sound_timer: u8,
    pc: u16,
    stack_pointer: u16,
    i: u16,
    input: ConsoleInput,
    timer_delay: u128,
    cpu_delay: u128,
    draw_flag: bool,
}

impl Chip8 {
    pub fn new(screen: Box<Screen>) -> Chip8 {
        Chip8 {
            screen: screen,
            stack: [0; 16],
            memory: Memory::new(),
            running: false,
            rom: None,
            registers: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
            pc: 0x200,
            stack_pointer: 0,
            i: 0,
            input: ConsoleInput::new(),
            timer_delay: 0,
            cpu_delay: 0,
            draw_flag: false,
        }
    }

    fn get_opcode(&self) -> OpCode {
        let pc = self.pc;
        let high = self.memory.read(pc);
        let low = self.memory.read(pc + 1);

        parse_opcode(high, low)
    }

    pub fn load_rom(&mut self, rom: Rom) {
        self.memory.reset();
        self.rom = Some(rom);
    }

    pub fn run(&mut self) {
        match &self.rom {
            None => panic!("Cannot run if no rom"),
            Some(rom) => {
                self.memory.set_range(0x200, rom.read_all());
            }
        }

        let mut rng = rand::thread_rng();

        self.running = true;
        while self.running {
            let now = time::Instant::now();
            let elapsed = now.elapsed().as_nanos();

            self.timer_delay += elapsed;
            if self.timer_delay >= 20000 {
                self.timer_delay = 0;
                if self.delay_timer > 0 {
                    self.delay_timer -= 1;
                    if self.delay_timer == 0 {
                        self.draw_flag = true;
                    }
                }

                if self.sound_timer > 0 {
                    self.sound_timer -= 1;
                    if self.sound_timer == 0 {
                        // todo: do sound
                    }
                }
            }
            self.cpu_delay += elapsed;

            if self.cpu_delay >= 1000 {
                self.cpu_delay = 0;

                match self.get_opcode() {
                    OpCode::Unknown(c) => panic!("Unknown opcode: {:04X}", c),
                    OpCode::CLS => {
                        self.screen.clear();
                        self.pc += 2;
                        self.draw_flag = true;
                    }
                    OpCode::RET => {
                        self.stack_pointer -= 1;
                        self.pc = self.stack[self.stack_pointer as usize];
                    }
                    OpCode::LD(op) => match op {
                        LoadOp::LD(vx, data) => {
                            self.registers[vx as usize] = data;
                            self.pc += 2;
                        }
                        LoadOp::LDI(data) => {
                            self.i = data;
                            self.pc += 2;
                        }
                        LoadOp::LDXY(vx, vy) => {
                            self.registers[vx as usize] = self.registers[vy as usize];
                            self.pc += 2;
                        }
                        LoadOp::LDVXDT(vx) => {
                            self.registers[vx as usize] = self.delay_timer;
                            self.pc += 2;
                        }
                        LoadOp::LDDTVX(vx) => {
                            self.delay_timer = self.registers[vx as usize];
                            self.pc += 2;
                        }
                        LoadOp::LDKEY(vx) => {
                            let key_press = self.input.get_key_pressed();
                            self.registers[vx as usize] = key_press;
                            self.pc += 2;
                        }
                        LoadOp::LDSTVX(vx) => {
                            self.sound_timer = self.registers[vx as usize];
                            self.pc += 2;
                        }
                        LoadOp::LDF(vx) => {
                            let sprite = self.registers[vx as usize];
                            self.pc += 2;

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
                            self.i = addr;
                        }
                        LoadOp::LDB(vx) => {
                            let val = self.registers[vx as usize];
                            let h = val - (val % 100);
                            let tmp_t = val - h;
                            let t = tmp_t - (tmp_t % 10);
                            let u = val - h - t;

                            self.memory.set(self.i as usize, h);
                            self.memory.set((self.i + 1) as usize, t);
                            self.memory.set((self.i + 2) as usize, u);

                            self.pc += 2;
                        }
                        LoadOp::LDIV0X(vx) => {
                            for v in 0..(vx + 1) {
                                let val = self.registers[v as usize];
                                let addr = self.i + v as u16;

                                self.memory.set(addr as usize, val);
                                self.pc += 2;
                            }
                        }
                        LoadOp::LDV0XI(vx) => {
                            for v in 0..(vx + 1) {
                                let addr = self.i + v as u16;
                                let val = self.memory.read(addr);

                                self.registers[v as usize] = val;
                                self.pc += 2;
                            }
                        }
                    },
                    OpCode::JP(jp) => match jp {
                        JumpOp::JP(loc) => {
                            self.pc = loc;
                        }
                        JumpOp::JPV0(loc) => {
                            let v0 = self.registers[0x0] as u16;
                            self.pc = loc + v0;
                        }
                    },
                    OpCode::CALL(loc) => {
                        self.stack[self.stack_pointer as usize] = loc;
                        self.stack_pointer += 1;
                        self.pc = loc;
                    }
                    OpCode::SKIP(op) => match op {
                        SkipOp::SE(vx, data) => {
                            let x = self.registers[vx as usize];
                            self.pc += 2;
                            if x == data {
                                self.pc += 2;
                            }
                        }
                        SkipOp::SNE(vx, data) => {
                            let x = self.registers[vx as usize];
                            self.pc += 2;
                            if x != data {
                                self.pc += 2;
                            }
                        }
                        SkipOp::SEXY(vx, vy) => {
                            let x = self.registers[vx as usize];
                            let y = self.registers[vy as usize];
                            self.pc += 2;
                            if x == y {
                                self.pc += 2;
                            }
                        }
                        SkipOp::SNEXY(vx, vy) => {
                            let x = self.registers[vx as usize];
                            let y = self.registers[vy as usize];
                            self.pc += 2;
                            if x != y {
                                self.pc += 2;
                            }
                        }
                        SkipOp::SKP(vx) => {
                            self.pc += 2;
                            let value = self.registers[vx as usize];
                            if self.input.is_key_pressed(value) {
                                self.pc += 2;
                            }
                        }
                        SkipOp::SKNP(vx) => {
                            self.pc += 2;
                            let value = self.registers[vx as usize];
                            if !self.input.is_key_pressed(value) {
                                self.pc += 2;
                            }
                        }
                    },
                    OpCode::ADD(op) => match op {
                        AddOp::ADD(vx, data) => {
                            let x = self.registers[vx as usize];
                            self.registers[vx as usize] = x + data;
                            self.pc += 2;
                        }
                        AddOp::ADDREG(vx, vy) => {
                            let x = self.registers[vx as usize] as u16;
                            let y = self.registers[vy as usize] as u16;

                            let result = x + y;
                            let carry = if result > 255 { 1 } else { 0 };
                            let low = result & 0x00FF;
                            self.registers[0xF] = carry;
                            self.registers[vx as usize] = low as u8;
                            self.pc += 2;
                        }
                        AddOp::ADDI(vx) => {
                            let x = self.registers[vx as usize];
                            let new_i = self.i + x as u16;
                            self.i = new_i;
                            self.pc += 2;
                        }
                    },
                    OpCode::SUB(vx, vy) => {
                        let x = self.registers[vx as usize];
                        let y = self.registers[vy as usize];

                        let new_x = x - y;
                        let not_borrow = if new_x > y { 1 } else { 0 };

                        self.registers[vx as usize] = new_x;
                        self.registers[0xF] = not_borrow;
                        self.pc += 2;
                    }
                    OpCode::SUBN(vx, vy) => {
                        let x = self.registers[vx as usize];
                        let y = self.registers[vy as usize];

                        let new_x = y - x;
                        let not_borrow = if new_x < y { 1 } else { 0 };

                        self.registers[vx as usize] = new_x;
                        self.registers[0xF] = not_borrow;
                        self.pc += 2;
                    }
                    OpCode::RND(vx, data) => {
                        let r: u8 = rng.gen();
                        let val = r & data;
                        self.registers[vx as usize] = val;
                        self.pc += 2;
                    }
                    OpCode::DRW(vx, vy, data) => {
                        let mut erased = false;
                        for i in 0..data {
                            let addr = self.i + i as u16;
                            let mem = self.memory.read(addr);

                            let e = self.screen.set_pixel(vx as i32, vy as i32, mem);
                            if i > 0 {
                                erased = erased | e;
                            }
                        }
                        self.registers[0xf] = if erased { 1 } else { 0 };

                        self.pc += 2;
                    }
                    OpCode::OR(vx, vy) => {
                        let x = self.registers[vx as usize];
                        let y = self.registers[vy as usize];

                        self.registers[vx as usize] = x | y;
                        self.pc += 2;
                    }
                    OpCode::AND(vx, vy) => {
                        let x = self.registers[vx as usize];
                        let y = self.registers[vy as usize];

                        self.registers[vx as usize] = x & y;
                        self.pc += 2;
                    }
                    OpCode::XOR(vx, vy) => {
                        let x = self.registers[vx as usize];
                        let y = self.registers[vy as usize];

                        self.registers[vx as usize] = x ^ y;
                        self.pc += 2;
                    }
                    OpCode::SHIFT(op) => match op {
                        ShiftOp::SHR(vx) => {
                            let x = self.registers[vx as usize];
                            self.registers[vx as usize] = x >> 1;
                            self.pc += 2;
                        }
                        ShiftOp::SHL(vx) => {
                            let x = self.registers[vx as usize];
                            self.registers[vx as usize] = x << 1;
                            self.pc += 2;
                        }
                    },
                };
            }

            if self.draw_flag {
                self.screen.draw();
                self.draw_flag = false;
            }
        }
    }
}
