use std::io::Write;

#[allow(dead_code)]
fn print1() {
    print!("Hello, ");
    std::io::stdout().flush().unwrap();
    eprintln!("An error occured: invalid input");
    let name = "Ryana";
    let age = 21;
    let message = format!("My name is {} and i am {} years old", name, age);
    println!("{}", message);
    println!("Hello, from Ryana!");
    /*
     * print! is buffered and eprintln! is not buffered so eprintln! is printed before print, or we can manually flush the buffer and print it.
     */
}
#[allow(dead_code)]
fn format1() {
    let name = "Ryana";
    let age = 21;

    let msg = format!(
        "My name is {user_name} and i am {user_age} years old",
        user_name = name,
        user_age = age,
    );

    println!("{}", msg);
}
#[allow(dead_code)]
fn clippy_test() {
    // using clippy is not mandatory but recommended, it's a tool
    let array = [1, 2, 3] ;

    for i in 0..=2 {
        println!("{}", array[i]) ;
        // The rust compiler will not give any warning but try cargo clippy
    }   

    for item in &array {
        println!("{}", item) ;
        // This will not give any warning in clippy because of iterating over an immutable array
    }

    // let pi = 3.14   // Clippy is not happy
    let pi = std::f32::consts::PI ; 
    let area = pi * 4.0 * 4.0 ;
    println!("{}", area) ;
}

fn main() {
    // print1() ;
    // format1() ;
    // clippy_test() ;
}
