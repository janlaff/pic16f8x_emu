use super::data_bus::*;
use super::rom_bus::*;

pub struct CPU {
    pub w: u8,
    pub cycles: usize,
    pub data_bus: DataBus,
    pub rom_bus: RomBus,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            w: 0,
            cycles: 0,
            data_bus: DataBus::new(),
            rom_bus: RomBus::new(),
        }
    }
}
