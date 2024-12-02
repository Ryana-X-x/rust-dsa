// https://leetcode.com/problems/move-zeroes/description/
// q283
struct Solution ;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut slow = 0 ;

        for fast in 0..nums.len() {
            if nums[fast] != 0 {
                nums[slow] = nums[fast] ;
                slow += 1 ;
            }
        }

        for i in slow..nums.len() {
            nums[i] = 0 ;
        }
    }
}


fn main() {
    let mut nums = vec![0, 1, 0, 3, 12] ;
    println!("Array before moving zeroes: {:?}", nums) ;
    Solution::move_zeroes(&mut nums) ;
    println!("Array after moving zeroes: {:?}", nums) ;
}