impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new() ;

        for ch in s.chars() {
            // Check which char is present now, if a closing parenthesis
            // then we check if the top(last inserted) 
            // parenthesis is the opening parenthesis of the same
            match ch {
                '(' | '{' | '[' => stack.push(ch) ,
                ')' => if stack.pop() != Some('(') { return false } ,
                '}' => if stack.pop() != Some('{') { return false }, 
                ']' => if stack.pop() != Some('[') { return false },  
                _=> return false ,
            } 
        }
        stack.is_empty() 
    }
}