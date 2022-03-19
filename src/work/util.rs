#![allow(clippy::missing_panics_doc)]
#![allow(dead_code)]
use yew::prelude::*;

use web_sys::HtmlInputElement;

pub fn get_f64_from_ref(node_ref: &NodeRef, name: &str) -> Result<f64, String> {
    match node_ref.cast::<HtmlInputElement>() {
        Some(element) => {
            match element.value().parse::<f64>() {
                Ok(value) => Ok(value),
                Err(err) => {
                    Err(format!("Unable to parse value {}, error: {}", name, err))
                }
            }
        },
        None => {
            Err(format!("Could not cast NodeRef to HtmlInputElement for value {}", name))
        }
    }
}

pub fn get_u32_from_ref(node_ref: &NodeRef, name: &str) -> Result<u32, String> {
    match node_ref.cast::<HtmlInputElement>() {
        Some(element) => {
            match element.value().parse::<u32>() {
                Ok(value) => Ok(value),
                Err(err) => {
                    Err(format!("Unable to parse value {}, error: {}", name, err))
                }
            }
        },
        None => {
            Err(format!("Could not cast NodeRef to HtmlInputElement for value {}", name))
        }
    }
}

pub fn set_value_on_ref(node_ref: &NodeRef, name: &str, value: &str) -> Result<(), String> {
    match node_ref.cast::<HtmlInputElement>() {
        Some(element) => {
            element.set_value(value);
            Ok(())
        },
        None => {
            Err(format!("Could not cast NodeRef to HtmlInputElement for value {}", name))
        }
    }
}

/*
pub fn set_f64_on_input(name: &str, value: f64) {
    window().expect("window not found")
        .document().expect("html document not found")
        .get_element_by_id(name).unwrap_or_else(|| panic!("element {} not found", name))
        .dyn_into::<HtmlInputElement>().expect("Not a HTMLInputElement")
        .set_value(&value.to_string());
}

pub fn set_u32_on_input(name: &str, value: u32) {
    window().expect("window not found")
        .document().expect("html document not found")
        .get_element_by_id(name).unwrap_or_else(|| panic!("element {} not found", name))
        .dyn_into::<HtmlInputElement>().expect("Not a HTMLInputElement")
        .set_value(&value.to_string());
}

#[must_use]
pub fn get_f64_from_input(name: &str) -> Option<f64> {
    match window().expect("window not found")
        .document().expect("html document not found")
        .get_element_by_id(name).unwrap_or_else(|| panic!("element {} not found", name))
        .dyn_into::<HtmlInputElement>().expect("Not a HTMLInputElement")
        .value().parse::<f64>()
    {
        Ok(value) => Some(value),
        Err(err) => {
            warn!("failed to convert {}: {}", name, err);
            None
        }
    }
}

#[must_use]
pub fn get_u32_from_input(name: &str) -> Option<u32> {
    match window().expect("window not found")
        .document().expect("html document not found")
        .get_element_by_id(name).expect("element not found")
        .dyn_into::<HtmlInputElement>().expect("Not a HTMLInputElement")
        .value().parse::<u32>()
    {
            Ok(value) => Some(value),
            Err(err) => {
                warn!("failed to convert {}: {}", name, err);
                None
            }
    }
}
*/

#[must_use]
pub fn find_escape_radius(c_norm: f64) -> f64 {
    // Newton iteration
    let mut radius = 2.0;

    // eprintln!("find_escape_radius({}): c_norm: {}, start: {}", c, c_norm, radius);
    for _idx in 0..20 {
        let delta_r = radius * radius - radius - c_norm;

        if (0.0..=0.01).contains(&delta_r) {
            break;
        }

        let gradient = 2.0 * radius - 1.0;
        if gradient == 0.0 {
            warn!("stuck on the zero gradient");
            radius = 2.0;
            break;
        }

        radius -= delta_r / gradient;
    }

    if radius * radius - radius - c_norm >= 0.0 && radius <= 2.0 {
        radius
    } else {
        2.0
    }
}


#[cfg(test)]
mod test {
    use super::find_escape_radius;
    use crate::complex::Complex;

    #[test]
    fn test_find_escape_radius() {
        let c_norm = Complex::new(0.3, -0.5).norm();
        let radius = find_escape_radius(c_norm);
        assert!(radius * radius - radius >= c_norm);
        assert!(radius * radius - radius - c_norm <= 0.01);

        let c_norm = Complex::new(1.0, -1.0).norm();
        let radius = find_escape_radius(c_norm);
        assert!(radius * radius - radius >= c_norm);
        assert!(radius * radius - radius - c_norm <= 0.01);
    }
}
