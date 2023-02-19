# rustyboy

`rustyboy` is a simple Game Boy emulator implemented in Rust. It is currently a WIP - the end goal is get Pokemon Red/Blue running. An emphasis is placed on understandable code over creating a super accurate emulator.

## Project Structure

The Cargo workspace feature is used to keep the emulator itself separate from any particular rendering/window management/input handling frontend. All code relating to the emulator itself is found in the `core/` directory, while particular usages of that core library are found in all other directories. A high-level overview of the design of the core emulator implementation can be found in `core/README.md`.

The `macroquad/` directory contains the source code for a frontend to the emulator using the Macroquad library to handle rendering and input handling. This can be compiled to run on desktop as well as in the browser via WASM (see the `README.md` file in that directory for instructions).
