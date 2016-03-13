
use std::fmt;
use std::f64;
extern crate rand;


fn sigmoid(x: f64) -> f64 {
    return 1.0 / (1.0 + f64::consts::E.powf(-x));
}

pub struct Neuron {
    memory: Vec<f64>
}

impl Neuron {
    pub fn new(inputs_count: i32) -> Neuron {
        let mut memory = Vec::new();
        for _ in 1..inputs_count {
            memory.push(rand::random());
        }
        Neuron{ memory: memory}
    }
    pub fn eval(&self, input: Vec<f64>) -> f64 {
        let sum = self.memory.iter()
            .zip(input.iter())
            .map(|(x, y)| x * y)
            .collect::<Vec<f64>>()
            .iter().fold(0.0, |acc, &x| acc + x);
        sigmoid(sum)
    }
}

impl fmt::Display for Neuron {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Neuron: {:?}", self.memory)
    }
}
