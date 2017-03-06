use std::fmt;
use std::collections::HashMap;
use std::any::Any;
use std::option::Option;

pub type MetaData = HashMap<String, Box<Any>>;

pub enum Operand {
    IntegerLiteral { size: u8, value: i64 },
    Register { index: u64, size: u8 },
    Subaddress { size: u8, value: u64 },
    Empty,
}

pub enum Instruction {
    // Arithmetic instructions
    Add (Operand, Operand, Operand, Option<MetaData>),
    Sub (Operand, Operand, Operand, Option<MetaData>),
    Mul (Operand, Operand, Operand, Option<MetaData>),
    Div (Operand, Operand, Operand, Option<MetaData>),
    Mod (Operand, Operand, Operand, Option<MetaData>),
    Bsh (Operand, Operand, Operand, Option<MetaData>),

    // Bitwise instructions
    And (Operand, Operand, Operand, Option<MetaData>),
    Or (Operand, Operand, Operand, Option<MetaData>),
    Xor (Operand, Operand, Operand, Option<MetaData>),

    // Data transfer instructions
    Ldm (Operand, Operand, Operand, Option<MetaData>),
    Stm (Operand, Operand, Operand, Option<MetaData>),
    Str (Operand, Operand, Operand, Option<MetaData>),

    // Conditional instructions
    Bisz (Operand, Operand, Operand, Option<MetaData>),
    Jcc (Operand, Operand, Operand, Option<MetaData>),

    // Other instructions,
    Undef (Operand, Operand, Operand, Option<MetaData>),
    Unkn (Operand, Operand, Operand, Option<MetaData>),
    Nop (Operand, Operand, Operand, Option<MetaData>),
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Operand::IntegerLiteral { size: s, value: v } => write!(f, "{:x}/b{}", v, s),
            Operand::Register { index: i, size: s } => write!(f, "t{}/b{}", i, s),
            Operand::Subaddress { size: s, value: v } => write!(f, "{:x}/b{}", v, s),
            Operand::Empty => write!(f, ""),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Instruction::Add(ref in1, ref in2, ref out, _) => {
                write!(f, "add {}, {}, {}", in1, in2, out)
            }
            Instruction::Nop(_, _, _, _) => write!(f, "nop"),
            Instruction::Unkn(_, _, _, _) => write!(f, "unkn"),
            _ => write!(f, "unkn"),
        }
    }
}
