mod cpu;
use cpu::CPU;
use std::{fs, path::Path};

fn execute_program(program: &[u8]) -> CPU {
    let mut state = CPU::new().load(program);
    loop {
        let instruction = program[state.pc as usize];
        let new_state = state.execute(instruction);
        match new_state {
            None => return state,
            Some(new_state) => state = new_state,
        }
    }
}

pub fn emulate(path: &Path) {
    let file = fs::read(path).expect("Could not open file!");
    let final_state = execute_program(&file);
    println!("Final cpu state \n : {:?}", final_state);
}
