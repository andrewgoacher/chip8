use std::io::{Read, Stdin};

pub trait Input {
    fn is_key_pressed(&self, key: u8) -> bool;
    fn get_key_pressed(&self) -> u8;
}

pub struct ConsoleInput {
    input: Stdin
}

impl ConsoleInput {
    pub fn new() -> Self {
        ConsoleInput {
            input: std::io::stdin()
        }
    }
}

fn read_key(input: &Stdin) -> u8 {
    let mut buffer = String::new();
    let mut handle = input.lock();

    match handle.read_to_string(&mut buffer) {
        Ok(_) => {
          match buffer.as_str() {
              "1" => 0x1,
              "2" => 0x2,
              "3" => 0x3,
              "4" => 0x4,
              "5" => 0x5,
              "6" => 0x6,
              "7" => 0x7,
              "8" => 0x8,
              "9" => 0x9,
              "0" => 0x0,
              "A" | "a" => 0xA,
              "B" | "b" => 0xB,
              "C" | "c" => 0xC,
              "D" | "d" => 0xD,
              "E" | "e" => 0xE,
              "F" | "f" => 0xF,
              _ => panic!("Unrecognised input") 
          }  
        },
        Err(_) => panic!("Failed to read input")
    }
}

impl Input for ConsoleInput {
    fn is_key_pressed(&self, key: u8) -> bool {
        let k = read_key(&self.input);
        k == key
    }

    fn get_key_pressed(&self) -> u8 {
        read_key(&self.input)
    }
}