use std::time::SystemTime; 


fn sum_of_cube(k:u8) -> u32{

    let mut sum:u32 = 0;

    for i in 1..=k {
        let cube = (i as u32) * (i as u32) * (i as u32);
        sum = cube + sum;
    }

    sum
}


fn main() {
    let before = SystemTime::now(); 
    

    // replace this function
    let result = sum_of_cube(10);
    println!("sum_of_cube result: {}", result);


    let after = SystemTime::now(); 
    let difference = after.duration_since(before); 
    let difference = difference.expect("Did the clock go back?"); 
    println!("Time it took: {:?}", difference); 
}