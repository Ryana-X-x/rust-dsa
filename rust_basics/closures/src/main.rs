fn main() {
    println!("Hello, from Ryana!");

    basic_closure_example();
    captured_environment_closure_example();
    function_using_closure();
    function_returning_closure();
    closure_ownership();
    increment_counter();
    anonymous_function();
}

fn basic_closure_example() {
    let add = |a: i32, b: i32| -> i32 { a +  b } ;
    println!("basic closure examples: 2 + 3 = {}", add (2, 3)) ;
}

fn captured_environment_closure_example() {
    let factor = 2  ;
    let multiply = |x: i32| -> i32 { x * factor } ;
    println!("Captured environment closure example: 5 * {}  = {} ", factor, multiply(5)) ;
}

//** Passing Closure as an argument 
/*
Rust's type system lets us pass closures as parameters to functions. 
Here, apply_operation accepts any closure(or function) that takes two i32 
values and returns i32
*/ 
fn apply_operation<F>(a: i32, b: i32, op: F) -> i32 
where 
    F: Fn(i32, i32) -> i32,
{
    op(a, b)
}

fn function_using_closure() {
    let sum = apply_operation(10, 20, |a,b | a + b) ;
    println!("Function using closure: 10 + 20 = {}", sum) ;
}

//** Returning a Closure from a Function 
/* For returning a closure from a function, Rust requires you to use the
impl Fn syntax so that the compiler knows hte function will return some type
implementing the Fn trait. We use the move keyword to ensure that the closure 
takes ownership of the captured varaible
*/

fn make_mulitplier(factor: i32) -> impl Fn(i32) -> i32 {
    move | x: i32| x * factor 
}

fn function_returning_closure() {
    let mutliplier = make_mulitplier(4) ;
    println!("function returning closure: 5 * 4 = {}", mutliplier(5)) ;
}

// Closure Ownership 
fn closure_ownership() {
    let text = String::from("Helloo from Ryana again!") ;

    let use_text = || {
        println!("{}", text) ;
    } ;

    use_text() ;
    println!("text is: {}", text) ; // this will not give an error 
    // rust inferred an immutable borrow &text because it is only reading the value and not changing it

    let consume_text = move || {
        println!("{}", text) ;
        // Forcing ownership move
    } ;
    consume_text() ;
    // println!("This would give an error: {}", text) ;
}

fn increment_counter() {
    let mut counter = 0 ;

    let mut increment = || { counter += 1; } ;

    increment() ;
    increment() ;

    println!("Counter is {}", counter) ;
}

// Anonymous functions 
fn anonymous_function() {
    fn apply<F>(func: F, value: i32) -> i32 
    where F: Fn(i32) -> i32 {
        func(value) // calls the closure
    }

    let sqaure = |x| x * x  ;

    println!("The sauqre of {} is: {}", 5, apply(sqaure, 5)) ;
}