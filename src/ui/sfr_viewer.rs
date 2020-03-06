use yew::prelude::*;

use std::slice::Chunks;

use crate::emulator::{SfrBank, get_bit};
use super::{CPUAgent, Response, Request};

#[derive(Serialize, Deserialize, Debug)]
pub enum SfrMsg {
    ContextMsg(Response),
}

pub struct SfrViewer {
    link: ComponentLink<Self>,
    context: Box<dyn Bridge<CPUAgent>>,
    localSfrs: SfrBank,
}

impl Component for SfrViewer {
    type Message = SfrMsg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|msg| SfrMsg::ContextMsg(msg));
        Self {
            link,
            context: CPUAgent::bridge(callback),
            localSfrs: SfrBank::new(),
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        self.context.send(Request::FetchSfrs);
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            SfrMsg::ContextMsg(Response::UpdatedMemory(address, value)) => {
                if address < 0x0c {
                    self.context.send(Request::FetchSfrs);
                }
                true
            }
            SfrMsg::ContextMsg(Response::FetchedSfrs(sfrs)) => {
                self.localSfrs = sfrs;
                true
            }
            _ => false,
        }
    }

    fn view(&self) -> Html {
        let sfr_data = vec![
            ("W", self.localSfrs.w),
            ("INDIRECT", self.localSfrs.indirect),
            ("PCL", self.localSfrs.pcl),
            ("STATUS", self.localSfrs.status),
            ("FSR", self.localSfrs.fsr),
            ("PCLATH", self.localSfrs.pclath),
            ("INTCON", self.localSfrs.intcon),
            ("TMR0", self.localSfrs.tmr0),
            ("PORTA", self.localSfrs.porta),
            ("PORTB", self.localSfrs.portb),
            ("EEDATA", self.localSfrs.eedata),
            ("EEADR", self.localSfrs.eeadr),
            ("OPTION", self.localSfrs.option),
            ("TRISA", self.localSfrs.trisa),
            ("TRISB", self.localSfrs.trisb),
            ("EECON1", self.localSfrs.eecon1),
            ("EECON2", self.localSfrs.eecon2),
        ];

        let render_sfr = |(label, value): &(&str, u8)| {
            html! {
                <tr>
                    <td>{ label }</td>
                    <td>{ hex::encode([*value]) }</td>
                </tr>
            }
        };

        html! {
            <div id="sfr-viewer" class="nes-container with-title is-dark">
                <p class="title">{ "SFR's" }</p>
                <table>
                    <tbody>
                        { for sfr_data.iter().map(render_sfr) }
                    </tbody>
                </table>
            </div>
        }
    }
}
