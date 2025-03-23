use std::collections::HashMap;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let s = s.as_bytes(); 
        let mut freq_map = HashMap::new();
        let mut max_count = 0;
        let mut left = 0;
        let mut max_length = 0;
        let k = k as usize; 

        for right in 0..s.len() {
            let char = s[right] as char;
            *freq_map.entry(char).or_insert(0) += 1;
            max_count = max_count.max(*freq_map.get(&char).unwrap());

            let window_size = right - left + 1;
            if window_size - max_count > k {
                let left_char = s[left] as char;
                *freq_map.entry(left_char).or_insert(0) -= 1;
                left += 1;
            }
            max_length = max_length.max(right - left + 1);
        }
        max_length as i32
    }
}
