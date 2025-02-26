use std::time::SystemTime;

fn fib(k: u32) -> u128 {
    if k == 0 {
        return 0;
    } else if k == 1 {
        return 1;
    }
    // Recursive calls as required: F_k = F_{k-2} + F_{k-1}
    fib(k - 2) + fib(k - 1)
}

fn main() {
    for k in 0..=49 {
        println!("Calculating for k = {}", k);

        // Measure time for each fib(k)
        let before = SystemTime::now();
        let result = fib(k);
        let after = SystemTime::now();
        let difference = after.duration_since(before).expect("Did the clock go back?");

        // Print k, F_k, and time
        println!("k: {} | F_k: {} | Time: {:?}", k, result, difference);
    }
}