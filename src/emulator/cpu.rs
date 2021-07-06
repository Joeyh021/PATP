use crate::instruction::Instruction;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CPU {
    pub memory: [u8; 32],
    z: bool,
    register: u8,
    pub pc: u8,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            memory: [0; 32],
            z: true,
            register: 0,
            pc: 0,
        }
    }

    //executes a single instruction, consuming the old state and returning a new one
    pub fn execute(&self, byte: u8) -> Option<CPU> {
        let instruction = Instruction::disassemble(byte);

        //make a new state from the old one
        let mut new_state: CPU = self.clone();

        //increase pc
        new_state.pc = Self::inc_pc(self.pc);

        match instruction {
            Instruction::CLEAR(0) => {
                new_state.register = 0;
                new_state.z = true;
            }
            Instruction::CLEAR(_) => {
                return None;
            }
            Instruction::INC => {
                new_state.register = u8::wrapping_add(self.register, 1);
                new_state.z = if new_state.register == 0 { true } else { false };
            }
            Instruction::ADD(op) => {
                new_state.register = u8::wrapping_add(self.register, op);
                new_state.z = if new_state.register == 0 { true } else { false };
            }
            Instruction::DEC => {
                new_state.register = u8::wrapping_sub(self.register, 1);
                new_state.z = if new_state.register == 0 { true } else { false };
            }
            Instruction::JMP(op) => new_state.pc = op,
            Instruction::BNZ(op) => {
                if !self.z {
                    new_state.pc = op
                }
            }
            //cant refer to old memory, but can just refer to new instead because we dont modify it anywhere else
            Instruction::LOAD(op) => new_state.register = new_state.memory[op as usize],
            Instruction::STORE(op) => new_state.memory[op as usize] = self.register,
        }

        return Some(new_state);
    }

    //wrap at 5 bits (cant go past 31)
    #[inline]
    fn inc_pc(old: u8) -> u8 {
        match old {
            31 => 0,
            _ => old + 1,
        }
    }

    //takes a CPU and loads a program into it's memory
    pub fn load(mut self, program: &[u8]) -> CPU {
        self.memory[..program.len()].copy_from_slice(program);
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instruction::Instruction;

    #[test]
    fn test_cpu_execute_single() {
        // all easy tests on blank CPU state
        assert_eq!(
            CPU::execute(&CPU::new(), Instruction::CLEAR(0).assemble().unwrap()),
            Some(CPU {
                pc: 1,
                ..CPU::new()
            })
        );
        assert_eq!(
            CPU::execute(&CPU::new(), Instruction::INC.assemble().unwrap()),
            Some(CPU {
                pc: 1,
                register: 1,
                z: false,
                ..CPU::new()
            })
        );
        assert_eq!(
            CPU::execute(&CPU::new(), Instruction::ADD(12).assemble().unwrap()),
            Some(CPU {
                register: 12,
                pc: 1,
                z: false,
                ..CPU::new()
            })
        );
        assert_eq!(
            CPU::execute(&CPU::new(), Instruction::DEC.assemble().unwrap()),
            Some(CPU {
                register: 255,
                pc: 1,
                z: false,
                ..CPU::new()
            })
        );
        assert_eq!(
            CPU::execute(&CPU::new(), Instruction::JMP(17).assemble().unwrap()),
            Some(CPU {
                pc: 17,
                ..CPU::new()
            })
        );

        //bnz when z is true
        //shouldn't branch
        assert_eq!(
            CPU::execute(&CPU::new(), Instruction::BNZ(21).assemble().unwrap()),
            Some(CPU {
                pc: 1,
                ..CPU::new()
            })
        );

        //bnz when z is false
        //should branch
        assert_eq!(
            CPU::execute(
                &CPU {
                    z: false,
                    ..CPU::new()
                },
                Instruction::BNZ(21).assemble().unwrap()
            ),
            Some(CPU {
                pc: 21,
                z: false,
                ..CPU::new()
            })
        );

        //store number 11 at address 1
        assert_eq!(
            CPU::execute(
                &CPU {
                    register: 11,
                    ..CPU::new()
                },
                Instruction::STORE(1).assemble().unwrap()
            ),
            Some(CPU {
                pc: 1,
                register: 11,
                memory: [
                    0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0
                ],
                ..CPU::new()
            })
        );

        //load number 11 from address 2
        assert_eq!(
            CPU::execute(
                &CPU {
                    memory: [
                        0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0
                    ],
                    ..CPU::new()
                },
                Instruction::LOAD(2).assemble().unwrap()
            ),
            Some(CPU {
                pc: 1,
                register: 11,
                memory: [
                    0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0
                ],
                ..CPU::new()
            })
        );
    }

    #[test]
    fn test_cpu_execute_program() {
        //runs a few steps in sequence, testing the CPU state is as it should be at each step
        let mut cpu = CPU::new();
        cpu = cpu.execute(Instruction::INC.assemble().unwrap()).unwrap();

        assert_eq!(
            CPU {
                pc: 1,
                register: 1,
                z: false,
                ..CPU::new()
            },
            cpu
        );

        cpu = cpu.execute(Instruction::INC.assemble().unwrap()).unwrap();

        assert_eq!(
            CPU {
                pc: 2,
                register: 2,
                z: false,
                ..CPU::new()
            },
            cpu
        );

        cpu = cpu
            .execute(Instruction::ADD(9).assemble().unwrap())
            .unwrap();

        assert_eq!(
            CPU {
                pc: 3,
                register: 11,
                z: false,
                ..CPU::new()
            },
            cpu
        );

        cpu = cpu.execute(Instruction::DEC.assemble().unwrap()).unwrap();

        assert_eq!(
            CPU {
                pc: 4,
                register: 10,
                z: false,
                ..CPU::new()
            },
            cpu
        );

        cpu = cpu
            .execute(Instruction::JMP(20).assemble().unwrap())
            .unwrap();

        assert_eq!(
            CPU {
                pc: 20,
                register: 10,
                z: false,
                ..CPU::new()
            },
            cpu
        );
    }

    #[test]
    fn test_cpu_execute_edge_cases() {
        //test wraparound
        assert_eq!(
            CPU::execute(
                &CPU {
                    pc: 0,
                    register: 255,
                    z: false,
                    ..CPU::new()
                },
                Instruction::INC.assemble().unwrap()
            ),
            Some(CPU {
                pc: 1,
                register: 0,
                z: true,
                ..CPU::new()
            })
        );

        //test execution halts on a stop
        assert_eq!(
            CPU::execute(&CPU::new(), Instruction::CLEAR(1).assemble().unwrap()),
            None
        );
    }
}
