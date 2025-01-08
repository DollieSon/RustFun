use std::io::{self, Write};

use rand::Rng;

const EXP: f64 = 2.718281828459045;

pub fn sigmoid(x: &f64) -> f64 {
    return 1.0 / (1.0 + EXP.powf(-x));
}

struct NeuronNode {
    bias: f64,
    weights: Vec<f64>,
    act_val: f64,
}

impl NeuronNode {
    fn new(nex_nodes: u32) -> Self {
        let mut rand_gen = rand::thread_rng();

        let mut weights = Vec::<f64>::new();
        for _ in 0..nex_nodes {
            weights.push(rand_gen.gen());
        }
        return NeuronNode {
            bias: 0.0,
            weights: weights,
            act_val: 0.0,
        };
    }

    fn reset_act(mut self) {
        self.act_val = 0.0;
    }
    fn activate(&mut self, prev_Nodes: &Vec<NeuronNode>, neuron_index: usize) {
        let mut activated_value = self.bias;
        for (index, neuron) in prev_Nodes.iter().enumerate() {
            activated_value += neuron.act_val * (neuron.weights.get(neuron_index).unwrap());
        }
        self.act_val = activated_value;
    }
}

pub struct NeuralNet {
    // Input -> input_layer -> hident node -> hidden_layer -> output
    pub input_layer: Vec<NeuronNode>,
    pub hidden_layer: Vec<NeuronNode>,
    pub output_layer: Vec<NeuronNode>, //
}

impl NeuralNet {
    pub fn new(input: u32, hidden: u32, output: u32) -> Self {
        if input == 0 || hidden == 0 || output == 0 {
            panic!("NN::new Cannot be zero!!");
        }
        let input_vec: Vec<NeuronNode> = (0..input).map(|_| NeuronNode::new(hidden)).collect();
        let hidden_vec: Vec<NeuronNode> = (0..hidden).map(|_| NeuronNode::new(output)).collect();
        let output_vec: Vec<NeuronNode> = (0..output).map(|_| NeuronNode::new(1)).collect();
        return NeuralNet {
            input_layer: input_vec,
            hidden_layer: hidden_vec,
            output_layer: output_vec,
        };
    }
    // we assume that input is valid as well as output
    pub fn train(&mut self, input: &Vec<f64>, output: &Vec<f64>) {
        //validate parameters
        if input.len() != self.input_layer.len() {
            panic!("input len is incorrect")
        }
        if output.len() != self.output_layer.len() {
            panic!("input len is incorrect")
        }

        // setting input nodes
        println!("Activating Input Layer");
        self.input_layer
            .iter_mut()
            .enumerate()
            .for_each(|(index, neuron)| neuron.act_val = *input.get(index).unwrap());

        self.input_layer
            .iter_mut()
            .enumerate()
            .for_each(|(index, neuron)| println!("{}", neuron.act_val));
        //activating hidden nodes
        println!("Activating Hidden Layer");
        self.hidden_layer
            .iter_mut()
            .enumerate()
            .for_each(|(index, neuron)| neuron.activate(&self.input_layer, index));

        self.hidden_layer
            .iter_mut()
            .enumerate()
            .for_each(|(index, neuron)| println!("{}", neuron.act_val));
        //calc output
        println!("Activating Output Layer");
        self.output_layer
            .iter_mut()
            .enumerate()
            .for_each(|(index, neuron)| neuron.activate(&self.hidden_layer, index));
        self.output_layer
            .iter_mut()
            .enumerate()
            .for_each(|(index, neuron)| println!("{}", neuron.act_val));

        //get difference desired output and actual output
        let mut error_component = Vec::<f64>::new();
        for (index, neuron) in self.output_layer.iter().enumerate() {
            error_component.push(output.get(index).unwrap() - neuron.act_val);
        }
        println!("Error Components:");
        error_component
            .iter()
            .for_each(|error| print!("{}, ", error));
        println!();
        //computing Der products
        let mut der_products = Vec::<f64>::new();
        for (index, neuron) in self.output_layer.iter().enumerate() {
            der_products.push(
                (1.0 - neuron.act_val) * neuron.act_val * error_component.get(index).unwrap(),
            );
        }
        println!("DER Products:");
        der_products.iter().for_each(|error| print!("{}, ", error));
        println!();
    }
}
