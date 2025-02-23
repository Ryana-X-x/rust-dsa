fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
    [nums.clone(), nums].concat()
}

fn main() {
    let nums = vec![1, 2, 1] ;
    let result = get_concatenation(nums) ;

    println!("Result is {:?}", result) ;
    
}