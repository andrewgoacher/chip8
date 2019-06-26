mod console_screen;
use console_screen::ConsoleScreen;

pub trait Screen {
    fn draw(&mut self);
    fn on(&mut self);
    fn off(&mut self);
    fn set_pixel(&mut self, x: i32, y: i32, on: bool);
}

#[derive(Clone)]
pub struct ScreenParams {
    width: Option<i32>,
    height: Option<i32>,
}

pub struct ConsoleBuilder {
    params: ScreenParams
}

impl ConsoleBuilder {
    pub fn build(&self) -> ConsoleScreen {
        let params = self.params.clone();
        let width = params.width.expect("Width is missing");
        let height = params.height.expect("Height is missing");
        ConsoleScreen::new(width, height)
    }
}

impl ScreenParams {
    pub fn new() -> Self {
        ScreenParams {
            width: None,
            height: None
        }
    }

    pub fn with_width(&self, width: i32) -> Self {
        ScreenParams {
            width: Some(width),
            height: self.height
        }
    }

    pub fn with_height(&self, height: i32) -> Self {
        ScreenParams {
            width: self.width,
            height: Some(height)
        }
    }

    pub fn with_dimensions(&self, dims: i32) -> Self {
        ScreenParams {
            width: Some(dims),
            height: Some(dims)
        }
    }

    pub fn for_console(&self) -> ConsoleBuilder {
        ConsoleBuilder {
            params: self.clone()
        }
    }
}

// pub struct ScreenBuilder {
//     screen_type: ScreenType,
//     width: i32,
//     height: i32
// }

// #[derive(Clone)]
// enum ScreenType {
//     Console
// }

// trait Builder<T> {
//     fn build(&self) -> T;
// }

// impl ScreenBuilder {
//     pub fn for_console() -> Builder<ConsoleScreen> {
//         ScreenBuilder {
//             screen_type: ScreenType::Console,
//             width: 0,
//             height: 0
//         }
//     }

//     pub fn with_size(&self, width: i32, height: i32) -> Self {
//         ScreenBuilder {
//             screen_type: self.screen_type.clone(),
//             width: width,
//             height: height
//         }
//     }

//     pub fn build(&self) -> Screen {
//         match self.screen_type {
//             ScreenType::Console -> ConsoleScreen::new(self.width, self.height)
//         }
//     }
// }
