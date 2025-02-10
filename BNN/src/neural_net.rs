use std::io::{self, Write};

use rand::Rng;

const EXP: f64 = 2.718281828459045;
const LRP: f64 = 0.2;
const LRPIN: f64 = 0.15;

pub fn sigmoid(x: &f64) -> f64 {
    return 1.0 / (1.0 + EXP.powf(-x));
}

pub struct NeuronNode {
    pub bias: f64,
    pub weights: Vec<f64>,
    pub act_val: f64,
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
    // For Custom Nodes and testing, Do not use please
    pub fn custom_node(bias: f64, weights: Vec<f64>, val: f64) -> Self {
        NeuronNode {
            bias: bias,
            weights: weights,
            act_val: val,
        }
    }

    fn activate(&mut self, prev_Nodes: &Vec<NeuronNode>, neuron_index: usize) {
        let mut activated_value = self.bias;
        for (index, neuron) in prev_Nodes.iter().enumerate() {
            activated_value += neuron.act_val * (neuron.weights.get(neuron_index).unwrap());
        }
        self.act_val = sigmoid(&activated_value);
    }
}

pub struct NeuralNet {
    // Input -> input_layer -> hident node -> hidden_layer -> output
    pub input_layer: Vec<NeuronNode>,
    pub hidden_layer: Vec<Vec<NeuronNode>>,
    pub output_layer: Vec<NeuronNode>, //
}

