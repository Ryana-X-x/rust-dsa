impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut candidate = nums[0] ;
        let mut count = 0 ;

        for num in nums {
            if count == 0 {
                candidate = num
            }

            if num == candidate {
                count += 1 ;
            }

            else {
                count -= 1 ;
            }
        }
        candidate
    }
}


fn main() {
    let nums = vec![3, 2, 3];
    println!("Majority Element: {}", majority_element(nums));

    let nums2 = vec![2, 2, 1, 1, 1, 2, 2];
    println!("Majority Element: {}", majority_element(nums2));
}