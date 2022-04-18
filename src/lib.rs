#[macro_use]
extern crate log;

use wasm_bindgen::prelude::*;

mod components;
mod agents;
mod work;


use components::root::Root;
// mod model;

#[allow(clippy::unused_unit)]
#[wasm_bindgen(start)]
pub fn start() {
    wasm_logger::init(wasm_logger::Config::default());
    info!("starting up");
    yew::start_app::<Root>();
}
