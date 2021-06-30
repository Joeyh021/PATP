mod cpu;

pub fn emulate(program: &[u8]) -> cpu::CPU {
     cpu::CPU::new()
}
