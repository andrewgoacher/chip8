extern crate lib_chip;
extern crate sdl2;
extern crate getopts;
use getopts::Options;
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

fn set_opts(args: Vec<String>) -> getopts::Result {
    let mut opts = Options::new();
    opts.optopt("i", "input", "Chip8 Program file to run", "File");
    opts.parse(args)
}

fn get_filename(matches: &getopts::Matches) -> String {
     match matches.opt_str("i") {
        Some(f) => f,
        None => panic!("File name required to run emulators")
    }
}

pub fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let matches = match set_opts(args) {
        Ok(m) => m,
        Err(fail) => panic!(fail.to_string())
    };

    let filename = get_filename(&matches);

    const SCALE: u32 = 10;

    const EMU_WIDTH: u32 = 64;
    const EMU_HEIGHT: u32 = 32;

    let mut state = State::new(EMU_WIDTH, EMU_HEIGHT);
    let rom = Rom::load(filename.as_str()).expect("Failed to load rom");
    let mut memory = Memory::new();
    memory.set_range(0x0200, rom.read_all());

    let mut screen: Vec<u8> = vec![0x00; (EMU_WIDTH * EMU_HEIGHT) as usize];

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window(format!("Chip8 - {}", filename).as_str(), EMU_WIDTH * SCALE, EMU_HEIGHT * SCALE)
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
                screen =  vec![0x00; (EMU_WIDTH * EMU_HEIGHT) as usize];
            }
            if state.draw_flag {
                draw(&mut texture, &screen, EMU_WIDTH, EMU_HEIGHT, SCALE)?;
                canvas.copy(&texture, None, Some(Rect::new(0, 0, EMU_WIDTH * SCALE, EMU_HEIGHT * SCALE)))?;
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