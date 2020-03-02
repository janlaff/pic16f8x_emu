# pic16f8x_emu
This is my personal implementation of an emulator for the PIC16F8X architecture.
### [Link to live version (GitHub Pages)](https://proman0973.github.io/pic16f8x_emu/live/index.html)

![alt text](images/screenshot.png)

You can try the live version right now in your browser

## Features:
- Written in pure rust
- Multithreaded native frontend web application

## Used technologies
- [WebAssembly](https://webassembly.org/)
- [yew](https://yew.rs/)

### CPU Emulation:
- [PIC16F8X Datasheet (Private)](https://moodle.dhbw.de/pluginfile.php/95874/mod_resource/content/2/PIC16F8x.pdf)
- [PIC16F8X Handbook (Private)](https://moodle.dhbw.de/pluginfile.php/101988/mod_resource/content/2/Themenblatt_PIC_programmieren.pdf)

## Dependencies
- Rust Compiler (https://www.rust-lang.org/tools/install)
- Rust nightly toolchain (`rustup install nightly`)
- cargo-web (`cargo install cargo-web`) 

## Build & Run
```
$ chmod +x build_and_run.sh
$ ./build_and_run.sh
```
