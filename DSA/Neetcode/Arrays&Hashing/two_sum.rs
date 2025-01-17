use std::collections::HashMap ;

fn two_sum(nums:Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new() ;

    for (i, &num) in nums.iter().enumerate() {  // enumerate gives both the index and the value for num
        let complement = target - num ;  
        if let Some(&index) = map.get(&complement) {// checking if a value at an index in hashmap = complement
            return vec![index as i32, i as i32] ;   // returning the index of the complement and the current value
        }
        map.insert(num, i) ;    // if not found then adding the current value and it's index for future check 
    }

    vec![]     // defualt case: Should not be reached if guranteed solution 
}


fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = two_sum(nums, target);
    println!("{:?}", result); // Output: [0, 1]
}