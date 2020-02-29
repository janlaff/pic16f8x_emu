#![feature(vec_remove_item)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;
extern crate hex;
extern crate regex;

mod emulator;
mod ui;

fn main() {
    yew::start_app::<ui::App>();
}
