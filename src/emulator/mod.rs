mod cpu;
use cpu::CPU;
pub fn emulate(program: &[u8]) -> CPU {
    let state = CPU::new().load(program);
    CPU::new()
}
