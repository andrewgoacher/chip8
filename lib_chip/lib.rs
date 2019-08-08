//! # lib chip
//!
//! A rust library for emulating chip8 programs.
//! 
//! This library will not provide any rendering functionality.  It represents all display code
//! as a buffer of bytes and passes this off to the consumer to handle rendering.
//! 
//! # Example:
//! 
//! ```
//! # use lib_chip::state::State;
//! # use lib_chip::rom::Rom;
//! # use lib_chip::memory::Memory;
//! let mut state = State::new(64, 32);
//! # let rom: Rom = Default::default();
//! # let mut memory = Memory::new();
//! # memory.set_range(0x200, rom.read_all());
//! # memory.set_range(0x200, vec![0x00, 0xE0]);
//! # let mut screen: Vec<u8> = vec![0x0; (32*64) as usize];
//! state = state.step(&mut memory, None, &mut screen);
//! ```

pub mod state;
pub mod memory;
pub mod rom;
pub mod keyboard;
pub mod opcode;