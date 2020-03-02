use super::data_bus::*;
use super::instruction::*;
use super::rom_bus::*;

use crate::ui::{CPUAgent, Request, MemoryMsg, SfrMsg};
use yew::agent::Dispatcher;

pub struct CPU {
    pub cycles: usize,
    pub data_bus: DataBus,
    pub rom_bus: RomBus,
    jump_performed: bool,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            cycles: 0,
            data_bus: DataBus::new(),
            rom_bus: RomBus::new(),
            jump_performed: false,
        }
    }

    pub fn step(&mut self, mut dispatcher: Dispatcher<CPUAgent>) -> Result<(), String> {
        let pc = self.data_bus.get_pc();
        let result = self.rom_bus.read_instruction(pc);

        if let Ok(instr) = result {
            debug!("Executing {:?}", instr);
            self.execute(instr);
        } else {
            return Err(result.err().unwrap());
        }

        // If jump was performed one additional cycle has to be added
        self.cycles += if self.jump_performed {
            2
        } else {
            self.data_bus.inc_pc(1);
            1
        };

        dispatcher.send(Request::FetchSfrs);

        Ok(())
    }

    fn execute(&mut self, instruction: Instruction) {
        self.jump_performed = false;
        // TODO: Implement instructions
        match instruction {
            Instruction::Goto(Address(idx)) => {
                self.data_bus.set_pc(idx);
                self.jump_performed = true
            }
            _ => {}
        };
    }
}
