use rand::Rng;

const EXP: f64 = 2.718281828459045;

pub fn sigmoid(x: &f64) -> f64 {
    return 1.0 / (1.0 + EXP.powf(-x));
}

struct NeuralNet {
    input_layer: Vec<f64>,
    hidden_layer: Vec<f64>,
    output_size: u32,
}

impl NeuralNet {
    pub fn new(input: u32, hidden: u32, output: u32) -> Self {
        if input == 0 || hidden == 0 || output == 0 {
            panic!("NN::new Cannot be zero!!");
        }

        let mut rand_gen = rand::thread_rng();

        let mut input_vec: Vec<f64> = Vec::new();
        let mut hidden_vec: Vec<f64> = Vec::new();
        for n in 0..input {
            input_vec.push(rand_gen.gen());
        }
        for n in 0..hidden {
            hidden_vec.push(rand_gen.gen());
        }
        return NeuralNet {
            input_layer: input_vec,
            hidden_layer: hidden_vec,
            output_size: output,
        };
    }
}
