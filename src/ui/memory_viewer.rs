use yew::prelude::*;

use std::slice::Chunks;

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
        self.context.send(Request::FetchMemory);
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            MemoryMsg::ContextMsg(Response::FetchedMemory(memory)) => {
                self.memory = memory;
                true
            }
            MemoryMsg::ContextMsg(Response::UpdatedMemory(address, value)) => {
                self.memory[address as usize] = value;
                true
            }
            _ => false,
        }
    }

    fn view(&self) -> Html {
        let render_cell = |cell: &u8| {
            html! {
                <td>{ hex::encode([*cell]) }</td>
            }
        };

        let render_chunk = |(idx, chunk): (usize, &[u8])| {
            html! {
                <tr>
                    <td>{ hex::encode([(idx * chunk.len()) as u8]) }{ ":" }</td>
                    { for chunk.iter().map(render_cell) }
                </tr>
            }
        };

        html! {
            <div id="memory-viewer" class="nes-container is-dark with-title">
                <p class="title">{ "Memory" }</p>
                <table>
                    <tbody>
                        { for self.memory.chunks(8).enumerate().map(render_chunk) }
                    </tbody>
                </table>
            </div>
        }
    }
}
