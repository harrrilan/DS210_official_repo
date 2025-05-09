use ndarray::Array2;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};


// Assume the data is in neumerical format
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

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr2;
    use tempfile::NamedTempFile;
    use std::io::Write;

    /// Helper that dumps `contents` into a fresh temporary file
    /// and returns the (still-open) file handle.
    fn write_temp(contents: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().expect("cannot create temp file");
        write!(file, "{}", contents).expect("cannot write temp file");
        file
    }

    #[test]
    fn load_csv_with_headers() {
        let csv = "c1,c2\n1.0,2.0\n3.0,4.0\n";
        let file = write_temp(csv);

        let got = load_csv(file.path().to_str().unwrap(), ',', true).unwrap();
        let want = arr2(&[[1.0, 2.0], [3.0, 4.0]]);

        assert_eq!(got, want);
    }

    #[test]
    fn load_csv_without_headers_semicolon_delim() {
        let csv = "1.0;2.0\n3.0;4.0\n";
        let file = write_temp(csv);

        let got = load_csv(file.path().to_str().unwrap(), ';', false).unwrap();
        let want = arr2(&[[1.0, 2.0], [3.0, 4.0]]);

        assert_eq!(got, want);
    }

    #[test]
    fn load_csv_inconsistent_columns_should_error() {
        let csv = "1,2,3\n4,5\n"; // second row has fewer columns
        let file = write_temp(csv);

        assert!(load_csv(file.path().to_str().unwrap(), ',', false).is_err());
    }

    #[test]
    fn print_head_does_not_panic() {
        // We don’t assert on stdout here—just make sure the function runs.
        let arr = arr2(&[[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]]);
        print_head(&arr, 2);
    }
}
