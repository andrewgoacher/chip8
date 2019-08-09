//! Contains the parsers to take the data from memory and return an opcode
use super::{AddOp, OpCode, ShiftOp, SkipOp, LoadOp, JumpOp};

/// Generates a 16bit opcode from 2 8bit operands.
/// These will generally be found at memory address x and x+1.
fn generate_opcode(high: u8, low: u8) -> u16 {
    (u16::from(high)) << 8 | u16::from(low)
}

/// Parses the higher and lower order bits of a 16bit opcode and returns
/// a structured opcode that can be matched against.
/// 
/// If the opcode is not recognised it will return `OpCode::Unknown(0xnnnn)`
/// where nnnn is the opcode in question.
/// 
/// # Examples:
/// 
/// This will show a failed opcode as chip8 has no operand for 0x0000.
/// This would cause a panic! further in the program.
/// 
/// ```
/// # use lib_chip::opcode::*;
/// # use lib_chip::opcode::parser::*;
/// const HIGH:u8 = 0x00;
/// const LOW:u8 = 0x00;
/// let opcode = parse_opcode(HIGH, LOW);
/// # assert_eq!(OpCode::Unknown(0x0000), opcode);
/// ```
/// 
/// 
/// A successful parsing will return a value from OpCode.  
/// 
/// In this instance the clear screen
/// opcode `CLS` which is 0x00E0.
/// 
/// ```
/// # use lib_chip::opcode::*;
/// # use lib_chip::opcode::parser::*;
/// const HIGH:u8 = 0x00;
/// const LOW:u8 = 0xE0;
/// let clear_screen = parse_opcode(HIGH, LOW);
/// # assert_eq!(OpCode::CLS, clear_screen);
/// ```
/// 
pub fn parse_opcode(high: u8, low: u8) -> OpCode {
   
    let opcode: u16 = generate_opcode(high, low);
    let x: u8   = ((opcode >> 8) & 0x000F) as u8; // the lower 4 bits of the high byte
    let y: u8   = ((opcode >> 4) & 0x000F) as u8; // the upper 4 bits of the low byte
    let n: u8   = (opcode & 0x000F) as u8; // the lowest 4 bits
    let kk: u8  = (opcode & 0x00FF) as u8; // the lowest 8 bits
    let nnn: u16 = opcode & 0x0FFF; // the lowest 12 bits

    match opcode & 0xF000 {
        0x0000 => {
            match kk {
                0x00E0 => OpCode::CLS,
                0x00EE => OpCode::RET,
                _ => OpCode::Unknown(opcode)
            }
        },
        0x1000 => OpCode::JP(JumpOp::JP(nnn)),
        0x2000 => OpCode::CALL(nnn),
        0x3000 => OpCode::SKIP(SkipOp::SE(x, kk)),
        0x4000 => OpCode::SKIP(SkipOp::SNE(x, kk)),
        0x5000 => OpCode::SKIP(SkipOp::SEXY(x, y)),
        0x6000 => OpCode::LD(LoadOp::LD(x, kk)),
        0x7000 => OpCode::ADD(AddOp::ADD(x, kk)),
        0x8000 => {
            match n {
                0x0 => OpCode::LD(LoadOp::LDXY(x, y)),
                0x1 => OpCode::OR(x, y),
                0x2 => OpCode::AND(x, y),
                0x3 => OpCode::XOR(x, y),
                0x4 => OpCode::ADD(AddOp::ADDREG(x, y)),
                0x5 => OpCode::SUB(x, y),
                0x6 => OpCode::SHIFT(ShiftOp::SHR(x)),
                0x7 => OpCode::SUBN(x, y),
                0xE => OpCode::SHIFT(ShiftOp::SHL(x)),
                _ => OpCode::Unknown(opcode)
            }
        },
        0x9000 => {
            match n {
                0x0 => OpCode::SKIP(SkipOp::SNEXY(x,y)),
                _ => OpCode::Unknown(opcode)
            }
        },
        0xA000 => OpCode::LD(LoadOp::LDI(nnn)),
        0xB000 => OpCode::JP(JumpOp::JPV0(nnn)),
        0xC000 => OpCode::RND(x, kk),
        0xD000 => OpCode::DRW(x, y, n),
        0xE000 => {
            match kk {
                0x9E => OpCode::SKIP(SkipOp::SKP(x)),
                0xA1 => OpCode::SKIP(SkipOp::SKNP(x)),
                _ => OpCode::Unknown(opcode)
            }
        },
        0xF000 => {
            match kk {
                    0x07 => OpCode::LD(LoadOp::LDVXDT(x)),
                    0x0A => OpCode::LD(LoadOp::LDKEY(x)),
                    0x15 => OpCode::LD(LoadOp::LDDTVX(x)),
                    0x18 => OpCode::LD(LoadOp::LDSTVX(x)),
                    0x1E => OpCode::ADD(AddOp::ADDI(x)),
                    0x29 => OpCode::LD(LoadOp::LDF(x)),
                    0x33 => OpCode::LD(LoadOp::LDB(x)),
                    0x55 => OpCode::LD(LoadOp::LDIV0X(x)),
                    0x65 => OpCode::LD(LoadOp::LDV0XI(x)),
                _ => OpCode::Unknown(opcode)
            }
        },
        _ => OpCode::Unknown(opcode)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::*;

    #[test]
    fn it_will_generate_opcode() {
        const HIGH: u8 = 0xFF;
        const LOW: u8 = 0xE0;

        let actual = generate_opcode(HIGH, LOW);
        assert_eq!(0xFFE0, actual);
    }

    #[test]
    fn it_will_return_clear_screen() {
        const HIGH: u8 = 0x00;
        const LOW: u8 = 0xE0;

        let opcode = parse_opcode(HIGH, LOW);
        assert_eq!(OpCode::CLS, opcode)
    }

    #[test]
    fn it_will_return_ret() {
        const HIGH:u8 = 0x00;
        const LOW: u8 = 0xEE;

        let actual = parse_opcode(HIGH, LOW);
        assert_eq!(OpCode::RET, actual)
    }

    #[test]
    fn it_will_jump_to_specified_address() {
        const HIGH:u8 = 0x11;
        const LOW:u8 = 0x23;

        let actual = parse_opcode(HIGH, LOW);
        assert_eq!(OpCode::JP(JumpOp::JP(0x0123)), actual);
    }

    #[test]
    fn it_will_call_at_specified_address() {
        const HIGH:u8 = 0x23;
        const LOW:u8 = 0x03;

        let actual = parse_opcode(HIGH, LOW);
        assert_eq!(OpCode::CALL(0x0303), actual);
    }

    #[test]
    fn it_will_return_skip_equal() {
        const HIGH:u8 = 0x32;
        const LOW:u8 = 0x33;

        let actual = parse_opcode(HIGH, LOW);
        assert_eq!(OpCode::SKIP(SkipOp::SE(0x02, 0x33)), actual);
    }

    #[test]
    fn it_will_return_skip_not_equal() {
        const HIGH:u8 = 0x45;
        const LOW:u8 = 0x27;

        let actual = parse_opcode(HIGH, LOW);
        assert_eq!(OpCode::SKIP(SkipOp::SNE(0x05, 0x27)), actual);       
    }

    #[test]
    fn it_will_return_skip_if_register_values_equal() {
        const HIGH:u8 = 0x52;
        const LOW:u8 = 0xE0;

        let actual = parse_opcode(HIGH, LOW);
        assert_eq!(OpCode::SKIP(SkipOp::SEXY(0x02, 0x0E)), actual);
    }

    #[test]
    fn it_will_return_load_into_register() {
        const HIGH:u8 = 0x6F;
        const LOW:u8 = 0xCD;

        let actual = parse_opcode(HIGH, LOW);
        assert_eq!(OpCode::LD(LoadOp::LD(0x0F, 0xCD)), actual);
    }

    #[test]
    fn it_will_return_add_into_register() {
        const HIGH:u8 = 0x73;
        const LOW:u8 = 0x22;

        let actual = parse_opcode(HIGH, LOW);
        assert_eq!(OpCode::ADD(AddOp::ADD(0x03, 0x22)), actual);
    }
}
