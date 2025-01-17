use std::collections::HashMap ;

fn is_anagram(s: String, t: String) -> bool {
    
    // check length 
    if s.len() != t.len() {
        return false ;
    }

    let mut char_counts = HashMap::new() ;

    for c in s.chars() {
        *char_counts.entry(c).or_insert(0) += 1 ;
    }

    for c in t.chars() {
        *char_counts.entry(c).or_insert(0) -= 1 ;
    }

    for count in char_counts.values() {
        if *count != 0 {
            return false ;
        }
    }

    true
}

fn main() {
    let s = "anagram".to_string(); // Example input
    let t = "nagaram".to_string(); // Example input
    let result = is_anagram(s, t);
    println!("Is anagram: {}", result);
}