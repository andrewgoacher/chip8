// use super::Screen;
// use super::chip_8::Chip8;

// use sdl2::pixels::PixelFormatEnum;
// use sdl2::rect::Rect;
// use sdl2::event::Event;
// use sdl2::keyboard::Keycode;
// use sdl2::EventPump;
// use sdl2::render::Canvas;

// pub struct SDLScreen {
//         orig_width: u16,
//         orig_height: u16,
//         width: u16,
//         height: u16,
//         buffer: Vec<u8>
// }

// impl SDLScreen {
//     pub fn new() -> Self {
//         SDLScreen {
//             orig_height: 64,
//             orig_width: 32,
//             width: 32 * 20,
//             height: 64 * 20,
//             buffer: Vec::new()
//         }
//     }

//     pub fn set_pixel(&mut self, usize idx, u8 pixel) {
//         self.buffer[idx] = pixel;
//     }

//     pub fn draw(&mut self) {  
//          texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
//             for x in 0..width {
//                 for y in 0..height {
//                     let idx = ((self.width * height) + wwidth) as usize;
//                     buffer[idx] = self.buffer[idx];
//                 }
//             }
//         }).expect("Failed to draw");
//     }

//     pub fn run(&mut self, emulator: &Chip8) -> bool {
//         let sdl_context = sdl2::init()?;
//         let video_subsystem = sdl_context.video()?;

//         let window = video_subsystem.window("Chip8 Emulator", self.width, self.height)
//             .position_centered()
//             .opengl()
//             .build()
//             .map_err(|e| e.to_string())?;

//             let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
//             let texture_creator = canvas.texture_creator();

//         let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGB24, 256, 256)
//             .map_err(|e| e.to_string())?;
        
//             for x in 0..width {
//                 for y in 0..height {
//                     let idx = ((self.width * height) + wwidth) as usize;
//                     buffer.push(0xF);
//                 }
//             }

//         canvas.clear();
//         canvas.copy(&texture, None, Some(Rect::new(100, 100, 256, 256)))?;
//         canvas.copy_ex(&texture, None,
//             Some(Rect::new(450, 100, 256, 256)), 30.0, None, false, false)?;
//         canvas.present();

//         let mut event_pump = sdl_context.event_pump()?;

//         let mut keys: Vec<u8> = Vec::new();
//         'running: loop {
//             for event in event_pump.poll_iter() {
//                 match event {
//                     Event::Quit {..}
//                     | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
//                         break 'running
//                     },
//                     _ => {}
//                 }
//             }

//             // The rest of the game loop goes here...
//             emulator.frame(self, keys);
//         }
//         true
//     }
// }