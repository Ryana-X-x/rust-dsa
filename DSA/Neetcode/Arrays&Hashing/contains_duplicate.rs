use std::collections::HashSet ;

fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut seen = HashSet::new() ;
    for num in nums {
        if seen.contains(&num) {
            return true ;
        }
        seen.insert(num) ;
    }
    false 
}

fn main() {
    let nums = vec![1, 2, 3, 1]; // Example input
    let result = contains_duplicate(nums);
    println!("Contains duplicate: {}", result);
}