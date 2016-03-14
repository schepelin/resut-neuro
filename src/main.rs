extern crate neuro;

fn main() {
    let l = neuro::layer::Layer::new(5, 3);
    println!("{}", l);
    println!("eval: {:?}", l.eval(&vec![1.0; 3], neuro::sigmoid));
}
