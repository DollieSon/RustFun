use neural_net::*;

pub mod neural_net;

fn main() {
    let mut NN = NeuralNet::new(2, 2, 1);
    let x = 0.65;
    NN.train(&vec![1.0, 0.0], &vec![1.0]);
}
