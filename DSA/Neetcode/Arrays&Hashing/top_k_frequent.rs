use std::collections::{HashMap, BinaryHeap} ;
use std::cmp::Reverse ;

fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut freq_map = HashMap::new() ;

    for num in nums {
        *freq_map.entry(num).or_insert(0) += 1 ;    // storing the frequencies of the values
    }

    let mut heap = BinaryHeap::new() ;
    for (num, freq) in freq_map {
        heap.push(Reverse((freq, num))) ;   // push reverse frequency and number.. By default heap stores in max.. Reverse becuase of rust tuple comparion (a,b) (c,d) a will be compared to c then if equal then b and d
        if heap.len() > k as usize {
            heap.pop() ;
        }
    }

    heap.into_iter().map(|Reverse((_freq, num))| num).collect()  // only need the num and not frequency
}

fn main() {
    let nums = vec![1,1,1,2,2,3] ;
    let k = 2 ;

    let result = top_k_frequent(nums,k ) ;

    println!("{:?}", result) ;
}