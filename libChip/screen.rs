use console::Term;
use console::Style;

pub struct Screen {
    display: Vec<bool>,
    width: i32,
    height: i32,
    first_run: bool
}

impl Screen {
    pub fn new(width: i32, height: i32) -> Screen {
        Screen {
            display: vec![false; (width * height) as usize],
            width: width,
            height: height,
            first_run: true
        }
    }

    pub fn draw(&mut self) {
        let term = Term::stdout();
        if !self.first_run {
            term.clear_last_lines(self.height as usize).expect("Should have worked");
        }
        self.first_run = false;

        let on_style = Style::new().bg(console::Color::Blue);
        let off_style = Style::new().bg(console::Color::Yellow);

        for w in 0 .. self.width {
            for h in 0 .. self.height {
                let idx = ((self.width * w) + h) as usize;
                let flag = self.display[idx];
                if flag {
                    print!("{}", on_style.apply_to("  "));
                } else {
                    print!("{}", off_style.apply_to("  "));
                }
            }
            println!();
        }
    }

    pub fn on(&mut self) {
          for w in 0 .. self.width {
            for h in 0 .. self.height {
                let idx = ((self.width * w) + h) as usize;
                self.display[idx] = true;
            }
        }
    }

        pub fn off(&mut self) {
          for w in 0 .. self.width {
            for h in 0 .. self.height {
                let idx = ((self.width * w) + h) as usize;
                self.display[idx] = false;
            }
        }
    }
}