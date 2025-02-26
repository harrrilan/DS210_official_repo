use std::io;
use std::time::SystemTime;

fn sum_of_cube(k: u8) -> u32 {
    let mut sum: u32 = 0;

    for i in 1..=k {
        let cube = (i as u32) * (i as u32) * (i as u32);
        sum = cube + sum;
    }

    sum
}

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();
    let k: u8 = input.parse().expect("Not a good number!");

    // Time and compute
    let before = SystemTime::now();
    let result = sum_of_cube(k);
    let after = SystemTime::now();
    let difference = after.duration_since(before).expect("Did the clock go back?");

    // Output
    println!("Sum of cubes from 1 to {}: {}", k, result);
    println!("Time it took: {:?}", difference);
}