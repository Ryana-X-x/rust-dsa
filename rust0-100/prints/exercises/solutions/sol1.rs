use std::io;

/*
    EMI calculator
    Fix all TODOs
*/

fn main() {
    let mut input = String::new();
    let currency_symbol = '₹' ; // ¥ $, £, ¥ 

    println!("Enter the principal amount {} (e.g., 100000): ", currency_symbol);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let principal: f64 = input.trim().parse().expect("Input number only!");
    input.clear();

    println!("Enter the annual rate of interest (%) (e.g., 7.5): ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let annual_rate: f64 = input.trim().parse().expect("Input number only!");
    input.clear();

    println!("Enter loan terms in months (e.g., 120): ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let months: u32 = input.trim().parse().expect("Input number only!");
    input.clear();

    let emi = calculate_emi(principal, annual_rate, months);
    println!("Your monthly EMI is {}{:.2}", currency_symbol, emi);
}

/*
   Formula for calculating the Loan EMI is: EMI = [P x R x (1+R)^N]/[(1+R)^N-1]
   where P is the principal loan amount
   R is the monthly interest rate (annual rate divided by 12)
   N is the number of monthly instalments or the loan tenure in months.
*/
fn calculate_emi(principal: f64, annual_rate: f64, months: u32) -> f64 {
    let monthly_rate = annual_rate / 12.0 / 100.0; // Convert percentage to a decimal and annual to monthly
    let numerator = principal * monthly_rate * (1.0 + monthly_rate).powf(months as f64);
    let denominator = (1.0 + monthly_rate).powf(months as f64) - 1.0;
    numerator / denominator
}