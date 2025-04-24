use ndarray::prelude::*;
use rand::Rng;


fn populate_array(arr: &mut Array2<f32>, m: usize, n: usize) {
    let mut rng = rand::rng();
    for i in 0..m {
        for j in 0..n {
            arr[(i, j)] = rng.random_range(-1.0..1.0);
        }
    }
}

// sigmoid activation function
fn sigmoid(x: &Array2<f32>) -> Array2<f32> {
    x.mapv(|v| 1.0 / (1.0 + (-v).exp()))
}

// Gradient Descent for Sigmoid
fn sigmoid_derivative(a: &Array2<f32>) -> Array2<f32> {
    // a is already σ(z)
    a * &(1.0 - a)
}


struct NeuralNetwork {
    input_size: usize,
    h1_size: usize,
    h2_size: usize,
    output_size: usize,
    w1: Array2<f32>,   
    b1: Array2<f32>,
    w2: Array2<f32>,   
    b2: Array2<f32>,
    w3: Array2<f32>,   
    b3: Array2<f32>,
    lr: f32,
}


impl NeuralNetwork{

    fn new(input_size: usize, hidden_size: usize, output_size: usize, learning_rate: f32) -> Self {

        let mut w1 = Array2::zeros((h1_size, input_size));
        let mut w2 = Array2::zeros((h2_size, h1_size));
        let mut w3 = Array2::zeros((output_size, h2_size));

        populate_array(&mut w1, h1_size, input_size);
        populate_array(&mut w2, h2_size, h1_size);
        populate_array(&mut w3, output_size, h2_size);

        let b1 = Array2::zeros((h1_size, 1));
        let b2 = Array2::zeros((h2_size, 1));
        let b3 = Array2::zeros((output_size, 1));

        NeuralNetwork {
            input_size,
            h1_size: hidden_size,
            h2_size: hidden_size,
            output_size,
            w1,
            b1,
            w2,
            b2,
            w3,
            b3,
            lr: learning_rate,
        }
    }

    fn forward(&self, input: &Array2<f32>) -> (Array2<f32>, Array2<f32>, Array2<f32>) {
        /// Forward pass

    }


    fn backward(&mut self, input: &Array2<f32>, target: &Array2<f32>) {
        /// Backward pass
    }


    fn train(&mut self, input: &Array2<f32>, target: &Array2<f32>) -> f32 {
        /// Train
    }
    





}