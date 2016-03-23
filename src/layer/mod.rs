// use std::thread;
use neuron;

use std::fmt;

pub struct Layer {
    pub neurons: Vec<neuron::Neuron>
}

impl fmt::Display for Layer {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let mut comma_separated = String::new();
        for n in &self.neurons {
            comma_separated.push_str(&n.to_string());
            comma_separated.push_str(",\n");

        }
        write!(formatter, "Layer: [\n{}]", comma_separated)
    }
}

impl Layer {
    pub fn new(neurons_count: i32, inputs_count: i32) -> Layer {
        let mut neurons: Vec<neuron::Neuron> = Vec::new();
        for _ in 0..neurons_count {
            neurons.push(neuron::Neuron::new(inputs_count));
        }
        Layer {neurons: neurons}
    }
    pub fn eval(&self, input: &Vec<f64>, func: fn(f64) -> f64) -> Vec<f64> {
        self.neurons.iter()
        .map(|ref neuron| neuron.eval(&input, func))
        .collect::<Vec<f64>>()
    }
}
