use std::collections::HashMap;

use super::bits::*;
use super::instruction::*;

pub struct RomBus {
    pub rom: [u8; 0xffff],
    min_rom_idx: u16,
    max_rom_idx: u16,
    cache: HashMap<u16, Instruction>,
}

impl RomBus {
    pub fn new() -> Self {
        Self {
            rom: [0; 0xffff],
            min_rom_idx: 0,
            max_rom_idx: 0,
            cache: HashMap::new(),
        }
    }

    pub fn load_program(&mut self, program: &[u8], starting_address: u16) {
        self.min_rom_idx = starting_address / 2;
        self.max_rom_idx = self.min_rom_idx + (program.len() as u16 - 1) / 2;
        for addr in 0..program.len() {
            let rom_addr = addr + starting_address as usize;
            self.rom[rom_addr] = program[addr];
        }
    }

    pub fn read_instruction(&mut self, index: u16) -> Result<Instruction, String> {
        debug!("Loading instruction at index {:04x}", index);

        if index > self.max_rom_idx || index < self.min_rom_idx {
            return Err(format!(
                "Tried to execute arbitrary data as code at index {:04x}",
                index
            ));
        }

        if let Some(instr) = self.cache.get(&index) {
            Ok(*instr)
        } else {
            let opcode = self.read_word(index * 2);
            debug!(
                "Opcode {:04x} not found in instruction cache! Trying to decode...",
                opcode
            );
            let decoded = Instruction::from(opcode);
            if let Ok(instr) = decoded {
                debug!("Opcode {:04x} decoded to {:?}", opcode, instr);
                self.cache.insert(index, instr);
            }
            decoded
        }
    }

    pub fn get_rom_boundary(&self) -> (u16, u16) {
        (self.min_rom_idx, self.max_rom_idx)
    }

    fn read_byte(&self, address: u16) -> u8 {
        let result = self.rom[address as usize];
        debug!("Reading address {:04x} from rom -> {:02x}", address, result);
        result
    }

    fn read_word(&self, address: u16) -> u16 {
        // Instruction are encoded in big endian
        join_bytes(self.read_byte(address), self.read_byte(address + 1))
    }
}
