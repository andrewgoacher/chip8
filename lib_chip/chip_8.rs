use super::registers::Registers;
use super::screen::Screen;
use super::memory::{Memory, load_text};
use super::rom::Rom;

use std::boxed::Box;

pub struct Chip8 {
    screen: Box<Screen>,
    stack: [u16; 16],
    memory: Memory,
    registers: Registers,
    running: bool,
    rom: Rom
}

enum OpCode {
    Unknown(u16)
}

impl Chip8 {
    pub fn new(screen: Box<Screen>, rom: Rom) -> Chip8 {
        println!("Creating emulator");

        let text = load_text();

        let mut memory = Memory::new();
        memory.set(0x0, text);
        memory.set(0x200, rom.read_all());

        Chip8 {
            screen: screen,
            stack: [0; 16],
            memory: memory,
            registers: Registers::new(),
            running: false,
            rom: rom
        }
    }

    fn get_opcode(&mut self) -> OpCode {
        let pc = self.registers.pc;
        let code = self.memory.read(pc << 8) as u16 + self.memory.read(pc +1) as u16;
        OpCode::Unknown(code)
    }

    pub fn run(&mut self) {
        self.running = true;
        while self.running {
            let opcode = match self.get_opcode() {
                OpCode::Unknown(c) => panic!("Unknown opcode: {:04X}", c)
            };
        }
    }
}
