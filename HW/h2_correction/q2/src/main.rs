use std::time::SystemTime;

fn main() {
    // Create array F of length 181 with type u128
    let mut f: [u128; 181] = [0; 181];

    // Set base cases
    f[0] = 0;
    f[1] = 1;

    // Compute F[i] using previous entries with a for loop
    for i in 2..=180 {
        f[i] = f[i - 2] + f[i - 1]; // Constant-time operation using prior values
    }

    // Output all computed Fibonacci numbers
    println!("Fibonacci numbers from F_0 to F_180:");
    for i in 0..=180 {
        println!("F_{}: {}", i, f[i]);
    }
}