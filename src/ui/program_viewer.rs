use yew::prelude::*;

use super::CPUAgent;
use super::Request;
use super::Response;
use crate::emulator::ParseResult;
use yew::utils::document;
use yew::services::ConsoleService;

pub struct ProgramViewer {
    link: ComponentLink<Self>,
    context: Box<dyn Bridge<CPUAgent>>,
    program: Option<ParseResult>,
    current_line: usize,
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
            current_line: 0,
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
            ProgramMsg::ContextMsg(Response::UpdatedMemory(PCL_ADDR, value)) => {
                if let Some(prog) = &self.program {
                    let key = value as u16;
                    self.current_line = *prog.address_info.get(&key).unwrap();

                    let id = format!("line-{}", self.current_line);
                    if let Some(window) = web_sys::window() {
                        if let Some(doc) = window.document() {
                            if let Some(line) = doc.get_element_by_id(id.as_str()) {
                                line.scroll_into_view();
                            } else {
                                warn!("Line element not found");
                            }
                        }
                    }

                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn view(&self) -> Html {
        let render_line = |(idx, (label, content)): (usize, &(String, String))| -> Html {
            if self.current_line == idx {
                html! {
                    <tr class="active" id={ "line-".to_owned() + &idx.to_string() }>
                        <td>{ label }</td>
                        <td>{ content }</td>
                    </tr>
                }
            } else {
                html! {
                    <tr id={ "line-".to_owned() + &idx.to_string() }>
                        <td>{ label }</td>
                        <td>{ content }</td>
                    </tr>
                }
            }
        };

        let render_program = || -> Html {
            if let Some(prog) = &self.program {
                html! {
                    <table id="program-content">
                        <tbody>
                            { for prog.content.iter().enumerate().map(render_line) }
                        </tbody>
                    </table>
                }
            } else {
                html! { <h3>{ "No program loaded" }</h3> }
            }
        };

        html! {
            <div id="program-viewer">
                <h1>{ "Program" }</h1>
                <div id="program-scroll-wrapper">
                    { render_program() }
                </div>
            </div>
        }
    }
}
