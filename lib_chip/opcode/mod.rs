pub mod display;
pub mod parser;

#[derive(Debug, Copy, Clone)]
pub enum OpCode {
    /// Represents an unknown opcode.  Panics when parsed.
    Unknown(u16),
    /// Clear screen - 
    /// 0x00E0
    CLS,
    /// Returns from a subroutine - 
    /// 0x00EE
    RET,
    /// Represents a collection of Load operations
    LD(LoadOp),
    /// Represents a collection of Jump Operations
    JP(JumpOp),
    /// Calls a subroutine at nnn - 
    /// 0x2nnn
    CALL(Location),
    /// Represents a collection of Skip operations 
    SKIP(SkipOp),
    /// Represents a collection of Add operations
    ADD(AddOp),
    /// Subtract a from b.  Set V[0xF] to a > b 
    /// 0x8xy5
    SUB(Register, Register),
    /// Subtract b from a.  Set V[0xF] to b > a
    /// 0x8xy7
    SUBN(Register, Register),
    /// Set V[x] = rand(0,255) AND x
    /// 0xCxkk
    RND(Register, u8),
    /// Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
    /// The interpreter reads n bytes from memory, starting at the address stored in I. 
    /// These bytes are then displayed as sprites on screen at coordinates (Vx, Vy). 
    /// Sprites are XORed onto the existing screen. If this causes any pixels to be erased, 
    /// VF is set to 1, otherwise it is set to 0. If the sprite is positioned so part of 
    /// it is outside the coordinates of the display, it wraps around to the opposite side 
    /// of the screen. See instruction 8xy3 for more information on XOR, and section 2.4, #
    /// Display, for more information on the Chip-8 screen and sprites.
    /// 0xDxyn
    DRW(Register, Register, u8),
    /// Computes a bitwise OR of a and b
    /// 0x8xy1
    OR(Register, Register),
    /// Computes a bitwise AND of a and b
    /// 0x8xy2
    AND(Register, Register),
    /// Computes a bitwise XOR of a and b
    /// 0x8xy3
    XOR(Register, Register),
    /// Represents a collection of bitwise SHIFT operations
    SHIFT(ShiftOp),
}

#[derive(Debug, Copy, Clone)]
pub enum ShiftOp {
    /// Set Vx = Vx SHR 1.
    /// If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. 
    /// Then Vx is divided by 2.
    /// 0x8xy6
    SHR(Register),
    /// Set Vx = Vx SHL 1.
    /// If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. 
    /// Then Vx is multiplied by 2.
    /// 0x8xyE
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