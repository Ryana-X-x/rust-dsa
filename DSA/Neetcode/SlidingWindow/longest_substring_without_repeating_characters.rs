use std::collections::HashMap ;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut char_map = HashMap::new() ; // To Store characters their indexes
    let mut max_length = 0 ;
    let mut left = 0  ; // Sliding window start
    for (right, c) in s.chars().enumerate() {
        if let Some(&prev_index) = char_map.get(&c) {
            // Move the left pointer to avoid the duplicate character
            left = left.max(prev_index + 1) ;
        }
        // Update the character's latest index
        char_map.insert(c, right) ;
        max_length = max_length.max(right - left + 1) ;
    }
    max_length as i32
}

fn main() {
    let s = String::from("abcabcbb") ;
    let result = length_of_longest_substring(s) ;
    println!("{}", result) ;
}