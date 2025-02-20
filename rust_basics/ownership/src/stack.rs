fn main() {
    let x = 10; 
    let y = 234 ;
    let z = add_values(x,  y) ;

    println!("The sum of {} and {} is {}", x, y, z);
}

fn add_values(a: i32, b: i32 ) -> i32 {
    a + b
}