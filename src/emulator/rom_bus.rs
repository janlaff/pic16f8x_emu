use std::collections::HashMap;

use super::bits::*;
use super::instruction::*;

pub struct RomBus {
    rom: [u8; 0xffff],
    min_rom_addr: u16,
    max_rom_addr: u16,
    cache: HashMap<u16, Instruction>,
}

impl RomBus {
    pub fn new() -> Self {
        Self {
            rom: [0; 0xffff],
            min_rom_addr: 0,
            max_rom_addr: 0,
            cache: HashMap::new(),
        }
    }

    pub fn load_program(&mut self, program: &[u8], starting_address: u16) {
        self.min_rom_addr = starting_address;
        self.max_rom_addr = starting_address + program.len() as u16 - 1;
        for addr in 0..program.len() {
            let rom_addr = addr + starting_address as usize;
            self.rom[rom_addr] = program[addr];
        }
    }

    pub fn read_instruction(&mut self, address: u16) -> Result<Instruction, String> {
        if address > self.max_rom_addr || address < self.min_rom_addr {
            return Err(format!(
                "Tried to execute arbitrary data as code at address {:04x}",
                address
            ));
        } else if address % 2 != 0 {
            // TODO: check if this rule applies for roms
            return Err(format!("Address {:04x} is not dividable by 2", address));
        }

        if let Some(instr) = self.cache.get(&address) {
            Ok(*instr)
        } else {
            let decoded = Instruction::from(self.read_word(address));
            if let Ok(instr) = decoded {
                self.cache.insert(address, instr);
            }
            decoded
        }
    }

    pub fn get_rom_boundary(&self) -> (u16, u16) {
        (self.min_rom_addr, self.max_rom_addr)
    }

    fn read_byte(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    fn read_word(&self, address: u16) -> u16 {
        // Instruction are encoded in big endian
        join_bytes(self.read_byte(address), self.read_byte(address + 1))
    }
}
