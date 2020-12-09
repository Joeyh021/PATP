use std::env;
use std::fs;

enum Instruction{
    CLEAR,        //000
    INC,          //001
    ADD(u8),      //010
    DEC,          //011
    JMP(u8),      //100     
    BNZ(u8),      //101
    LOAD(u8),     //110
    STORE(u8),    //111
}

impl Instruction {
    fn assemble(&self) -> u8{
        match *self{
            Instruction::CLEAR => 0x00,
            Instruction::INC => 0x01,
            Instruction::ADD(op) => (0x02 << 5) | op,
            Instruction::DEC => 0x03,
            Instruction::JMP(op) => (0x04 << 5) | op,
            Instruction::BNZ(op) => (0x05 << 5) | op,
            Instruction::LOAD(op) => (0x06 << 5) | op,
            Instruction::STORE(op) => (0x07 << 5) | op,
        }
    }
}

//take a line, parse the relevant symbols/words, return the instruction it represents
//instructions are newline-seperated, and of format label: instruction opcode ; comment
fn parseline(line:&str) -> Instruction{
    let strip_comment: &str = line.splitn(2,";").collect::<Vec<&str>>()[0];
    let mut instr: &str;
    if strip_comment.contains(":"){
        instr = strip_comment.splitn(2,":").collect::<Vec<&str>>()[1].trim();
        let label = strip_comment.splitn(2,":").collect::<Vec<&str>>()[0];
        //add to symbol table
    }else{
        instr = strip_comment.splitn(2,":").collect::<Vec<&str>>()[0].trim();
    }
    let opcode: &str = instr.splitn(2," ").collect::<Vec<&str>>()[0];
    let mut operand: &str = "";
    let mut operand_value:u8 = 0;

    if opcode != "CLEAR" && opcode != "INC" && opcode != "DEC"{
        operand = instr.splitn(2," ").collect::<Vec<&str>>()[1];
        operand_value = operand.parse().unwrap();
    }
    match opcode {
        "CLEAR" => Instruction::CLEAR,
        "INC" => Instruction::INC,
        "ADD" => Instruction::ADD(operand_value),
        "DEC" => Instruction::DEC,
        "JMP"=> Instruction::JMP(operand_value),
        "BUZ" | "BNZ" | "BZC"|"BNE"=> Instruction::BNZ(operand_value),
        "LOAD"=> Instruction::LOAD(operand_value),
        "STORE"=> Instruction::STORE(operand_value),
        _ => panic!("Error"),
    }
}

fn main() {
    //get the args from the command line as a vector of Strings
    let args: Vec<String> = env::args().collect();
    if args.len() < 2{
        panic!("please enter a filename")
    }
    let filename: &String = &args[1];
    let filetext: String = fs::read_to_string(filename).expect("Error reading from file");
    let mut binary: Vec<u8> = Vec::new();

    let mut lineno: isize = 0;
    for line in filetext.lines(){
        println!("instruction is {}", line);
        let instr: u8 = parseline(line).assemble();
        binary.push(instr);
        println!("binary is {:8b} ", instr);
        lineno+=1;
    }
    fs::write("output",&binary).expect("Error writing to file");
   
}
