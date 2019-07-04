extern crate lib_chip;
use lib_chip::{
    screen::{builder::{ScreenParams}},
    chip_8::Chip8
};

fn main() {
    const WIDTH:i32 = 64i32;
    const HEIGHT:i32 = 32i32;

    let screen = ScreenParams::new()
        .with_height(HEIGHT)
        .with_width(WIDTH)
        .for_console()
        .build();
    
    let file = r"C:\Users\andre\Downloads\Cave.ch8";
    let mut chip8 = Chip8::new(Box::new(screen), file);
    chip8.run();
}
