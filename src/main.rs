
// src/main.rs
mod activations;
mod loss;
mod matrix;
mod network;

use matrix::Matrix;
use network::Network;

fn main() {
    let input_size = 2;  // For XOR, inputs are pairs of binary values
    let hidden_size = 2; // Simplified network
    let output_size = 1; // XOR output is a single binary value

    //let network = Network::new(input_size, hidden_size, output_size);

    // Define XOR inputs and expected outputs
    let inputs = Matrix::new(4, 2, vec![0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0]);
    let expected_outputs = Matrix::new(4, 1, vec![0.0, 1.0, 1.0, 0.0]);

    // Assume training loop and backpropagation logic here

    inputs.show(); //Test : this is for testing only
    expected_outputs.show();
    println!("Setup complete. Network is ready for training.");
}