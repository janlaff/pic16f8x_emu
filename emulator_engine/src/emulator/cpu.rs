use super::data_bus::*;
use super::instruction::*;
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

    pub fn step(&mut self) -> Result<(), String> {
        let pc = self.data_bus.get_pc();
        let result = self.rom_bus.read_instruction(pc);

        if let Ok(instr) = result {
            debug!("Executing {:?}", instr);
            self.execute(instr);
        } else {
            return Err(result.err().unwrap());
        }

        // If jump was performed one additional cycle has to be added
        self.cycles += if self.data_bus.get_pc() != pc {
            2
        } else {
            self.data_bus.inc_pc(1);
            1
        };

        Ok(())
    }

    fn execute(&mut self, instruction: Instruction) {
        // TODO: Implement instructions
    }
}
