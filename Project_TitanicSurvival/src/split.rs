use ndarray::{Array2, Axis};
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};

pub fn train_test_split(
    data: &Array2<f32>,
    test_ratio: f32,
    seed: Option<u64>,
) -> (Array2<f32>, Array2<f32>) {
    assert!((0.0..1.0).contains(&test_ratio), "test_ratio must be between 0 and 1 (exclusive)");

    let n_rows = data.nrows();
    let n_test = ((n_rows as f32) * test_ratio).round() as usize;

    // Build index vector and shuffle it deterministically if a seed is given.
    let mut indices: Vec<usize> = (0..n_rows).collect();
    let mut rng: StdRng = match seed {
        Some(s) => SeedableRng::seed_from_u64(s),
        None => SeedableRng::from_entropy(),
    };
    indices.shuffle(&mut rng);

    // First `n_test` indices → test, remainder → train.
    let (test_idx, train_idx) = indices.split_at(n_test);

    let test  = data.select(Axis(0), test_idx);
    let train = data.select(Axis(0), train_idx);

    (train, test)
}


///////////test code////////////
#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;
    use ordered_float::NotNan;
    use std::collections::HashSet;

    fn row_set(a: &Array2<f32>) -> HashSet<Vec<NotNan<f32>>> {
        (0..a.nrows())
            .map(|i| {
                a.row(i)
                    .iter()
                    .map(|&x| NotNan::new(x).unwrap())   // safe: our test data has no NaNs
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    #[test]
    fn split_sizes_are_correct() {
        let data = array![[1., 2.], [3., 4.], [5., 6.], [7., 8.]]; // 4 rows
        let (train, test) = train_test_split(&data, 0.25, Some(0));
        assert_eq!(test.nrows(), 1);
        assert_eq!(train.nrows(), 3);
    }

    #[test]
    fn reproducible_with_same_seed() {
        let data = array![[0.], [1.], [2.], [3.], [4.]];
        let (train1, test1) = train_test_split(&data, 0.4, Some(42));
        let (train2, test2) = train_test_split(&data, 0.4, Some(42));
        assert_eq!(train1, train2);
        assert_eq!(test1, test2);
    }

    #[test]
    fn different_seeds_give_different_split() {
        let data = array![[0.], [1.], [2.], [3.], [4.]];
        let (train1, test1) = train_test_split(&data, 0.4, Some(1));
        let (train2, test2) = train_test_split(&data, 0.4, Some(2));
        // Not a strict equality test—just extremely unlikely to be identical
        assert!(train1 != train2 || test1 != test2,
            "identical splits with different seeds are highly improbable");
    }

    #[test]
    fn no_overlap_and_no_missing_rows() {
        let data = array![[10.], [20.], [30.], [40.]];
        let (train, test) = train_test_split(&data, 0.5, Some(7));

        let train_set = row_set(&train);
        let test_set  = row_set(&test);
        let full_set  = row_set(&data);

        // Intersection should be empty
        assert!(train_set.is_disjoint(&test_set));

        // Union should equal original
        let mut combined = train_set;
        combined.extend(test_set);
        assert_eq!(combined, full_set);
    }

    #[test]
    fn handles_tiny_inputs() {
        let data = array![[42.]];
        let (train, test) = train_test_split(&data, 0.5, Some(0));
        // 1 row → with rounding you’ll either get (1,0) or (0,1) – both valid
        assert_eq!(train.nrows() + test.nrows(), 1);
    }
}
