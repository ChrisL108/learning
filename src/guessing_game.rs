use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io;

fn main() {
  let secret_number = thread_rng().gen_range(1..101);
  const MAX_GUESSES: u32 = 3;
  let mut guess_count = 0;
  let mut guess;
  loop {
    guess = String::new();
    println!("Guess the number!");
    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");

    println!("You have guessed {} times", guess_count);

    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };
    // .expect("Please type a number between 1 and 100.");
    // println!("You guessed {} ( {} guesses left )", guess, (3 - guesses));

    //* A match expression is made up of arms.
    // An arm consists of a pattern and the code that
    // should be run if the value given to the beginning
    //  of the match expression fits that arm’s pattern.
    //
    //? match expression
    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),  //? arm
      Ordering::Greater => println!("Too big!"), //? arm
      Ordering::Equal => {
        println!("YOU WIN!");
        break;
      } //? arm
    }
    guess_count = guess_count + 1;
    // Check if we still have attempts
    match guess_count.cmp(&MAX_GUESSES) {
      Ordering::Less => println!("{} more guesses", (3 - guess_count)),
      Ordering::Greater => {}
      Ordering::Equal => {
        println!("You've run outta guesses :(");
        break;
      }
    }
  }
}
