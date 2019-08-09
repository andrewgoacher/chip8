//! Contains the formatting logic to be able to print the opcodes to console.

use std::fmt::{self, Formatter, Display};
use super::{AddOp, OpCode, ShiftOp, SkipOp, LoadOp, JumpOp};

impl Display for OpCode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            OpCode::Unknown(_) => write!(f, "Unknown opcode"),
            OpCode::CLS => write!(f, "(0x00E0): Clear Screen"),
            OpCode::RET => write!(f, "(0x00EE): Return"),
            OpCode::SHIFT(x) => write!(f, "{}", x),
            OpCode::ADD(x) => write!(f,"{}", x),
            OpCode::SKIP(x) => write!(f,"{}", x),
            OpCode::JP(x) => write!(f,"{}", x),
            OpCode::LD(x) => write!(f,"{}", x),
            OpCode::CALL(nnn) => write!(f, "(0x2nnn): Call routine at {}", nnn),
            OpCode::SUB(x, y) => write!(f, "(0x8xy5): Subtract V[{}] from V[{}]", y, x),
            OpCode::SUBN(x, y) => write!(f, "(0x8xy7): Subtract V[{}] from V[{}]", x, y),
            OpCode::RND(x, a) => write!(f, "(0xcxkk): Set V[{}] to RND & {}", x, a),
            OpCode::DRW(x, y, n) => write!(f, "(0xDxyn): Draw {} at ({},{})", n, x, y),
            OpCode::OR(x, y) => write!(f, "(0x8xy1): Logically OR V[{}] and V[{}]", x, y),
            OpCode::AND(x, y) => write!(f, "(0x8xy2): Logically AND V[{}] and V[{}]", x, y),
            OpCode::XOR(x, y) => write!(f, "0x8xy3): Logically XOR V[{}] and V[{}]", x, y)
        }
    }
}

impl Display for ShiftOp {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ShiftOp::SHR(x) => write!(f, "(0x8xy6): Shift Right {}", x),
            ShiftOp::SHL(x) => write!(f, "(0x8xyE): Shift Left {}", x)
        }
    }
}

impl Display for AddOp {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
           AddOp::ADD(x,kk) => write!(f, "(0x7xkk): Add {} to V[{}]", kk, x),
           AddOp::ADDREG(x,y) => write!(f, "(0x8xy4): Add {} to {}", y, x),
           AddOp::ADDI(x) => write!(f, "(0xFx1E): Add {} to I", x)
        }
    }
}

impl Display for SkipOp {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            SkipOp::SE(x,kkk) => write!(f, "(0x3xkk): Skip if V[{}] == {}", x, kkk),
            SkipOp::SNE(x,kkk) => write!(f, "(0x4xkkk): Skip is V[{}] != {}", x, kkk),
            SkipOp::SEXY(x,y) => write!(f, "(0x5xy0): Skip if V[{}] == V[{}]", x, y),
            SkipOp::SNEXY(x,y) => write!(f, "(0x9xy0): Skip if V[{}] != V[{}]", x, y),
            SkipOp::SKP(x) => write!(f, "(0xEx9E): Skip if Key with V[{}] is pressed", x),
            SkipOp::SKNP(x) => write!(f, "(0xExA1): Skip is key with V[{}] is not pressed", x)
        }
    }
}

impl Display for JumpOp {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            JumpOp::JP(nnn) => write!(f, "(0x0nnn) | (0x1nnn): Jump to {}", nnn),
            JumpOp::JPV0(nnn) => write!(f, "(0xBnnn): Jump to V0 + {}", nnn)
        }
    }
}

impl Display for LoadOp {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            LoadOp::LD(x,kk) => write!(f,"(0x6xkk): Load {} into V[{}]", kk, x),
            LoadOp::LDI(nnn) => write!(f, "(0xAnnn): Set I to {}", nnn),
            LoadOp::LDXY(x,y) => write!(f, "(0x8xy0): Set V[{}] to V[{}]", x, y),
            LoadOp::LDVXDT(x) => write!(f, "(0xFx07): Set V[{}] to DT", x),
            LoadOp::LDDTVX(x) => write!(f, "(0xFx15): Set DT to V[{}]", x),
            LoadOp::LDKEY(x) => write!(f, "(0xFx0A) wait for keypress and set to V[{}]", x),
            LoadOp::LDSTVX(x) => write!(f, "(0xFx18): Set ST to V[{}]", x),
            LoadOp::LDF(x) => write!(f, "(0xFx29): Load Sprite at V[{}] into I", x),
            LoadOp::LDB(x) => write!(f, "(0xFx33): Load into I, I+1 and I+2 the BCD representation of V[{}]", x),
            LoadOp::LDIV0X(x) => write!(f, "(0xFx55): Load From I V0 to V[{}]", x),
            LoadOp::LDV0XI(x) => write!(f, "(0xFx65): Read starting at I from V0 to V[{}]", x)
        }
    }
}
