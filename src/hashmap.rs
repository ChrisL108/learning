use console::style;
use std::{self, cmp::Ordering, collections::HashMap as HM, io};
// use std::io;
fn main() {
    println!("\n{}\n", style("Hello!").bold().cyan());

    let mut map: HashMap<u8, u8> = HashMap::new();

    map.insert(0, 10);
    map.insert(1, 20);
    map.insert(2, 30);
    map.insert(3, 40);
    map.insert(4, 50);

    println!("Hash Map: {:?}", map);
    // match map {
    //     Ok() => {}
    //     _ => {
    //         println!("No Value");
    //     }
    // };
}
