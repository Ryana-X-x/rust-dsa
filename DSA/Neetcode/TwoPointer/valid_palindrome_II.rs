impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let mut chars: Vec<char> = s.chars().collect() ;
        let (mut left, mut right, mut counter) = (0, chars.len() - 1, 0) ;

        while left < right {
            if !chars[left].is_alphanumeric() {
                left += 1 ;
                continue ;
            }
            if !chars[right].is_alphanumeric() {
                right -= 1 ;
                continue ;
            }

            if chars[left].to_ascii_lowercase() != chars[right].to_ascii_lowercase()  {
                if counter < 1 {
                    return Self::valid_sub_palindrome(&chars, left + 1, right) || Self::valid_sub_palindrome(&chars, left, right - 1) ;
                }else {
                    return false ;
                }
            }
            left += 1 ;
            right -= 1 ;
        }
        true
    }
    fn valid_sub_palindrome(chars: &Vec<char>, mut left: usize, mut right: usize) -> bool {
        while left < right {
            if chars[left].to_ascii_lowercase() != chars[right].to_ascii_lowercase() {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }
}