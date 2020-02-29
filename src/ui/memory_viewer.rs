use yew::prelude::*;

use super::CPUAgent;
use super::Request;
use super::Response;

pub struct MemoryViewer {
    link: ComponentLink<Self>,
    context: Box<dyn Bridge<CPUAgent>>,
    memory: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MemoryMsg {
    ContextMsg(Response),
    FetchMemory,
}

impl Component for MemoryViewer {
    type Message = MemoryMsg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|msg| MemoryMsg::ContextMsg(msg));
        Self {
            link,
            context: CPUAgent::bridge(callback),
            memory: Vec::new(),
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        self.context.send(Request::Memory(MemoryMsg::FetchMemory));
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            MemoryMsg::ContextMsg(Response::FetchedMemory(memory)) => {
                self.memory = memory;
                true
            }
            _ => false,
        }
    }

    fn view(&self) -> Html {
        let render_cell = |cell| {
            html! {
                <li>{ cell }</li>
            }
        };

        html! {
            <ul>
                { for self.memory.iter().map(render_cell) }
            </ul>
        }
    }
}
