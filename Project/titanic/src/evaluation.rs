use ndarray::Array1;

/// Counts of true/false positives/negatives.
#[derive(Debug, Clone, Copy)]
pub struct ConfusionMatrix {
    pub tp: usize,
    pub tn: usize,
    pub fp: usize,
    pub fn_: usize,
}

impl std::fmt::Display for ConfusionMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "\nPredicted:\n\
             |        |  Pos  |  Neg  |\n\
             |--------|-------|-------|\n\
             |  Pos   |  {}   |  {}   |\n\
             |  Neg   |  {}   |  {}   |",
            self.tp, self.fn_, 
            self.fp, self.tn
        )
    }
}

/// Common classification metrics.
#[derive(Debug, Clone, Copy)]
pub struct Metrics {
    pub accuracy: f32,
    pub f1: f32,
    pub confusion: ConfusionMatrix,
}

/// Internal helper that tallies the elements of a confusion matrix.
fn confusion_counts(pred: &Array1<i32>, truth: &Array1<i32>) -> (usize, usize, usize, usize) {
    let (mut tp, mut tn, mut fp, mut fn_) = (0, 0, 0, 0);
    for (&p, &t) in pred.iter().zip(truth.iter()) {
        match (p, t) {
            (1, 1) => tp += 1,
            (0, 0) => tn += 1,
            (1, 0) => fp += 1,
            (0, 1) => fn_ += 1,
            _ => {}
        }
    }
    (tp, tn, fp, fn_)
}

/// Evaluate predicted labels against ground‑truth labels.
///
/// # Arguments
/// * `pred`  – Model predictions, expected to be 0 or 1.
/// * `truth` – Ground‑truth labels (0 or 1).
///
/// # Returns
/// A [`Metrics`] struct that contains accuracy, F1 score and a full [`ConfusionMatrix`].
pub fn evaluate(pred: &Array1<i32>, truth: &Array1<i32>) -> Metrics {
    assert_eq!(pred.len(), truth.len(), "predictions and truth lengths differ");

    let (tp, tn, fp, fn_) = confusion_counts(pred, truth);

    let accuracy = (tp + tn) as f32 / truth.len() as f32;
    let precision = if tp + fp == 0 {
        0.0
    } else {
        tp as f32 / (tp + fp) as f32
    };
    let recall = if tp + fn_ == 0 {
        0.0
    } else {
        tp as f32 / (tp + fn_) as f32
    };
    let f1 = if (precision + recall).abs() < f32::EPSILON {
        0.0
    } else {
        2.0 * precision * recall / (precision + recall)
    };

    Metrics {
        accuracy,
        f1,
        confusion: ConfusionMatrix { tp, tn, fp, fn_ },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn confusion_counts_matches_expected() {
        let pred = array![1, 0, 1, 0];
        let truth = array![1, 0, 0, 1];
        let (tp, tn, fp, fn_) = super::confusion_counts(&pred, &truth);

        assert_eq!((tp, tn, fp, fn_), (1, 1, 1, 1));
    }

    #[test]
    fn evaluate_on_perfect_predictions() {
        let pred = array![0, 1, 0, 1];
        let truth = array![0, 1, 0, 1];
        let m = evaluate(&pred, &truth);

        assert_eq!(m.accuracy, 1.0);
        assert_eq!(m.f1, 1.0);
        assert_eq!(m.confusion.fp, 0);
        assert_eq!(m.confusion.fn_, 0);
    }
}