impl NeuralNet {
    pub fn new(input: u32, hidden: Vec<u32>, output: u32) -> Self {
        if input == 0 || hidden.len() == 0 || output == 0 {
            panic!("NN::new Cannot be zero!!");
        }
        let input_vec: Vec<NeuronNode> = (0..input)
            .map(|_| NeuronNode::new(*hidden.get(0).unwrap()))
            .collect();
        let hidden_vec: Vec<Vec<NeuronNode>> = hidden
            .iter()
            .enumerate()
            .map(|(ind, len)| {
                if ind + 1 == hidden.len() {
                    return (0..*len).map(|_| NeuronNode::new(output)).collect();
                } else {
                    return (0..*len)
                        .map(|_| NeuronNode::new(*hidden.get(ind + 1).unwrap()))
                        .collect();
                }
            })
            .collect();
        let output_vec: Vec<NeuronNode> = (0..output).map(|_| NeuronNode::new(1)).collect();
        return NeuralNet {
            input_layer: input_vec,
            hidden_layer: hidden_vec,
            output_layer: output_vec,
        };
    }
    // we assume that input is valid as well as output
    pub fn train(&mut self, input: &Vec<f64>, output: &Vec<f64>, is_debug: bool) {
        //validate parameters
        if input.len() != self.input_layer.len() {
            panic!("input len is incorrect")
        }
        if output.len() != self.output_layer.len() {
            panic!("input len is incorrect")
        }

        // setting input nodes
        if is_debug {
            println!("Activating Input Layer");
        }
        self.input_layer
            .iter_mut()
            .enumerate()
            .for_each(|(index, neuron)| neuron.act_val = *input.get(index).unwrap());
        if is_debug {
            self.input_layer
                .iter_mut()
                .enumerate()
                .for_each(|(index, neuron)| println!("{}", neuron.act_val));
        }

        //activating hidden nodes
        self.hidden_layer
            .iter_mut()
            .enumerate()
            .for_each(|(index, neuron)| neuron.activate(&self.input_layer, index));

        if is_debug {
            println!("Activated Hidden Layer");
            self.hidden_layer
                .iter_mut()
                .enumerate()
                .for_each(|(index, neuron)| println!("{}", neuron.act_val));
        }

        //calc output
        self.output_layer
            .iter_mut()
            .enumerate()
            .for_each(|(index, neuron)| neuron.activate(&self.hidden_layer, index));

        if is_debug {
            println!("Activating Output Layer");
            self.output_layer
                .iter_mut()
                .enumerate()
                .for_each(|(index, neuron)| println!("{}", neuron.act_val));
        }
        //get difference desired output and actual output
        let mut error_component = Vec::<f64>::new();
        for (index, neuron) in self.output_layer.iter().enumerate() {
            error_component.push(output.get(index).unwrap() - neuron.act_val);
        }
        if is_debug {
            println!("Error Components / EC:");
            error_component
                .iter()
                .for_each(|error| print!("{}, ", error));
            println!();
        }
        // computing Der products
        let mut der_products = Vec::<f64>::new();
        for (index, neuron) in self.output_layer.iter().enumerate() {
            der_products.push(
                (1.0 - neuron.act_val) * neuron.act_val * error_component.get(index).unwrap(),
            );
        }
        if is_debug {
            println!("DER Products:");
            der_products.iter().for_each(|error| print!("{}, ", error));
            println!();
        }
        // calclulate the error rate of each neuron
        let mut error_rate = Vec::<f64>::new();
        for (index, neuron) in self.hidden_layer.iter().enumerate() {
            let mut error = 0.0;
            neuron
                .weights
                .iter()
                .enumerate()
                .for_each(|(index, weight)| error += (der_products.get(index).unwrap() * weight));
            error *= neuron.act_val * (1.0 - neuron.act_val);
            error_rate.push(error);
        }
        if is_debug {
            println!("Error Rates(Hidden) / ERR:");
            error_rate.iter().for_each(|error| print!("{}, ", error));
            println!();
        }
        //6 calculate ADJHO and change the weights
        // println!("Old Vector Weights:");
        // self.hidden_layer
        //     .iter()
        //     .enumerate()
        //     .for_each(|(index, node)| {
        //         print!("Node {}: ", index);
        //         node.weights.iter().for_each(|weight| print!("{} ", weight));
        //         print!("\n");
        //     });
        for (index, neuron) in self.hidden_layer.iter_mut().enumerate() {
            let lrpres = neuron.act_val * LRP;
            der_products.iter().enumerate().for_each(|(index, der)| {
                let added_weight = der * lrpres;
                if is_debug {
                    println!("adjustment: {}", added_weight);
                }
                let weight = neuron.weights.get(index).unwrap() + added_weight;
                if let Some(data) = neuron.weights.get_mut(index) {
                    *data = weight;
                }
            });
        }
        if is_debug {
            println!("New Vector Weights:");
            self.hidden_layer
                .iter()
                .enumerate()
                .for_each(|(index, node)| {
                    print!("Node {}: ", index);
                    node.weights.iter().for_each(|weight| print!("{} ", weight));
                    print!("\n");
                });
        }

        //setting weights of input node (input -> hidden)
        for (index, neuron) in self.input_layer.iter_mut().enumerate() {
            let weight_adjustment = error_rate.get(index).unwrap() * neuron.act_val;
            neuron
                .weights
                .iter_mut()
                .for_each(|weight| *weight += weight_adjustment);
        }
        //changing bias of output nodes
        for (index, neuron) in self.output_layer.iter_mut().enumerate() {
            neuron.bias += LRP * der_products.get(index).unwrap();
        }
        //changing bias of hidden nodes
        self.hidden_layer
            .iter_mut()
            .enumerate()
            .for_each(|(index, neuron)| {
                let change = -LRPIN * error_rate.get(index).unwrap();
                if is_debug {
                    println!("change(hidden bias) {}", change);
                }
                neuron.bias += change
            });

        if is_debug {
            self.output_layer.iter().for_each(|neuron| {
                println!("{}", neuron.act_val);
            });
        }
    }
    pub fn get_result(&mut self, input: &Vec<f64>, is_debug: bool) -> Vec<f64> {
        if is_debug {
            println!("Activating Input Layer")
        }
        self.input_layer
            .iter_mut()
            .enumerate()
            .for_each(|(index, neuron)| neuron.act_val = *input.get(index).unwrap());
        for ind in (0..self.hidden_layer.len()) {
            let (prev_layer, curr_layer) = {
                let (left, right) = self.hidden_layer.split_at_mut(ind + 1);
                if ind == 0 {
                    (&self.input_layer, &mut right[0])
                } else {
                    (&left[ind], &mut right[0])
                }
            };
            curr_layer.iter_mut().enumerate().for_each(|(ind, neuron)| {
                neuron.activate(prev_layer, ind);
            });
        }
        self.output_layer
            .iter_mut()
            .enumerate()
            .for_each(|(index, neuron)| {
                neuron.activate(
                    self.hidden_layer.get(self.hidden_layer.len() - 1).unwrap(),
                    index,
                )
            });
        let mut res = Vec::<f64>::new();
        self.output_layer.iter().for_each(|neuron| {
            res.push(neuron.act_val);
            println!("{}", neuron.act_val);
        });
        return res;
    }

    pub fn net_print(&self) {
        println!("Input Layer:");
        self.input_layer.iter().for_each(|neuron| {
            println!("Neuron Bias {}", neuron.bias);
            neuron.weights.iter().for_each(|w| println!("{}", w));
        });
        println!("Hidden Layer:");
        self.hidden_layer.iter().for_each(|neuron| {
            println!("Neuron Bias {}", neuron.bias);
            neuron.weights.iter().for_each(|w| println!("{}", w));
        });
        println!("Output Layer:");
        self.output_layer.iter().for_each(|neuron| {
            println!("Neuron Bias {}", neuron.bias);
            neuron.weights.iter().for_each(|w| println!("{}", w));
        });
    }
}
