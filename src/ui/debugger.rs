use yew::prelude::*;

use super::Controls;
use super::MemoryViewer;
use super::SfrViewer;

pub struct Debugger {
    link: ComponentLink<Self>,
}

impl Component for Debugger {
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
            <div>
                <Controls />
                <MemoryViewer />
                <SfrViewer />
            </div>
        }
    }
}
