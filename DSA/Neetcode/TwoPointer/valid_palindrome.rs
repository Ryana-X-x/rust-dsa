pub fn is_palindrome(s: String) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let (mut left, mut right) = (0, chars.len() - 1);
    while left < right {
        if !chars[left].is_alphanumeric() {
            left += 1;
            continue;
        }
        if !chars[right].is_alphanumeric() {
            right -= 1;
            continue;
        }
        if chars[left].to_ascii_lowercase() != chars[right].to_ascii_lowercase() {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}

fn main() {
    let s = "A man, a plan, a canal: Panama".to_string();
    println!("Is palindrome: {}", is_palindrome(s));
}
