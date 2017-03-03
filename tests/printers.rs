extern crate tracks;

use tracks::il::reil::Operand;
use tracks::il::reil::Instruction;

#[test]
fn print_reil_add() {
    let instr = Instruction::Add(Operand::IntegerLiteral{size: 1, value: 1}
    , Operand::IntegerLiteral{size: 1, value: 1}, Operand::Register{index: 1, size: 1});
    let res = format!("{}", instr);
    assert_eq!("add 1/b1, 1/b1, t1/b1", res);
}
