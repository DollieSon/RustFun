use neural_net::*;

pub mod neural_net;

fn main() {
    let mut NN = NeuralNet::new(2, 2, 1);
    // NN.train(&vec![1.0, 0.0], &vec![1.0]);
    // NN.train(&vec![0.0, 1.0], &vec![1.0]);
    // NN.train(&vec![0.0, 0.0], &vec![0.0]);
    // NN.train(&vec![1.0, 1.0], &vec![1.0]);

    for _ in (0..10) {
        println!("_______________EPOCH DONE:________________");
        NN.train(&vec![1.0, 0.0], &vec![1.0]);
        NN.train(&vec![1.0, 1.0], &vec![1.0]);
        NN.train(&vec![0.0, 1.0], &vec![1.0]);
        NN.train(&vec![0.0, 0.0], &vec![0.0]);
    }
    // println!("{:?}", NN.get_result(&vec![0.0, 0.0]));
    // println!("{:?}", NN.get_result(&vec![1.0, 0.0]));
    // println!("{:?}", NN.get_result(&vec![1.0, 1.0]));
    // println!("{:?}", NN.get_result(&vec![0.0, 1.0]));
}
