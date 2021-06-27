use super::instruction::Instruction;

struct CPU {
    memory: Box<[u8; 32]>,
    zero_flag: bool,
    register: u8,
    pc: u8,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            memory: Box::new([0; 32]),
            zero_flag: false,
            register: 0,
            pc: 0,
        }
    }

    pub fn execute(state: CPU, byte: u8) -> CPU {
        let instruction = Instruction::disassemble(byte);
        match instruction {
            Instruction::CLEAR => CPU::new(),
            Instruction::INC => CPU {
                register: state.register + 1,
                ..state
            },
            Instruction::ADD(op) => CPU {
                register: state.register + op,
                ..state
            },
            Instruction::DEC => CPU {
                register: state.register - 1,
                ..state
            },
            Instruction::JMP(op) => CPU { pc: op, ..state },
            Instruction::BNZ(op) if state.zero_flag => CPU { pc: op, ..state },
            Instruction::BNZ(_) if !state.zero_flag => state,
            Instruction::LOAD(op) => CPU {
                register: state.memory[op as usize],
                ..state
            },
            Instruction::STORE(op) => CPU {
                memory: Self::store(state.memory, op, state.register),
                ..state
            },
            _ => panic!("bad instruction"),
        }
    }

    fn store(array: Box<[u8; 32]>, index: u8, num: u8) -> Box<[u8; 32]> {
        let mut array = array;
        array[index as usize] = num;
        return array;
    }
}
