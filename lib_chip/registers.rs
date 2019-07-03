pub struct Registers {
    v: [u8; 16],
    i: u16,
    delay: u8,
    sound: u8,
    pc: u16,
    sp: u8   
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            v: [0; 16],
            i: 0,
            delay: 0,
            sound: 0,
            pc: 0,
            sp: 0
        }
    }
}