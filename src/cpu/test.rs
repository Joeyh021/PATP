//unit tests for cpu execution
#![cfg(test)]

use super::*;

#[test]
fn test_cpu_execute_single() {
    // all easy tests of a single instruction

    assert_eq!(
        Cpu::execute(&mut Cpu::new(), Instruction::Clear(0)),
        Some(Cpu {
            pc: 1,
            ..Cpu::new()
        })
    );
    assert_eq!(
        Cpu::execute(&mut Cpu::new(), Instruction::Inc),
        Some(Cpu {
            pc: 1,
            register: 1,
            z: false,
            ..Cpu::new()
        })
    );
    assert_eq!(
        Cpu::execute(&mut Cpu::new(), Instruction::Add(12)),
        Some(Cpu {
            register: 12,
            pc: 1,
            z: false,
            ..Cpu::new()
        })
    );
    assert_eq!(
        Cpu::execute(&mut Cpu::new(), Instruction::Dec),
        Some(Cpu {
            register: 255,
            pc: 1,
            z: false,
            ..Cpu::new()
        })
    );
    assert_eq!(
        Cpu::execute(&mut Cpu::new(), Instruction::Jump(17)),
        Some(Cpu {
            pc: 17,
            ..Cpu::new()
        })
    );

    //bnz when z is true
    //shouldn't branch
    assert_eq!(
        Cpu::execute(&mut Cpu::new(), Instruction::Bnz(21)),
        Some(Cpu {
            pc: 1,
            ..Cpu::new()
        })
    );

    //bnz when z is false
    //should branch
    assert_eq!(
        Cpu::execute(
            &mut Cpu {
                z: false,
                ..Cpu::new()
            },
            Instruction::Bnz(21)
        ),
        Some(Cpu {
            pc: 21,
            z: false,
            ..Cpu::new()
        })
    );

    //store number 11 at address 1
    assert_eq!(
        Cpu::execute(
            &mut Cpu {
                register: 11,
                ..Cpu::new()
            },
            Instruction::Store(1)
        ),
        Some(Cpu {
            pc: 1,
            register: 11,
            memory: [
                0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0
            ],
            ..Cpu::new()
        })
    );

    //load number 11 from address 2
    assert_eq!(
        Cpu::execute(
            &mut Cpu {
                memory: [
                    0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0
                ],
                ..Cpu::new()
            },
            Instruction::Load(2)
        ),
        Some(Cpu {
            pc: 1,
            register: 11,
            memory: [
                0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0
            ],
            ..Cpu::new()
        })
    );
}

#[test]
fn test_cpu_execute_program() {
    //runs a few steps in sequence, testing the CPU state is as it should be at each step
    let mut cpu = Cpu::new();
    cpu = cpu.execute(Instruction::Inc).unwrap();

    assert_eq!(
        Cpu {
            pc: 1,
            register: 1,
            z: false,
            ..Cpu::new()
        },
        cpu
    );

    cpu = cpu.execute(Instruction::Inc).unwrap();

    assert_eq!(
        Cpu {
            pc: 2,
            register: 2,
            z: false,
            ..Cpu::new()
        },
        cpu
    );

    cpu = cpu.execute(Instruction::Add(9)).unwrap();

    assert_eq!(
        Cpu {
            pc: 3,
            register: 11,
            z: false,
            ..Cpu::new()
        },
        cpu
    );

    cpu = cpu.execute(Instruction::Dec).unwrap();

    assert_eq!(
        Cpu {
            pc: 4,
            register: 10,
            z: false,
            ..Cpu::new()
        },
        cpu
    );

    cpu = cpu.execute(Instruction::Jump(20)).unwrap();

    assert_eq!(
        Cpu {
            pc: 20,
            register: 10,
            z: false,
            ..Cpu::new()
        },
        cpu
    );
}

#[test]
fn test_cpu_execute_edge_cases() {
    //test wraparound
    assert_eq!(
        Cpu::execute(
            &mut Cpu {
                pc: 0,
                register: 255,
                z: false,
                ..Cpu::new()
            },
            Instruction::Inc
        ),
        Some(Cpu {
            pc: 1,
            register: 0,
            z: true,
            ..Cpu::new()
        })
    );

    //test execution halts on a stop
    assert_eq!(Cpu::execute(&mut Cpu::new(), Instruction::Clear(1)), None);
}
