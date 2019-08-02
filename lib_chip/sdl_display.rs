// use sdl2::render::{Texture, WindowCanvas};
// use sdl2::video::Window;
// use sdl2::pixels::PixelFormatEnum;

// fn sanitise_width(x: u32, width: u32) -> u32 {
//     x % width
// }

// fn sanitise_height(y: u32, height: u32) -> u32 {
//     y % height
// }

// pub struct SdlDisplay<'a> {
//     texture: Texture<'a>,
//     buffer: Vec<u8>,
//     dirty: bool,
//     width: u32,
//     height: u32,
//     canvas: WindowCanvas
// }

// impl<'a> SdlDisplay<'a> {
//     pub fn new(canvas: WindowCanvas, width: u32, height: u32) -> Self {
//           let texture_creator = canvas.texture_creator();

//         let texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGB24, 256, 256)
//             .expect("Failed to create texture");

//         SdlDisplay {
//             texture: texture,
//             canvas: canvas,
//             buffer: Vec::new(),
//             width: width,
//             height: height,
//             dirty: true
//         }
//     }

//     pub fn set_pixel(&mut self, x: u32, y: u32, color: u8) {
//         let w = sanitise_width(x, self.width);
//         let h = sanitise_height(y, self.height);
//         let idx = ((self.width * h) + w) as usize;

//         self.buffer[idx] = color;
//         self.dirty = true;
//         ()
//     }

//     pub fn set_pixels(&mut self, colors: Vec<u8>) {
//         for x in 0 .. colors.len() {
//             self.buffer[x] = colors[x];
//         }
//         self.dirty = true;
//         ()
//     }

//     pub fn draw(&mut self) -> Result<(), String> {
//         if !self.dirty {
//             return Ok(());
//         }

//         self.texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
//             for y in 0..256 {
//                 for x in 0..256 {
//                     let offset = y*pitch + x*3;
//                     buffer[offset] = x as u8;
//                     buffer[offset + 1] = y as u8;
//                     buffer[offset + 2] = 0;
//                 }
//             }
//         })?;

//         self.canvas.clear();
//         self.canvas.copy(&self.texture, None, None)?;
//         // canvas.copy(&texture, None, Some(Rect::new(100, 100, 256, 256)))?;
//         // canvas.copy_ex(&texture, None,
//         //     Some(Rect::new(450, 100, 256, 256)), 30.0, None, false, false)?;
//         self.canvas.present();
//         self.dirty = false;

//         Ok(())
//     }

//     pub fn clear(&mut self) {
//         self.canvas.clear();
//         self.canvas.present();
//     }
// }