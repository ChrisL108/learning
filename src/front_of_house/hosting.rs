// pub mod hosting {
// }

pub struct Reservation {
  name: String,
  position: u8,
}
// const mut current_position: u8 = 0;
pub fn add_to_waitlist(name: String) -> Reservation {
  // current_position += 1
  // println!("Adding {} to position {}", name, current_position);
  Reservation { name, position: 0 }
}
