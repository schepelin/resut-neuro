extern crate neuro;

fn main() {
    let input = vec![1.0; 5];
    let network = neuro::network::Network::new(5, vec![3, 2], neuro::sigmoid);
    println!("{:?}", input);
    println!("{}", network);
    println!("eval: {:?}", network.eval(input));
}
