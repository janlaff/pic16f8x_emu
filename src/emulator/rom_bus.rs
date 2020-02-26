use std::collections::HashMap;

use super::instruction::*;

pub struct RomBus {
    rom: [u8; 0xffff],
    cache: HashMap<u8, Instruction>,
}

impl RomBus {
    pub fn new() -> Self {
        Self {
            rom: [0; 0xffff],
            cache: HashMap::new(),
        }
    }

    pub fn load_program(&mut self, program: &[u8], starting_address: u16) {
        for addr in 0..program.len() {
            let rom_addr = addr + starting_address as usize;
            self.rom[rom_addr] = program[addr];
        }
    }
}
