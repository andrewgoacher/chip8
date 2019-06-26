extern crate lib_chip;
use lib_chip::screen::{ConsoleScreen, Screen};

use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
    let mut screen = ConsoleScreen::new(20, 20);
    let mut on = true;
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    loop {
        for _ in 0..5 {
            if x == 20 {
                x = 0;
                y += 1;
            }

            if y == 20 {
                y = 0;
                x = 0;
                on = !on;
            }
            screen.set_pixel(x, y, on);
            x += 1;
        }
        thread::sleep(Duration::from_millis(10));
        screen.draw();
    }
}
