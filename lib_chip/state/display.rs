use std::fmt::{self, Formatter, Display};
use super::State;

fn stack_to_string(stack: &[u16]) -> String {
    let mut s = String::new();
    s.push('[');
    for i in 0 .. 16 {
        s.push_str(&format!("0x{:04X}", stack[i]));
        if i != 15 {
            s.push(',');
        }
    }
    s.push(']');
    s
 }

impl Display for State {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "state: ({})", self.last_opcode)?;
        match self.opcode {
            None => (),
            Some(x) => {writeln!(f, "stored: {:?}", x)?;()}
        };
        writeln!(f, "registers: {:?}", self.registers)?;
        writeln!(f, "stack: {:?}", stack_to_string(&self.stack[..]))?;
        writeln!(f, "delay: {}, sound {}", self.delay_timer, self.sound_timer)?;
        writeln!(f, "pc: {} | stack pointer: {} | i: {}", self.pc,
            self.stack_pointer, self.i)?;
        writeln!(f, "draw: {} | run: {} | clear: {} ", self.draw_flag, 
            self.run_flag, self.clear_flag)
    }
}