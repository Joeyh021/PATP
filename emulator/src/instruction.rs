//the type to represent instructions
#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    CLEAR,     //000
    INC,       //001
    ADD(u8),   //010
    DEC,       //011
    JMP(u8),   //100
    BNZ(u8),   //101
    LOAD(u8),  //110
    STORE(u8), //111
}

//methods to convert from/to our enum format
impl Instruction {
    pub fn assemble(&self) -> u8 {
        match *self {
            Instruction::CLEAR => 0x00,
            Instruction::INC => 0x01,
            Instruction::ADD(op) if op < 32 => (0x02 << 5) | op,
            Instruction::DEC => 0x03,
            Instruction::JMP(op) if op < 32 => (0x04 << 5) | op,
            Instruction::BNZ(op) if op < 32 => (0x05 << 5) | op,
            Instruction::LOAD(op) if op < 32 => (0x06 << 5) | op,
            Instruction::STORE(op) if op < 32 => (0x07 << 5) | op,
            _ => panic!("bad instruction"),
        }
    }

    pub fn disassemble(byte: u8) -> Self {
        let opcode = (byte & 0b111_00000) >> 5;
        let operand = byte & 0b000_11111;
        match opcode {
            0b000 => Self::CLEAR,
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
