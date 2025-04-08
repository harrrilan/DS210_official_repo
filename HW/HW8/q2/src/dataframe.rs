// src/dataframe.rs

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

    /// Reads a CSV file and builds a DataFrame.
    ///
    /// This expects a CSV file with the following content:
    ///
    /// ```csv
    /// Name,Number,PPG,YearBorn,TotalPoints,LikesPizza
    /// Kareem,33,24.6,1947,48387,true
    /// Karl,32,25.1,1963,46928,false
    /// LeBron,23,27.0,1984,46381,false
    /// Kobe,24,25.0,1978,43643,true
    /// Michael,23,30.1,1963,42292,false
    /// ```
    ///
    /// With expected types: String, i64, f64, i64, i64, bool.
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

    /// Adds a new column to the DataFrame.
    ///
    /// This checks that the new column has the same number of rows as the existing DataFrame.
    /// Returns a new DataFrame with the extra column.
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

    /// Merges two DataFrames by appending the rows.
    ///
    /// This checks that the labels and column types match.
    pub fn merge_frame(&self, other: &DataFrame) -> Result<DataFrame, DataFrameError> {
        if self.labels != other.labels {
            return Err(DataFrameError::LabelColumnMismatch);
        }
        if self.columns.len() != other.columns.len() {
            return Err(DataFrameError::LabelColumnMismatch);
        }

        let mut merged_columns = Vec::with_capacity(self.columns.len());
        for (col1, col2) in self.columns.iter().zip(&other.columns) {
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
                _ => return Err(DataFrameError::TypeMismatch("Column types do not match".to_string())),
            };
            merged_columns.push(merged);
        }
        DataFrame::new(self.labels.clone(), merged_columns)
    }

    /// Returns a new DataFrame with only the columns having labels in `desired`.
    ///
    /// Returns an error if any of the desired labels do not exist.
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

    /// Filters the DataFrame by applying a closure on an f64 column.
    ///
    /// Returns a new DataFrame containing only the rows for which the predicate returns true.
    pub fn filter_f64<F>(&self, column_label: &str, predicate: F) -> Result<DataFrame, DataFrameError>
    where
        F: Fn(&f64) -> bool,
    {
        let pos = self.labels.iter().position(|l| l == column_label)
            .ok_or_else(|| DataFrameError::ColumnNotFound(column_label.to_string()))?;
        let indices: Vec<usize> = match &self.columns[pos] {
            ColumnData::F64Vec(vec) => {
                vec.iter().enumerate()
                   .filter_map(|(i, val)| if predicate(val) { Some(i) } else { None })
                   .collect()
            },
            _ => return Err(DataFrameError::TypeMismatch(format!("Column {} is not of type f64", column_label))),
        };
        let new_columns: Vec<ColumnData> = self.columns.iter()
            .map(|col| col.filter_by_indices(&indices))
            .collect();
        DataFrame::new(self.labels.clone(), new_columns)
    }

    /// Generic column operation, applies an arbitrary operation on the entire set of columns.
    pub fn column_op<F, R>(&self, op: F) -> R 
    where 
        F: Fn(&[ColumnData]) -> R,
    {
        op(&self.columns)
    }
}
