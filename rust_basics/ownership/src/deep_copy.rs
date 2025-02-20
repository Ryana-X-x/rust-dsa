fn main() {
    let s1 = String::from("Hekllo") ;
    // we need to explicity call the clone method to deep copy the string value of s1 to s2
    let s2 = s1.clone() ;

    println!("s1 = {}, s2 = {}", s1, s2) ;
}