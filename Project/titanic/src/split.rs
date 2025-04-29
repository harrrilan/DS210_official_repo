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



#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn split_sizes_are_correct() {
        let data = array![[1., 2.], [3., 4.], [5., 6.], [7., 8.]]; // 4 rows
        let (train, test) = train_test_split(&data, 0.25, Some(0));
        assert_eq!(test.nrows(), 1);
        assert_eq!(train.nrows(), 3);
    }
}
