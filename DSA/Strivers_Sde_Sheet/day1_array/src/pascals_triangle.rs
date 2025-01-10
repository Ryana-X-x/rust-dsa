// leetcode 118  
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let num_rows = num_rows as usize ;  // Convert i32 to usize for indexing
        let mut result: Vec<Vec<i32>> = Vec::new() ;

        for i in 0..num_rows {
            let mut row: Vec<i32> = vec![1; i + 1] ;

            for j in 1..i {
                row[j] = result[i-1][j-1] + result[i - 1][j] ;
            }

            result.push(row) ;  // Add the completed row to the result
        }

        result
    }
}
