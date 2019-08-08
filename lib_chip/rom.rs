use std::fs::File;
use std::io::prelude::*;

pub struct Rom {
    data: Vec<u8>
}

impl Rom {
    pub fn load(path: &str) -> Result<Rom, std::io::Error> {
        let data = load_rom_data(path)?;

        let rom = Rom {
            data
        };
        
        Ok(rom)
    }

    pub fn read_all(&self) -> Vec<u8> {
        // todo: Handle cannot move out of borrowed context
        self.data.clone()
    }
}

fn load_rom_data(file: &str) -> Result<Vec<u8>, std::io::Error>  {
    let mut buffer = Vec::new();
    let mut f = File::open(file)?;
    f.read_to_end(&mut buffer)?;
    Ok(buffer)
}