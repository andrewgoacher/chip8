mod console_screen;
pub mod builder;

pub trait Screen {
    fn draw(&mut self);
    fn set_pixel(&mut self, x: i32, y: i32, color: u32);
    fn clear(&mut self);
}