mod load_file;
mod split;
mod regression;
mod scaling;
pub mod evaluation;

use load_file::{load_csv, print_head};
use split::train_test_split;
use regression::{fit_logistic_model, evaluate_model};

use std::error::Error;
use std::env;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Current dir: {:?}", env::current_dir()?);

    // specify the path to your CSV file
    let data = load_csv("data/processed_titanic.csv", ',', true)?;
    println!("Shape: {:?}", data.dim());

    print_head(&data, 5);

    let (mut train, mut test) = train_test_split(&data, 0.20, Some(42));
    println!("Train shape: {:?}", train.dim());
    println!("Test  shape: {:?}", test.dim());

    scaling::standard_scale(&mut train, &mut test);

    // fitting the model
    println!("Fitting logistic regression model...");
    let model = fit_logistic_model(&train, 500)?;
    println!("Model training complete.");

    // Evaluate model
    let metrics = evaluate_model(&model, &test)?;
    println!("Evaluation Metrics:");
    println!("Accuracy: {:.4}", metrics.accuracy);
    println!("F1 Score: {:.4}", metrics.f1);
    println!("Confusion Matrix:\n{}", metrics.confusion);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;               // bring `main` and the pipeline modules into scope
    use std::env;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::TempDir;

    /// Helper: build a minimal Titanic-style CSV in `data/processed_titanic.csv`
    /// inside a brand-new temporary directory.
    fn make_dummy_project_tree() -> TempDir {
        // 1. fresh temp workspace
        let tmp_dir = TempDir::new().expect("cannot create temp dir");

        // 2. create `data/` sub-dir
        let data_dir = tmp_dir.path().join("data");
        fs::create_dir(&data_dir).unwrap();

        // 3. write a tiny, valid CSV file the pipeline can ingest
        let csv_path = data_dir.join("processed_titanic.csv");
        let mut f = File::create(&csv_path).unwrap();
        writeln!(f, "survived,pclass,age").unwrap(); // header row
        writeln!(f, "0,3,22").unwrap();
        writeln!(f, "1,1,38").unwrap();
        writeln!(f, "1,3,26").unwrap();
        writeln!(f, "0,2,35").unwrap();

        tmp_dir
    }

    /// End-to-end smoke test: the whole `main()` pipeline should run without error.
    #[test]
    fn main_smoke_test() {
        // Build fake project tree & move into it
        let tmp_project = make_dummy_project_tree();
        let old_cwd = env::current_dir().unwrap();
        env::set_current_dir(tmp_project.path()).unwrap();

        // Run `main()` and capture its Result
        let result = crate::main();

        // Restore the original working directory
        env::set_current_dir(old_cwd).unwrap();

        // The pipeline should succeed
        assert!(result.is_ok(), "main() returned an error: {:?}", result.err());
    }
}
