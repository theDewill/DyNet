// src/activations.rs
pub fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + f64::exp(-x))
}

pub fn sigmoid_derivative(x: f64) -> f64 {
    let sig = sigmoid(x);
    sig * (1.0 - sig)
}