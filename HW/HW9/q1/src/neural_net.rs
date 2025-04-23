use ndarray::prelude::*;
use rand::Rng;


fn populate_array(arr: &mut Array2<f32>, m: usize, n: usize) {
    let mut rng = rand::rng();
    for i in 0..m {
        for j in 0..n {
            arr[(i, j)] = rng.random_range(-1.0..1.0);
        }
    }
}

// sigmoid function




// derivative of sigmoid function




