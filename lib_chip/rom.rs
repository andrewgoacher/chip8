use std::fs::File;
use std::io::prelude::*;

pub fn load_rom(file: &str) -> Vec<u8> {
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
    buffer
}