use yew::prelude::*;

use super::CPUAgent;
use super::Request;
use super::Response;
use crate::emulator::ParseResult;

pub struct ProgramViewer {
    link: ComponentLink<Self>,
    context: Box<dyn Bridge<CPUAgent>>,
    program: Option<ParseResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ProgramMsg {
    ContextMsg(Response),
}

impl Component for ProgramViewer {
    type Message = ProgramMsg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|msg| ProgramMsg::ContextMsg(msg));
        Self {
            link,
            context: CPUAgent::bridge(callback),
            program: None,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            ProgramMsg::ContextMsg(Response::LoadedProgram(program)) => {
                self.program = Some(program);
                true
            }
            _ => false,
        }
    }

    fn view(&self) -> Html {
        let render_program = || -> Html {
            if let Some(prog) = &self.program {
                html! { <h3>{ "Program loaded" }</h3> }
            } else {
                html! { <h3>{ "No program loaded" }</h3> }
            }
        };

        html! {
            <div>
                <h1>{ "Program" }</h1>
                <table>
                    <tbody>
                        { render_program() }
                    </tbody>
                </table>
            </div>
        }
    }
}
