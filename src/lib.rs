#![feature(vec_remove_item)]
#![recursion_limit = "512"]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;
extern crate hex;
extern crate regex;

mod emulator;
mod ui;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    console_log::init_with_level(log::Level::Debug);
    yew::start_app::<ui::App>();

    Ok(())
}
