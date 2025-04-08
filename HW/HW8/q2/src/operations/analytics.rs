// src/operations/analytics.rs

use crate::dataframe::{DataFrame, ColumnData, DataFrameError};

impl DataFrame {
    /// Computes and returns the median for the "PPG" numerical column.
    ///
    /// It uses the `column_op()` method.
    pub fn median_ppg(&self) -> Result<f64, DataFrameError> {
        self.column_op(|cols| {
            let pos = self.labels.iter()
                .position(|l| l == "PPG")
                .ok_or_else(|| DataFrameError::ColumnNotFound("PPG".to_string()))?;
            match &cols[pos] {
                ColumnData::F64Vec(values) => {
                    if values.is_empty() {
                        return Err(DataFrameError::CSVParseError("PPG column is empty".to_string()));
                    }
                    let mut sorted = values.clone();
                    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
                    let n = sorted.len();
                    let median = if n % 2 == 1 {
                        sorted[n / 2]
                    } else {
                        (sorted[n / 2 - 1] + sorted[n / 2]) / 2.0
                    };
                    Ok(median)
                },
                _ => Err(DataFrameError::TypeMismatch("PPG column is not of type f64".to_string()))
            }
        })
    }

    /// Performs row-wise subtraction of the "TotalPoints" and "YearBorn" columns.
    ///
    /// Returns a vector of the result for each row (TotalPoints - YearBorn).
    /// It uses the `column_op()` method.
    pub fn sub_columns(&self) -> Result<Vec<i64>, DataFrameError> {
        self.column_op(|cols| {
            let tp_index = self.labels.iter()
                .position(|l| l == "TotalPoints")
                .ok_or_else(|| DataFrameError::ColumnNotFound("TotalPoints".to_string()))?;
            let yb_index = self.labels.iter()
                .position(|l| l == "YearBorn")
                .ok_or_else(|| DataFrameError::ColumnNotFound("YearBorn".to_string()))?;
            match (&cols[tp_index], &cols[yb_index]) {
                (ColumnData::I64Vec(totalpoints), ColumnData::I64Vec(yearborn)) => {
                    if totalpoints.len() != yearborn.len() {
                        return Err(DataFrameError::InconsistentRowCount(totalpoints.len(), yearborn.len()));
                    }
                    let result: Vec<i64> = totalpoints.iter()
                        .zip(yearborn.iter())
                        .map(|(tp, yb)| tp - yb)
                        .collect();
                    Ok(result)
                },
                _ => Err(DataFrameError::TypeMismatch("Either TotalPoints or YearBorn is not of type i64".to_string()))
            }
        })
    }
}
