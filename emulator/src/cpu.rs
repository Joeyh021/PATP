use super::instruction::Instruction;

struct CPU {
    memory: Box<[u8; 32]>,
    zero_flag: bool,
    register: u8,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            memory: Box::new([0; 32]),
            zero_flag: false,
            register: 0,
        }
    }

    pub fn execute(state: CPU, byte: u8) -> CPU {
        let instruction = Instruction::disassemble(byte);
        CPU::new()
    }
}
