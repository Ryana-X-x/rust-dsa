impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums.clone() ;
        nums.sort() ;
        let mut result = Vec::new() ;
        let n = nums.len() ;

        for i in 0..n {
            if i > 0 && nums[i] == nums[i -1] {
                continue ;  // Skip the duplicates
            }

            let target = -nums[i] ;
            let (mut left, mut right) = (i + 1, n - 1) ;
            
            while left < right {
                let sum = nums[left] + nums[right] ;
                if sum == target {
                    result.push(vec![nums[i], nums[left], nums[right]]) ;

                    // move pointers to skip duplicates
                    while left < right && nums[left] == nums[left + 1] {
                        left += 1 ;
                    }

                    while left < right && nums[right] == nums[right - 1] {
                        right -= 1 ;
                    }

                    left += 1 ;
                    right -= 1 ;
                } else if sum < target {
                    left += 1 ;
                } else {
                    right -= 1 ;
                }

            }
        }
        result
    }
}
