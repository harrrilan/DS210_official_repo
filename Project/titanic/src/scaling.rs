use ndarray::{Array2};

pub fn standard_scale(train: &mut Array2<f32>, test: &mut Array2<f32>,) -> Vec<(f32, f32)> {
    let n_cols = train.ncols();
    assert_eq!(
        n_cols,
        test.ncols(),
        "train and test must have same number of columns"
    );

    let mut stats = Vec::with_capacity(n_cols - 1);

    // iterate over feature columns (skip label col 0)
    for col in 1..n_cols {
        let mut col_train = train.column_mut(col);
        let mean = col_train.mean().unwrap();
        // unbiased std (same as scikit-learn's default with ddof=0)
        let std  = col_train.std(0.0);

        // avoid division by zero
        let std = if std == 0.0 { 1.0 } else { std };

        // scale train
        col_train -= mean;
        col_train /= std;

        // scale test using *train* stats
        let mut col_test = test.column_mut(col);
        col_test -= mean;
        col_test /= std;

        stats.push((mean, std));
    }

    stats
}


mod tests {
    use ndarray::{array, s, Array2};
    use super::standard_scale;


    fn close(a: f32, b: f32, tol: f32) -> bool {
        (a - b).abs() < tol
    }
    
    #[test]
    fn standard_scale_basic() {
        // 3 rows, 1 label column (col 0) + 2 feature columns (1,2)
        let mut train: Array2<f32> = array![
            [0., 1.,  2.],
            [1., 3.,  4.],
            [0., 5.,  6.],
        ];
        let mut test: Array2<f32> = array![
            [1., 2.,  3.],
            [0., 4.,  5.],
        ];

        let stats = standard_scale(&mut train, &mut test);
        assert_eq!(stats.len(), 2);                         // two feature columns

        // training features should now have mean≈0 and std≈1
        let means = train.slice(s![.., 1..]).mean_axis(ndarray::Axis(0)).unwrap();
        let stds  = train.slice(s![.., 1..]).std_axis(ndarray::Axis(0), 0.0);
        for &m in means.iter() { close(m, 0.0, 1e-6); }
        for &s in stds.iter()  { close(s, 1.0, 1e-6); }

        // label column (0) must remain untouched
        assert_eq!(train.column(0).to_vec(), vec![0., 1., 0.]);
        assert_eq!(test .column(0).to_vec(), vec![1., 0.]);
    }

    #[test]
    fn standard_scale_constant_feature() {
        // feature in col 1 is constant (=5)
        let mut train: Array2<f32> = array![
            [0., 5.0, 1.0],
            [1., 5.0, 2.0],
            [0., 5.0, 3.0],
        ];
        let mut test = train.clone(); // columns match

        let stats = standard_scale(&mut train, &mut test);

        // std reported as 1.0 for that constant column
        assert_eq!(stats[0].1, 1.0);

        // scaled values for constant column should all be 0
        assert!(train.column(1).iter().all(|v| v.abs() < 1e-6));
        assert!(test .column(1).iter().all(|v| v.abs() < 1e-6));
    }

    #[test]
    #[should_panic(expected = "train and test must have same number of columns")]
    fn standard_scale_panics_on_column_mismatch() {
        let mut train: Array2<f32> = array![[0., 1.]];
        let mut test : Array2<f32> = array![[0., 1., 2.]]; // extra feature
        let _ = standard_scale(&mut train, &mut test);
    }
}