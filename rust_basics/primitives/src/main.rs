fn main() {
    let x = 5 ;
    //? x = 10 ; // Cannot do this if x is not mutable
    println!("{}", x) ;

    let mut y = 10 ;    // * mut Allows to update value later
    println!("{}", y) ;
    y = 7 ;
    println!("{}", y) ;

    /*
    ? Assigning Types 
    * i8, i16, i32, i64, isize
    * u8, u16, u32, u64, usize
    * f32, f64
    */

    //? bool: true/false

    //? char
    //let c = 'z' ;
    let _c: char = 'z' ;

    //? tuples
    let t: (i32, f64, char) = (10, 4.20, 'b') ;
    // t.0, t.1, t.2
    
    //? Pattern Match

    let (p, q, r) = t ;
    println!("{}, {}, {}", p, q, r) ;

    let (_, _, r) = t ; // r:char = 'b'
    println!("{}", r) ;

    //? Arrays
    let arr = [10, 50, 60, 80, 100] ;
    let arr1 = arr[0] ;
    println!("{}", arr1) ;
     

}
