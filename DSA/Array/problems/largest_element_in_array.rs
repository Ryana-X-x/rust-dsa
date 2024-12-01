// https://www.geeksforgeeks.org/problems/largest-element-in-array4009/1?itm_source=geeksforgeeks&itm_medium=article&itm_campaign=bottom_sticky_on_article

//* Brute Force
// ? Soring array in ascendind order, then taking the last element as out largest element. TC: O(n log n), SC: O(n)

fn largest_element_brute_force(arr: &[i32]) -> i32 {
    let mut sorted_arr = arr.to_vec() ; // Clone the array
    sorted_arr.sort() ; // Sort the array
    sorted_arr[sorted_arr.len() - 1]  // Return the last element
}

// ? Using a single for loop and a max variable. TC: O(n), SC: O(1)

fn largest_element_optimized(arr: &[i32]) -> i32 {
    let mut max = i32::MIN ;    // Initializes the smalled possible value
    for &num in arr {
        if num > max {
            max = num ;
        }
    }
    max
}

fn main() {
    let array = [1, 10, 2, 34, 56, 34, 23, 43, 11, 90, 78] ;
    
    let largest_brute = largest_element_brute_force(&array) ;
    println!("Largest element using brute force: {}", largest_brute) ;

    let largest_optimized = largest_element_optimized(&array) ;
    println!("Largest element using optimized approach: {}", largest_optimized) ;
}