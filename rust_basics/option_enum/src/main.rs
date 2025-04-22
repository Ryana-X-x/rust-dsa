fn main() {
    println!("Hello from Ryana!");
}

// enum Option<T> {
//     Some(T) ,
//     None ,
// }

fn find_even_number(numbers: Vec<i32>) -> Option<i32> {
    for num in numbers {
        if num % 2 == 0 {
            return Some(num) ;
        }
    }
    None
}

fn get_username(user_id: u32) -> Option<String> {
    match user_id {
        1 => Some(String::from("Ryana")) ,
        2 => Some(String::from("Zoro")) ,
        _=> None ,
    }
}

fn divide_numbers(a: f64, b: f64) -> Option<f64> {
    
}

