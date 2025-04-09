// mod order ;

fn main() {
    println!("Hello from Ryana!");
    match_number(10);
    match_direction(Direction::East);
    match_option(Some(14));
    match_tuples((0,5));
    match_struct(Point {x: 19, y: 20});
}

fn match_number(n: u8) {
    match n {
        1 => println!("One") ,
        2 | 3 => println!("Two or Three") ,
        4..=10 => println!("Between 4 and 10") ,    // includes 4 and 10
        _=> println!("Something else") ,
    }
}

// Matching with Enums
enum Direction {
    North, 
    South, 
    East, 
    West,
}
fn match_direction(dir: Direction) {
    match dir {
        Direction::North => println!("Going North") ,
        Direction::South => println!("Going South") ,
        Direction::East => println!("Going East") ,
        Direction::West => println!("Going West") ,
    }
}

// Matching with Option
fn match_option(val: Option<i32>) {
    match val {
        Some(x) => println!("Value is: {}", x) ,
        None => println!("No Value") ,
    }
}

// Matching Tuples 
fn match_tuples(pair: (i32, i32)) {
    match pair {
        // will choose according to the signature
        (0, y) => println!("First is 0, second is: {}", y) ,
        (x, 0) => println!("First is {}, second is: 0", x) ,
        (x, y) => println!("First is {}, second is: {}", x, y) ,
    }
}

// Matching Structs 
struct Point {
    x: i32, 
    y: i32, 
}

fn match_struct(p: Point) {
    match p {
        Point { x: 0, y} => println!("On Y axis at {}", y), 
        Point { x, y: 0} => println!("On x axis at {}", x), 
        Point { x, y} => println!("At ({}, {}) axis",x,  y), 
    }
}