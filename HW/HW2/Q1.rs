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

    let k_2 = 0;
    let k_1 = 1;
    let k = 1;

    for i in 0..=k {

        let k = k_2 + k_1;

        let k_1 = k;
        let k_2 = k_1;
        k += 1;

    }
    return k;
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