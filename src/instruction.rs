//the type to represent instructions
#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    CLEAR(u8), //000
    INC,       //001
    ADD(u8),   //010
    DEC,       //011
    JMP(u8),   //100
    BNZ(u8),   //101
    LOAD(u8),  //110
    STORE(u8), //111
}

//methods to convert from/to our enum format
//STOP is represented internally as a CLEAR with a non-zero operand
impl Instruction {
    pub fn assemble(&self) -> Option<u8> {
        match *self {
            Instruction::CLEAR(op) => Some(0b000_00000 | op),
            Instruction::INC => Some(0b001_00000),
            Instruction::ADD(op) if op < 32 => Some(0b010_00000 | op),
            Instruction::DEC => Some(0b011_00000),
            Instruction::JMP(op) if op < 32 => Some(0b100_00000 | op),
            Instruction::BNZ(op) if op < 32 => Some(0b101_00000 | op),
            Instruction::LOAD(op) if op < 32 => Some(0b110_00000 | op),
            Instruction::STORE(op) if op < 32 => Some(0b111_00000 | op),
            _ => None,
        }
    }

    pub fn disassemble(byte: u8) -> Self {
        let opcode = (byte & 0b111_00000) >> 5;
        let operand = byte & 0b000_11111;
        match opcode {
            0b000 => Self::CLEAR(operand),
            0b001 => Self::INC,
            0b010 => Self::ADD(operand),
            0b011 => Self::DEC,
            0b100 => Self::JMP(operand),
            0b101 => Self::BNZ(operand),
            0b110 => Self::LOAD(operand),
            0b111 => Self::STORE(operand),
            _ => panic!("not sure how this could ever occur but okay rust"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // test a few instructions disassemble correctly
    #[test]
    fn test_disassemble() {
        assert_eq!(Instruction::disassemble(0), Instruction::CLEAR(0));
        assert_eq!(Instruction::disassemble(1), Instruction::CLEAR(1));
        assert_eq!(Instruction::disassemble(0b001_00000), Instruction::INC);
        assert_eq!(Instruction::disassemble(0b011_10101), Instruction::DEC);
        assert_eq!(Instruction::disassemble(0b100_11111), Instruction::JMP(31));
        assert_eq!(Instruction::disassemble(0b101_01100), Instruction::BNZ(12));
        assert_eq!(Instruction::disassemble(0b111_00001), Instruction::STORE(1));
    }

    //test they assemble correctly too
    #[test]
    fn test_assemble() {
        assert_eq!(Instruction::CLEAR(0).assemble(), Some(0));
        assert_eq!(Instruction::CLEAR(1).assemble(), Some(1));
        assert_eq!(Instruction::INC.assemble(), Some(0b001_00000));
        assert_eq!(Instruction::ADD(1).assemble(), Some(0b010_00001));
        assert_eq!(Instruction::DEC.assemble(), Some(0b011_00000));
        assert_eq!(Instruction::JMP(7).assemble(), Some(0b100_00111));
        assert_eq!(Instruction::BNZ(6).assemble(), Some(0b101_00110));
        assert_eq!(Instruction::LOAD(15).assemble(), Some(0b110_01111));
        assert_eq!(Instruction::STORE(31).assemble(), Some(0b111_11111));
        // should None
        assert_eq!(Instruction::ADD(68).assemble(), None);
    }
}
