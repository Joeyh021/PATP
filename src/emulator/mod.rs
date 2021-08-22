mod cpu;
use cpu::Cpu;
use std::{fs, path::Path};

fn execute_program(program: &[u8]) -> Cpu {
    let mut state = Cpu::new().load(program);
    loop {
        let instruction = state.memory[state.pc as usize];
        let new_state = state.execute(instruction);
        match new_state {
            None => return state,
            Some(new_state) => state = new_state,
        }
    }
}

pub fn emulate(path: &Path) -> Result<(), String> {
    let file = fs::read(path).map_err(|err| err.to_string())?;
    if file.len() > 32 {
        return Err(String::from("Program is too large to load."));
    }
    let final_state = execute_program(&file);
    println!("Final CPU State: \n{}", final_state);
    Ok(())
}
