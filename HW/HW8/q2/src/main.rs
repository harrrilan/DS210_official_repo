mod dataframe;
mod operations;

use crate::dataframe::{DataFrame, ColumnData};
use crate::operations::analytics::*; // Import our added methods.
use std::fs;

fn main() {
    // Write a sample CSV file for demonstration.
    let csv_data = "\
    Name,Number,PPG,YearBorn,TotalPoints,LikesPizza
    Kareem,33,24.6,1947,48387,true
    Karl,32,25.1,1963,46928,false
    LeBron,23,27.0,1984,46381,false
    Kobe,24,25.0,1978,43643,true
    Michael,23,30.1,1963,42292,false";

    fs::write("data.csv", csv_data).expect("Error writing data.csv");

    // Read CSV and build DataFrame.
    let df = DataFrame::read_csv("data.csv").expect("Error reading CSV");
    println!("Original DataFrame: {:#?}", df);

    // Use our analytics methods.
    let median = df.median_ppg().expect("Error computing median");
    println!("Median of PPG: {}", median);

    let subtraction = df.sub_columns().expect("Error performing subtraction");
    println!("Row-wise subtraction (TotalPoints - YearBorn): {:?}", subtraction);
}
