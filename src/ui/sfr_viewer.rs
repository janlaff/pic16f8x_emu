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
    local_sfrs: SfrBank,
}

impl Component for SfrViewer {
    type Message = SfrMsg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|msg| SfrMsg::ContextMsg(msg));
        Self {
            link,
            context: CPUAgent::bridge(callback),
            local_sfrs: SfrBank::new(),
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
                self.local_sfrs = sfrs;
                true
            }
            _ => false,
        }
    }

    fn view(&self) -> Html {
        let sfr_data = vec![
            ("W", self.local_sfrs.w),
            ("INDIRECT", self.local_sfrs.indirect),
            ("PCL", self.local_sfrs.pcl),
            ("FSR", self.local_sfrs.fsr),
            ("PCLATH", self.local_sfrs.pclath),
            ("INTCON", self.local_sfrs.intcon),
            ("TMR0", self.local_sfrs.tmr0),
            ("PORTA", self.local_sfrs.porta),
            ("PORTB", self.local_sfrs.portb),
            ("EEDATA", self.local_sfrs.eedata),
            ("EEADR", self.local_sfrs.eeadr),
            ("OPTION", self.local_sfrs.option),
            ("TRISA", self.local_sfrs.trisa),
            ("TRISB", self.local_sfrs.trisb),
            ("EECON1", self.local_sfrs.eecon1),
            ("EECON2", self.local_sfrs.eecon2),
        ];

        let bit_regs = vec![
            ("STATUS", self.local_sfrs.status, ["IRP", "RP1", "RP0", "TO", "PD", "Z", "DC", "C"])
        ];

        let render_sfr = |(label, value): &(&str, u8)| {
            html! {
                <tr>
                    <td>{ label }</td>
                    <td>{ hex::encode([*value]) }</td>
                </tr>
            }
        };

        let render_bit_register = |(label, value, bits): &(&str, u8, [&str; 8])| {
            html! {
                <tr>
                    <td>{label}</td>
                    <td>
                        <table class="nes-table is-bordered is-dark">
                            <tbody>
                                <tr>
                                    {for (0...7).map(|idx| html! {
                                        <td>{ bits[idx] }</td>
                                    })}
                                </tr>
                                <tr>
                                    {for (0...7).map(|idx| html! {
                                        <td>{ hex::encode([get_bit(*value, 7 - idx) as u8]) }</td>
                                    })}
                                </tr>
                            </tbody>
                        </table>
                    </td>
                </tr>
            }
        };

        html! {
            <div id="sfr-viewer" class="nes-container with-title is-dark">
                <p class="title">{ "SFR's" }</p>
                <table>
                    <tbody>
                        { for sfr_data.iter().map(render_sfr) }
                        { for bit_regs.iter().map(render_bit_register) }
                    </tbody>
                </table>
            </div>
        }
    }
}
