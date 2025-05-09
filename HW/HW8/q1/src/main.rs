use std::fs;
use std::fmt;
use std::error::Error;

/// Define the possible underlying column types.
#[derive(Debug, Clone)]
pub enum ColumnData {
    StringVec(Vec<String>),
    F64Vec(Vec<f64>),
    I64Vec(Vec<i64>),
    BoolVec(Vec<bool>),
}

impl ColumnData {
    /// Returns the length (number of rows) in the column.
    pub fn len(&self) -> usize {
        match self {
            ColumnData::StringVec(v) => v.len(),
            ColumnData::F64Vec(v) => v.len(),
            ColumnData::I64Vec(v) => v.len(),
            ColumnData::BoolVec(v) => v.len(),
        }
    }

    /// Helper method: given a list of row indices, return a new ColumnData with only those rows.
    pub fn filter_by_indices(&self, indices: &[usize]) -> ColumnData {
        match self {
            ColumnData::StringVec(vec) => {
                let filtered = indices.iter().filter_map(|&i| vec.get(i).cloned()).collect();
                ColumnData::StringVec(filtered)
            },
            ColumnData::F64Vec(vec) => {
                let filtered = indices.iter().filter_map(|&i| vec.get(i).cloned()).collect();
                ColumnData::F64Vec(filtered)
            },
            ColumnData::I64Vec(vec) => {
                let filtered = indices.iter().filter_map(|&i| vec.get(i).cloned()).collect();
                ColumnData::I64Vec(filtered)
            },
            ColumnData::BoolVec(vec) => {
                let filtered = indices.iter().filter_map(|&i| vec.get(i).cloned()).collect();
                ColumnData::BoolVec(filtered)
            },
        }
    }
}


/// DataFrame struct, representing data organized by columns and rows.
#[derive(Debug, Clone)]
pub struct DataFrame {
    pub labels: Vec<String>,
    pub columns: Vec<ColumnData>,
}

#[derive(Debug, Clone)]
pub enum DataFrameError {
    LabelColumnMismatch,
    InconsistentRowCount(usize, usize), // expected, found
    ColumnNotFound(String),
    TypeMismatch(String),
    FileError(String),
    CSVParseError(String),
}

impl fmt::Display for DataFrameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataFrameError::LabelColumnMismatch => write!(f, "Number of labels does not match number of columns"),
            DataFrameError::InconsistentRowCount(expected, found) => write!(f, "Inconsistent row count: expected {}, found {}", expected, found),
            DataFrameError::ColumnNotFound(label) => write!(f, "Column not found: {}", label),
            DataFrameError::TypeMismatch(msg) => write!(f, "Type mismatch error: {}", msg),
            DataFrameError::FileError(msg) => write!(f, "File error: {}", msg),
            DataFrameError::CSVParseError(msg) => write!(f, "CSV parse error: {}", msg),
        }
    }
}

impl Error for DataFrameError {}

impl DataFrame {
    /// Auxiliary constructor function.
    pub fn new(labels: Vec<String>, columns: Vec<ColumnData>) -> Result<DataFrame, DataFrameError> {
        // Validate label count.
        if labels.len() != columns.len() {
            return Err(DataFrameError::LabelColumnMismatch);
        }
        // Validation: All columns must have the same number of rows.
        if !columns.is_empty() {
            let expected_rows = columns[0].len();
            for col in columns.iter().skip(1) {
                if col.len() != expected_rows {
                    return Err(DataFrameError::InconsistentRowCount(expected_rows, col.len()));
                }
            }
        }
        Ok(DataFrame { labels, columns })
    }

