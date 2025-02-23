use std::io::{self, BufRead};

/*
 * Complete the 'plusMinus' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn plusMinus(arr: &[i32]) {
    let mut pos = 0 ;
    let mut neg = 0 ;
    let mut zero = 0 ;

    // WHY DO WE NEED TO USE *i INSTEAD OF i?
    // we need to dereference i because i is a reference to an element in the array 
    for i in arr {
        if *i > 0 {
            pos += 1 ;
        } else if *i < 0 {
            neg += 1 ;
        } else {
            zero += 1 ;
        }
    }
    
    // WHY DO WE NEED TO USE f64?
    // we need to use f64 because we are dividing integers and we want to get a float
    let n = arr.len() as f64 ;
    let pos = pos as f64 / n ;
    let neg = neg as f64 / n ;
    let zero = zero as f64 / n ;

    // WHY DO WE NEED TO USE {:.6}?
    // we need to use {:.6} to format the output to 6 decimal places
    println!("{:.6}", pos) ;    
    println!("{:.6}", neg) ;
    println!("{:.6}", zero) ;
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    plusMinus(&arr);
}
