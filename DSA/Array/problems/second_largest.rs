// https://www.geeksforgeeks.org/problems/second-largest3735/1

//? We will take 2 variables, largest and second largest.

fn second_largest_optimized(arr: &[i32]) -> Option<i32> {
    if arr.len() < 2 {
        return None ;
    }

    let mut largest = i32::MIN ;
    let mut second_largest = i32::MIN ;

    for &num in arr {
        if num > largest {
            second_largest = largest ;
            largest = num ;
        } else if num > second_largest && num < largest {
            second_largest = num ;
        }
    }

    if second_largest == i32::MIN {
        None 
    } else {
        Some(second_largest) 
    }
}

fn main() {
    let array = [1, 10, 2, 34, 56, 34, 23, 43, 11, 90, 78] ;
    
    match second_largest_optimized(&array) {
        Some(value) => println!("Second largest: {}", value) ,
        None => println!("No second largest element")
    }
}