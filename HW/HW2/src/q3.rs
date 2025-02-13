use std::time::SystemTime; 


fn sum_of_cube(k:u8) -> u32{

    let sum = 0;

    for i in 1..=k {

        let cube = i ** 3;
        let sum = cube + sum;
    }
    return sum;
}


fn main() {
    let before = SystemTime::now(); 
    

    // replace this function
    let result = sum_of_cube(10);
    println!("fibonacci result: {}", result)


    let after = SystemTime::now(); 
    let difference = after.duration_since(before); 
    let difference = difference.expect("Did the clock go back?"); 
    println!("Time it took: {:?}", difference); 
}