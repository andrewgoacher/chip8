use console::Style;
use console::Term;

use super::Screen;

fn sanitise_width(x: i32, width: i32) -> i32 {
    x % width
}

fn sanitise_height(y: i32, height: i32) -> i32 {
    y % height
}

pub struct ConsoleScreen {
    display: Vec<bool>,
    width: i32,
    height: i32,
    first_run: bool,
    terminal: console::Term,
    off: console::Color,
    on: console::Color,
}
impl ConsoleScreen {
    pub fn new(width: i32, height: i32) -> Self {
        ConsoleScreen {
            display: vec![false; (width * height) as usize],
            width: width,
            height: height,
            first_run: true,
            terminal: Term::buffered_stdout(),
            off: console::Color::Black,
            on: console::Color::White,
        }
    }
}
impl Screen for ConsoleScreen {
    fn clear(&mut self) {
        self.terminal
            .clear_screen()
            .expect("terminal failed to clear");
        self.terminal.flush().expect("Error flushing after clear");
        self.display = vec![false; (self.width * self.height) as usize];
    }

    fn draw(&mut self) {
        if !self.first_run {
            self.terminal
                .move_cursor_up(self.height as usize)
                .expect("Failed to move the cursor");
        }
        self.first_run = false;

        let on_style = Style::new().bg(self.on);
        let off_style = Style::new().bg(self.off);
        let mut strings: Vec<String> = Vec::new();

        for h in 0..self.height {
            for w in 0..self.width {
                let idx = ((self.width * h) + w) as usize;
                let flag = self.display[idx];
                if flag {
                    strings.push(format!("{}", on_style.apply_to("  ")));
                } else {
                    strings.push(format!("{}", off_style.apply_to("  ")));
                }
            }
            self.terminal
                .write_line(&strings.join(""))
                .expect("Failed to write output");
            strings.clear();
            self.terminal.flush().expect("failed to flush terminal");
        }
    }

    fn set_pixel(&mut self, start_x: i32, start_y: i32, pixel: u8) -> bool {
        let mut erased = false;
        let matchers: Vec<u8> = vec![0x80, 0x40, 0x20, 0x10, 0x08, 0x04, 0x02, 0x01];
        for i in 0..8 {
            let w = sanitise_width(start_x + i, self.width);
            let h = sanitise_height(start_y, self.height);

            let idx = ((self.width * h) + w) as usize;
            let old_pixel = self.display[idx];
            let fragment = (pixel & (matchers[i as usize] >> (7 - i))) == 1;
            let new_pixel: bool = old_pixel ^ fragment;
            if new_pixel == false && old_pixel == true {
                erased = true;
            }
            self.display[idx] = new_pixel;
        }
        erased
    }
}
