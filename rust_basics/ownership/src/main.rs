fn main() {         
   let s = String::from("Hello")  ;     // s comes into scope
   takes_ownership(s) ; // s moves into the function and is no longer valid here

   let x = 5 ;  // x comes into scope
   makes_copy(x) ;  // but i32 is Copy, so it's okay to still use x afterwards

   let s2 = gives_ownership() ;

   let s3  = takes_and_gives_back(s2) ;
}

fn takes_owership(some_string: String) {
    println!("{}", some_string) ;
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer) ;
}   

fn gives_ownership() -> String {
    let some_string = String::from("Gives ownership") ;
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

/*
- Scaler values with fixed sizes will automatically get copied in the stack, copying here is cheap
- Dynamically sized data won't get copied, but moved, copying would be too expensive
- We can use the clone method to create a deep copy of the heap data
- Rust will never automatically create deep copies of your data, it will always move it 
*/
