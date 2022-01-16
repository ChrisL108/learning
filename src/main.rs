use console::style;
use std::io;

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    println!("\n{}\n", style("Hello!").bold().cyan());

    let home = IpAddr::V4(109, 168, 0, 1);
    let work = IpAddr::V6(String::from("::1"));
    
    
}
