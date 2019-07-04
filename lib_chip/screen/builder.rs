use super::console_screen::ConsoleScreen;

#[derive(Clone)]
pub struct ScreenParams {
    width: Option<i32>,
    height: Option<i32>,
    clear_color: u32
}

pub struct ConsoleBuilder {
    params: ScreenParams
}

impl ConsoleBuilder {
    pub fn build(&self) -> ConsoleScreen {
        let params = self.params.clone();
        let width = params.width.expect("Width is missing");
        let height = params.height.expect("Height is missing");
        let clear_color = params.clear_color;
        ConsoleScreen::new(width, height, clear_color)
    }
}

impl ScreenParams {
    pub fn new() -> Self {
        ScreenParams {
            width: None,
            height: None,
            clear_color: 0
        }
    }

    pub fn with_width(&self, width: i32) -> Self {
        ScreenParams {
            width: Some(width),
            height: self.height,
            clear_color: self.clear_color
        }
    }

    pub fn with_height(&self, height: i32) -> Self {
        ScreenParams {
            width: self.width,
            height: Some(height),
            clear_color: self.clear_color
        }
    }

    pub fn with_dimensions(&self, dims: i32) -> Self {
        ScreenParams {
            width: Some(dims),
            height: Some(dims),
            clear_color: self.clear_color
        }
    }

    pub fn with_clear_color(&self, color: u32) -> Self {
        ScreenParams {
            width: self.width,
            height: self.height,
            clear_color: color
        }
    }

    pub fn for_console(&self) -> ConsoleBuilder {
        ConsoleBuilder {
            params: self.clone()
        }
    }
}