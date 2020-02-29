use yew::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

use super::Debugger;
use crate::emulator;

pub struct App {
    link: ComponentLink<Self>,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Debugger />
        }
    }
}
