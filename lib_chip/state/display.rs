use std::fmt::{self, Formatter, Display};
use super::State;

impl Display for State {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "state: ({:?})", self.last_opcode)?;
        match self.opcode {
            None => (),
            Some(x) => {writeln!(f, "stored: {:?}", x)?;()}
        };
        writeln!(f, "delay: {}, sound {}", self.delay_timer, self.sound_timer)?;
        writeln!(f, "pc: {} | stack pointer: {} | interrupt: {}", self.pc,
            self.stack_pointer, self.i)?;
        writeln!(f, "stack: 0x{:04X}", self.stack[self.stack_pointer as usize])?;
        writeln!(f, "draw: {} | run: {} | clear: {} ", self.draw_flag, 
            self.run_flag, self.clear_flag)
    }
}