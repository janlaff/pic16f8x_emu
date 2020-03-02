use serde::{Deserialize, Serialize};
use yew::prelude::worker::*;
use yew::prelude::*;
use yew::services::ConsoleService;
use yew::agent::Dispatched;

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
    FetchSfrs,
    FetchMemory,
    UpdateMemory(u8, u8),
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
            Request::Run => {
                self.console.log("Tried to run program");
            }
            Request::Step => {
                self.console.log("Running experimental step");
                self.cpu.step(Self::dispatcher()).unwrap();
            }
            Request::Stop => {
                self.console.log("Tried to stop program");
                self.cpu.data_bus.set_pc(self.cpu.rom_bus.get_rom_boundary().0);
                self.cpu.data_bus.sfr_bank = SfrBank::new();
                self.cpu.data_bus.memory = [0; 0x80];

                for id in &self.handlers {
                    self.link.respond(*id, Response::FetchedSfrs(self.cpu.data_bus.sfr_bank));
                    self.link.respond(*id, Response::FetchedMemory(self.cpu.data_bus.memory.to_vec()));
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
        }
    }
}
