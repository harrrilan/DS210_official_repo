use std::time::SystemTime; 

fn fib(k:u32) -> u128{
    
    if k == 0 {
        return 0;
    }
    else if k == 1 {
        return 1;
    }
    else if k < 49 {
        println!("k is less than 180");
        break;
    }

    let mut k_2 = 0;
    let mut k_1 = 1;
    let mut result = 1;

    for i in 2..=k {
        let result = k_2 + k_1;

        k_2 = k_1;
        k_1 = result;
        
    }
    k_1
}

fn main() {
    let before = SystemTime::now(); 
    
    let result = fib(10);
    println!("fibonacci result: {}", result)


    let after = SystemTime::now(); 
    let difference = after.duration_since(before); 
    let difference = difference.expect("Did the clock go back?"); 
    println!("Time it took: {:?}", difference); 
}