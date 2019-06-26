mod console_screen;
pub mod builder;

pub trait Screen {
    fn draw(&mut self);
    fn on(&mut self);
    fn off(&mut self);
    fn set_pixel(&mut self, x: i32, y: i32, on: bool);
    fn clear(&self);
}