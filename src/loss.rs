// src/losses.rs
pub fn mse(target: f64, output: f64) -> f64 {
    0.5 * (target - output).powi(2)
}

pub fn mse_derivative(target: f64, output: f64) -> f64 {
    output - target
}