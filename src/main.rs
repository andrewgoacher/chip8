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

fn draw(texture: &mut Texture, screen: &Vec<u8>) -> Result<(), String> {
    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
         for x in 0.. screen.len() {
             buffer[x*3] = screen[x];
             buffer[(x*3)+1] = screen[x];
             buffer[(x*3) +2] = screen[x];
         }
    })
}

pub fn main() -> Result<(), String> {
    const width: u32 = 640;
    const height: u32 = 320;

    let mut state = State::new();
    let rom = Rom::load("./TETRIS").expect("Failed to load rom");
    let mut memory = Memory::new();
    memory.set_range(0x200, rom.read_all());

    let screen: Vec<u8> = vec![0; (width * height) as usize];

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
        state = state.step(&mut memory, get_key_mapped(key));

        if state.clear_flag || state.draw_flag {
            canvas.clear();
            state.clear_flag = false;
        }
        if state.draw_flag {
            draw(&mut texture, &screen);
            canvas.copy(&texture, None, None)?;
            state.draw_flag = false;
        }
        if state.clear_flag || state.draw_flag {
            canvas.present();
        }
    }

    Ok(())
}