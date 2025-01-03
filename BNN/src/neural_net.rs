use rand::Rng;

const EXP: f64 = 2.718281828459045;

pub fn sigmoid(x: &f64) -> f64 {
    return 1.0 / (1.0 + EXP.powf(-x));
}

struct Node {
    bias: f64,
    weights: Vec<f64>,
    act_val: f64,
}

impl Node {
    fn new(nex_nodes: u32) -> Self {
        let mut rand_gen = rand::thread_rng();

        let mut weights = Vec::<f64>::new();
        for _ in 0..nex_nodes {
            weights.push(rand_gen.gen());
        }
        return Node {
            bias: 0.0,
            weights: weights,
            act_val: 0.0,
        };
    }
    fn reset_act(mut self) {
        self.act_val = 0.0;
    }
}

pub struct NeuralNet {
    // Input -> input_layer -> hident node -> hidden_layer -> output
    pub input_layer: Vec<Node>,
    pub hidden_layer: Vec<Node>,
    pub output_size: u32,
    //
}

impl NeuralNet {
    pub fn new(input: u32, hidden: u32, output: u32) -> Self {
        if input == 0 || hidden == 0 || output == 0 {
            panic!("NN::new Cannot be zero!!");
        }
        let input_vec: Vec<Node> = (0..input).map(|_| Node::new(hidden)).collect();
        let hidden_vec: Vec<Node> = (0..input).map(|_| Node::new(output)).collect();

        return NeuralNet {
            input_layer: input_vec,
            hidden_layer: hidden_vec,
            output_size: output,
        };

        // for n in 0..input {
        //     input_vec.push(rand_gen.gen());
        // }
        // for n in 0..hidden {
        //     hidden_vec.push(rand_gen.gen());
        // }
        // return NeuralNet {
        //     input_layer: input_vec,
        //     hidden_layer: hidden_vec,
        //     output_size: output,
        // };
    }
    pub fn train(&self, input: &Vec<f64>, output: &Vec<f64>) {
        if input.len() != self.input_layer.len() || output.len() != output.len() {
            panic!("Training: Vector Sizes are Inoconsistent");
        }
        let input_res: Vec<f64> = Vec::new();
    }
}
