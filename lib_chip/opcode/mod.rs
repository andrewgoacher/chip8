pub mod display;
pub mod parser;

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

#[derive(Debug, Copy, Clone)]
pub enum ShiftOp {
    SHR(Register),
    SHL(Register),
}

#[derive(Debug, Copy, Clone)]
pub enum AddOp {
    ADD(Register, u8),
    ADDREG(Register, Register),
    ADDI(Register),
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

#[derive(Debug, Copy, Clone)]
pub enum JumpOp {
    JP(Location),
    JPV0(u16),
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

pub type Location = u16;
pub type Register = u8;