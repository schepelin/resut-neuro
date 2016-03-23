use layer;

use std::fmt;

pub struct Network {
    pub layers: Vec<layer::Layer>,
    activate: fn(f64) -> f64,
}

impl fmt::Display for Network {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let mut comma_separated = String::new();
        for layer in &self.layers {
            comma_separated.push_str(&layer.to_string());
            comma_separated.push_str(",\n");

        }
        write!(formatter, "Network: [\n{}]", comma_separated)
    }
}

impl Network {
    // for given vector initialize where layers count is a length of vector
    // and neurons in each layer it is a value in vector index
    pub fn new(
        initial_input_size: i32,
        layers_config: Vec<i32>,
        activate: fn(f64) -> f64,
    ) -> Network {
        let mut inputs_count = initial_input_size;
        let mut layers: Vec<layer::Layer> = Vec::new();
        for neurons_count in layers_config {
            layers.push(layer::Layer::new(neurons_count, inputs_count));
            inputs_count = neurons_count;
        }
        Network {layers: layers, activate: activate}

    }
    pub fn eval(&self, input: Vec<f64>) -> Vec<f64> {
        self.layers.iter()
            .fold(input, |inp, ref layer| layer.eval(&inp, self.activate))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        fn func(x: f64) -> f64 { x };
        let network = Network::new(5, vec![3, 2, 4], func);
        assert_eq!(3, network.layers[0].neurons.len());
        assert_eq!(2, network.layers[1].neurons.len());
        assert_eq!(4, network.layers[2].neurons.len());

        // check initial inputs len
        assert_eq!(5, network.layers[0].neurons[0].memory.len());

        // check number of inputs is equal to outpust length of previous layer
        assert_eq!(3, network.layers[1].neurons[0].memory.len());
        assert_eq!(2, network.layers[2].neurons[0].memory.len());
    }
    fn test_eval() {
        fn func(x: f64) -> f64 { x };
        let network = Network::new(5, vec![1, 1, 1], func);
        assert_eq!(0.0, network.eval(vec![0.0, 0.0, 0.0, 0.0, 0.0])[0]);
    }
}
