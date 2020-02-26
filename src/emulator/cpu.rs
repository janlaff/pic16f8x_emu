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

    pub fn step(&mut self) {
        let pc = self.data_bus.get_pc();
        let result = self.rom_bus.read_instruction(pc);

        if let Ok(instr) = result {
            println!("Executing {:?}", instr);
        } else {
            eprintln!("Error: {}", result.err().unwrap());
        }

        self.data_bus.inc_pc(2);
    }
}
