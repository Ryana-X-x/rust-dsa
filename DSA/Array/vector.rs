//* from inbuilt macro
// fn main() {
//     let vec:Vec<u16> = vec![10, 30 , 21, 23, 43,55] ;
//     println!("Vector Array: {:?}", vec) ;
// }

//* Vector from array
// fn main() {
//     let vec = Vec::from([10, 30 , 21, 23, 43,55]) ;
//     println!("Vector Array: {:?}", vec) ;
// }

//* Empty vector 
// fn main() {
//     let mut _vec: Vec<u8> = Vec::new() ;
// }


// -----------------------

//* 2D Vector */

fn main() {
    // Empty 2D Vector
    //let mut _vec: Vec<Vec<Char>> = Vec:new() ;

    // method 1: 
    let vec: Vec<Vec<u8>> = Vec::from([Vec::from([1, 2, 3]), Vec::from([5, 6, 7])]) ;
    println!("2D vector: {:?}", vec)  ;

    // method 2: 
    let vect: Vec<Vec<u8>> = vec![vec![1, 2 ,3], vec![4, 5, 6]] ;
    println!("2D vector: {:?}", vect)  ;
}