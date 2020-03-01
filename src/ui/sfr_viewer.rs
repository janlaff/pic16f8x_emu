use yew::prelude::*;

use std::slice::Chunks;

use super::CPUAgent;
use super::MemoryMsg;
use super::Request;
use super::Response;

pub struct SfrViewer {
    link: ComponentLink<Self>,
    context: Box<dyn Bridge<CPUAgent>>,
}

impl Component for SfrViewer {
    type Message = MemoryMsg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|msg| MemoryMsg::ContextMsg(msg));
        Self {
            link,
            context: CPUAgent::bridge(callback),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            MemoryMsg::ContextMsg(Response::UpdatedMemory(address, value)) => {
                // TODO
                true
            }
            _ => false,
        }
    }

    fn view(&self) -> Html {
        html! {
            <h1>{ "Special Function Registers" }</h1>
        }
    }
}
