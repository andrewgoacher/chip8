use std::fmt::{self, Formatter, Display};
use super::{AddOp, OpCode, ShiftOp, SkipOp, LoadOp, JumpOp};

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

impl Display for ShiftOp {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ShiftOp::SHR(_) => write!(f, "Shift R"),
            ShiftOp::SHL(_) => write!(f, "Shift L")
        }
    }
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

impl Display for JumpOp {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            JumpOp::JP(_) => write!(f, "JP"),
            JumpOp::JPV0(_) => write!(f, "JPV0")
        }
    }
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
