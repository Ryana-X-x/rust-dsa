// In rust you Cannot have a mutable reference when there is already immutable reference

fn main() {
    let mut s = String::from("Ryana") ;

    let s1 = &s ;
    let s2 = &s  ;

    println!("{}, {}",s1, s2 ) ;    // Used s1 and s2

    let s3 = &mut s ;  
    println!("{}", s3) ;    // Is Valid Because s1 and s2 have been used 
}