use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum CPUError {
    #[error("Program is too large to load into memory")]
    ProgramTooLarge,
    #[error("Could not assemble instruction{0}: operand is out of bounds (greater than 32)")]
    AssemblyError(super::Instruction),
}
