use super::{AddOp, OpCode, ShiftOp, SkipOp, LoadOp, JumpOp};


pub fn parse_opcode(high: u8, low: u8) -> OpCode {
   
    let opcode: u16 = (high as u16) << 8 | low as u16;
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
