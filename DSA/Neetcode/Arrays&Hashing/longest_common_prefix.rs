impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::new() ;
        }

        let mut sorted_strs = strs.clone() ;
        sorted_strs.sort() ;

        let first = &sorted_strs[0] ;
        let last = &sorted_strs[strs.len() - 1] ;

        let mut prefix = String::new() ;

        for (c1, c2) in first.chars().zip(last.chars()) {
            if c1 == c2 {
                prefix.push(c1) ;
            }else {
                break ;
            }
        }
        prefix
    }
}

fn main() {
    let strs = vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ];
    let result = longest_common_prefix(strs);
    println!("Longest Common Prefix: {}", result);
}