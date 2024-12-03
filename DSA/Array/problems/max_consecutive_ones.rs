// https://leetcode.com/problems/max-consecutive-ones/description/
// q485

struct Solution ;

impl Solution {
    pub fn find_max_consective_ones(nums: Vec<i32>) -> i32 {
        let mut max_count = 0 ;
        let mut count = 0 ;

        for num in nums {
            if num == 1 {
                count += 1;
                max_count = max_count.max(count) ;
            } else {
                count = 0 ; 
            }
        }

        max_count 
    }
}


fn main() {
    let nums = vec![1, 1, 0, 1, 1, 1] ;
    let result = Solution::find_max_consective_ones(nums) ;
    println!("Maximum consecutive ones: {}", result) ;

}