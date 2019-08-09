pub struct Memory {
    data: [u8; 1024 * 4]
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory {
    /// Creates a new memory buffer and loads in all font data from 0x0 to 0x200
    /// 
    /// memory is set to 4kb
    pub fn new() -> Memory {
        let mut memory = Memory { data: [0; 1024 * 4]};
        memory.reset();
        memory
    }

    /// Reads a piece of data at set address
    /// 
    /// Example:
    /// 
    /// ```
    /// # use lib_chip::memory::Memory;
    /// # let mut memory:Memory = Default::default();
    /// # memory.set(0x200, 0xF4);
    /// let data = memory.read(0x200);
    /// # assert_eq!(0xF4, data);
    /// ```
    pub fn read(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    /// Resets the memory and reloads font information from 0x0 to 0x200
    /// 
    /// Example:
    /// 
    /// ```
    /// # use lib_chip::memory::Memory;
    /// # let mut memory:Memory = Default::default();
    /// # memory.set(0x200, 0xF4);
    /// memory.reset();
    /// # assert_eq!(0x00, memory.read(0x200));
    /// ```
    pub fn reset(&mut self) {
        self.data = [0; 1024 * 4];
        let text = load_text();
        self.set_range(0x0, &text[..]);
    }

    /// Sets an array of data into memory from specified address
    /// 
    /// Example:
    /// 
    /// ```
    /// # use lib_chip::memory::Memory;
    /// # let mut memory:Memory = Default::default();
    /// memory.set_range(0x200, &vec![0x01, 0x02, 0x03, 0x04, 0x05][..]);
    /// # assert_eq!(0x01, memory.read(0x200));
    /// ```
    pub fn set_range(&mut self, from: usize, data: &[u8]) {
        self.data[from..(data.len()+from)].clone_from_slice(data)
    }

    /// Sets data at specified address
    /// 
    /// Example:
    /// 
    /// ```
    /// # use lib_chip::memory::Memory;
    /// # let mut memory:Memory = Default::default();
    /// memory.set(0x200, 0x01);
    /// # assert_eq!(0x01, memory.read(0x200));
    /// ```
    pub fn set(&mut self, address: usize, data: u8) {
        self.data[address] = data;
    }
}

/// Loads the font data into a buffer
fn load_text() -> Vec<u8> {
    let mem: [u8; 80] = [
        0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
        0x20, 0x60, 0x20, 0x20, 0x70, // 1
        0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
        0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
        0x90, 0x90, 0xF0, 0x10, 0x10, // 4
        0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
        0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
        0xF0, 0x10, 0x20, 0x40, 0x40, // 7
        0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
        0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
        0xF0, 0x90, 0xF0, 0x90, 0x90, // A
        0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
        0xF0, 0x80, 0x80, 0x80, 0xF0, // C
        0xE0, 0x90, 0x90, 0x90, 0xE0, // D
        0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
        0xF0, 0x80, 0xF0, 0x80, 0x80  // F 
    ];

    mem.to_vec()
}