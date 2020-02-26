use std::collections::HashMap;

use super::bits::*;
use super::instruction::*;

pub struct RomBus {
    rom: [u8; 0xffff],
    cache: HashMap<u16, Instruction>,
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

    pub fn read_instruction(&mut self, address: u16) -> Result<Instruction, String> {
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

    fn read_byte(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    fn read_word(&self, address: u16) -> u16 {
        // TODO: check endianess
        join_bytes(self.read_byte(address), self.read_byte(address + 1))
    }
}
