//===================Question 1===================


fn fib(k:u32) -> u128{
    
    if k == 0 {
        return 0;
    }
    else if k == 1 {
        return 1;
    }
    
    let k_2 = 0;
    let k_1 = 1;
    let k = 1;

    for i in 0..=k {

        let k = k_2 + k_1;

        let k_1 = k;
        let k_2 = k_1;
        k += 1;

        let prev2 = k_1 - k_2;
        let prev1 = k_2;

    }
    return k;
}

fib(10)





//===================Question 2===================

















//===================Question 3===================










