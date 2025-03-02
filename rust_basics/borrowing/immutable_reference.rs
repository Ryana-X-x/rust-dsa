fn print_length(s:&String) {
    println!("Length is: {}", s.len()) ;
}

fn main() {
    let name = String::from("Ryana") ;
    print_length(&name) ;  // borrowing
    println!("Original string is: {}", name) ;  // name is still accessible
}