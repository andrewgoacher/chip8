// use super::console_screen::ConsoleScreen;

// #[derive(Clone)]
// pub struct ScreenParams {
//     width: Option<i32>,
//     height: Option<i32>
// }

// pub struct ConsoleBuilder {
//     params: ScreenParams
// }

// impl ConsoleBuilder {
//     pub fn build(&self) -> ConsoleScreen {
//         let params = self.params.clone();
//         let width = params.width.expect("Width is missing");
//         let height = params.height.expect("Height is missing");
//         ConsoleScreen::new(width, height)
//     }
// }

// impl ScreenParams {
//     pub fn new() -> Self {
//         ScreenParams {
//             width: None,
//             height: None
//         }
//     }

//     pub fn with_width(&self, width: i32) -> Self {
//         ScreenParams {
//             width: Some(width),
//             height: self.height
//         }
//     }

//     pub fn with_height(&self, height: i32) -> Self {
//         ScreenParams {
//             width: self.width,
//             height: Some(height)
//         }
//     }

//     pub fn with_dimensions(&self, dims: i32) -> Self {
//         ScreenParams {
//             width: Some(dims),
//             height: Some(dims)
//         }
//     }

//     pub fn for_console(&self) -> ConsoleBuilder {
//         ConsoleBuilder {
//             params: self.clone()
//         }
//     }
// }