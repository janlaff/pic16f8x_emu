# pic16f8x_emu
This is my personal implementation of an emulator for the PIC16F8X architecture.

## Features:
- Working cpu emulation
- Working user interface (GUI)

### CPU Emulation:
- [PIC16F8X Datasheet](https://moodle.dhbw.de/pluginfile.php/95874/mod_resource/content/2/PIC16F8x.pdf)
- [PIC16F8X Handbook](https://moodle.dhbw.de/pluginfile.php/101988/mod_resource/content/2/Themenblatt_PIC_programmieren.pdf)

### User Interface:
- Web Browser (Virtual DOM)
- [WebAssembly](https://webassembly.org/)

## Dependencies
- Rust Compiler (https://www.rust-lang.org/tools/install)
- cargo-web (`cargo install cargo-web`) 

## Build & Run
```
$ cargo web start --target wasm32-unknown-unknown
```