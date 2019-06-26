extern crate lib_chip;
use lib_chip::screen::{ScreenParams, Screen, ConsoleBuilder};

use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
    const width:i32 = 20i32;
    const height:i32 = 20i32;
    
    let mut screen = ScreenParams::new()
        .with_height(height)
        .with_width(width)
        .for_console()
        .build();

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
