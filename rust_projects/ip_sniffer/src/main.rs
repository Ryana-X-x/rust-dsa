use std::env ;
use std::net::IpAddr ;

struct Arguments {
    flag: String, 
    ipaddr: IpAddr,
    threads: u16, 
}

impl Arguments {
    fn new(args)
}

fn main() {
    let args:Vec<String> = env::args().collect() ;
    let program = args[0].clone() ;
}