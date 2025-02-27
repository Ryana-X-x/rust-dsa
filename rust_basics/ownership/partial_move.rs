// Partial move is when we move a part of the value from a variable to another variable.

struct Employee {
    name: String, 
    id: i32 ,
}

fn main() {
    let employee = Employee {
        name: String::from("roro") ,
        id: 101, 
    } ;

    // Partial move
    let name = employee.name ;

    println!("Employee name: {}", name) ;
    println!("Employee id: {}", employee.id) ;
}