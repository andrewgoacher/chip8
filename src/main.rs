extern crate lib_chip;
//use lib_chip::sdl_display::*;
// use lib_chip::{
//     // screen::{builder::{ScreenParams}},
//     chip_8::Chip8,
//     rom::Rom
// };


use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub fn main() -> Result<(), String> {
    const width: u32 = 640;
    const height: u32 = 320;

     let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem.window("Chip8 Emulator", width, height)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        let mut canvas = window.into_canvas()
            .accelerated()
            .present_vsync()
            .build().expect("Failed to build canvas");

        // let mut display = SdlDisplay::new(canvas, width, height);
        // let black: Vec<u8> = vec![0xF, width & height];
        // display.set_pixels(black);
          
        
            // for x in 0..width {
            //     for y in 0..height {
            //         let idx = ((width * height) + wwidth) as usize;
            //         buffer.push(0xF);
            //     }
            // }

      

        let mut event_pump = sdl_context.event_pump()?;

        // let mut keys: Vec<u8> = Vec::new();
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..}
                    | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => {}
                }
            }

            // display.draw();

            // The rest of the game loop goes here...
            // emulator.frame(self, keys);
        }
        
        Ok(())
}