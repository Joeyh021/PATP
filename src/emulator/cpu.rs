use crate::instruction::Instruction;

#[derive(Debug, PartialEq, Eq)]
pub struct CPU {
    memory: Box<[u8; 32]>,
    z: bool,
    register: u8,
    pc: u8,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            memory: Box::new([0; 32]),
            z: true,
            register: 0,
            pc: 0,
        }
    }

    //mutates the given state 
    pub fn execute(&mut self, byte: u8) -> bool {
        let instruction = Instruction::disassemble(byte);

        //increase pc
        self.pc = Self::inc_pc(self.pc);

        match instruction {
            Instruction::CLEAR(0) => {
                self.register = 0;
                self.z = true;
            }
            Instruction::CLEAR(_) =>{
                return false;
            }
            Instruction::INC => {
                self.register = u8::wrapping_add(self.register, 1);
                self.z = if self.register == 0 { true } else { false };
            }
            Instruction::ADD(op) => {
                self.register = u8::wrapping_add(self.register, op);
                self.z = if self.register == 0 { true } else { false };
            }
            Instruction::DEC => {
                self.register = u8::wrapping_sub(self.register, 1);
                self.z = if self.register == 0 { true } else { false };
            }
            Instruction::JMP(op) => self.pc = op,
            Instruction::BNZ(op) => {
                if !self.z {
                    self.pc = op
                }
            }
            //cant refer to old memory, but can just refer to new instead because we dont modify it anywhere else
            Instruction::LOAD(op) => self.register = self.memory[op as usize],
            Instruction::STORE(op) => self.memory[op as usize] = self.register,
        }

        return true;
    }

    //wrap at 5 bits (cant go past 31)
    #[inline]
    fn inc_pc(old: u8) -> u8 {
        match old {
            31 => 0,
            _ => old + 1,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instruction::Instruction;

    #[test]
    fn test_cpu_execute_single() {
        // all easy tests of a single instruction
        
        assert_eq!(
            CPU::execute(CPU::new(), Instruction::CLEAR.assemble()),
            CPU {
                pc: 1,
                ..CPU::new()
            }
        );
        assert_eq!(
            CPU::execute(CPU::new(), Instruction::INC.assemble()),
            CPU {
                pc: 1,
                register: 1,
                z: false,
                ..CPU::new()
            }
        );
        assert_eq!(
            CPU::execute(CPU::new(), Instruction::ADD(12).assemble()),
            CPU {
                register: 12,
                pc: 1,
                z: false,
                ..CPU::new()
            }
        );
        assert_eq!(
            CPU::execute(CPU::new(), Instruction::DEC.assemble()),
            CPU {
                register: 255,
                pc: 1,
                z: false,
                ..CPU::new()
            }
        );
        assert_eq!(
            CPU::execute(CPU::new(), Instruction::JMP(17).assemble()),
            CPU {
                pc: 17,
                ..CPU::new()
            }
        );

        //bnz when z is true
        //shouldn't branch
        assert_eq!(
            CPU::execute(CPU::new(), Instruction::BNZ(21).assemble()),
            CPU {
                pc: 1,
                ..CPU::new()
            }
        );

        //bnz when z is false
        //should branch
        assert_eq!(
            CPU::execute(
                CPU {
                    z: false,
                    ..CPU::new()
                },
                Instruction::BNZ(21).assemble()
            ),
            CPU {
                pc: 21,
                z: false,
                ..CPU::new()
            }
        );

        //store number 11 at address 1
        assert_eq!(
            CPU::execute(
                CPU {
                    register: 11,
                    ..CPU::new()
                },
                Instruction::STORE(1).assemble()
            ),
            CPU {
                pc: 1,
                register: 11,
                memory: Box::new([
                    0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0
                ]),
                ..CPU::new()
            }
        );

        //load number 11 from address 2
        assert_eq!(
            CPU::execute(
                CPU {
                    memory: Box::new([
                        0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0
                    ]),
                    ..CPU::new()
                },
                Instruction::LOAD(2).assemble()
            ),
            CPU {
                pc: 1,
                register: 11,
                memory: Box::new([
                    0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0
                ]),
                ..CPU::new()
            }
        );
    }

    #[test]
    fn test_cpu_execute_program() {
        //runs a few steps in sequence, testing the CPU state is as it should be at each step
        let mut cpu = CPU::new();
        cpu = cpu.execute(Instruction::INC.assemble());

        assert_eq!(
            CPU {
                pc: 1,
                register: 1,
                z: false,
                ..CPU::new()
            },
            cpu
        );

        cpu = cpu.execute(Instruction::INC.assemble());

        assert_eq!(
            CPU {
                pc: 2,
                register: 2,
                z: false,
                ..CPU::new()
            },
            cpu
        );

        cpu = cpu.execute(Instruction::ADD(9).assemble());

        assert_eq!(
            CPU {
                pc: 3,
                register: 11,
                z: false,
                ..CPU::new()
            },
            cpu
        );

        cpu = cpu.execute(Instruction::DEC.assemble());

        assert_eq!(
            CPU {
                pc: 4,
                register: 10,
                z: false,
                ..CPU::new()
            },
            cpu
        );

        cpu = cpu.execute(Instruction::JMP(20).assemble());

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
                CPU {
                    pc: 0,
                    register: 255,
                    z: false,
                    ..CPU::new()
                },
                Instruction::INC.assemble()
            ),
            CPU {
                pc: 1,
                register: 0,
                z: true,
                ..CPU::new()
            }
        );
    }
}
