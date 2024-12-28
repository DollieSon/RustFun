use neural_net::*;

pub mod neural_net;

fn main() {
    let mut NN = NeuralNet::new(2, 2, 1);
    let x = 0.65;
    println!("{:?}", sigmoid(&x));
    println!("hidden: {:?}", NN.hidden_layer);
    println!("input: {:?}", NN.input_layer);
    println!("output: {:?}", NN.output_size);
}
