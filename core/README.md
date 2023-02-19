# rustyboy (core)

## Architecture

This emulator is not designed to be a direct 1-to-1 recreation of the internals of the Game Boy. Instead, focus is placed primarily on recreating the actual behaviour of the system.

The real Game Boy is, in high-level terms, composed of individual chips which interact primarily via an address bus. The purpose of this bus is to map 16-bit addresses to the appropriate pieces of hardware, whether that may be working memory, timer registers, ROM on the game cartirdge, etc. Recreating this setup by having a memory bus that holds references to all the pieces of hardware it maps to could easily become a lifetime nightmare or at least become a mess of different references pointing all over the place.

Instead, it was decided that the memory bus would 'own' the different pieces of hardware to which it may map to addresses. The structure would look something like this (think of each element as a Rust `struct` with its members listed in curly braces):

```
memory {
  memory bank controller
  working RAM
  high RAM
  timer {
    divider
    counter
    modulo
  }
  interrupts {
    enable
    flag
  }
  GPU {
    video RAM
  }
  ...
}
```

Then the Game Boy itself would simply be defined as a pair consisting of the CPU and the memory structure above.

```
game boy {
  CPU
  memory
}
```

As instructions are executed on the CPU, the number of cycles taken to execute each instruction is taken and passed to an 'update' method on the memory structure so that the timer, GPU, etc. can be appropriately updated.

## Roadmap

* [x] CPU
  * [x] Registers and flags
  * [x] Implement instructions
  * [x] Interrupt handling
  * [x] Pass all Blargg test ROMs
* [x] Memory map
* [ ] Timer
* [ ] Cartridges
  * [ ] No MBC
  * [ ] MBC-1
  * [ ] MBC-3
  * [ ] MBC-5
* [ ] Joypad input
* [ ] Audio
