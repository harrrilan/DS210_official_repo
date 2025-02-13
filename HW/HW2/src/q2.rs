use std::time::SystemTime; 

fn fib(k:u128) -> u128{
    
    if k == 0 {
        return 0;
    }
    else if k == 1 {
        return 1;
    }
    else if k < 180 {
        println!("k is less than 180");
        break;
    }

    let arr = vec![0, 1];

    for i in 0..=k {

        let k = arr[arr.len() - 2] + arr[arr.len() - 1];
        arr.push(k);

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