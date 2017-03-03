use std::fmt;

pub enum Operand {
    IntegerLiteral {size: u8, value: i64},
    Register {index: u64, size: u8},
    Subaddress {size: u8, value: u64},
    Empty
}

pub enum Instruction {
    // Arithmetic instructions
    Add (Operand, Operand, Operand),
    Sub (Operand, Operand, Operand),
    Mul (Operand, Operand, Operand),
    Div (Operand, Operand, Operand),
    Mod (Operand, Operand, Operand),
    Bsh (Operand, Operand, Operand),

    // Bitwise instructions
    And (Operand, Operand, Operand),
    Or (Operand, Operand, Operand),
    Xor (Operand, Operand, Operand),

    // Data transfer instructions
    Ldm (Operand, Operand, Operand),
    Stm (Operand, Operand, Operand),
    Str (Operand, Operand, Operand),

    // Conditional instructions
    Bisz (Operand, Operand, Operand),
    Jcc (Operand, Operand, Operand),

    // Other instructions,
    Undef (Operand, Operand, Operand),
    Unkn (Operand, Operand, Operand),
    Nop (Operand, Operand, Operand),
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Operand::IntegerLiteral{size:s, value:v} => write!(f, "{:x}/b{}", v, s),
            Operand::Register{index:i, size:s} => write!(f, "t{}/b{}", i, s),
            Operand::Subaddress{size:s, value:v} => write!(f, "{:x}/b{}", v, s),
            Operand::Empty => write!(f, ""),
        }
    }
}

impl fmt::Display for Instruction{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Instruction::Add(ref in1, ref in2, ref out) => write!(f, "add {}, {}, {}", in1, in2, out),
            _ => write!(f, "unkn"),
        }
    }
}
