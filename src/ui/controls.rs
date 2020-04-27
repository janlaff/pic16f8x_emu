use yew::agent::{Dispatched, Dispatcher};
use yew::prelude::*;

use super::CPUAgent;
use super::Request;

pub struct Controls {
    link: ComponentLink<Self>,
    dispatcher: Dispatcher<CPUAgent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ControlMsg {
    ContextMsg(Request),
}

impl Component for Controls {
    type Message = ControlMsg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            dispatcher: CPUAgent::dispatcher(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            ControlMsg::ContextMsg(request) => self.dispatcher.send(request),
        }

        false
    }

    fn view(&self) -> Html {
        let load_cb = self.link.callback(|_| ControlMsg::ContextMsg(Request::LoadLstFile(String::from(include_str!("../../examples/TPicSim3.LST")))));
        let run_cb = self.link.callback(|_| ControlMsg::ContextMsg(Request::Run));
        let step_cb = self.link.callback(|_| ControlMsg::ContextMsg(Request::Step));
        let stop_cb = self.link.callback(|_| ControlMsg::ContextMsg(Request::Stop));

        html! {
            <div id="controls" class="nes-container is-dark with-title">
                <p class="title">{ "Controls" }</p>
                <button onclick=load_cb class="nes-btn is-primary">{ "Load demo rom" }</button>
                <button onclick=run_cb class="nes-btn is-success">{ "Run" }</button>
                <button onclick=step_cb class="nes-btn is-success">{ "Step" }</button>
                <button onclick=stop_cb class="nes-btn is-error">{ "Stop" }</button>
            </div>
        }
    }
}
