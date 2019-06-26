extern crate lib_chip;
use lib_chip::screen::Screen;

use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
    let mut screen = Screen::new(20, 20);
    let mut on = true;

    loop {
        if on {
            screen.off();
            on = false;
        } else {
            screen.on();
            on = true;
        }
        screen.draw();
        thread::sleep(Duration::from_millis(2000));
    }
}
