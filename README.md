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
  * [ ] Background scolling
  * [ ] Draw window
  * [x] Draw sprites
  * [x] Handle flipped sprites
  * [ ] Handle 8x16 sprites
* [x] Frontends
  * [x] Desktop
  * [x] Web
  * [x] Terminal

## Working Games

* [x] Tetris
* [x] Dr. Mario
* [ ] The Legend of Zelda: Link's Awakening
* [ ] Kirby's Dream Land
* [ ] Super Mario Land
* [ ] Pokemon Red/Blue
