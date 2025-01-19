pub mod neural_net;

use neural_net::*;
fn main() {
    let input_layer = vec![
        NeuronNode {
            bias: 0.0,
            weights: vec![0.7, 0.6],
            act_val: 0.0,
        },
        NeuronNode {
            bias: 0.0,
            weights: vec![0.1, 0.5],
            act_val: 0.0,
        },
    ];
    let hidden_layer = vec![
        NeuronNode {
            bias: 0.3,
            weights: vec![0.6],
            act_val: 0.0,
        },
        NeuronNode {
            bias: 0.2,
            weights: vec![0.2],
            act_val: 0.0,
        },
    ];
    let output_layer = vec![NeuronNode {
        bias: 0.0,
        weights: Vec::<f64>::new(),
        act_val: 0.0,
    }];
    let mut NN = NeuralNet {
        input_layer,
        hidden_layer,
        output_layer,
    };
    NN.net_print();

    // for _ in 0..10 {
    //     NN.train(&vec![1.0, 0.0], &vec![1.0], true);
    //     NN.train(&vec![1.0, 1.0], &vec![1.0], false);
    //     NN.train(&vec![0.0, 1.0], &vec![1.0], false);
    //     NN.train(&vec![0.0, 0.0], &vec![0.0], true);
    // }
    // println!("{:?}", NN.get_result(&vec![1.0, 0.0]));
    // let mut NN = NeuralNet::new(2, 2, 1);
    // NN.train(&vec![1.0, 0.0], &vec![1.0]);
    // NN.train(&vec![0.0, 1.0], &vec![1.0]);
    // NN.train(&vec![0.0, 0.0], &vec![0.0]);
    // NN.train(&vec![1.0, 1.0], &vec![1.0]);

    let mut new_NN = NeuralNet::new(2, 16, 1);
    for _ in 0..1 {
        println!("________EPOCH____");
        new_NN.train(&vec![1.0, 0.0], &vec![1.0], false);
        new_NN.train(&vec![1.0, 1.0], &vec![1.0], false);
        new_NN.train(&vec![0.0, 1.0], &vec![1.0], false);
        new_NN.train(&vec![0.0, 0.0], &vec![0.0], true);
    }

    println!("TEST: {:?}", new_NN.get_result(&vec![1.0, 0.0]));
}
