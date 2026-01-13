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

    pub fn feed_forward(&mut self, inputs: Matrix) -> Matrix {
        assert!(
            self.layers[0] == inputs.data.len(),
            "incompatible number of inputs"
        );

        let mut current: Matrix = inputs;

        self.data = vec![current.clone()];

        for i in 0..self.layers.len() - 1 {
            current = self.weights[i]
                .multiply_matrices(&current)
                .unwrap()
                .add_matrices(&self.biases[i])
                .unwrap()
                .map(self.activation.function);

            self.data.push(current.clone());
        }
        current
    }

    pub fn back_propogate(&mut self, inputs: Matrix, targets: Matrix) {
        let mut errors = targets.subtract_matrices(&inputs).unwrap();

        let mut gradients = inputs.clone().map(self.activation.derivative);

        for i in (0..self.layers.len() - 1).rev() {
            gradients = gradients
                .elementwise_multiplication(&errors)
                .unwrap()
                .map(|x| x * 0.5);

            self.weights[i] = self.weights[i]
                .add_matrices(
                    &gradients
                        .multiply_matrices(&self.data[i].transpose())
                        .unwrap(),
                )
                .unwrap();

            self.biases[i] = self.biases[i].add_matrices(&gradients).unwrap();

            errors = self.weights[i]
                .transpose()
                .multiply_matrices(&errors)
                .unwrap();

            gradients = self.data[i].map(self.activation.derivative);
        }
    }

    pub fn train(&mut self, inputs: Vec<Vec<f64>>, targets: Vec<Vec<f64>>, epochs: usize) {
        for i in 0..epochs {
            for j in 0..inputs.len() {
                let outputs = self.feed_forward(Matrix::from(inputs[j].clone()));
            }
        }
    }
}
