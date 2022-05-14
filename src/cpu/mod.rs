use std::fmt;

use anyhow::Result;
use thiserror::Error;

use crate::instruction::Instruction;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Cpu {
    memory: [u8; 32],
    z: bool,
    register: u8,
    pc: u8,
}

//errors that may occur during CPU execution
#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("Program is too large to load into memory")]
    ProgramTooLarge,
    #[error("Could not assemble instruction{0}: operand is out of bounds (greater than 32)")]
    AssemblyError(Instruction),

    #[error("CPU has finished execution")]
    Stop(Cpu),
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
        Cpu::default()
    }

    pub fn fetch(&mut self) -> Instruction {
        let instruction = self.memory[self.pc as usize];
        //increase pc
        self.pc = Self::inc_pc(self.pc);
        Instruction::disassemble(instruction)
    }

    //executes a single instruction
    //consumes self and returns Ok(new state), or returns finishing state wrapped in an error if a STOP is hit
    pub fn execute(mut self, instruction: Instruction) -> Result<Cpu, Error> {
        match instruction {
            Instruction::Clear(0) => {
                self.register = 0;
                self.z = true;
            }
            Instruction::Clear(_) => {
                return Err(Error::Stop(self));
            }
            Instruction::Inc => {
                self.register = u8::wrapping_add(self.register, 1);
                self.z = self.register == 0;
            }
            Instruction::Add(op) => {
                self.register = u8::wrapping_add(self.register, op);
                self.z = self.register == 0;
            }
            Instruction::Dec => {
                self.register = u8::wrapping_sub(self.register, 1);
                self.z = self.register == 0;
            }
            Instruction::Jump(op) => self.pc = op,
            Instruction::Bnz(op) => {
                if !self.z {
                    self.pc = op
                }
            }
            Instruction::Load(op) => self.register = self.memory[op as usize],
            Instruction::Store(op) => self.memory[op as usize] = self.register,
        }

        Ok(self)
    }

    //wrap at 5 bits (cant go past 31)
    fn inc_pc(old: u8) -> u8 {
        match old {
            31 => 0,
            _ => old + 1,
        }
    }

    //takes a CPU and loads a program into it's memory
    pub fn load(mut self, program: &[u8]) -> Result<Cpu, Error> {
        if program.len() > 32 {
            Err(Error::ProgramTooLarge)
        } else {
            self.memory[..program.len()].copy_from_slice(program);
            Ok(self)
        }
    }
}
