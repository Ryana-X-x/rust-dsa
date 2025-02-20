fn main() {
    // copy 
    let x = 5 ; 
    let y = x  ;
    // here the integer value of x is copied to y, both variables are usable, because i32 value has been copied. 
    // x and y both store the value 5 but they are stored in different memory locations.

    // move
    let s1 = String::from("Hello") ;
    let s2 = s1 ;
    // here the string value of s1 is moved to s2, s1 is no longer usable, because the ownership of the string value has been moved to s2.
    // As s1 is just a pointer to the datta on the heap Just the pointer will get copied into s2, NOT the whole data on the heap

}