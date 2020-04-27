use super::data_bus::*;
use super::instruction::*;
use super::rom_bus::*;
use super::bits::*;

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

    pub fn run(&mut self, mut dispatcher: Dispatcher<CPUAgent>) -> Result<(), String> {
        // TODO: implement clock
        Ok(Default::default())
    }

    pub fn step(&mut self, mut dispatcher: Dispatcher<CPUAgent>) -> Result<(), String> {
        let pc = self.data_bus.get_pc();
        let result = self.rom_bus.read_instruction(pc);

        if let Ok(instr) = result {
            debug!("Executing {:?}", instr);
            self.execute(instr, &mut dispatcher);
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
        dispatcher.send(Request::UpdateMemory(PCL_ADDR, self.data_bus.sfr_bank.pcl));

        Ok(())
    }

    fn execute(&mut self, instruction: Instruction, dispatcher: &mut Dispatcher<CPUAgent>) {
        self.jump_performed = false;
        // TODO: Implement instructions

        match instruction {
            Instruction::Goto(Address(idx)) => {
                self.data_bus.load_pc(idx);
                self.jump_performed = true
            }
            Instruction::MovLw(Literal(value)) => {
                self.data_bus.sfr_bank.w = value;
            }
            Instruction::AndLw(Literal(value)) => {
                self.data_bus.sfr_bank.w &= value;
                set_bit_enabled(&mut self.data_bus.sfr_bank.status, Z, self.data_bus.sfr_bank.w == 0);
            }
            Instruction::BsF(FileRegister(destination), BitIndex(idx)) => {
                self.data_bus.set_bit(destination, idx);
            }
            Instruction::MovWf(FileRegister(destination)) => {
                self.data_bus.write_byte(destination, self.data_bus.sfr_bank.w);
            }
            Instruction::BcF(FileRegister(destination), BitIndex(idx)) => {
                self.data_bus.clear_bit(destination, idx);
            }
            Instruction::IorLw(Literal(value)) => {
                self.data_bus.sfr_bank.w |= value;
                set_bit_enabled(&mut self.data_bus.sfr_bank.status, Z, self.data_bus.sfr_bank.w == 0);
            }
            Instruction::SubLw(Literal(value)) => {
                let result = value - self.data_bus.sfr_bank.w;
                set_bit_enabled(&mut self.data_bus.sfr_bank.status, Z, result == 0);
                set_bit_enabled(&mut self.data_bus.sfr_bank.status, C, result >= 0);

                let dc = (((value & 0xf) + (!self.data_bus.sfr_bank.w + 1)) & 0xF0) != 0;
                set_bit_enabled(&mut self.data_bus.sfr_bank.status, DC, dc);

                self.data_bus.sfr_bank.w = result;
            }
            Instruction::XorLw(Literal(value)) => {
                self.data_bus.sfr_bank.w ^= value;
                set_bit_enabled(&mut self.data_bus.sfr_bank.status, Z, self.data_bus.sfr_bank.w == 0);
            }
            Instruction::AddLw(Literal(value)) => {
                let (result, carry) = self.data_bus.sfr_bank.w.overflowing_add(value);

                set_bit_enabled(&mut self.data_bus.sfr_bank.status, Z, result == 0);
                set_bit_enabled(&mut self.data_bus.sfr_bank.status, C, carry);

                let dc = (((value & 0xf) + self.data_bus.sfr_bank.w) & 0xF0) != 0;
                set_bit_enabled(&mut self.data_bus.sfr_bank.status, DC, dc);

                self.data_bus.sfr_bank.w = result;
            }
            _ => {}
        };
    }
}
