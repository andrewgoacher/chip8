use console::Style;
use console::Term;

use super::Screen;

pub struct ConsoleScreen {
    display: Vec<u32>,
    width: i32,
    height: i32,
    first_run: bool,
    terminal: console::Term,
    clear_color: u32,
}
impl ConsoleScreen {
        pub fn new(width: i32, height: i32, clear_color: u32) -> Self {
        ConsoleScreen {
            display: vec![0; (width * height) as usize],
            width: width,
            height: height,
            first_run: true,
            terminal: Term::buffered_stdout(),
            clear_color: clear_color
        }
    }
}
impl Screen for ConsoleScreen {
    fn clear(&mut self) {
        self.terminal
            .clear_screen()
            .expect("terminal failed to clear");
        self.terminal.flush().expect("Error flushing after clear");
        self.display = vec![self.clear_color; (self.width * self.height) as usize];
    }

    fn draw(&mut self) {
        if !self.first_run {
            self.terminal
                .move_cursor_up(self.height as usize)
                .expect("Failed to move the cursor");
        }
        self.first_run = false;

        let on_style = Style::new().bg(console::Color::Blue);
        let off_style = Style::new().bg(console::Color::Yellow);
        let mut strings: Vec<String> = Vec::new();

        for h in 0..self.height {
            for w in 0..self.width {
                let idx = ((self.width * h) + w) as usize;
                let flag = self.display[idx] != 0;
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

    fn set_pixel(&mut self, x: i32, y: i32, color: u32) {
        let idx = ((self.width * y) + x) as usize;
        self.display[idx] = color;
    }
}
