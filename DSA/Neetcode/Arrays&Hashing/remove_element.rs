impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut j = 0 ; // write pointer

        for i in 0..nums.len() {
            if nums[i] != val {
                nums[j] = nums[i] ;
                j += 1 ;
            }
        }

        j as i32    //new length
    }
}


fn main() {
    let mut nums = vec![3, 2, 2, 3];
    let val = 3;
    let new_length = remove_element(&mut nums, val);
    
    println!("New length: {}", new_length);
    println!("Modified array: {:?}", &nums[..new_length as usize]);
}