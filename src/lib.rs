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

#[cfg(test)]
pub mod test {
    use crate::work::complex::Complex;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_powi() {
        let c = Complex::new(2.0, 2.0);
        let res = c.powi(0);
        assert_eq!(res, Complex::new(1.0,0.0));
        let res = c.powi(1);
        assert_eq!(res, c);
        let res = c.powi(2);
        assert_eq!(res, c.mul_by(&c));
        let res = c.powi(3);
        assert_eq!(res, c.mul_by(&c).mul_by(&c));
        let res = c.powi(4);
        assert_eq!(res, c.mul_by(&c).mul_by(&c).mul_by(&c));
        let res = c.powi(5);
        assert_eq!(res, c.mul_by(&c).mul_by(&c).mul_by(&c).mul_by(&c));
    }
}

