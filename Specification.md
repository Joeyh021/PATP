# Processor Specification

The PATP is an 8-bit microprocessor with a 5-bit address space. The processor features:

- A whopping 32 bytes of memory
- A 5 bit program counter
- A single accumulator register
- A CCR with only one bit flag (set when the result of an operation is zero)

## Assembly language

- Each instruction is 8 bits: 3 for the opcode and 5 for the operand.
- Operands should be specified as denary numbers

| Opcode | Operand         | Description                                                                                                  |
| ------ | --------------- | ------------------------------------------------------------------------------------------------------------ |
| CLEAR  | None            | Set accumulator to 0 (and set Z flag).                                                                       |
| INC    | None            | Increment accumulator. Z is set if the result is 0.                                                          |
| DEC    | None            | Decrement accumulator. Z is set if the result is 0.                                                          |
| ADD    | Integer         | Add the operand to the accumulator, storing the result back in the accumulator. Z is set if the result is 0. |
| LOAD   | Memory location | Load the value from the location given into the accumulator.                                                 |
| STORE  | Memory location | Store the value in the accumulator at the memory location given.                                             |
| JMP    | Memory location | Jump to the instruction at the memory location given                                                         |
| BNZ    | Memory location | Branch to the instruction at the memory location given if the previous instruction set Z to 0.               |
| STOP   | None            | Halts execution.                                                                                             |

Programs are loaded into memory at 0, and instructions and data share memory. If you write a program larger than 32 bytes, the emulator won't load it.
