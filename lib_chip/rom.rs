//! Represents a rom file in memory
use std::fs::File;
use std::io::prelude::*;

pub struct Rom {
    data: Vec<u8>
}

impl Rom {
    /// Loads the contents of a file into a buffer.
    /// 
    /// If the file cannot be found it will return an error.
    pub fn load(path: &str) -> Result<Rom, std::io::Error> {
        let data = load_rom_data(path)?;

        let rom = Rom {
            data
        };
        
        Ok(rom)
    }

    /// Loads a rom from memory into a buffer
    pub fn from_memory(data: Vec<u8>) -> Rom {
        Rom {
            data: data
        }
    }

    /// Returns the contents of the rom as an array slice.
    /// 
    /// Example:
    /// 
    /// ```
    /// # use lib_chip::rom::Rom;
    /// # let rom = Rom::from_memory(vec![0x01, 0x02, 0x03, 0x04, 0x05]);
    /// let data = rom.read_all();
    /// # assert_eq!([0x01, 0x02, 0x03, 0x04, 0x05], data);
    /// ```
    pub fn read_all(&self) -> &[u8] {
        &self.data[..]
    }
}

fn load_rom_data(file: &str) -> Result<Vec<u8>, std::io::Error>  {
    let mut buffer = Vec::new();
    let mut f = File::open(file)?;
    f.read_to_end(&mut buffer)?;
    Ok(buffer)
}