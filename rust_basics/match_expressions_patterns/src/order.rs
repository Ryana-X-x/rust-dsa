enum Order {
    Physical { name: String, weight: f32 },
    Digital { name: String, download_link: String } ,
    Subscription { name: String, duration_months: u8} , 
}

fn process_order(order: Order) {
    match order {
        Order::Physical { name, weight } => {
            println!("Shipping '{}' weighinh {}kg", name, weight) ;
        }, 
        Order::Digital { name, download_link } => {
            println!("Sending digital download link for {}: {}", name, download_link) ;
        } ,
        Order::Subscription { name, duration_months } => {
            println!("Activating {} Subscription for {} months", name, duration_months) ;
        } ,
    }
}

fn main() {
    let order1 = Order::Physical { 
        name: "Gaming Mouse".to_string(),
        weight: 0.45 , 
    } ;

    let order2 = Order::Digital {
        name: "E-Book: Rust in Action".to_string() ,
        download_link: "https://youtu.be/xvFZjo5PgG0?si=KUHUmuiY9KWFrVJN".to_string(),
    } ;

    let order3 = Order::Subscription { name: "Spotify Subscription".to_string(), duration_months: 2 } ;

    process_order(order1);
    process_order(order2);
    process_order(order3);
}