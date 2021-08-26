use super::error::CPUError;
use super::Instruction;
use anyhow::Result;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Cpu {
    pub memory: [u8; 32],
    z: bool,
    register: u8,
    pub pc: u8,
}

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Program Counter: {} \nRegister: {} \nZ flag: {} \nMemory: {:?}",
            self.pc, self.register, self.z as i32, self.memory
        )
    }
}
impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            memory: [0; 32],
            z: true,
            register: 0,
            pc: 0,
        }
    }

    //executes a single instruction, consuming the old state and returning a new one
    //returns None when finished executing
    pub fn execute(&self, byte: u8) -> Option<Cpu> {
        let instruction = Instruction::disassemble(byte);

        //make a new state from the old one
        let mut new_state: Cpu = self.clone();

        //increase pc
        new_state.pc = Self::inc_pc(self.pc);

        match instruction {
            Instruction::Clear(0) => {
                new_state.register = 0;
                new_state.z = true;
            }
            Instruction::Clear(_) => {
                return None;
            }
            Instruction::Inc => {
                new_state.register = u8::wrapping_add(self.register, 1);
                new_state.z = new_state.register == 0;
            }
            Instruction::Add(op) => {
                new_state.register = u8::wrapping_add(self.register, op);
                new_state.z = new_state.register == 0;
            }
            Instruction::Dec => {
                new_state.register = u8::wrapping_sub(self.register, 1);
                new_state.z = new_state.register == 0;
            }
            Instruction::Jump(op) => new_state.pc = op,
            Instruction::Bnz(op) => {
                if !self.z {
                    new_state.pc = op
                }
            }
            //cant refer to old memory, but can just refer to new instead because we dont modify it anywhere else
            Instruction::Load(op) => new_state.register = new_state.memory[op as usize],
            Instruction::Store(op) => new_state.memory[op as usize] = self.register,
        }

        Some(new_state)
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
    pub fn load(mut self, program: &[u8]) -> Result<Cpu, CPUError> {
        if program.len() > 32 {
            Err(CPUError::ProgramTooLarge)
        } else {
            self.memory[..program.len()].copy_from_slice(program);
            Ok(self)
        }
    }
}

//unit tests for cpu execution
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cpu_execute_single() {
        // all easy tests on blank CPU state
        assert_eq!(
            Cpu::execute(&Cpu::new(), Instruction::Clear(0).assemble().unwrap()),
            Some(Cpu {
                pc: 1,
                ..Cpu::new()
            })
        );
        assert_eq!(
            Cpu::execute(&Cpu::new(), Instruction::Inc.assemble().unwrap()),
            Some(Cpu {
                pc: 1,
                register: 1,
                z: false,
                ..Cpu::new()
            })
        );
        assert_eq!(
            Cpu::execute(&Cpu::new(), Instruction::Add(12).assemble().unwrap()),
            Some(Cpu {
                register: 12,
                pc: 1,
                z: false,
                ..Cpu::new()
            })
        );
        assert_eq!(
            Cpu::execute(&Cpu::new(), Instruction::Dec.assemble().unwrap()),
            Some(Cpu {
                register: 255,
                pc: 1,
                z: false,
                ..Cpu::new()
            })
        );
        assert_eq!(
            Cpu::execute(&Cpu::new(), Instruction::Jump(17).assemble().unwrap()),
            Some(Cpu {
                pc: 17,
                ..Cpu::new()
            })
        );

        //bnz when z is true
        //shouldn't branch
        assert_eq!(
            Cpu::execute(&Cpu::new(), Instruction::Bnz(21).assemble().unwrap()),
            Some(Cpu {
                pc: 1,
                ..Cpu::new()
            })
        );

        //bnz when z is false
        //should branch
        assert_eq!(
            Cpu::execute(
                &Cpu {
                    z: false,
                    ..Cpu::new()
                },
                Instruction::Bnz(21).assemble().unwrap()
            ),
            Some(Cpu {
                pc: 21,
                z: false,
                ..Cpu::new()
            })
        );

        //store number 11 at address 1
        assert_eq!(
            Cpu::execute(
                &Cpu {
                    register: 11,
                    ..Cpu::new()
                },
                Instruction::Store(1).assemble().unwrap()
            ),
            Some(Cpu {
                pc: 1,
                register: 11,
                memory: [
                    0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0
                ],
                ..Cpu::new()
            })
        );

        //load number 11 from address 2
        assert_eq!(
            Cpu::execute(
                &Cpu {
                    memory: [
                        0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0
                    ],
                    ..Cpu::new()
                },
                Instruction::Load(2).assemble().unwrap()
            ),
            Some(Cpu {
                pc: 1,
                register: 11,
                memory: [
                    0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0
                ],
                ..Cpu::new()
            })
        );
    }

    #[test]
    fn test_cpu_execute_program() {
        //runs a few steps in sequence, testing the CPU state is as it should be at each step
        let mut cpu = Cpu::new();
        cpu = cpu.execute(Instruction::Inc.assemble().unwrap()).unwrap();

        assert_eq!(
            Cpu {
                pc: 1,
                register: 1,
                z: false,
                ..Cpu::new()
            },
            cpu
        );

        cpu = cpu.execute(Instruction::Inc.assemble().unwrap()).unwrap();

        assert_eq!(
            Cpu {
                pc: 2,
                register: 2,
                z: false,
                ..Cpu::new()
            },
            cpu
        );

        cpu = cpu
            .execute(Instruction::Add(9).assemble().unwrap())
            .unwrap();

        assert_eq!(
            Cpu {
                pc: 3,
                register: 11,
                z: false,
                ..Cpu::new()
            },
            cpu
        );

        cpu = cpu.execute(Instruction::Dec.assemble().unwrap()).unwrap();

        assert_eq!(
            Cpu {
                pc: 4,
                register: 10,
                z: false,
                ..Cpu::new()
            },
            cpu
        );

        cpu = cpu
            .execute(Instruction::Jump(20).assemble().unwrap())
            .unwrap();

        assert_eq!(
            Cpu {
                pc: 20,
                register: 10,
                z: false,
                ..Cpu::new()
            },
            cpu
        );
    }

    #[test]
    fn test_cpu_execute_edge_cases() {
        //test wraparound
        assert_eq!(
            Cpu::execute(
                &Cpu {
                    pc: 0,
                    register: 255,
                    z: false,
                    ..Cpu::new()
                },
                Instruction::Inc.assemble().unwrap()
            ),
            Some(Cpu {
                pc: 1,
                register: 0,
                z: true,
                ..Cpu::new()
            })
        );

        //test execution halts on a stop
        assert_eq!(
            Cpu::execute(&Cpu::new(), Instruction::Clear(1).assemble().unwrap()),
            None
        );
    }
}
