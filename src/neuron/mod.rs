
use std::fmt;
extern crate rand;


pub struct Neuron {
    memory: Vec<f64>
}

impl Neuron {
    pub fn new(inputs_count: i32) -> Neuron {
        let mut memory = Vec::new();
        for _ in 0..inputs_count {
            memory.push(rand::random());
        }
        Neuron{ memory: memory }
    }
    fn with_memory(memory: Vec<f64>) -> Neuron {
        Neuron { memory: memory }
    }
    pub fn eval(&self, input: &Vec<f64>, func: fn(f64) -> f64) -> f64 {
        let sum = self.memory.iter()
            .zip(input.iter())
            .map(|(x, y)| x * y)
            .collect::<Vec<f64>>()
            .iter().fold(0.0, |acc, &x| acc + x);
        func(sum)
    }
}

impl fmt::Display for Neuron {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Neuron: {:?}", self.memory)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval() {
        let neuron = Neuron::with_memory(vec!(1.0, 1.0, 1.0));
        fn func(x: f64) -> f64 { x };
        assert_eq!(3.0, neuron.eval(&vec!(1.0, 1.0, 1.0), func));
    }
}
