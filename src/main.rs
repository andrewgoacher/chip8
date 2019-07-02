extern crate lib_chip;
use lib_chip::{
    screen::{Screen,builder::{ScreenParams}},
    chip_8::Chip8
};

use std::thread;
use std::time::Duration;

fn start_banner(screen: &mut Screen) {
    screen.off();
    
    screen.set_pixel(16, 9, true);
    screen.set_pixel(17, 9, true);
    screen.set_pixel(18, 9, true);
    screen.set_pixel(20, 9, true);
    screen.set_pixel(22, 9, true);
    screen.set_pixel(24, 9, true);
    screen.set_pixel(26, 9, true);
    screen.set_pixel(27, 9, true);
    screen.set_pixel(28, 9, true);

    screen.set_pixel(16, 10, true);
    screen.set_pixel(20, 10, true);
    screen.set_pixel(21, 10, true);
    screen.set_pixel(22, 10, true);
    screen.set_pixel(24, 10, true);
    screen.set_pixel(26, 10, true);
    screen.set_pixel(27, 10, true);
    screen.set_pixel(28, 10, true);

    screen.set_pixel(16, 11, true);
    screen.set_pixel(20, 11, true);
    screen.set_pixel(22, 11, true);
    screen.set_pixel(24, 11, true);
    screen.set_pixel(26, 11, true);

    screen.set_pixel(16, 12, true);
    screen.set_pixel(17, 12, true);
    screen.set_pixel(18, 12, true);
    screen.set_pixel(20, 12, true);
    screen.set_pixel(22, 12, true);
    screen.set_pixel(24, 12, true);
    screen.set_pixel(26, 12, true);

    screen.set_pixel(21, 15, true);
    screen.set_pixel(22, 15, true);
    screen.set_pixel(23, 15, true);

    screen.set_pixel(21, 17, true);
    screen.set_pixel(22, 17, true);
    screen.set_pixel(23, 17, true);

    screen.set_pixel(21, 19, true);
    screen.set_pixel(22, 19, true);
    screen.set_pixel(23, 19, true);

    screen.set_pixel(21, 16, true);
    screen.set_pixel(23, 16, true);
    screen.set_pixel(21, 18, true);
    screen.set_pixel(23, 18, true);
}

fn tick(screen: &mut Screen, width: i32, height: i32) {
    let mut on = true;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut total_time: i32 = 0;

    while total_time < 1500 {
        for _ in 0..5 {
            if x == width {
                x = 0;
                y += 1;
            }

            if y == height {
                y = 0;
                x = 0;
                on = !on;
            }
            screen.set_pixel(x, y, on);
            x += 1;
        }
        thread::sleep(Duration::from_millis(10));
        screen.draw();
        total_time += 10;
    }
    screen.clear();
}

fn main() {
    const WIDTH:i32 = 64i32;
    const HEIGHT:i32 = 32i32;

    let mut screen = ScreenParams::new()
        .with_height(HEIGHT)
        .with_width(WIDTH)
        .for_console()
        .build();
        
    let mut chip8 = Chip8::new(Box::new(screen));
    chip8.run();
}
