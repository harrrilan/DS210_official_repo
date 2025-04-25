use ndarray::prelude::*;
use rand::Rng;

fn populate_array(arr: &mut Array2<f32>, m: usize, n: usize) {
    let mut rng = rand::thread_rng();
    for i in 0..m {
        for j in 0..n {
            arr[(i, j)] = rng.gen_range(-0.75..1.0);
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

pub struct NeuralNetwork {
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
    learning_rate: f32,
}

impl NeuralNetwork{

    pub fn new(input_size: usize, hidden_size: usize, output_size: usize, learning_rate: f32) -> Self {

        let h1_size = hidden_size;
        let h2_size = hidden_size;

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
            learning_rate: learning_rate,
        }
    }

    pub fn forward(&self, input: &Array2<f32>) -> (Array2<f32>, Array2<f32>, Array2<f32>, Array2<f32>) {
        // Forward pass
        let z1 = self.w1.dot(input) + &self.b1;
        let hidden_1 = sigmoid(&z1);

        let z2 = self.w2.dot(&hidden_1) + &self.b2;
        let hidden_2 = sigmoid(&z2);

        let z3 = self.w3.dot(&hidden_2) + &self.b3;
        let output = sigmoid(&z3);

        (hidden_1, hidden_2, z3, output)

    }


    pub fn backward(
        &mut self,
        input: &Array2<f32>,
        hidden_1: &Array2<f32>,
        hidden_2: &Array2<f32>,
        output: &Array2<f32>,
        target: &Array2<f32>,
        z3: &Array2<f32>,
    ) {

        // Backward pass
        let batch_size = input.shape()[1] as f32;

        // Output error: delta3 = (output - target) * sigmoid'(z3)
        let delta3 = (output - target) * &sigmoid_derivative(z3);
    
        // Gradients for w3 and b3
        let d_w3 = delta3.dot(&hidden_2.t()) / batch_size;
        let d_b3 = delta3.sum_axis(Axis(1)).insert_axis(Axis(1)) / batch_size;

        // Backpropagate to hidden layer 2
        let delta2 = self.w3.t().dot(&delta3) * &sigmoid_derivative(&hidden_2);
        let d_w2 = delta2.dot(&hidden_1.t()) / batch_size;
        let d_b2 = delta2.sum_axis(Axis(1)).insert_axis(Axis(1)) / batch_size;

        // Backpropagate to hidden layer 1
        let delta1 = self.w2.t().dot(&delta2) * &sigmoid_derivative(&hidden_1);
        let d_w1 = delta1.dot(&input.t()) / batch_size;
        let d_b1 = delta1.sum_axis(Axis(1)).insert_axis(Axis(1)) / batch_size;

        // Update weights and biases
        self.w3 = &self.w3 - &(d_w3 * self.learning_rate);
        self.b3 = &self.b3 - &(d_b3 * self.learning_rate);
        self.w2 = &self.w2 - &(d_w2 * self.learning_rate);
        self.b2 = &self.b2 - &(d_b2 * self.learning_rate);
        self.w1 = &self.w1 - &(d_w1 * self.learning_rate);
        self.b1 = &self.b1 - &(d_b1 * self.learning_rate);
    }

    pub fn train(&mut self, input: &Array2<f32>, target: &Array2<f32>) -> f32 {
        // Forward pass
        let (hidden_1, hidden_2, z3, output) = self.forward(input);
    
        // Cross-entropy loss
        let epsilon = 1e-15;
        let loss_matrix = -target * &output.mapv(|x| (x + epsilon).ln());
        let batch_size = input.shape()[1] as f32;
        let loss = loss_matrix.sum() / batch_size;
    
        // Backward pass
        self.backward(input, &hidden_1, &hidden_2, &output, target, &z3);
    
        loss
    }
}