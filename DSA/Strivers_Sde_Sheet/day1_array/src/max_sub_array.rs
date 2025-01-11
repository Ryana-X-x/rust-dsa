// Leetcode 53
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_sum = nums[0] ;
        let mut curr_sum = nums[0] ;

        for i in 1..nums.len() {
            curr_sum = nums[i].max(curr_sum + nums[i]) ;
            max_sum = max_sum.max(curr_sum) ;
        }
        max_sum
    }
}