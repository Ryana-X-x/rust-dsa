fn main() {
    println!("Hello from Ryana!");

    test_if() ;
}

fn test_if() {
    let age_to_drive = 18u8 ;

    println!("Enter the Person's age: ") ;
    let my_input: &mut String = &mut String::from("") ;
    std::io::stdin().read_line(my_input).unwrap() ;

    let age: u8 = my_input.replace("\n", "").parse::<u8>().unwrap() ;
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