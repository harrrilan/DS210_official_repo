use ndarray::prelude::*;
use rand::Rng;
use csv::Reader;
use std::error::Error;
use std::env;

/// Load CSV of MNIST: returns (features, one-hot labels)
pub fn load_csv(
    path: &str,
) -> Result<(Array2<f32>, Array2<f32>), Box<dyn Error>> {
    let mut rdr = Reader::from_path(path)?;
    let mut features: Vec<f32> = Vec::new();
    let mut labels: Vec<f32> = Vec::new();
    let mut record_count = 0;

    for result in rdr.records() {
        let record = result?;
        // First column is label
        let label: usize = record[0].parse()?;
        // Build one-hot label
        let mut one_hot = vec![0.0f32; 10];
        one_hot[label] = 1.0;
        labels.extend(&one_hot);
        
        // Parse pixel values and normalize
        for pix in record.iter().skip(1) {
            let v: f32 = pix.parse::<f32>()? / 255.0;
            features.push(v);
        }
        record_count += 1;
    }
    // Create ndarray: (samples, 784)
    let feature_array = Array2::from_shape_vec((record_count, 784), features)?;
    let label_array   = Array2::from_shape_vec((record_count, 10), labels)?;
    Ok((feature_array, label_array))
}

// Sigmoid activation and derivative
fn sigmoid(x: &Array2<f32>) -> Array2<f32> {
    x.mapv(|v| 1.0 / (1.0 + (-v).exp()))
}
fn sigmoid_derivative(x: &Array2<f32>) -> Array2<f32> {
    x.mapv(|v| v * (1.0 - v))
}

/// Simple 4-layer NN: 784 -> hidden1 -> hidden2 -> 10
struct NeuralNetwork {
    w1: Array2<f32>, b1: Array2<f32>,
    w2: Array2<f32>, b2: Array2<f32>,
    w3: Array2<f32>, b3: Array2<f32>,
    lr: f32,
}
impl NeuralNetwork {
    fn new(hidden1: usize, hidden2: usize, lr: f32) -> Self {
        let mut rng = rand::thread_rng();
        
        // Improved weight initialization with Xavier/Glorot
        let w1 = Array::from_shape_fn((hidden1, 784), |_| {
            let scale = (6.0 / (784.0 + hidden1 as f32)).sqrt();
            rng.gen_range(-scale..scale)
        });
        let b1 = Array2::zeros((hidden1, 1));
        
        let w2 = Array::from_shape_fn((hidden2, hidden1), |_| {
            let scale = (6.0 / (hidden1 as f32 + hidden2 as f32)).sqrt();
            rng.gen_range(-scale..scale)
        });
        let b2 = Array2::zeros((hidden2, 1));
        
        let w3 = Array::from_shape_fn((10, hidden2), |_| {
            let scale = (6.0 / (hidden2 as f32 + 10.0)).sqrt();
            rng.gen_range(-scale..scale)
        });
        let b3 = Array2::zeros((10, 1));
        
        NeuralNetwork { w1, b1, w2, b2, w3, b3, lr }
    }

    fn forward(&self, x: &Array2<f32>) -> (Array2<f32>, Array2<f32>, Array2<f32>) {
        let z1 = &self.w1.dot(x) + &self.b1;
        let a1 = sigmoid(&z1);
        let z2 = &self.w2.dot(&a1) + &self.b2;
        let a2 = sigmoid(&z2);
        let z3 = &self.w3.dot(&a2) + &self.b3;
        let a3 = sigmoid(&z3);
        (a1, a2, a3)
    }

    fn backward(
        &mut self,
        x: &Array2<f32>,
        a1: &Array2<f32>,
        a2: &Array2<f32>,
        a3: &Array2<f32>,
        y: &Array2<f32>,
    ) {
        let dz3 = a3 - y;
        let dw3 = dz3.dot(&a2.t());
        let db3 = dz3.clone();

        let da2 = self.w3.t().dot(&dz3);
        let dz2 = da2 * sigmoid_derivative(a2);
        let dw2 = dz2.dot(&a1.t());
        let db2 = dz2.clone();

        let da1 = self.w2.t().dot(&dz2);
        let dz1 = da1 * sigmoid_derivative(a1);
        let dw1 = dz1.dot(&x.t());
        let db1 = dz1.clone();

        self.w3 = &self.w3 - &(dw3 * self.lr);
        self.b3 = &self.b3 - &(db3 * self.lr);
        self.w2 = &self.w2 - &(dw2 * self.lr);
        self.b2 = &self.b2 - &(db2 * self.lr);
        self.w1 = &self.w1 - &(dw1 * self.lr);
        self.b1 = &self.b1 - &(db1 * self.lr);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Path resolution
    let root = env!("CARGO_MANIFEST_DIR");
    let train_path = format!("{}/data/mnist_train.csv", root);
    let test_path  = format!("{}/data/mnist_test.csv",  root);

    // Load datasets
    let (train_X, train_Y) = load_csv(&train_path)?;
    let (test_X,  test_Y)  = load_csv(&test_path)?;
    let samples = train_X.dim().0;

    // Instantiate network with smaller learning rate
    let mut nn = NeuralNetwork::new(128, 64, 0.01);

    // Train 3 epochs
    for epoch in 1..=3 {
        let mut correct = 0;
        for i in 0..samples {
            // slice row i -> column vector
            let x = train_X.row(i).to_owned().into_shape((784, 1))?;
            let y = train_Y.row(i).to_owned().into_shape((10, 1))?;
            
            // Calculate forward pass once
            let (a1, a2, a3) = nn.forward(&x);
            
            // Use the activations from the same forward pass for backward
            nn.backward(&x, &a1, &a2, &a3, &y);
            
            // Track training accuracy
            let pred = a3
                .indexed_iter()
                .max_by(|(_, v1), (_, v2)| v1.partial_cmp(v2).unwrap())
                .unwrap()
                .0
                .0;
            let true_label = y
                .indexed_iter()
                .max_by(|(_, v1), (_, v2)| v1.partial_cmp(v2).unwrap())
                .unwrap()
                .0
                .0;
            if pred == true_label { correct += 1; }
        }
        let train_acc = 100.0 * correct as f32 / samples as f32;
        println!("Epoch {} complete - Training accuracy: {:.2}%", epoch, train_acc);
    }

    // Evaluate accuracy
    let mut correct = 0;
    let test_samples = test_X.dim().0;
    for i in 0..test_samples {
        let x = test_X.row(i).to_owned().into_shape((784, 1))?;
        let (_, _, a3) = nn.forward(&x);
        let pred = a3
            .indexed_iter()
            .max_by(|(_, v1), (_, v2)| v1.partial_cmp(v2).unwrap())
            .unwrap()
            .0
            .0;
        let true_label = test_Y.row(i)
            .iter()
            .position(|&v| v == 1.0)
            .unwrap();
        if pred == true_label { correct += 1; }
    }
    let acc = 100.0 * correct as f32 / test_samples as f32;
    println!("Test accuracy: {:.2}%", acc);
    Ok(())
}