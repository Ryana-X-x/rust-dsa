// https://www.geeksforgeeks.org/problems/union-of-two-sorted-arrays-1587115621/1?utm_source=youtube&utm_medium=collab_striver_ytdescription&utm_campaign=union-of-two-sorted-arrays

struct Solution ;

impl Solution {
    pub fn union_sorted_arrays(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new() ;
        let mut i = 0 ;
        let mut j = 0 ;

        while i < arr1.len() && j < arr2.len() {
            if arr1[i] < arr2[j] {
                if result.last() != Some(&arr1[i]) {
                    result.push(arr1[i]) ;
                }
                i += 1 ;
            }else if arr1[i] > arr2[j] {
                if result.last() != Some(&arr2[j]) {
                    result.push(arr2[j]) ;
                }
                j += 1 ;
            } else {
                if result.last() != Some(&arr1[i]) {
                    result.push(arr1[i]) ;
                }

                i += 1;
                j += 1 ;
            }
        }

        // for remaining elements of arr1
        while i < arr1.len() {
            if result.last() != Some(&arr1[i]) {
                result.push(arr1[i]) ;
            }
            i += 1 ;
        }

        // for remaining elements of arr2
        while j < arr2.len() {
            if result.last() != Some(&arr2[j]) {
                result.push(arr2[j]) ;
            }
            j += 1 ;
        }

        result
    }
}

fn main() {
    let arr1 = vec![1, 2, 2, 3, 4] ;
    let arr2 = vec![2, 2, 3, 5, 6] ;

    let result = Solution::union_sorted_arrays(arr1, arr2) ;
    println!("Union of two sorted Arrays with duplicates removed: {:?}", result) ;
}