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

fn print_operand(op: ReilOperand) -> String {
    match op {
        ReilOperand::IntegerLiteral{size:s, value:v} => format!("{:x}/b{}", v, s),
        ReilOperand::Register{index:i, size:s} => format!("t{}/b{}", i, s),
        ReilOperand::Subaddress{size:s, value:v} => format!("{:x}/b{}", v, s),
    }
}

pub fn print_instruction(instr: ReilInstruction) -> String {
    match instr {
    ReilInstruction::Add(in1, in2, out) => format!("add {}, {}, {}", print_operand(in1),
        print_operand(in2), print_operand(out)),
    _ => "unkn".to_string(),
    }
}
