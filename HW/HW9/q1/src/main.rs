mod load_data;
mod neural_net;
use load_data::load_csv;
use neural_net::NeuralNetwork;
use ndarray::Axis;



// fn main() {
//     let train_file = "data/mnist_train.csv";

//     match load_csv(train_file) {
//         Ok((inputs, labels)) => {
//             println!("Loaded training data successfully!");
//             println!("Inputs shape: {:?}", inputs.dim());
//             println!("Labels shape: {:?}", labels.dim());
//         }
//         Err(e) => {
//             eprintln!("Failed to load training data: {}", e);
//         }
//     }
// }




// new code
fn main() {
    let train_file = "data/mnist_train.csv";
    let test_file = "data/mnist_test.csv";

    // Load training data
    let (train_inputs, train_labels) = match load_csv(train_file) {
        Ok((inputs, labels)) => {
            println!("Loaded training data successfully!");
            (inputs, labels)
        }
        Err(e) => {
            eprintln!("Failed to load training data: {}", e);
            return;
        }
    };

    // Load test data
    let (test_inputs, test_labels) = match load_csv(test_file) {
        Ok((inputs, labels)) => {
            println!("Loaded test data successfully!");
            (inputs, labels)
        }
        Err(e) => {
            eprintln!("Failed to load test data: {}", e);
            return;
        }
    };

    let input_size = 784;
    let hidden_size = 128; // Your choice
    let output_size = 10;
    let learning_rate = 0.001;

    let mut nn = NeuralNetwork::new(input_size, hidden_size, output_size, learning_rate);

    let epochs = 3;
    for epoch in 0..epochs {
        let loss = nn.train(&train_inputs, &train_labels);
        println!("Epoch {}/{} - Loss: {}", epoch + 1, epochs, loss);
    }

    // Evaluate on test data
    let (_, _, _, predictions) = nn.forward(&test_inputs);

    let predicted_labels = predictions
        .axis_iter(Axis(1))
        .map(|col| col.iter().enumerate().max_by(|a, b| a.1.partial_cmp(b.1).unwrap()).unwrap().0)
        .collect::<Vec<_>>();

    let true_labels = test_labels
        .axis_iter(Axis(1))
        .map(|col| col.iter().position(|&x| x == 1.0).unwrap())
        .collect::<Vec<_>>();

    let correct = predicted_labels
        .iter()
        .zip(true_labels.iter())
        .filter(|(pred, truth)| pred == truth)
        .count();

    let accuracy = correct as f32 / predicted_labels.len() as f32 * 100.0;
    println!("Test Accuracy: {:.2}%", accuracy);
}
