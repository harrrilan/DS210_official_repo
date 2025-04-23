mod load_data;
use load_data::load_csv;

fn main() {
    let train_file = "data/mnist_train.csv";

    match load_csv(train_file) {
        Ok((inputs, labels)) => {
            println!("Loaded training data successfully!");
            println!("Inputs shape: {:?}", inputs.dim());
            println!("Labels shape: {:?}", labels.dim());
        }
        Err(e) => {
            eprintln!("Failed to load training data: {}", e);
        }
    }
}
