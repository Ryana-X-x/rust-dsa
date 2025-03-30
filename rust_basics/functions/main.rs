pub mod helpers ;

fn main() {
    println!("Hello From Ryana!") ;

    // helpers::database ;
    let my_result = helpers::namehelpers::get_full_name("Shane", "Jones") ;
    println!("Hello from {0}", my_result) ;

    let new_age = helpers::privatefns::get_age_plus_5(21) ;
    println!("The new age is {0}", new_age) ;
}

#[allow(dead_code)]
fn test_func() {
    println!("This is a test function") ;
}
