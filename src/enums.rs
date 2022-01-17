use console::style;
// use std::io;

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    println!("\n{}\n", style("Hello!").bold().cyan());

    let five_pennies = value_in_cents(Coin::Penny) * 5;
    println!("\n{}\n", style(five_pennies).bold().yellow());
}
