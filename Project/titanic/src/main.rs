mod load_file;
mod split;
mod regression;
mod scaling;  

use load_file::{load_csv, print_head};
use split::train_test_split;
use regression::{fit_logistic_model, evaluate_model};

use std::error::Error;
use std::env;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Current dir: {:?}", env::current_dir()?);

    // specify the path to your CSV file
    let data = load_csv("data/processed_titanic.csv", ',', true)?;
    println!("Shape: {:?}", data.dim());

    print_head(&data, 5);

    let (mut train, mut test) = train_test_split(&data, 0.20, Some(42));
    println!("Train shape: {:?}", train.dim());
    println!("Test  shape: {:?}", test.dim());

    scaling::standard_scale(&mut train, &mut test);

    // fitting the model
    println!("Fitting logistic regression model...");
    let model = fit_logistic_model(&train, 500)?;
    println!("Model training complete.");

    // Evaluate model
    let metrics = evaluate_model(&model, &test)?;
    println!("Evaluation Metrics:");
    println!("Accuracy: {:.4}", metrics.accuracy);
    println!("F1 Score: {:.4}", metrics.f1);
    println!("Confusion Matrix:\n{}", metrics.confusion);

    Ok(())
}