use console::Style;
use console::Term;

pub trait Screen {
    fn draw(&mut self);
    fn on(&mut self);
    fn off(&mut self);
    fn set_pixel(&mut self, x: i32, y: i32, on: bool);
}

pub struct ConsoleScreen {
    display: Vec<bool>,
    width: i32,
    height: i32,
    first_run: bool,
}

impl ConsoleScreen {
    pub fn new(width: i32, height: i32) -> ConsoleScreen {
        ConsoleScreen {
            display: vec![false; (width * height) as usize],
            width: width,
            height: height,
            first_run: true,
        }
    }
}

impl Screen for ConsoleScreen {
    fn draw(&mut self) {
        let term = Term::buffered_stdout();
        if !self.first_run {
            term.move_cursor_up(self.height as usize)
                .expect("Some error here");
            // term.clear_screen().expect("clear screen");
            // term.flush().expect("flushy");
        }
        self.first_run = false;

        let on_style = Style::new().bg(console::Color::Blue);
        let off_style = Style::new().bg(console::Color::Yellow);
        let mut strings: Vec<String> = Vec::new();

        for w in 0..self.width {
            for h in 0..self.height {
                let idx = ((self.width * w) + h) as usize;
                let flag = self.display[idx];
                if flag {
                    strings.push(format!("{}", on_style.apply_to("  ")));
                } else {
                    strings.push(format!("{}", off_style.apply_to("  ")));
                }
            }
            term.write_line(&strings.join(""))
                .expect("Error expectedly errored");
            strings.clear();
            term.flush().expect("FLUHSY");
        }
    }

    fn on(&mut self) {
        for w in 0..self.width {
            for h in 0..self.height {
                let idx = ((self.width * w) + h) as usize;
                self.display[idx] = true;
            }
        }
    }

    fn off(&mut self) {
        for w in 0..self.width {
            for h in 0..self.height {
                let idx = ((self.width * w) + h) as usize;
                self.display[idx] = false;
            }
        }
    }

    fn set_pixel(&mut self, x: i32, y: i32, on: bool) {
        let idx = ((self.width * y) + x) as usize;
        self.display[idx] = on;
    }
}
