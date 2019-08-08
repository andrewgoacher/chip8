extern crate lib_chip;
extern crate sdl2;

use lib_chip::{
    state::State,
    rom::Rom,
    memory::Memory,
    keyboard::get_key_mapped
};

use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Texture;

fn draw(texture: &mut Texture, screen: &[u8], width: u32, height: u32, scale: u32) -> Result<(), String> {
    texture.with_lock(None, |buffer: &mut [u8], _pitch: usize| {
        for y in 0 .. height {
            for x in 0 .. width {
                let idx = ((width * y) + x) as usize;
                let slot = screen[idx];
                let color = if slot == 0 { 0xFF } else { 0x00 };
                
                for i in 0..scale {
                    for j in 0..scale {
                        let ix = (x*scale) + i;
                        let iy = (y*scale) + j;
                        let tote_width = width * scale;
                        let _tote_height = height * scale;
                        let offset = ((tote_width * iy) + ix) as usize;
            
                        buffer[offset*3] = color;
                        buffer[(offset*3)+1] = color;
                        buffer[(offset*3)+2] = color;
                    }
                }
            }
        }
    })
}



fn print_state(state: &State) {
    println!("{}", state)
}

pub fn main() -> Result<(), String> {
    const SCALE: u32 = 10;

    const EMU_WIDTH: u32 = 64;
    const EMU_HEIGHT: u32 = 32;

    let mut state = State::new(EMU_WIDTH, EMU_HEIGHT);
    let rom = Rom::load("./TETRIS").expect("Failed to load rom");
    let mut memory = Memory::new();
    memory.set_range(0x0200, rom.read_all());

    let mut screen: Vec<u8> = vec![0x00; (EMU_WIDTH * EMU_HEIGHT) as usize];

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("rust-sdl2 demo: Video", EMU_WIDTH * SCALE, EMU_HEIGHT * SCALE)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGB24,
     EMU_WIDTH*SCALE, EMU_HEIGHT*SCALE)
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
        print_state(&state);
        println!("\n\n");
        state = state.step(&mut memory, get_key_mapped(key), &mut screen);

        if state.clear_flag {
            screen =  vec![0x00; (EMU_WIDTH * EMU_HEIGHT) as usize];
        }
        if state.draw_flag {
            canvas.clear();
            draw(&mut texture, &screen, EMU_WIDTH, EMU_HEIGHT, SCALE)?;
            canvas.copy(&texture, None, Some(Rect::new(0, 0, EMU_WIDTH * SCALE, EMU_HEIGHT * SCALE)))?;
        }
        if state.clear_flag || state.draw_flag {
            state.draw_flag = false;
            state.clear_flag = false;
            canvas.present(); 
        }
    }

    Ok(())
}