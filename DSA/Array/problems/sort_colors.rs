// https://leetcode.com/problems/sort-colors/
// q75

// low = 0
// mid = 0
// high = len -1

// mid == 0 , swap mid and low
// mid == 2, swap mid and high

struct Solution ;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut low = 0 ;
        let mut mid = 0 ;
        let mut high = nums.len() - 1 ;

        while mid <= high {
            if nums[mid] == 0 {
                nums.swap(low, mid) ;
                low += 1 ;
                mid += 1 ;
            } else if nums[mid] == 1 {
                mid += 1 ;
            } else {    // mid == 2
                nums.swap(mid, high) ;
                if high == 0 { break; } // preventing undeflow
                high -= 1 ;
            }
        }
    }
}

fn main() {
    let mut nums = vec![2, 0, 2, 1, 1, 0] ;
    println!("Before sorting: {:?}", nums) ;

    Solution::sort_colors(&mut nums) ;
    println!("After sorting: {:?}", nums) ;
}