mod cpu;

pub fn emulate(program: &[u8]) -> cpu::CPU {
    program.iter().fold(cpu::CPU::new(), |state, instruction| {
        cpu::CPU::execute(state, *instruction)
    })
}
