use yew::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

use crate::emulator;

pub struct Debugger {
    link: ComponentLink<Self>,
}

pub enum DebuggerMsg {
    HighlightLine(usize),
}

impl Component for Debugger {
    type Message = DebuggerMsg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            DebuggerMsg::HighlightLine(usize) => {
                // Update your model on events
                true
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <h1>{ "Hello World" }</h1>
        }
    }
}
