use linfa::prelude::*;
use linfa_logistic::{FittedLogisticRegression, LogisticRegression};
use ndarray::{Array1, Array2, Axis, Slice};
use std::error::Error;
use crate::evaluation::{Metrics, evaluate};


pub type Model = FittedLogisticRegression<f32, i32>;

pub fn fit_logistic_model(
    data: &Array2<f32>,
    max_iter: u32,
) -> Result<Model, Box<dyn Error>> {
    let (features, targets) = split_features_labels(data)?;

    let dataset = Dataset::new(features, targets);

    let model = LogisticRegression::default()
        .max_iterations(max_iter.into())
        .fit(&dataset)
        .map_err(|e| format!("Failed to fit model: {}", e))?;

    Ok(model)
}

fn split_features_labels(
    data: &Array2<f32>,
) -> Result<(Array2<f32>, Array1<i32>), Box<dyn Error>> {
    let n_cols = data.ncols();
    if n_cols < 2 {
        return Err("need ≥2 columns (features + label)".into());
    }

    let features = data
        .slice_axis(Axis(1), Slice::from(1..)) // ← columns 1 to end
        .to_owned();

    let labels = data
        .slice_axis(Axis(1), Slice::from(..1)) // ← column 0
        .to_owned()
        .into_shape(data.nrows())?
        .mapv(|x| x as i32);

    Ok((features, labels))
}

pub fn evaluate_model(model: &Model, data: &Array2<f32>) -> Result<Metrics, Box<dyn Error>> {
    let (features, labels) = split_features_labels(data)?;
    let predictions = model.predict(&features);
    Ok(evaluate(&predictions, &labels))
}


mod tests {
    use super::*;
    use ndarray::{array, Array1};

    #[test]
    fn split_features_labels_basic() {
        let data = array![
            [0., 10., 20.],
            [1., 30., 40.]
        ];

        let (x, y) = split_features_labels(&data).unwrap();
        assert_eq!(x.dim(), (2, 2));
        assert_eq!(y, Array1::from_vec(vec![0, 1]));
    }

    #[test]
    fn split_features_labels_single_column_errors() {
        let bad = array![[1.], [0.]];
        assert!(split_features_labels(&bad).is_err());
    }

    #[test]
    fn fit_and_evaluate_on_simple_separable_data() {
        // two clearly separable blobs
        let data = array![
            [0., 0.0],
            [0., 0.1],
            [0., 0.2],
            [1., 0.8],
            [1., 0.9],
            [1., 1.0]
        ];

        // 1. fit
        let model = fit_logistic_model(&data, 200).expect("model fits");

        // 2. predict
        let (features, labels) = split_features_labels(&data).unwrap();
        let preds = model.predict(&features);

        // 3. evaluate (via the new crate)
        let metrics = evaluate(&preds, &labels);  // Changed from evaluate_predictions to evaluate

        assert!(metrics.confusion.tp > 0);
        assert!(metrics.confusion.tn > 0);
        assert_eq!(metrics.confusion.fp, 0);
        assert_eq!(metrics.confusion.fn_, 0);
    }
}
