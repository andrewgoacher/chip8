pub mod display;
pub mod parser;

#[derive(Debug, Copy, Clone,PartialEq)]
/// Represents all known opcodes for the Chip8 Emulator.
/// All opcodes are written with their hex values.  Where required
/// the value will have a substitution.
/// The following are used.
/// 
/// nnn or addr - A 12-bit value, the lowest 12 bits of the instruction
/// n or nibble - A 4-bit value, the lowest 4 bits of the instruction
/// x - A 4-bit value, the lower 4 bits of the high byte of the instruction
/// y - A 4-bit value, the upper 4 bits of the low byte of the instruction
/// kk or byte - An 8-bit value, the lowest 8 bits of the instruction
/// 
/// Example:
/// 0x00E0 will be OpCode::CLS
/// 0xCxkk will be OpCode::RND
/// where x will be the register index and kk will be the value to AND against random.
pub enum OpCode {
    /// Represents an unknown opcode.  Panics when parsed.
    Unknown(u16),
    /// Clear screen - 
    /// 
    /// 0x00E0
    CLS,
    /// Returns from a subroutine - 
    /// 
    /// 0x00EE
    RET,
    /// Represents a collection of Load operations
    LD(LoadOp),
    /// Represents a collection of Jump Operations
    JP(JumpOp),
    /// Calls a subroutine at nnn - 
    /// 
    /// 0x2nnn
    CALL(Location),
    /// Represents a collection of Skip operations 
    SKIP(SkipOp),
    /// Represents a collection of Add operations
    ADD(AddOp),
    /// Subtract a from b.  
    /// 
    /// Set V[0xF] to a > b 
    /// 
    /// 0x8xy5
    SUB(Register, Register),
    /// Subtract b from a.  
    /// 
    /// Set V[0xF] to b > a
    /// 
    /// 0x8xy7
    SUBN(Register, Register),
    /// Set V[x] = rand(0,255) AND x
    /// 
    /// 0xCxkk
    RND(Register, u8),
    /// Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
    /// 
    /// The interpreter reads n bytes from memory, starting at the address stored in I. 
    /// These bytes are then displayed as sprites on screen at coordinates (Vx, Vy). 
    /// Sprites are XORed onto the existing screen. If this causes any pixels to be erased, 
    /// VF is set to 1, otherwise it is set to 0. If the sprite is positioned so part of 
    /// it is outside the coordinates of the display, it wraps around to the opposite side 
    /// of the screen. See instruction 8xy3 for more information on XOR, and section 2.4, #
    /// Display, for more information on the Chip-8 screen and sprites.
    /// 
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

#[derive(Debug, Copy, Clone,PartialEq)]
pub enum ShiftOp {
    /// Set Vx = Vx SHR 1.
    /// 
    /// If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. 
    /// Then Vx is divided by 2.
    /// 
    /// 0x8xy6
    SHR(Register),
    /// Set Vx = Vx SHL 1.
    /// 
    /// If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. 
    /// Then Vx is multiplied by 2.
    /// 
    /// 0x8xyE
    SHL(Register),
}

#[derive(Debug, Copy, Clone,PartialEq)]
pub enum AddOp {
    /// Set Vx = Vx + kk.
    /// 
    /// Adds the value kk to the value of register Vx, then stores the result in Vx.
    ///  
    /// 0x7xkk
    ADD(Register, u8),
    /// Set Vx = Vx AND Vy.
    /// 
    /// Performs a bitwise AND on the values of Vx and Vy, then 
    /// stores the result in Vx. A bitwise AND compares the corrseponding 
    /// bits from two values, and if both bits are 1, then the same bit in 
    /// the result is also 1. Otherwise, it is 0.
    /// 
    /// 0x8xy2
    ADDREG(Register, Register),
    /// Set I = I + Vx.
    /// 
    /// The values of I and Vx are added, and the results are stored in I.
    /// 
    /// 0xFx1E
    ADDI(Register),
}

#[derive(Debug, Copy, Clone,PartialEq)]
pub enum SkipOp {
    /// Skip next instruction if Vx = kk.
    /// 
    /// The interpreter compares register Vx to kk, and if 
    /// they are equal, increments the program counter by 2.
    /// 
    /// 0x3xkk
    SE(Register, u8),
    /// Skip next instruction if Vx != kk.
    /// 
    /// The interpreter compares register Vx to kk, 
    /// and if they are not equal, increments the program counter by 2.
    /// 
    /// 0x4xkk
    SNE(Register, u8),
    /// Skip next instruction if Vx = Vy.
    /// 
    /// The interpreter compares register Vx to register Vy, 
    /// and if they are equal, increments the program counter by 2.
    /// 
    /// 0x5xy0
    SEXY(Register, Register),
    /// Skip next instruction if Vx != Vy.
    /// 
    /// The values of Vx and Vy are compared, and if they are not equal, 
    /// the program counter is increased by 2.
    /// 
    /// 0x9xy0
    SNEXY(Register, Register),
    /// Skip next instruction if key with the value of Vx is pressed.
    /// 
    /// Checks the keyboard, and if the key corresponding to the value 
    /// of Vx is currently in the down position, PC is increased by 2.
    /// 
    /// 0xEx9E
    SKP(Register),
    /// Skip next instruction if key with the value of Vx is not pressed.
    /// 
    /// Checks the keyboard, and if the key corresponding to the value of Vx 
    /// is currently in the up position, PC is increased by 2.
    /// 
    /// 0xExA1
    SKNP(Register),
}

#[derive(Debug, Copy, Clone,PartialEq)]
pub enum JumpOp {
    /// Jump to location nnn.
    /// 
    /// The interpreter sets the program counter to nnn.
    /// 
    /// 0x1nnn
    JP(Location),
    /// Jump to location nnn + V0.
    /// 
    /// The program counter is set to nnn plus the value of V0.
    /// 
    /// 0xBnnn
    JPV0(u16),
}

#[derive(Debug, Copy, Clone,PartialEq)]
pub enum LoadOp {
    /// Set Vx = kk.
    /// 
    /// The interpreter puts the value kk into register Vx.
    /// 
    /// 0x6xkk
    LD(Register, u8),
    /// Set I = nnn.
    /// 
    /// The value of register I is set to nnn.
    /// 
    /// 0xAnnn
    LDI(u16),
    /// Set Vx = Vy.
    /// 
    /// Stores the value of register Vy in register Vx.
    /// 
    /// 0x8xy0
    LDXY(Register, Register),
    /// Set Vx = delay timer value.
    /// 
    /// The value of DT is placed into Vx.
    /// 
    /// 0xFx07
    LDVXDT(Register),
    /// Set delay timer = Vx.
    /// 
    /// DT is set equal to the value of Vx.
    /// 
    /// 0xFx15
    LDDTVX(Register),
    /// Wait for keypress, store value in A
    /// 
    /// (0xFx0A)
    LDKEY(Register),
    /// Set sound timer = Vx.
    /// 
    /// ST is set equal to the value of Vx.
    /// 
    /// 0xFx18
    LDSTVX(Register),
    /// Set I = location of sprite for digit Vx.
    /// 
    /// The value of I is set to the location for the hexadecimal sprite 
    /// corresponding to the value of Vx. See section 2.4, Display, for more 
    /// information on the Chip-8 hexadecimal font.
    /// 
    /// (0xFx29)
    LDF(Register),
    /// Store BCD representation of Vx in memory locations I, I+1, and I+2.
    /// The interpreter takes the decimal value of Vx, and places the hundreds 
    /// digit in memory at location in I, the tens digit at location I+1, and 
    /// the ones digit at location I+2.
    /// 
    /// 0xFx33
    LDB(Register),
    /// Store registers V0 through Vx in memory starting at location I.
    /// The interpreter copies the values of registers V0 through Vx into memory, 
    /// starting at the address in I.
    /// 
    /// (0xFx55)
    LDIV0X(Register),
    /// Read registers V0 through Vx from memory starting at location I.
    /// 
    /// The interpreter reads values from memory starting at location I into 
    /// registers V0 through Vx.
    /// 
    /// (0xFx65)
    LDV0XI(Register),
}

/// Represents a location in memory.
pub type Location = u16;
/// Represents a chip8 register.
pub type Register = u8;