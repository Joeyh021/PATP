//the type to represent instructions
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

//a method for instructions to convert them to their binary
impl Instruction {
    pub fn assemble(&self) -> u8 {
        match *self {
            Instruction::CLEAR => 0x00,
            Instruction::INC => 0x01,
            Instruction::ADD(op) => (0x02 << 5) | op,
            Instruction::DEC => 0x03,
            Instruction::JMP(op) if op < 32 => (0x04 << 5) | op,
            Instruction::BNZ(op) if op < 32 => (0x05 << 5) | op,
            Instruction::LOAD(op) if op < 32 => (0x06 << 5) | op,
            Instruction::STORE(op) if op < 32 => (0x07 << 5) | op,
            _ => panic!("bad mem addr"),
        }
    }
}
