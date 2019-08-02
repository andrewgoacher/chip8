use super::memory::Memory;
use super::opcode::{parse_opcode, AddOp, JumpOp, LoadOp, OpCode, ShiftOp, SkipOp};
use rand::Rng;
use std::boxed::Box;
use std::{thread, time};

pub struct State {
    stack: [u16; 16],
    registers: [u8; 16],
    delay_timer: u8,
    sound_timer: u8,
    pc: u16,
    stack_pointer: u16,
    i: u16,
    draw_flag: bool,
    run_flag: bool,
    clear_flag: bool
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
            draw_flag: false,
            run_flag: false,
            clear_flag: false
        }
    }

    fn get_opcode(&self, memory: &Memory) -> OpCode {
        let pc = self.pc;
        let high = memory.read(pc);
        let low = memory.read(pc + 1);

        parse_opcode(high, low)
    }

    // pub fn load_rom(&mut self, rom: Rom) {
    //     self.memory.reset();
    //     self.rom = Some(rom);
    // }

    pub fn run(&self) -> State {
        // match &self.rom {
        //     None => panic!("Cannot run if no rom"),
        //     Some(rom) => {
        //         self.memory.set_range(0x200, rom.read_all());
        //     }
        // }

        State {
            stack: self.stack,
            registers: self.registers,
            delay_timer: self.delay_timer,
            sound_timer: self.sound_timer,
            pc: self.pc,
            stack_pointer: self.stack_pointer,
            i: self.i,
            draw_flag: self.draw_flag,
            run_flag: true,
            clear_flag: self.clear_flag
        }
    }

    pub fn step(&self, memory: &mut Memory) -> State {

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


            if self.delay_timer > 0 {
                delay_timer -= 1;
                if delay_timer == 0 {
                    draw_flag = true;
                }
            }

            if self.sound_timer > 0 {
                sound_timer -= 1;
                if sound_timer == 0 {
                    // todo: do sound
                }
            }

            match self.get_opcode(memory) {
                OpCode::Unknown(c) => panic!("Unknown opcode: {:04X}", c),
                OpCode::CLS => {
                    clear_flag = true;
                    // self.screen.clear();
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
                        // todo: keypress
                        // let key_press = self.input.get_key_pressed();
                       // self.registers[vx as usize] = key_press;
                        pc += 2;
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
                    stack[stack_pointer as usize] = loc;
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
                        pc += 2;
                        let value = registers[vx as usize];
                        // todo: inpit
                        // if self.input.is_key_pressed(value) {
                        //     self.pc += 2;
                        // }
                    }
                    SkipOp::SKNP(vx) => {
                        pc += 2;
                        let value = registers[vx as usize];
                        // todo: input
                        // if !self.input.is_key_pressed(value) {
                        //     self.pc += 2;
                        // }
                    }
                },
                OpCode::ADD(op) => match op {
                    AddOp::ADD(vx, data) => {
                        let x = registers[vx as usize];
                        registers[vx as usize] = x + data;
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
                        // todo: screen
                        // self.screen.set_pixel(vx as i32, y, mem);
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
            clear_flag: clear_flag
        }
    }
}
