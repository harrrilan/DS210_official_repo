mod load_file;
use load_file::{load_csv, print_head};
use std::error::Error;
use std::env;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Current dir: {:?}", env::current_dir()?);
    let data = load_csv("data/processed_titanic.csv", ',', true)?;
    println!("Shape: {:?}", data.dim());

    print_head(&data, 5);

    Ok(())
}