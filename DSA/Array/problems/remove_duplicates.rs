// https://leetcode.com/problems/remove-duplicates-from-sorted-array/description/

fn remove_duplicates_brute_force(arr: &mut Vec<i32>) -> usize {
    let mut unique = vec![] ;
    for &num in arr.iter() {
        if unique.is_empty() || unique.last() != Some(&num) {
            unique.push(num) ;
        }
    }
    arr.clear() ;
    arr.extend(unique) ;
    arr.len()
}

fn remove_duplicates_optimized(arr: &mut Vec<i32>) -> usize {
    if arr.is_empty() {
        return 0 ;
    }

    let mut slow = 0  ;
    for fast in 1..arr.len() {
        if arr[fast] != arr[slow] {
            slow += 1; 
            arr[slow] = arr[fast] ;
        }
    }
    slow + 1
}

fn main() {
    let mut array1 = vec![1,1,2,2,3,3,4] ;
    let mut array2 = array1.clone() ;
    
    
    // * Brute force 
    let len_brute = remove_duplicates_brute_force(&mut array1) ;
    println!("Array after removing duplicates with brute force: {:?} and length: {:?}", array1, len_brute ) ;

    // * Optimized approach
    let len_optimized = remove_duplicates_optimized(&mut array2) ;
    println!("Array after removing duplicates with optimized approach: {:?} and length: {:?}", &array2[..len_optimized], len_optimized ) ;
}



// ***************** Leetcode solution 26
// impl Solution {
//     pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
//         if nums.is_empty() {
//             return 0;
//         }

//         let mut slow = 0;
//         for fast in 1..nums.len() {
//             if nums[fast] != nums[slow] {
//                 slow += 1;
//                 nums[slow] = nums[fast];
//             } 
//         } 
//         (slow + 1) as i32 
//     }
// }
