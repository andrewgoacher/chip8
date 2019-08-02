use std::fmt::{self, Formatter, Display};

#[derive(Debug, Copy, Clone)]
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

impl Display for OpCode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // match self {
        //     OpCode::Unknown(_) => write!(f, "Unknown opcode"),
        //     OpCode::CLS => write!(f, "Clear Screen"),
        //     OpCode::RET => write!(f, "Return"),
        //     x => write!(f, "{}", x)
        // }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ShiftOp {
    SHR(Register),
    SHL(Register),
}

impl Display for ShiftOp {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ShiftOp::SHR(_) => write!(f, "Shift R"),
            ShiftOp::SHL(_) => write!(f, "Shift L")
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum AddOp {
    ADD(Register, u8),
    ADDREG(Register, Register),
    ADDI(Register),
}

impl Display for AddOp {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
           AddOp::ADD(_,_) => write!(f, "ADD"),
           AddOp::ADDREG(_, _) => write!(f, "ADDREG"),
           AddOp::ADDI(_) => write!(f, "ADDI")
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum SkipOp {
    SE(Register, u8),
    SNE(Register, u8),
    SEXY(Register, Register),
    SNEXY(Register, Register),
    SKP(Register),
    SKNP(Register),
}

impl Display for SkipOp {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            SkipOp::SE(_,_) => write!(f, "SE"),
            SkipOp::SNE(_,_) => write!(f, "SNE"),
            SkipOp::SEXY(_,_) => write!(f, "SEXY"),
            SkipOp::SNEXY(_,_) => write!(f, "SNEXY"),
            SkipOp::SKP(_) => write!(f, "SKP"),
            SkipOp::SKNP(_) => write!(f, "SKNP")
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum JumpOp {
    JP(Location),
    JPV0(u16),
}

impl Display for JumpOp {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            JumpOp::JP(_) => write!(f, "JP"),
            JumpOp::JPV0(_) => write!(f, "JPV0")
        }
    }
}

#[derive(Debug, Copy, Clone)]
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

impl Display for LoadOp {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            LoadOp::LD(_,_) => write!(f,"LD"),
            LoadOp::LDI(_) => write!(f, "LDI"),
            LoadOp::LDXY(_,_) => write!(f, "LDXY"),
            LoadOp::LDVXDT(_) => write!(f, "LDVXDT"),
            LoadOp::LDDTVX(_) => write!(f, "LDDTVX"),
            LoadOp::LDKEY(_) => write!(f, "LDKEY"),
            LoadOp::LDSTVX(_) => write!(f, "LDSTVX"),
            LoadOp::LDF(_) => write!(f, "LDF"),
            LoadOp::LDB(_) => write!(f, "LDB"),
            LoadOp::LDIV0X(_) => write!(f, "LDIV0X"),
            LoadOp::LDV0XI(_) => write!(f, "LDV0XI")
        }
    }
}

pub type Location = u16;
pub type Register = u8;

pub fn parse_opcode(high: u8, low: u8) -> OpCode {
    //let raw: u16 = ((high as u16) << 8) | ((kk as u16) & 0xFF);
    let raw = (high as u16) << 8 | low as u16;
    match raw {
        0x00E0 => OpCode::CLS,
        0x00EE => OpCode::RET,
        _ => {
            let x = ((raw >> 8) & 0x000F) as u8;
            let y = ((raw >> 4) & 0x000F) as u8;
            let n = (raw & 0x000F) as u8;
            let nnn = raw & 0x0FFF;
            let kk = (raw & 0x00FF) as u8;
            let operand_high = (raw & 0xF000);
            // let operand_high = raw & 0xF000;
            // let x = high & 0x0F;
            // let y = kk & 0xF0;
            // let nnn = raw & 0x0FFF;
            // let n = kk & 0x0F;
            match operand_high {
                0x1000 => OpCode::JP(JumpOp::JP(nnn)),
                0x2000 => OpCode::CALL(nnn),
                0x3000 => OpCode::SKIP(SkipOp::SE(x, kk)),
                0x4000 => OpCode::SKIP(SkipOp::SNE(x, kk)),
                0x5000 => OpCode::SKIP(SkipOp::SEXY(x, y)),
                0x6000 => OpCode::LD(LoadOp::LD(x, kk)),
                0x7000 => OpCode::ADD(AddOp::ADD(x, kk)),
                0x8000 => match n {
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
                0xD000 => OpCode::DRW(x, y, n),
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
