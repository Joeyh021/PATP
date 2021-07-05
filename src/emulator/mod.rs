mod cpu;

use cpu::CPU;
pub fn emulate(program: &[u8]) -> CPU {
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
