// src/network.rs

use crate::activations::*;
use crate::loss::*;
use crate::matrix::*;

pub struct Network {
    weights_input_hidden: Matrix,
    weights_hidden_output: Matrix,
    bias_hidden: Matrix,
    bias_output: Matrix,
}

impl Network {
    pub fn new(input_size: usize, hidden_size: usize, output_size: usize) -> Self {
        let weights_input_hidden = Matrix::zero(input_size, hidden_size);
        let weights_hidden_output = Matrix::zero(hidden_size, output_size);
        let bias_hidden = Matrix::zero(1, hidden_size);
        let bias_output = Matrix::zero(1, output_size);

        Network {
            weights_input_hidden,
            weights_hidden_output,
            bias_hidden,
            bias_output,
        }
    }

    pub fn forward(&self, inputs: &Matrix) -> (Matrix, Matrix) {
        let hidden_input = inputs.multiply(&self.weights_input_hidden);
        hidden_input.apply(sigmoid);

        let output_input = hidden_input.multiply(&self.weights_hidden_output);
        output_input.apply(sigmoid);

        (hidden_input, output_input)
    }

    // Placeholder for backpropagation and training functions
}