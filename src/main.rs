extern crate lib_chip;
use lib_chip::{
    state::State,
    rom::Rom,
    memory::Memory,
    keyboard::get_key_mapped
};

extern crate sdl2;

use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Texture;

fn draw(texture: &mut Texture, screen: &Vec<u8>, width: u32, height: u32) -> Result<(), String> {
    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {

        for y in 0 .. height {
            for x in 0 .. width {
                let idx = ((width * y) + x) as usize;
                let offset = idx; // ((y * pitch as u32) + x) as usize;

                let slot = screen[idx];
                let color = if slot == 0 { 0xFF } else { 0x00 };

                buffer[offset*3] = color;
                buffer[(offset*3)+1] = color;
                buffer[(offset*3)+2] = color;
            }
        }

        //  for x in 0.. screen.len() {
        //      let sx = screen[x];
        //      let col = if sx == 0 { 0xFF } else { 0x00 };

        //      buffer[x*3] =  col;
        //      buffer[(x*3)+1] =col;
        //      buffer[(x*3) +2] =col;
        //  }
    })
}

pub fn main() -> Result<(), String> {
    const width: u32 = 640;
    const height: u32 = 320;

    let mut state = State::new(width, height);
    let rom = Rom::load("./tetris.ch8").expect("Failed to load rom");
    let mut memory = Memory::new();
    memory.set_range(0x200, rom.read_all());

    let mut screen: Vec<u8> = vec![0x00; (width * height) as usize];

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("rust-sdl2 demo: Video", width, height)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGB24, width, height)
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        let mut key: Option<Keycode> = None;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..}
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: x, .. } => key=x,
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        // println!("{}", state);
        state = state.step(&mut memory, get_key_mapped(key), &mut screen);

        if state.clear_flag || state.draw_flag {
            canvas.clear();
        }
        if state.draw_flag {
            draw(&mut texture, &screen, width, height);
            //canvas.copy(&texture, None, None)?;
            canvas.copy(&texture, None, Some(Rect::new(0, 0, width, height)))?;
        }
        if state.clear_flag || state.draw_flag {
            state.draw_flag = false;
            state.clear_flag = false;
            canvas.present();
        }
    }

    Ok(())
}