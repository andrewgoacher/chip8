use super::screen::Screen;
use std::boxed::Box;

pub struct Chip8 {
    screen: Box<Screen>
}

impl Chip8 {
    pub fn new(screen: Box<Screen>) -> Chip8 {
        Chip8 {
            screen: screen
        }
    }

    pub fn run(&mut self) {
        loop {
            self.screen.draw();
        }
    }
}