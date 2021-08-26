# PATP toolkit

This is an assembler and emulator for the PATP (Pedagogically Advanced Teaching Processor), a simple microprocessor designed for teaching CS132 Computer Organisation and Architecture at The University of Warwick. The processor specification and assembly language documentation is outlined in [the specification](Specification.md).

## Usage

The program can be installed with `cargo install patp` . If you don't already have cargo installed, see [here](https://www.rust-lang.org/tools/install) for instructions on how to install rust and cargo.

- `patp test` will run all the unit tests
- `patp run [--emulate/--assemble/--run] <file>` will run the program in your chosen mode
  - `patp emulate <file>` will execute a binary file and return the final CPU state
  - `patp assemble <file>` will assemble the `.patp` file and create a new binary file
  - `patp run <file>` will assemble and then execute a file

If you'd prefer to download the source and compile yourself, clone the repo and run `cargo build`.
