use super::registers::Registers;
use super::screen::Screen;
use super::memory::{Memory, load_text};
use super::rom::load_rom;

use std::boxed::Box;

pub struct Chip8 {
    screen: Box<Screen>,
    stack: [u16; 16],
    memory: Memory,
    registers: Registers,
    running: bool
}

enum OpCode {
    Unknown
}

impl Chip8 {
    pub fn new(screen: Box<Screen>, file: &str) -> Chip8 {
        println!("Creating emulator");

        let text = load_text();
        let rom = load_rom(file);

        let mut memory = Memory::new();
        memory.set(0x0, text);
        memory.set(0x512, rom);

        Chip8 {
            screen: screen,
            stack: [0; 16],
            memory: memory,
            registers: Registers::new(),
            running: false
        }
    }

    fn get_opcode(&self) -> Option<OpCode> {
        None
    }

    pub fn run(&mut self) {
        self.running = true;
        while self.running {
            let opcode = match self.get_opcode() {
                None => panic!("No opcode returned!"),
                Some(opcode) => panic!("shouldn't get here")
            };
        }
    }
}
