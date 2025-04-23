use std::error::Error;
use csv::Reader;
use ndarray::Array2;

pub fn load_csv(path: &str) -> Result<(Array2<f32>, Array2<f32>), Box<dyn Error>> {
    let mut rdr = Reader::from_path(path)?;
    let mut features = vec![];
    let mut labels = vec![];

    for result in rdr.records() {
        let record = result?;
        let label: u8 = record[0].parse()?;
        let pixels: Vec<f32> = record.iter().skip(1)
                                     .map(|x| x.parse::<f32>().unwrap() / 255.0)
                                     .collect();

        let mut one_hot = vec![0.0; 10];
        one_hot[label as usize] = 1.0;

        features.extend(pixels);
        labels.extend(one_hot);
    }

    let sample_count = labels.len() / 10;
    let input_array = Array2::from_shape_vec((sample_count, 784), features)?;
    let label_array = Array2::from_shape_vec((sample_count, 10), labels)?;

    Ok((input_array.t().to_owned(), label_array.t().to_owned()))
}
