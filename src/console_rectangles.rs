use console::style;
use std::io;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[cfg(windows)]
const TOP_BOTTOM_LINE: &str = "---";

#[cfg(not(windows))]
const TOP_BOTTOM_LINE: &str = "~~~";

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn update_width(&mut self, width: u32) {
        self.width = width;
    }
    fn update_height(&mut self, height: u32) {
        self.height = height;
    }
    fn print(&self) {
        let mut output = String::from(".");

        for _ in 0..(self.width) {
            output += TOP_BOTTOM_LINE;
        }
        output += ".\n";
        for _ in 0..self.height {
            output += "|";
            for _ in 0..self.width * 3 {
                output += " "
            }
            output += "|\n";
        }
        output += "'";
        for _ in 0..self.width {
            output += TOP_BOTTOM_LINE;
        }
        output += "'";
        println!("{}", output);
    }
}

fn main() {
    show_intro();

    // vector to hold all submissions
    let mut results: Vec<Rectangle> = Vec::new();

    loop {
        let (input_width, input_height) = ask_dimensions();

        let rect1 = Rectangle {
            width: input_width,
            height: input_height,
        };

        println!(
            "{}: {}",
            style("Rec. Area").magenta(),
            style(rect1.area()).bold().blue()
        );

        rect1.print();
        results.push(rect1);

        let mut user_input = String::new();
        println!("Go Again...? (Y/n)");
        io::stdin()
            .read_line(&mut user_input)
            .expect("Need 'y' or 'n'");
        let first_char = user_input.as_bytes();

        if first_char[0] == b'n' || first_char[0] == b'N' {
            break;
        }
    }

    for rec in results {
        println!("Area: {}", rec.area());
        rec.print();
    }
}

fn ask_dimensions() -> (u32, u32) {
    let mut user_input = String::new();

    println!("{}", style("Please enter a width:").bold().yellow());
    io::stdin()
        .read_line(&mut user_input)
        .expect("Please enter valid input.");

    let input_width: u32 = str_to_num(&user_input);

    user_input.clear();

    println!("{}", style("Please enter a height:").bold().yellow());
    io::stdin()
        .read_line(&mut user_input)
        .expect("Please enter valid input.");

    let input_height: u32 = str_to_num(&user_input);

    (input_width, input_height)
}
fn str_to_num(str: &str) -> u32 {
    str.trim().parse().expect("Please type a valid number!")
}

fn show_intro() -> () {
    println!("\n");
    println!("{}", style("  - RECTANGLER -  ").cyan());
    println!(" ^^------------^^ ");
    println!("     /.\\  /.\\   ");
    println!("  (     >      )  ");
    println!("                  ");
    println!("    \\'''''''/    ");
    println!("     \\ ___ /     ");
    println!("      \\'''/      ");
    println!("\n");
}

//* ASSIGNING LOOP's RETURN VALUE TO VARIABLE
// let mut counter = 0;
// let y = loop {
//     counter += 1;
//     if counter == 10 {
//         break counter * 2;
//     }
// };
//* WHILE LOOP
// let mut counter = 0;
// while counter <= 10 {
//     counter += 1;
// }
//* FOR LOOP
// let a = [10, 20, 30, 40, 50];
// let mut index = 0;

// for elem in a {
//     println!("elem is {}", elem);
//     println!("a[index] is {}", a[index]);
//     index += 1;
// }
