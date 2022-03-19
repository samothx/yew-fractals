use yew;

#[macro_use]
extern crate log;

mod components;
mod agents;
mod work;


use components::root::Root;
// mod model;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    info!("starting up");
    yew::start_app::<Root>();
}
