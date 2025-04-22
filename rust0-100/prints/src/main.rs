use std::io::Write ;

#[allow(dead_code)]
fn print1() {
    print!("Hello, ") ;
    std::io::stdout().flush().unwrap() ;
    eprintln!("An error occured: invalid input") ;
    let name = "Ryana" ;
    let age = 21 ;
    let message = format!("My name is {} and i am {} years old", name, age) ;
    println!("{}", message) ;
    println!("Hello, from Ryana!") ;
    /*
    * print! is buffered and eprintln! is not buffered so eprintln! is printed before print, or we can manually flush the buffer and print it.
    */
}
#[allow(dead_code)]
fn format1() {
    let name = "Ryana" ;
    let age = 21 ;

    let msg = format!(
                "My name is {user_name} and i am {user_age} years old",
                user_name = name, 
                user_age = age,
    ) ;

    println!("{}", msg) ;


}

fn main() {
    // print1() ;
    format1() ;
}