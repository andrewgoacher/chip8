use crate::opcode::parser::parse_opcode;
use crate::opcode::OpCode;
use crate::memory::Memory;
use super::State;

pub fn get_opcode(state: &State, memory: &Memory) -> OpCode {
    let pc = state.pc;
    let high = memory.read(pc);
    let low = memory.read(pc + 1);
    parse_opcode(high, low)
}

pub fn draw_sprite(state: &State, memory: &Memory, screen: &mut Vec<u8>,
  x: u8, y: u8, n: u8) -> bool {
    let row = x;
    let col = y;
    let mut erased = false;
    let width = state.width;
    let height = state.height;

    for byte_index in 0 .. n {
        let byte = memory.read(state.i + byte_index as u16);
        let mut buts: [u8;8] = [0;8];
        let mut ps: [u8;8] = [0;8];

        for bit_index in 0 .. 8 {
            let bit: u8 = (byte >> bit_index) & 0x1;
            buts[bit_index] = bit;

            let curr_x = (row + byte_index) as u32 % width;
            let curr_y = (col + (7-bit)) as u32 % height;
            let curr_idx = ((width * curr_y) + curr_x) as usize;
            let curr_pixel = screen[curr_idx];

            if bit == 1 && curr_pixel == 1 {
                erased = true;
            }

            let pixel = curr_pixel ^ bit;
            ps[bit_index] = pixel;
            screen[curr_idx] = pixel;
        }
    }

    erased
}