use crate::lin_alg::matrix::*;
use crate::net::activations::*;

pub struct NeuralNet {
    layers: Vec<usize>,
    weights: Vec<Matrix>,
    biases: Vec<Matrix>,
    data: Vec<Matrix>,
    activation: Activation,
    learning_rate: f64,
}

impl NeuralNet {
    pub fn new(layers: Vec<usize>, activation: Activation, learning_rate: f64) -> Self {
        let mut weights = vec![];
        let mut biases = vec![];

        for i in 0..layers.len() - 1 {
            weights.push(Matrix::new_random(layers[i + 1], layers[i]));
            biases.push(Matrix::new_random(layers[i + 1], 1));
        }

        NeuralNet {
            layers,
            weights,
            biases,
            data: vec![],
            activation,
            learning_rate,
        }
    }
}
