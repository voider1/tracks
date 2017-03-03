use std::fmt;

pub enum ReilOperand {
    IntegerLiteral {size: u8, value: i64},
    Register {index: u64, size: u8},
    Subaddress {size: u8, value: u64},
}

pub enum ReilInstruction {
    // Arithmetic instructions
    Add (ReilOperand, ReilOperand, ReilOperand),
    Sub (ReilOperand, ReilOperand, ReilOperand),
    Mul (ReilOperand, ReilOperand, ReilOperand),
    Div (ReilOperand, ReilOperand, ReilOperand),
    Mod (ReilOperand, ReilOperand, ReilOperand),
    Bsh (ReilOperand, ReilOperand, ReilOperand),

    // Bitwise instructions
    And (ReilOperand, ReilOperand, ReilOperand),
    Or (ReilOperand, ReilOperand, ReilOperand),
    Xor (ReilOperand, ReilOperand, ReilOperand),

    // Data transfer instructions
    Ldm (ReilOperand, ReilOperand),
    Stm (ReilOperand, ReilOperand),
    Str (ReilOperand, ReilOperand),

    // Conditional instructions
    Bisz (ReilOperand, ReilOperand),
    Jcc (ReilOperand, ReilOperand),

    // Other instructions,
    Undef (ReilOperand),
    Unkn,
    Nop,
}

impl fmt::Display for ReilOperand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ReilOperand::IntegerLiteral{size:s, value:v} => write!(f, "{:x}/b{}", v, s),
            ReilOperand::Register{index:i, size:s} => write!(f, "t{}/b{}", i, s),
            ReilOperand::Subaddress{size:s, value:v} => write!(f, "{:x}/b{}", v, s),
        }
    }
}

impl fmt::Display for ReilInstruction{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ReilInstruction::Add(ref in1, ref in2, ref out) => write!(f, "add {}, {}, {}", in1, in2, out),
            _ => write!(f, "unkn"),
        }
    }
}
