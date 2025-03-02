fn modify_string(s: &mut String) {
    s.push_str(", Ryana") ;
}

fn main() {
    let mut greeting = String::from("Hello") ;
    modify_string(&mut greeting) ;
    println!("{}", greeting) ;
}