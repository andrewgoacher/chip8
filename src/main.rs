extern crate lib_chip;
use lib_chip::screen::{Screen,builder::{ScreenParams}};

use std::thread;
use std::time::Duration;

fn main() {
    const WIDTH:i32 = 64i32;
    const HEIGHT:i32 = 32i32;

    let mut screen = ScreenParams::new()
        .with_height(HEIGHT)
        .with_width(WIDTH)
        .for_console()
        .build();

    let mut on = true;
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    loop {
        for _ in 0..5 {
            if x == WIDTH {
                x = 0;
                y += 1;
            }

            if y == HEIGHT {
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
