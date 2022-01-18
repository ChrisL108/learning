//* Get words from string
fn main() {
  let mut user_input = String::new();
  println!("Enter a sentence");

  io::stdin()
    .read_line(&mut user_input)
    .expect("Please enter valid string");

  // println!("user_input: {}", user_input);
  let first_word = get_first_word(&user_input);
  let second_word = get_second_word(&user_input);

  println!("first_word: {}, Second Word: {}", first_word, second_word);
}

fn get_first_word(s: &str) -> &str {
  let bytes = s.as_bytes();
  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[..i];
    }
  }
  &s[..]
}

fn get_second_word(s: &str) -> &str {
  let bytes = s.as_bytes();
  let mut index1: usize = 0;
  let mut index2: usize = s.len();
  let mut spaces: usize = 0;
  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      println!("found Space - item: {}, i: {}", item, i);

      if spaces == 0 {
        // index1 = if (i + 1) > (index2 - 1) { i + 1 } else { i };
        index1 = i + 1;
        spaces += 1;
      } else if spaces == 1 {
        index2 = i;
        break;
      }
    } else if item == b'\r' || item == b'\n' {
      println!("found EOL - item: {}, i: {}", item, i);
      index2 = i;
      break;
    }
  }
  &s[index1..index2]
}
