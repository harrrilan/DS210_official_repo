use linfa::prelude::*;
use linfa_logistic::{FittedLogisticRegression, LogisticRegression};
use ndarray::{Array1, Array2, Axis, Slice};
use std::error::Error;


pub type Model = FittedLogisticRegression<f32, i32>;

pub fn fit_logistic_model(
    data: &Array2<f32>,
    max_iter: u32,
) -> Result<Model, Box<dyn Error>> {
    let (features, targets) = split_features_labels(data)?;

    let dataset = Dataset::new(features, targets);

    let model = LogisticRegression::default()
        .max_iterations(max_iter.into()) // API expects u64
        .fit(&dataset)?;

    Ok(model)
}

fn confusion_counts(pred: &Array1<i32>, truth: &Array1<i32>) -> (usize, usize, usize, usize) {
    let (mut tp, mut tn, mut fp, mut fn_) = (0, 0, 0, 0);
    for (&p, &t) in pred.iter().zip(truth.iter()) {
        match (p, t) {
            (1, 1) => tp += 1,
            (0, 0) => tn += 1,
            (1, 0) => fp += 1,
            (0, 1) => fn_ += 1,
            _      => {}   // ignore unexpected labels
        }
    }
    (tp, tn, fp, fn_)
}

pub struct Metrics {
    pub accuracy: f32,
    pub f1: f32,
    pub confusion: String,
}

pub fn evaluate_model(
    model: &Model,
    data: &Array2<f32>,
) -> Result<Metrics, Box<dyn Error>> {

    let (features, targets) = split_features_labels(data)?;
    let dataset = Dataset::new(features, targets.clone());

    let preds = model.predict(&dataset);
    
    let (tp, tn, fp, fn_) = confusion_counts(&preds, &targets);
    let accuracy  = (tp + tn) as f32 / targets.len() as f32;
    let precision = tp as f32 / (tp + fp) as f32;
    let recall    = tp as f32 / (tp + fn_) as f32;
    let f1        = 2.0 * precision * recall / (precision + recall + f32::EPSILON);

    Ok(Metrics {
        accuracy,
        f1,
        confusion: format!("TP: {}, TN: {}, FP: {}, FN: {}", tp, tn, fp, fn_),
    })
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


#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::{array, Array1};

    #[test]
    fn split_features_labels_basic() {
        let data = array![
            [0., 10., 20.],   // label 0, two features
            [1., 30., 40.]    // label 1
        ];

        let (x, y) = super::split_features_labels(&data).unwrap();
        assert_eq!(x.dim(), (2, 2));                 // 2 rows × 2 feature-cols
        assert_eq!(y, Array1::from_vec(vec![0, 1])); // labels cast to i32
    }

    #[test]
    fn split_features_labels_single_column_errors() {
        let bad = array![[1.], [0.]];
        assert!(super::split_features_labels(&bad).is_err());
    }

    #[test]
    fn confusion_counts_matches_manual_tally() {
        let pred  = array![1, 0, 1, 0];
        let truth = array![1, 0, 0, 1];
        let (tp, tn, fp, fn_) = super::confusion_counts(&pred, &truth);

        assert_eq!((tp, tn, fp, fn_), (1, 1, 1, 1)); // hand-checked
    }

    #[test]
    fn fit_and_evaluate_on_simple_separable_data() {
        // Two clearly separable blobs along the single feature axis.
        let data = array![
            [0., 0.0],
            [0., 0.1],
            [0., 0.2],
            [1., 0.8],
            [1., 0.9],
            [1., 1.0]
        ];

        let model   = fit_logistic_model(&data, 200).expect("model fits");
        let metrics = evaluate_model(&model, &data).expect("metrics OK");

        assert!(metrics.accuracy >= 0.95);
        assert!(metrics.f1        >= 0.95);

        assert!(metrics.confusion.contains("TP:"));
        assert!(metrics.confusion.contains("TN:"));
    }
}
