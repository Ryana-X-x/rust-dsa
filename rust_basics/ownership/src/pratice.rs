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

// Question 6
// Make the necessary variable mutable 
fn main() {
    let s = String::from("Hello ") ;

    // let s1 = s  ;   //! this will not work as s1 is not mutable
    let mut s1 = s ;

    s1.push_str("World") ;

    println!("Success") ;
}

// Question 7
fn main() {
    let x = Box::new(5) ;

    let ...     // update this line dont change other lines ;

    *y = 4 ;

    assert_eq!(*x, 5) ;

    println!("Success") ;
}

// Solution 7
fn main() {
    // The box type is used for heap allocation, when a Box<T> goes out of scope, the memory is deallocated
    // In this example, Box::new(5), allocates an integer 5 on the heap and returns a Box<i32> pointer to it. When x will go out of scope, Rust will autnomatically deallocate the memory
    let x = Box::new(5) ;

    let mut y = Box::new(1) ;

    *y = 4 ;

    assert_eq!(*x, 5) ;

    println!("Success") ;
}





// **************************************************
// EXAMPLE for box type
// recursive data structure, using box to define a linked list: 
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil} ;

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))) ) ;

    print_list(&list) ;
}

// fucntion to print the elements of the list
fn print_list(list: &List) {
    match list {
        Cons(i, next) => {
            println!("{}", i) ;
            print_list(next) ;
        },
        Nil => println!("End of list") ,
    }
}
// **************************************************





// **************************************************
// explanation of the example above
enum List {
    // Cons is a tuple struct that wraps an element and a pointer to the next node
    Cons(i32, Box<List>),
    // Nil is a unit struct that represents the end of the list
    Nil,
}

fn main() {
    // creating a linked list using the enum List 
    // Cons is a tuple struct that wraps an element and a pointer to the next node
    // making a linked list with 3 elements: 1, 2, 3
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))) ) ;

    print_list(&list) ;
}

//
fn print_list(list: &List) {
    // match the list with the enum List
    match list {
        // if the list is Cons, print the element and call the print_list function recursively with the next element
        Cons(i, next) => {
            // print the element
            println!("{}", i) ;
            // call the print_list function recursively with the next element
            print_list(next) ;
        },
        Nil => println!("End of list") ,
    }
}

// *************************************************