impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut left = 0 ;
        let mut right = height.len() - 1 ;
        let mut left_max = 0 ;
        let mut right_max = 0 ;
        let mut water_trapped = 0 ;

        while left < right {
            if height[left] < height[right] {
                if height[left] >= left_max {
                    left_max = height[left] ;
                } else {
                    water_trapped += left_max - height[left] ;
                }
                left += 1 ;
            } else {
                if height[right] >= right_max {
                    right_max = height[right] ;
                } else {
                    water_trapped += right_max - height[right] ;
                }
                right -= 1 ;
            }
        }
        water_trapped
    }
}