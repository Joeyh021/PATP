use super::cpu::CPU;
use crate::instruction::Instruction;

#[test]
fn test_cpu_execute() {
    assert_eq!(
        CPU::execute(CPU::new(), Instruction::CLEAR.assemble()),
        CPU::new()
    );
}
