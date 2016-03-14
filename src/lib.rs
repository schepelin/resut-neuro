use std::f64;

pub mod neuron;
pub mod layer;


pub fn sigmoid(x: f64) -> f64 {
    return 1.0 / (1.0 + f64::consts::E.powf(-x));
}

#[test]
fn it_works() {
}
