# PATP toolkit

This is an assembler and emulator for the PATP (Pedagogically Advanced Teaching Processor), a simple microprocessor designed by Dr Matthew Leeke for teaching CS132 at The University of Warwick. The processor specification and assembly language documentation is outlined in [the specification](Specification.md).

## Usage

The project is built with `cargo`.

- `cargo test` will run all the unit tests
- `cargo run -- [emulate/assemble] <file>` will run the program in your chosen mode
- `cargo build --release` will spit out an executable which can then be run in the usual way on your system of choice
