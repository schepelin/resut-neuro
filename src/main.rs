extern crate neuro;

fn main() {
    let n = neuro::neuron::Neuron::new(5);
    println!("neuron: {}", n);
    println!("eval: {}", n.eval(vec![1.0; 5]));
}
