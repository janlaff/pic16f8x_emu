#[macro_use]
extern crate log;

extern crate console_log;

mod emulator;
mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn initialize_emulator() {
    console_log::init_with_level(log::Level::Debug).unwrap();
    info!("Initialising PIC16F8X emulation engine");

    let mut cpu = emulator::CPU::new();
    cpu.rom_bus
        .load_program(include_bytes!("../SimTest02.bin"), 0);
    let (min, max) = cpu.rom_bus.get_rom_boundary();

    while cpu.data_bus.get_pc() <= max {
        cpu.step();
    }
}
