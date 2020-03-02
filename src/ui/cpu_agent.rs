use serde::{Deserialize, Serialize};
use yew::prelude::worker::*;
use yew::prelude::*;
use yew::services::{ConsoleService, DialogService};
use yew::agent::Dispatched;

use crate::emulator::{parse_lst_file, ParseResult, SfrBank, CPU, parse_bin_file, PCL_ADDR};

pub struct CPUAgent {
    link: AgentLink<Self>,
    cpu: CPU,
    console: ConsoleService,
    dialog: DialogService,
    handlers: Vec<HandlerId>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    FetchSfrs,
    FetchMemory,
    UpdateMemory(u8, u8),
    LoadLstFile(String),
    LoadBinFile(Vec<u8>),
    Run,
    Step,
    Stop,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    Empty,
    FetchedMemory(Vec<u8>),
    FetchedSfrs(SfrBank),
    UpdatedMemory(u8, u8),
    LoadedProgram(ParseResult),
}

impl Agent for CPUAgent {
    type Reach = Context;
    type Message = ();
    type Input = Request;
    type Output = Response;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            cpu: CPU::new(),
            console: ConsoleService::new(),
            dialog: DialogService::new(),
            handlers: Vec::new(),
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.handlers.push(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.handlers.remove_item(&id);
    }

    fn update(&mut self, _: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, who: HandlerId) {
        match msg {
            Request::Run => {
                self.console.log("Tried to run program");
            }
            Request::Step => {
                self.console.log("Running experimental step");
                // TODO: find sth better to pass, that can respond directly to all handlers
                if let Err(err) = self.cpu.step(Self::dispatcher()) {
                    self.dialog.alert(err.as_str());
                }
            }
            Request::Stop => {
                self.console.log("Tried to stop program");
                self.cpu.data_bus.set_pc(self.cpu.rom_bus.get_rom_boundary().0);
                self.cpu.data_bus.sfr_bank = SfrBank::new();
                self.cpu.data_bus.memory = [0; 0x80];

                for id in &self.handlers {
                    self.link.respond(*id, Response::FetchedSfrs(self.cpu.data_bus.sfr_bank));
                    self.link.respond(*id, Response::FetchedMemory(self.cpu.data_bus.memory.to_vec()));
                    self.link.respond(*id, Response::UpdatedMemory(PCL_ADDR, self.cpu.data_bus.sfr_bank.pcl));
                }
            }
            Request::FetchMemory => {
                for id in &self.handlers {
                    self.link.respond(
                        *id,
                        Response::FetchedMemory(self.cpu.data_bus.memory.to_vec()),
                    );
                }
            }
            Request::UpdateMemory(address, value) => {
                for id in &self.handlers {
                    self.link
                        .respond(*id, Response::UpdatedMemory(address, value));
                }
            }
            Request::FetchSfrs => {
                for id in &self.handlers {
                    self.link
                        .respond(*id, Response::FetchedSfrs(self.cpu.data_bus.sfr_bank));
                }
            }
            Request::LoadBinFile(binary) => {
                self.console.log("Loading bin file");

                let result = parse_bin_file(binary.as_slice());
                self.cpu.rom_bus.load_program(result.bytecode.as_slice(), 0);
                self.cpu.data_bus.set_pc(self.cpu.rom_bus.get_rom_boundary().0);
                self.cpu.data_bus.sfr_bank = SfrBank::new();
                self.cpu.data_bus.memory = [0; 0x80];

                for id in &self.handlers {
                    self.link.respond(*id, Response::LoadedProgram(result.clone()));
                    self.link.respond(*id, Response::FetchedMemory(self.cpu.data_bus.memory.to_vec()));
                    self.link.respond(*id, Response::FetchedSfrs(self.cpu.data_bus.sfr_bank));
                    self.link.respond(*id, Response::UpdatedMemory(PCL_ADDR, self.cpu.data_bus.sfr_bank.pcl));
                }
            }
            Request::LoadLstFile(contents) => {
                self.console.log("Loading lst file");

                let result = parse_lst_file(contents.as_str());
                self.cpu.rom_bus.load_program(result.bytecode.as_slice(), 0);
                self.cpu.data_bus.set_pc(self.cpu.rom_bus.get_rom_boundary().0);
                self.cpu.data_bus.sfr_bank = SfrBank::new();
                self.cpu.data_bus.memory = [0; 0x80];

                for id in &self.handlers {
                    self.link.respond(*id, Response::LoadedProgram(result.clone()));
                    self.link.respond(*id, Response::FetchedMemory(self.cpu.data_bus.memory.to_vec()));
                    self.link.respond(*id, Response::FetchedSfrs(self.cpu.data_bus.sfr_bank));
                    self.link.respond(*id, Response::UpdatedMemory(PCL_ADDR, self.cpu.data_bus.sfr_bank.pcl));
                }
            }
        }
    }
}
