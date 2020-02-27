#[macro_use]
extern crate log;
extern crate console_log;
extern crate hex;
extern crate regex;

use crate::emulator::LstParser;
use wasm_bindgen::prelude::*;

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
    parser: Option<emulator::LstParser>,
}

#[wasm_bindgen]
impl EmulatorEngine {
    pub fn new() -> Self {
        console_log::init_with_level(log::Level::Debug).unwrap();
        info!("Initialising PIC16F8X emulator engine");

        let mut tmp = Self {
            cpu: emulator::CPU::new(),
            parser: None,
        };

        tmp
    }

    pub fn run_step(&mut self) -> usize {
        self.cpu.step();

        if let Some(parser) = &self.parser {
            if let Some(idx) = parser.address_info.get(&self.cpu.data_bus.get_pc()) {
                *idx
            } else {
                0
            }
        } else {
            0
        }
    }

    pub fn reset(&mut self) -> usize {
        let (start_addr, _) = self.cpu.rom_bus.get_rom_boundary();
        self.cpu.data_bus.set_pc(start_addr);

        if let Some(parser) = &self.parser {
            if let Some(idx) = parser.address_info.get(&start_addr) {
                *idx
            } else {
                0
            }
        } else {
            0
        }
    }

    pub fn load_lst_file(&mut self, content: &str) {
        let parser = LstParser::from_lst_file(String::from(content));
        self.cpu.rom_bus.load_program(parser.bytecode.as_slice(), 0);
        self.parser = Some(parser);
    }

    pub fn get_debug_info_line_count(&self) -> usize {
        if let Some(parser) = &self.parser {
            parser.content.len()
        } else {
            0
        }
    }

    pub fn get_debug_info_line(&self, index: usize) -> String {
        if let Some(parser) = &self.parser {
            parser.content[index].1.clone()
        } else {
            String::from("Something veryyy wrong happened")
        }
    }

    pub fn get_debug_info_line_label(&self, index: usize) -> String {
        if let Some(parser) = &self.parser {
            parser.content[index].0.clone()
        } else {
            String::from("Something veryyy wrong happened")
        }
    }

    pub fn run_example(&mut self) {
        self.cpu.data_bus.write_byte(0x7f, 0xFF);
        debug!("Status is {:02x}", self.cpu.data_bus.sfr_bank.status);
        self.cpu.data_bus.sfr_bank.status += 1;
    }

    pub fn read_sfrs(&self) -> emulator::SFRBank {
        self.cpu.data_bus.sfr_bank
    }

    pub fn set_status(&mut self, value: u8) {
        self.cpu.data_bus.sfr_bank.status = value;
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

#[test]
fn disassemble_rom() {
    let mut cpu = emulator::CPU::new();
    cpu.rom_bus
        .load_program(include_bytes!("../SimTest01.bin"), 0);
    let (min, max) = cpu.rom_bus.get_rom_boundary();

    while cpu.data_bus.get_pc() <= max {
        cpu.step();
    }
}

#[test]
fn disassemble_lst_file() {
    let file = include_str!("../SimTest01.LST");
    let mut parser = LstParser::from_lst_file(String::from(file));

    for (address, index) in parser.address_info {
        println!("{:04x} -> {} = ('{}' - '{}')", address, index, parser.content[index].0, parser.content[index].1);
    }

    for (label, command) in parser.content {
        println!("label: '{}', command: '{}'", label, command)
    }
}
