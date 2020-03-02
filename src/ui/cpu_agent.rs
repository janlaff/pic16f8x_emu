use serde::{Deserialize, Serialize};
use yew::prelude::worker::*;
use yew::prelude::*;
use yew::services::ConsoleService;
use yew::agent::Dispatched;

use super::ControlMsg;
use super::MemoryMsg;
use super::SfrMsg;
use crate::emulator::{parse_lst_file, ParseResult, SfrBank, CPU};

pub struct CPUAgent {
    link: AgentLink<Self>,
    cpu: CPU,
    console: ConsoleService,
    handlers: Vec<HandlerId>,
    program: Option<ParseResult>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    Control(ControlMsg),
    Memory(MemoryMsg),
    Sfr(SfrMsg),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    Empty,
    FetchedMemory(Vec<u8>),
    FetchedSfrs(SfrBank),
    UpdatedMemory(u8, u8),
}

impl Agent for CPUAgent {
    type Reach = Context;
    type Message = ();
    type Input = Request;
    type Output = Response;

    fn create(link: AgentLink<Self>) -> Self {
        let mut cpu = CPU::new();

        let result = parse_lst_file(include_str!("../../SimTest01.LST"));
        cpu.rom_bus.load_program(result.bytecode.as_slice(), 0);

        Self {
            link,
            cpu,
            console: ConsoleService::new(),
            handlers: Vec::new(),
            program: Some(result),
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
            Request::Control(control_msg) => match control_msg {
                ControlMsg::Run => {
                    self.console.log("Tried to run program");
                }
                ControlMsg::Step => {
                    self.console.log("Running experimental step");
                    // TODO: pass component link to raise individual memory updates
                    self.cpu.step(Self::dispatcher()).unwrap();
                }
                ControlMsg::Stop => {
                    self.console.log("Tried to stop program");
                    self.cpu.data_bus.set_pc(self.cpu.rom_bus.get_rom_boundary().0);

                    for id in &self.handlers {
                        self.link.respond(*id, Response::FetchedSfrs(self.cpu.data_bus.sfr_bank));
                    }
                }
            },
            Request::Memory(memory_msg) => match memory_msg {
                MemoryMsg::FetchMemory => {
                    for id in &self.handlers {
                        self.link.respond(
                            *id,
                            Response::FetchedMemory(self.cpu.data_bus.memory.to_vec()),
                        );
                    }
                }
                MemoryMsg::UpdateMemory(address, value) => {
                    self.cpu.data_bus.write_byte(address, value);

                    for id in &self.handlers {
                        self.link
                            .respond(*id, Response::UpdatedMemory(address, value));
                    }
                }
                _ => {}
            },
            Request::Sfr(sfr_msg) => match sfr_msg {
                SfrMsg::FetchSfrs => {
                    for id in &self.handlers {
                        self.link
                            .respond(*id, Response::FetchedSfrs(self.cpu.data_bus.sfr_bank));
                    }
                }
                _ => {}
            },
        }
    }
}
