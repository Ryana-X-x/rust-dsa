// Rust by practice

// question 3
fn main() { 
    let s: String  = gives_ownership() ;
    println!("{}",s ) ;
}

fn gives_ownership() -> String{
    let s: String = String::from("Hello")  ;
    // let _s = s.into_bytes() ;   // this will make the string into a byte vector, and s will no longer be valid
    let _s = s.as_bytes() ;   // this will make the string into a byte slice, and s will still be valid
    s
}


// question 4
// fix the code without removing any code line
fn main() {
    let s = String::from("Hello") ;

    // print_str(s) ;   // this will give error as s has been moved to the function
    print_str(s.clone()) ;  // this will work as the string value of s has been cloned and passed to the function
    println!("{}", s) ;
}

fn print_str(s: String) {
    println!("{}", s) ;
}


//question 5 
// dont clone, use copy instead
fn main() {
    let x = (1, 2, (), "hello".to_string()) ;
    let y = x.clone() ;
    println!("{:?} {:?}", x, y) ;
}

// Solution
fn main() {
    // using string literals instead of string values to avoid cloning
    let x: (i32, i32, (), &str) = (1, 2, (), "hello") ;
    let y = x ;
    println!("{:?} {:?}", x, y) ;
}