extern crate tracks;

#[test]
fn print_add() {
    let instr = tracks::ReilInstruction::Add(tracks::ReilOperand::IntegerLiteral{size: 1, value: 1}
    , tracks::ReilOperand::IntegerLiteral{size: 1, value: 1}, tracks::ReilOperand::Register{index: 1, size: 1});
    let res = tracks::print_instruction(instr);
    assert_eq!("add 1/b1, 1/b1, t1/b1", res);
}
