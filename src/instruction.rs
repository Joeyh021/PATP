//the type to represent instructions
#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    Clear(u8), //000
    Inc,       //001
    Add(u8),   //010
    Dec,       //011
    Jump(u8),  //100
    Bnz(u8),   //101
    Load(u8),  //110
    Store(u8), //111
}

//methods to convert from/to our enum format
//STOP is represented internally as a CLEAR with a non-zero operand
impl Instruction {
    pub fn assemble(&self) -> Option<u8> {
        match *self {
            Instruction::Clear(op) => Some(op),
            Instruction::Inc => Some(0b0010_0000),
            Instruction::Add(op) if op < 32 => Some(0b0100_0000 | op),
            Instruction::Dec => Some(0b0110_0000),
            Instruction::Jump(op) if op < 32 => Some(0b1000_0000 | op),
            Instruction::Bnz(op) if op < 32 => Some(0b1010_0000 | op),
            Instruction::Load(op) if op < 32 => Some(0b1100_0000 | op),
            Instruction::Store(op) if op < 32 => Some(0b1110_0000 | op),
            _ => None,
        }
    }

    pub fn disassemble(byte: u8) -> Self {
        let opcode = (byte & 0b1110_0000) >> 5;
        let operand = byte & 0b0001_1111;
        match opcode {
            0b000 => Self::Clear(operand),
            0b001 => Self::Inc,
            0b010 => Self::Add(operand),
            0b011 => Self::Dec,
            0b100 => Self::Jump(operand),
            0b101 => Self::Bnz(operand),
            0b110 => Self::Load(operand),
            0b111 => Self::Store(operand),
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
        assert_eq!(Instruction::disassemble(0), Instruction::Clear(0));
        assert_eq!(Instruction::disassemble(1), Instruction::Clear(1));
        assert_eq!(Instruction::disassemble(0b0010_0000), Instruction::Inc);
        assert_eq!(Instruction::disassemble(0b0111_0101), Instruction::Dec);
        assert_eq!(Instruction::disassemble(0b1001_1111), Instruction::Jump(31));
        assert_eq!(Instruction::disassemble(0b1010_1100), Instruction::Bnz(12));
        assert_eq!(Instruction::disassemble(0b1110_0001), Instruction::Store(1));
    }

    //test they assemble correctly too
    #[test]
    fn test_assemble() {
        assert_eq!(Instruction::Clear(0).assemble(), Some(0));
        assert_eq!(Instruction::Clear(1).assemble(), Some(1));
        assert_eq!(Instruction::Inc.assemble(), Some(0b0010_0000));
        assert_eq!(Instruction::Add(1).assemble(), Some(0b0100_0001));
        assert_eq!(Instruction::Dec.assemble(), Some(0b0110_0000));
        assert_eq!(Instruction::Jump(7).assemble(), Some(0b1000_0111));
        assert_eq!(Instruction::Bnz(6).assemble(), Some(0b1010_0110));
        assert_eq!(Instruction::Load(15).assemble(), Some(0b1100_1111));
        assert_eq!(Instruction::Store(31).assemble(), Some(0b1111_1111));
        // should None
        assert_eq!(Instruction::Add(68).assemble(), None);
    }
}
