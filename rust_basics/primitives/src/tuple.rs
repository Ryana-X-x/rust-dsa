//* Adding a tuple in a tuple 
fn main() {
    let t = (1, 'a', false) ;
    let f = (2 , t) ; 
    println!("{}, {}, {}, {}, {}", t.0, t.1, t.2,f.0, f.1.1) ;
    println!("{:#?}", f.1) ;
}