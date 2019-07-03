use super::registers::Registers;
use super::screen::Screen;

use std::boxed::Box;
use std::fs::File;
use std::io;
use std::io::prelude::*;

pub struct Chip8 {
    screen: Box<Screen>,
    stack: [u16; 16],
    memory: Vec<u8>,
    registers: Registers,
}

fn load_text_into_mem() -> [u8; 80] {
    println!("Creating text region");
    let mut mem: [u8; 80] = [0; 80];
    // 0
    mem[0] = 0xF0;
    mem[1] = 0x90;
    mem[2] = 0x90;
    mem[3] = 0x90;
    mem[4] = 0xF0;

    // 1
    mem[5] = 0x20;
    mem[6] = 0x60;
    mem[7] = 0x20;
    mem[8] = 0x20;
    mem[9] = 0x70;

    // 2
    mem[10] = 0xF0;
    mem[11] = 0x10;
    mem[12] = 0xF0;
    mem[13] = 0x80;
    mem[14] = 0xF0;

    // 3
    mem[15] = 0xF0;
    mem[16] = 0x10;
    mem[17] = 0xF0;
    mem[18] = 0x10;
    mem[19] = 0xF0;

    // 4
    mem[20] = 0x90;
    mem[21] = 0x90;
    mem[22] = 0xF0;
    mem[23] = 0x10;
    mem[24] = 0x10;

    // 5
    mem[25] = 0xF0;
    mem[26] = 0x80;
    mem[27] = 0xF0;
    mem[28] = 0x10;
    mem[29] = 0x10;

    // 6
    mem[30] = 0xF0;
    mem[31] = 0x80;
    mem[32] = 0xF0;
    mem[33] = 0x90;
    mem[34] = 0xF0;

    // 7
    mem[35] = 0xF0;
    mem[36] = 0x10;
    mem[37] = 0x20;
    mem[38] = 0x40;
    mem[39] = 0x40;

    // 8
    mem[40] = 0xF0;
    mem[41] = 0x90;
    mem[42] = 0xF0;
    mem[43] = 0x90;
    mem[44] = 0xF0;

    // 9
    mem[45] = 0xF0;
    mem[46] = 0x90;
    mem[47] = 0xF0;
    mem[48] = 0x10;
    mem[49] = 0xF0;

    // A
    mem[50] = 0xF0;
    mem[51] = 0x90;
    mem[52] = 0xF0;
    mem[53] = 0x90;
    mem[54] = 0x90;

    // B
    mem[55] = 0xE0;
    mem[56] = 0x90;
    mem[57] = 0xE0;
    mem[58] = 0x90;
    mem[59] = 0xE0;

    // C
    mem[60] = 0xF0;
    mem[61] = 0x80;
    mem[62] = 0x80;
    mem[63] = 0x80;
    mem[64] = 0xF0;

    // D
    mem[65] = 0xE0;
    mem[66] = 0x90;
    mem[67] = 0x90;
    mem[68] = 0x90;
    mem[69] = 0xE0;

    // E
    mem[70] = 0xF0;
    mem[71] = 0x80;
    mem[72] = 0xF0;
    mem[73] = 0x80;
    mem[74] = 0xF0;

    // F
    mem[75] = 0xF0;
    mem[76] = 0x80;
    mem[77] = 0xF0;
    mem[78] = 0x80;
    mem[79] = 0x80;

    mem
}

fn load_rom(file: &str, memory: &[u8]) -> Vec<u8> {
    println!("Loading rom");
    let mut f = match File::open(file) {
        Ok(file) => file,
        Err(e) => match e {
            NotFound => panic!("File not found!"),
            _ => panic!("Not sure what's going on here!"),
        },
    };

    let mut buffer = Vec::new();
    match f.read_to_end(&mut buffer) {
        Err(e) => panic!("Something went wrong"),
        _ => (),
    };

    let mut new_memory: Vec<u8> = vec![0; 1024 * 4];
    // clone the text
    new_memory[0..79].clone_from_slice(&memory[0..79]);

    for i in 0..buffer.len() {
        new_memory[i + 512] = buffer[i];
    }
    println!("Rom loaded into memory");
    new_memory
}

impl Chip8 {
    pub fn new(screen: Box<Screen>, file: &str) -> Chip8 {
        println!("Creating emulator");

        let mut mem = load_text_into_mem();
        let memory = load_rom(file, &mem);

        Chip8 {
            screen: screen,
            stack: [0; 16],
            memory: memory,
            registers: Registers::new(),
        }
    }

    pub fn run(&mut self) {
        loop {}
    }
}
