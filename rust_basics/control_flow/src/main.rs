fn main() {
    println!("Hello from Ryana!");

    test_if() ;
    test_while();

    test_loop();
    test_for();
}

fn test_if() {
    let age_to_drive = 18u8 ;

    println!("Enter the Person's age: ") ;
    let my_input: &mut String = &mut String::from("") ;
    std::io::stdin().read_line(my_input).unwrap() ;

    // let age: u8 = my_input.replace("\n", "").parse::<u8>().unwrap() ;
    let age: u8 = my_input.trim().parse::<u8>().unwrap() ;  // better than replace here
    if age > age_to_drive {
        println!("Issuing driver's license") ;
        
    } else if age == age_to_drive {
        println!("Exam de jaake") ;
    } else {
        println!("Ghanta kuch nahi milega") ;
    }

    let driver_license: bool = if age >= age_to_drive { true } else { false } ;
    println!("Driver's license: {0}", driver_license) ;
}

fn test_while() {
    let age_to_drive = 18u8  ;
    
    let mut current_age = 0u8 ;
    while current_age < age_to_drive {
        println!("Waiting..") ;
        current_age += 1 ;

        if current_age == 18 {
            println!("Finally of age!") ;
        }
    }
}

fn test_loop() {
    let mut x: i32 = 1 ;

    loop {
        println!("Hello From Ryana!") ;

        if x > 5 { break } ;

        x += 1 ; 
    }
}

fn test_for() {
    let ages = [14, 18, 23, 45, 67] ;
    let age_to_drive = 18i32  ;

    for value in ages {
        println!("The current age is: {0}", value) ;
        if value >= age_to_drive { println!("You can get a driver's license")} 
        else { println!("You cannot get driver's license")} ;
    }

}