    pub fn read_csv(file_path: &str) -> Result<DataFrame, DataFrameError> {
        let content = fs::read_to_string(file_path)
            .map_err(|e| DataFrameError::FileError(e.to_string()))?;
        let mut lines = content.lines();
        // Read header.
        let header = lines.next().ok_or_else(|| DataFrameError::CSVParseError("Missing header".to_string()))?;
        let labels: Vec<String> = header.split(',')
                                          .map(|s| s.trim().to_string())
                                          .collect();

        // Prepare vectors for each column.
        let mut names: Vec<String> = Vec::new();
        let mut numbers: Vec<i64> = Vec::new();
        let mut ppgs: Vec<f64> = Vec::new();
        let mut yearborns: Vec<i64> = Vec::new();
        let mut totalpoints: Vec<i64> = Vec::new();
        let mut likespizza: Vec<bool> = Vec::new();

        // Process each row.
        for (lineno, line) in lines.enumerate() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            let fields: Vec<&str> = trimmed.split(',')
                                           .map(|s| s.trim())
                                           .collect();
            if fields.len() != 6 {
                return Err(DataFrameError::CSVParseError(
                    format!("Line {} does not have 6 fields", lineno + 2)
                ));
            }
            names.push(fields[0].to_string());
            numbers.push(fields[1]
                .parse::<i64>()
                .map_err(|e| DataFrameError::CSVParseError(format!("Error parsing Number on line {}: {}", lineno + 2, e)))?);
            ppgs.push(fields[2]
                .parse::<f64>()
                .map_err(|e| DataFrameError::CSVParseError(format!("Error parsing PPG on line {}: {}", lineno + 2, e)))?);
            yearborns.push(fields[3]
                .parse::<i64>()
                .map_err(|e| DataFrameError::CSVParseError(format!("Error parsing YearBorn on line {}: {}", lineno + 2, e)))?);
            totalpoints.push(fields[4]
                .parse::<i64>()
                .map_err(|e| DataFrameError::CSVParseError(format!("Error parsing TotalPoints on line {}: {}", lineno + 2, e)))?);
            likespizza.push(fields[5]
                .parse::<bool>()
                .map_err(|e| DataFrameError::CSVParseError(format!("Error parsing LikesPizza on line {}: {}", lineno + 2, e)))?);
        }
        // Build the DataFrame.
        let columns = vec![
            ColumnData::StringVec(names),
            ColumnData::I64Vec(numbers),
            ColumnData::F64Vec(ppgs),
            ColumnData::I64Vec(yearborns),
            ColumnData::I64Vec(totalpoints),
            ColumnData::BoolVec(likespizza),
        ];
        DataFrame::new(labels, columns)
    }

    pub fn add_column(&self, label: String, data: ColumnData) -> Result<DataFrame, DataFrameError> {
        let current_rows = if self.columns.is_empty() { 0 } else { self.columns[0].len() };
        if data.len() != current_rows {
            return Err(DataFrameError::InconsistentRowCount(current_rows, data.len()));
        }
        let mut new_labels = self.labels.clone();
        new_labels.push(label);
        let mut new_columns = self.columns.clone();
        new_columns.push(data);
        DataFrame::new(new_labels, new_columns)
    }

    pub fn merge_frame(&self, other: &DataFrame) -> Result<DataFrame, DataFrameError> {
        // Check that the labels match exactly.
        if self.labels != other.labels {
            return Err(DataFrameError::LabelColumnMismatch);
        }
        if self.columns.len() != other.columns.len() {
            return Err(DataFrameError::LabelColumnMismatch);
        }

        let mut merged_columns = Vec::with_capacity(self.columns.len());
        for (col1, col2) in self.columns.iter().zip(&other.columns) {
            // Check that the variants match.
            let merged = match (col1, col2) {
                (ColumnData::StringVec(v1), ColumnData::StringVec(v2)) => {
                    let mut v = v1.clone();
                    v.extend(v2.clone());
                    ColumnData::StringVec(v)
                },
                (ColumnData::F64Vec(v1), ColumnData::F64Vec(v2)) => {
                    let mut v = v1.clone();
                    v.extend(v2.clone());
                    ColumnData::F64Vec(v)
                },
                (ColumnData::I64Vec(v1), ColumnData::I64Vec(v2)) => {
                    let mut v = v1.clone();
                    v.extend(v2.clone());
                    ColumnData::I64Vec(v)
                },
                (ColumnData::BoolVec(v1), ColumnData::BoolVec(v2)) => {
                    let mut v = v1.clone();
                    v.extend(v2.clone());
                    ColumnData::BoolVec(v)
                },
                // Incompatible types.
                _ => return Err(DataFrameError::TypeMismatch("Column types do not match".to_string())),
            };
            merged_columns.push(merged);
        }
        DataFrame::new(self.labels.clone(), merged_columns)
    }

    pub fn restrict_columns(&self, desired: &[String]) -> Result<DataFrame, DataFrameError> {
        let mut new_labels = Vec::with_capacity(desired.len());
        let mut new_columns = Vec::with_capacity(desired.len());
        for label in desired {
            if let Some(pos) = self.labels.iter().position(|l| l == label) {
                new_labels.push(self.labels[pos].clone());
                new_columns.push(self.columns[pos].clone());
            } else {
                return Err(DataFrameError::ColumnNotFound(label.clone()));
            }
        }
        DataFrame::new(new_labels, new_columns)
    }

    pub fn filter<F>(&self, column_label: &str, predicate: F) -> Result<DataFrame, DataFrameError>
    where
        F: Fn(&f64) -> bool,
    {
        // Find the position of the requested column.
        let pos = self.labels.iter().position(|l| l == column_label)
            .ok_or_else(|| DataFrameError::ColumnNotFound(column_label.to_string()))?;
        
        // Expect the column to be f64 type.
        let indices: Vec<usize> = match &self.columns[pos] {
            ColumnData::F64Vec(vec) => {
                vec.iter()
                   .enumerate()
                   .filter_map(|(i, val)| if predicate(val) { Some(i) } else { None })
                   .collect()
            },
            _ => return Err(DataFrameError::TypeMismatch(format!("Column {} is not of type f64", column_label))),
        };

        // Use the collected indices to filter every column.
        let new_columns: Vec<ColumnData> = self.columns.iter()
            .map(|col| col.filter_by_indices(&indices))
            .collect();
        // The labels remain the same.
        DataFrame::new(self.labels.clone(), new_columns)
    }

    pub fn column_op<F, R>(&self, op: F) -> R 
    where 
        F: Fn(&[ColumnData]) -> R,
    {
        op(&self.columns)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read a DataFrame from CSV.
    let df = DataFrame::read_csv("data.csv")?;
    println!("Original DataFrame: {:?}", df);

    // Add a new column "hall_of_fame" (for demonstration, mark players with PPG > 25.0)
    let hall_of_fame: Vec<bool> = if let ColumnData::F64Vec(ppg) = &df.columns[2] {
        ppg.iter().map(|&val| val > 25.0).collect()
    } else {
        vec![]
    };
    let df_with_hof = df.add_column("hall_of_fame".to_string(), ColumnData::BoolVec(hall_of_fame))?;
    println!("After adding hall_of_fame: {:?}", df_with_hof);

    // Merge with another DataFrame.
    let csv_data = "\
Name,Number,PPG,YearBorn,TotalPoints,LikesPizza
Magic,32,19.6,1959,17707,true
Larry,33,24.3,1956,21791,true";
    fs::write("data2.csv", csv_data)?;
    let df2 = DataFrame::read_csv("data2.csv")?;
    // Merge the two DataFrames (requires that the labels and column types match).
    let merged = df.merge_frame(&df2)?;
    println!("Merged DataFrame: {:?}", merged);

    // Restrict to only the columns "Name" and "TotalPoints".
    let restricted = merged.restrict_columns(&vec!["Name".to_string(), "TotalPoints".to_string()])?;
    println!("Restricted DataFrame: {:?}", restricted);

    // Filter rows: find all rows with PPG > 25.0.
    let filtered = df.filter("PPG", |val| *val > 25.0)?;
    println!("Filtered DataFrame: {:?}", filtered);

    // Column operation: simply count the number of rows in the first column.
    let row_count = df.column_op(|cols| cols[0].len());
    println!("Row count (via column_op): {}", row_count);

    Ok(())
}