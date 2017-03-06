use std::fmt;
use std::collections::HashMap;
use std::any::Any;
use std::option::Option;

pub type MetaData<'a> = HashMap<&'a str, &'a Any>;

pub enum Operand {
    IntegerLiteral { size: u8, value: i64 },
    Register { index: u64, size: u8 },
    Subaddress { size: u8, value: u64 },
    Empty,
}

pub enum Instruction<'a> {
    // Arithmetic instructions
    Add (Operand, Operand, Operand, Option<&'a MetaData<'a>>),
    Sub (Operand, Operand, Operand, Option<&'a MetaData<'a>>),
    Mul (Operand, Operand, Operand, Option<&'a MetaData<'a>>),
    Div (Operand, Operand, Operand, Option<&'a MetaData<'a>>),
    Mod (Operand, Operand, Operand, Option<&'a MetaData<'a>>),
    Bsh (Operand, Operand, Operand, Option<&'a MetaData<'a>>),

    // Bitwise instructions
    And (Operand, Operand, Operand, Option<&'a MetaData<'a>>),
    Or (Operand, Operand, Operand, Option<&'a MetaData<'a>>),
    Xor (Operand, Operand, Operand, Option<&'a MetaData<'a>>),

    // Data transfer instructions
    Ldm (Operand, Operand, Operand, Option<&'a MetaData<'a>>),
    Stm (Operand, Operand, Operand, Option<&'a MetaData<'a>>),
    Str (Operand, Operand, Operand, Option<&'a MetaData<'a>>),

    // Conditional instructions
    Bisz (Operand, Operand, Operand, Option<&'a MetaData<'a>>),
    Jcc (Operand, Operand, Operand, Option<&'a MetaData<'a>>),

    // Other instructions,
    Undef (Operand, Operand, Operand, Option<&'a MetaData<'a>>),
    Unkn (Operand, Operand, Operand, Option<&'a MetaData<'a>>),
    Nop (Operand, Operand, Operand, Option<&'a MetaData<'a>>),
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

impl<'a> fmt::Display for Instruction<'a> {
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
