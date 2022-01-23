use console::{style, Color};

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
fn main() {
    let mut my_vec = vec![1, 2, 3, 4, 5];

    for item in &mut my_vec {
        // To change the value that the mutable
        // reference refers to, we have to use the
        //* dereference operator (*)
        // to get the value in [item] before we can
        // use the +=/*=/etc operators.
        *item *= 3;
    }

    println!("my_vec: {:?}", style(&my_vec).black().bg(Color::White));

    let mut multi_type_vec = vec![
        SpreadsheetCell::Float(20.5),
        SpreadsheetCell::Int(5),
        SpreadsheetCell::Text(String::from("Chris")),
    ];

    println!(
        "multi_type_vec: {:?}",
        style(&multi_type_vec).black().bg(Color::Green)
    );

    multi_type_vec.push(SpreadsheetCell::Int(33));
    println!(
        "multi_type_vec (after): {:?}",
        style(multi_type_vec).white().bg(Color::Red)
    );
}
