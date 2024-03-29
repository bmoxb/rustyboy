# rustyboy

`rustyboy` is a simple Game Boy emulator implemented in Rust. It is currently a
WIP - the end goal is get Pokemon Red/Blue running. An emphasis is placed on
understandable code over creating a super accurate emulator.

## Project Structure

The Cargo workspace feature is used to keep the emulator itself separate from
any particular rendering/window management/input handling frontend. All code
relating to the emulator itself is found in the `core/` directory, while
particular usages of that core library are found in other directories (see the
list of emulator frontends below).

## Emulator Frontends

* `wgpu/` - Targets desktop platforms (Linux, Mac, Windows) and the browser
  (WASM + WebGPU) via [pixels](https://github.com/parasyte/pixels).
* `terminal/` - Execute the emulator inside a terminal with the Game Boy display
  expressed with a grid of Unicode characters.
* `gbdoctor/` - Run the emulator without a display and the CPU state logged in
  the format expected by the [Game Boy
  Doctor](https://robertheaton.com/gameboy-doctor/) tool. This frontend exists
  for development and testing purposes.

## Roadmap

* [x] CPU
  * [x] Registers and flags
  * [x] Implement instructions
  * [x] Interrupt handling
  * [x] Pass all Blargg test ROMs
  * [ ] STOP instruction
* [x] Memory map
* [x] Timer
* [ ] Cartridges
  * [x] No MBC
  * [x] MBC-1
  * [ ] MBC-5
* [x] Joypad input
* [ ] Graphics
  * [x] Draw background
  * [x] Background scolling
  * [x] Draw window
  * [x] Draw sprites
  * [x] Handle flipped sprites
  * [x] Handle 8x16 sprites
  * [ ] Correct ordering of sprite and background tiles
* [x] Frontends
  * [x] Desktop
  * [x] Web
  * [x] Terminal
* [ ] Refactoring
  * [x] Remove unnecessary dependencies
  * [ ] Simplify cartridge/MBC API
  * [x] Make internal emulator state (e.g., CPU registers) publicly accessible
  * [x] Remove GB Doctor part of public API
  * [ ] Tidy up and better document GPU code
  * [x] Simplify cycle counting

## Working Games

* [x] Tetris
* [x] Dr. Mario
* [x] The Legend of Zelda: Link's Awakening
* [x] Kirby's Dream Land
* [x] Super Mario Land
* [ ] Pokemon Red/Blue
