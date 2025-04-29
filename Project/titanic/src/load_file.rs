use ndarray::Array2;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load_csv(
    path: &str,
    delimiter: char,
    has_headers: bool,
) -> Result<Array2<f32>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut data = Vec::new();
    let mut num_columns = None;
    let mut num_rows = 0;

    for (i, line_result) in reader.lines().enumerate() {
        let line = line_result?;
        if has_headers && i == 0 {
            continue;
        }

        let row: Vec<f32> = line
            .split(delimiter)
            .map(|s| s.trim().parse::<f32>())
            .collect::<Result<_, _>>()?;

        if let Some(cols) = num_columns {
            if row.len() != cols {
                return Err(format!("Row {} has inconsistent number of columns", i).into());
            }
        } else {
            num_columns = Some(row.len());
        }

        data.extend(row);
        num_rows += 1;
    }

    let cols = num_columns.ok_or("No data found")?;
    let array = Array2::from_shape_vec((num_rows, cols), data)?;

    Ok(array)
}

pub fn print_head(array: &Array2<f32>, n: usize) {
    let rows = array.rows();
    let total_rows = array.nrows();
    let total_cols = array.ncols();

    let n = n.min(total_rows); // Don’t print more than available

    println!("Showing first {} of {} rows, {} columns:", n, total_rows, total_cols);
    for i in 0..n {
        let row = rows.clone().into_iter().nth(i).unwrap();
        println!("{:?}", row);
    }
}