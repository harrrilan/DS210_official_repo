mod load_file;
use load_file::{load_csv, print_head};
mod split;
use split::train_test_split;
use std::error::Error;
use std::env;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Current dir: {:?}", env::current_dir()?);
    let data = load_csv("data/processed_titanic.csv", ',', true)?;
    println!("Shape: {:?}", data.dim());

    print_head(&data, 5);

    let (train, test) = train_test_split(&data, 0.20, Some(42));
    println!("Train shape: {:?}", train.dim());
    println!("Test  shape: {:?}", test.dim());

    Ok(())
}