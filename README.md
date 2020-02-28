# pic16f8x_emu
This is my personal implementation of an emulator for the PIC16F8X architecture.

## Features:
- Working cpu emulation
- Working user interface (GUI)

### CPU Emulation:
- [PIC16F8X Datasheet](https://moodle.dhbw.de/pluginfile.php/95874/mod_resource/content/2/PIC16F8x.pdf)
- [PIC16F8X Handbook](https://moodle.dhbw.de/pluginfile.php/101988/mod_resource/content/2/Themenblatt_PIC_programmieren.pdf)

### User Interface:
![Screenshot](screenshots/screenshot.png)
- Vue.js [WebAssembly](https://webassembly.org/)

## Dependencies
- Rust Compiler (https://www.rust-lang.org/tools/install)
- wasm-pack (`cargo install wasm-pack`)
- npm (https://nodejs.org/en/) 

## Build & Run
### Build emulator engine
```
$ cd emulator_engine
$ wasm-pack build
```

### Run frontend
```
$ cd vue-frontend
$ npm install
$ npm run serve
```
