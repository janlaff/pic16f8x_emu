#[macro_use]
extern crate log;
extern crate hex;
extern crate regex;

mod ui;
mod emulator;

fn main() {
    yew::start_app::<ui::App>();
}