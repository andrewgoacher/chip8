use super::memory::Memory;
use super::opcodes::{parse_opcode, LoadOpCode, OpCode};
use super::rom::Rom;
use super::screen::Screen;

use std::boxed::Box;

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
                self.memory.set(0x200, rom.read_all());
            }
        }

        self.running = true;
        while self.running {
            match self.get_opcode() {
                OpCode::Unknown(c) => panic!("Unknown opcode: {:04X}", c),
                OpCode::ClearScreen => {
                    self.screen.clear();
                    self.pc += 2;
                }
                OpCode::Return => {
                    self.pc = self.stack[self.stack_pointer as usize];
                    self.stack_pointer -= 1;
                }
                OpCode::Load(l) => {
                    self.pc += 2;
                    self.registers[l.register as usize] = l.value;
                }
            };
        }
    }
}
