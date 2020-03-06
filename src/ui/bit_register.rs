use yew::prelude::*;

pub struct BitRegister {
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone)]
pub struct BitRegisterProps {
    bits: [String; 8],
    value: u8,
}

impl Component for BitRegister {
    type Message = ();
    type Properties = BitRegisterProps;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>{ "Bit register here" }</div>
        }
    }
}
