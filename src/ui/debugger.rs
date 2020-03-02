use yew::prelude::*;

use super::Controls;
use super::MemoryViewer;
use super::SfrViewer;
use super::ProgramViewer;

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
                <div style="display: flex; flex-direction: row">
                    <div>
                        <MemoryViewer />
                        <SfrViewer />
                    </div>
                    <div>
                        <Controls />
                        <ProgramViewer />
                    </div>
                </div>
            </div>
        }
    }
}
