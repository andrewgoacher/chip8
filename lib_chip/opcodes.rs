pub enum OpCode {
    Unknown(u16),
    ClearScreen,
    Return,
    Load(LoadOpCode)
}

pub struct LoadOpCode {
    pub register: u8,
    pub value: u8
}

pub fn parse_opcode(high: u8, low: u8) -> OpCode {
    let raw: u16 = ((high as u16) << 8) | ((low as u16) & 0xFF);

    match raw {
        0x00E0 => OpCode::ClearScreen,
        0x00EE => OpCode::Return,
        _ => {
            let operand_high = high & 0xF0;
            match operand_high {
                0x60 => {
                    let operand_low = high & 0x0F;
                    OpCode::Load(LoadOpCode {
                        register: operand_low,
                        value: low
                    })
                },
                _ => OpCode::Unknown(raw)
            }
        }
    }
}