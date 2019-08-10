extern crate lib_chip;
extern crate sdl2;
extern crate getopts;

use std::env;

use lib_chip::{
    state::{State,sound_timer, delay_timer},
    rom::Rom,
    memory::Memory,
    keyboard::get_key_mapped
};

use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::time::SystemTime;

mod draw;
use draw::draw;

mod opts;
use opts::{set_opts, get_filename};




pub fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let matches = match set_opts(args) {
        Ok(m) => m,
        Err(fail) => panic!(fail.to_string())
    };

    let filename = get_filename(&matches);

    const SCALE: u32 = 10;

    let mut state:State = Default::default();
    let rom = Rom::load(filename.as_str()).expect("Failed to load rom");
    let mut memory = Memory::new();
    memory.set_range(0x0200, rom.read_all());

    let mut screen = state.create_buffer();

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window(format!("Chip8 - {}", filename).as_str(), 
        state.width * SCALE, state.height * SCALE)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGB24,
     state.width*SCALE, state.height*SCALE)
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    let cpu_hz_ms = 10000u128;
    let timer_hz_ms = 16667u128;

    let mut cpu_elapsed_ms = 0u128;
    let mut timer_elapsed_ms = 0u128;
    let mut last_elapsed_ms = 0u128;
    let now = SystemTime::now();

    'running: loop {
        let mut key: Option<Keycode> = None;
        let elapsed = now.elapsed().expect("Elapsed time missing");
        let elapsed_ms = elapsed.as_nanos();
        let actual_elapsed = elapsed_ms - last_elapsed_ms;
        last_elapsed_ms = elapsed_ms;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..}
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: x, .. } =>{
                    key=x;
                    println!("SDL Key: {:?}", key);
                } ,
                    
                _ => {}
            }
        }

        cpu_elapsed_ms += actual_elapsed;
        timer_elapsed_ms += actual_elapsed;

        let mut delay = state.delay_timer;
        let mut sound = state.sound_timer;

        if timer_elapsed_ms >= timer_hz_ms {
            timer_elapsed_ms = 0u128;
            delay = delay_timer(&state);
            sound = sound_timer(&state);
        }

        if cpu_elapsed_ms >= cpu_hz_ms {
            cpu_elapsed_ms = 0u128;

            state = State {
                delay_timer: delay,
                sound_timer: sound,
                ..state
            };

            state = state.step(&mut memory, get_key_mapped(key), &mut screen);

            if state.clear_flag || state.draw_flag {
                canvas.clear();
            }
            if state.clear_flag {
                screen = state.create_buffer();
            }
            if state.draw_flag {
                draw(&mut texture, &screen, state.width, state.height, SCALE)?;
                canvas.copy(&texture, None, Some(Rect::new(0, 0, state.width * SCALE, state.height * SCALE)))?;
            }
            if state.clear_flag || state.draw_flag {
                state.draw_flag = false;
                state.clear_flag = false;
                canvas.present(); 
            }
        }
    }

    Ok(())
}