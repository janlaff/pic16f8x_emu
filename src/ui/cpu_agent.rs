use serde::{Deserialize, Serialize};
use yew::prelude::worker::*;
use yew::prelude::*;
use yew::services::ConsoleService;

use super::ControlMsg;
use crate::emulator::CPU;

pub struct CPUAgent {
    link: AgentLink<Self>,
    cpu: CPU,
    console: ConsoleService,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    Control(ControlMsg),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    Empty,
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
        }
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
        }

        self.link.respond(who, Response::Empty)
    }
}
