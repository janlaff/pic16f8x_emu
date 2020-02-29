use serde::{Deserialize, Serialize};
use yew::prelude::worker::*;
use yew::prelude::*;
use yew::services::ConsoleService;

use super::ControlMsg;
use super::MemoryMsg;
use crate::emulator::CPU;

pub struct CPUAgent {
    link: AgentLink<Self>,
    cpu: CPU,
    console: ConsoleService,
    handlers: Vec<HandlerId>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    Control(ControlMsg),
    Memory(MemoryMsg),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    Empty,
    FetchedMemory(Vec<u8>),
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
            Request::Control(control_msg) => match control_msg {
                ControlMsg::Run => {
                    self.console.log("Tried to run program");
                }
                ControlMsg::Step => {
                    self.console.log("Tried to step program");
                }
                ControlMsg::Stop => {
                    self.console.log("Tried to stop program");
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
                _ => {}
            },
        }

        self.link.respond(who, Response::Empty)
    }
}
