use std::env;
use std::fs;
use itertools::Itertools;

enum Instruction{
    CLEAR,        //000
    INC,          //001
    ADD(u8),      //010
    DEC,          //011
    JMP(u8),      //100
    BUZ(u8),      //101
    BNZ(u8),
    BZC(u8),
    BNE(u8),
    LOAD(u8),     //110
    STORE(u8),    //111
}

impl Instruction {
    fn assemble(&self) -> u8{
        match *self{
            Instruction::CLEAR => 0x00,
            Instruction::INC => 0x01,
            Instruction::ADD(op) => (0x02 << 5) & op,
            Instruction::DEC => 0x03,
            Instruction::JMP(op) => (0x04 << 5) & op,
            Instruction::BUZ(op) | Instruction::BNZ(op) | Instruction::BZC(op) | Instruction::BNE(op) => (0x05 << 5) & op,
            Instruction::LOAD(op) => (0x06 << 5) & op,
            Instruction::STORE(op) => (0x07 << 5) & op,
        }
    }
}

//take a line, parse the relevant symbols/words, return the instruction it represents
//instructions are newline-seperated, and of format label: instruction opcode ; comment
fn parseline(line:&str) -> Instruction{
    let strip_comment: &str = line.splitn(2,";").collect::<Vec<&str>>()[0];
    if strip_comment.contains(":"){
        let instruction = strip_comment.splitn(2,":").collect::<Vec<&str>>()[1];
        let label = 
    }else{
        let instruction = strip_comment.splitn(2,":").collect::<Vec<&str>>()[0];
    }
    
    Instruction::CLEAR
}

fn main() {
    //get the args from the command line as a vector of Strings
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];
    let filetext: String = fs::read_to_string(filename).expect("File Error");

    let mut lineno: isize = 0;
    for line in filetext.lines(){
        let instr: Instruction = parseline(line);
    }
   
}
