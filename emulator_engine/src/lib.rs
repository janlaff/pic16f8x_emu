#[macro_use]
extern crate log;

extern crate console_log;

use wasm_bindgen::prelude::*;
use crate::emulator::SFRBank;

mod emulator;
mod utils;

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
struct EmulatorEngine {
    cpu: emulator::CPU,
}

#[wasm_bindgen]
impl EmulatorEngine {
    pub fn new() -> Self {
        console_log::init_with_level(log::Level::Debug).unwrap();
        info!("Initialising PIC16F8X emulator engine");

        let mut tmp = Self {
            cpu: emulator::CPU::new(),
        };

        tmp
    }

    pub fn run_example(&mut self) {
        self.cpu.data_bus.write_byte(0x7f, 0xFF);
        debug!("Status is {:02x}", self.cpu.data_bus.sfr_bank.status);
        self.cpu.data_bus.sfr_bank.status += 1;
    }

    pub fn read_sfrs(&self) -> SFRBank {
        self.cpu.data_bus.sfr_bank
    }

    pub fn ram(&self) -> *const u8 {
        self.cpu.data_bus.memory.as_ptr()
    }

    pub fn ram_size(&self) -> usize {
        self.cpu.data_bus.memory.len()
    }

    pub fn rom_size(&self) -> usize {
        self.cpu.rom_bus.rom.len()
    }

    pub fn rom(&self) -> *const u8 {
        self.cpu.rom_bus.rom.as_ptr()
    }
}

/*#[wasm_bindgen]
pub fn initialize_emulator() {
    let mut cpu = emulator::CPU::new();
    cpu.rom_bus
        .load_program(include_bytes!("../SimTest02.bin"), 0);
    let (min, max) = cpu.rom_bus.get_rom_boundary();

    while cpu.data_bus.get_pc() <= max {
        cpu.step();
    }
}*/
