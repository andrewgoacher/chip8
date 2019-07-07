pub struct Registers {
    pub v: [u8; 16],
    pub i: u16,
    pub delay: u8,
    pub sound: u8,
    pub pc: u16,
    pub sp: u8   
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            v: [0; 16],
            i: 0,
            delay: 0,
            sound: 0,
            pc: 0x200,
            sp: 0
        }
    }
}