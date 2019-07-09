pub enum OpCode {
    Unknown(u16),
    CLS,
    RET,
    LD(LoadOp),
    JP(JumpOp),
    CALL(Location),
    SKIP(SkipOp),
    ADD(AddOp),
    SUB(Register, Register),
    SUBN(Register, Register),
    RND(Register, u8),
    DRW(Register, Register, u8),
    OR(Register, Register),
    AND(Register, Register),
    XOR(Register, Register),
    SHIFT(ShiftOp),
}

pub enum ShiftOp {
    SHR(Register),
    SHL(Register),
}

pub enum AddOp {
    ADD(Register, u8),
    ADDREG(Register, Register),
    ADDI(Register),
}

pub enum SkipOp {
    SE(Register, u8),
    SNE(Register, u8),
    SEXY(Register, Register),
    SNEXY(Register, Register),
    SKP(Register),
    SKNP(Register),
}

pub enum JumpOp {
    JP(Location),
    JPV0(u16),
}

pub enum LoadOp {
    LD(Register, u8),
    LDI(u16),
    LDXY(Register, Register),
    LDVXDT(Register),
    LDDTVX(Register),
    LDKEY(Register),
    LDSTVX(Register),
    LDF(Register),
    LDB(Register),
    LDIV0X(Register),
    LDV0XI(Register),
}

pub type Location = u16;
pub type Register = u8;

pub fn parse_opcode(high: u8, kk: u8) -> OpCode {
    let raw: u16 = ((high as u16) << 8) | ((kk as u16) & 0xFF);

    match raw {
        0x00E0 => OpCode::CLS,
        0x00EE => OpCode::RET,
        _ => {
            let operand_high = raw & 0xF000;
            let x = high & 0x0F;
            let y = kk & 0xF0;
            let nnn = raw & 0x0FFF;
            let nibble = kk & 0x0F;
            match operand_high {
                0x1000 => OpCode::JP(JumpOp::JP(nnn)),
                0x2000 => OpCode::CALL(nnn),
                0x3000 => OpCode::SKIP(SkipOp::SE(x, kk)),
                0x4000 => OpCode::SKIP(SkipOp::SNE(x, kk)),
                0x5000 => OpCode::SKIP(SkipOp::SEXY(x, y)),
                0x6000 => OpCode::LD(LoadOp::LD(x, kk)),
                0x7000 => OpCode::ADD(AddOp::ADD(x, kk)),
                0x8000 => match nibble {
                    0x0 => OpCode::LD(LoadOp::LDXY(x, y)),
                    0x1 => OpCode::OR(x, y),
                    0x2 => OpCode::AND(x, y),
                    0x3 => OpCode::XOR(x, y),
                    0x4 => OpCode::ADD(AddOp::ADDREG(x, y)),
                    0x5 => OpCode::SUB(x, y),
                    0x6 => OpCode::SHIFT(ShiftOp::SHR(x)),
                    0x7 => OpCode::SUBN(x, y),
                    0xE => OpCode::SHIFT(ShiftOp::SHL(x)),
                    _ => OpCode::Unknown(raw),
                },
                0x9000 => OpCode::SKIP(SkipOp::SNEXY(x, y)),
                0xA000 => OpCode::LD(LoadOp::LDI(nnn)),
                0xB000 => OpCode::JP(JumpOp::JPV0(nnn)),
                0xC000 => OpCode::RND(x, kk),
                0xD000 => OpCode::DRW(x, y, nibble),
                0xE000 => match kk {
                    0x9E => OpCode::SKIP(SkipOp::SKP(x)),
                    0xA1 => OpCode::SKIP(SkipOp::SKNP(x)),
                    _ => OpCode::Unknown(raw),
                },
                0xF000 => match kk {
                    0x07 => OpCode::LD(LoadOp::LDVXDT(x)),
                    0x0A => OpCode::LD(LoadOp::LDKEY(x)),
                    0x15 => OpCode::LD(LoadOp::LDDTVX(x)),
                    0x18 => OpCode::LD(LoadOp::LDSTVX(x)),
                    0x1E => OpCode::ADD(AddOp::ADDI(x)),
                    0x29 => OpCode::LD(LoadOp::LDF(x)),
                    0x33 => OpCode::LD(LoadOp::LDB(x)),
                    0x55 => OpCode::LD(LoadOp::LDIV0X(x)),
                    0x65 => OpCode::LD(LoadOp::LDV0XI(x)),
                    _ => OpCode::Unknown(raw),
                },
                _ => OpCode::Unknown(raw),
            }
        }
    }
}
