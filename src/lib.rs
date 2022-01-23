
// use front_of_house::hosting;
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn arrive_at_restaraunt() {
  hosting::add_to_waitlist(String::from("Chris"));
  hosting::add_to_waitlist(String::from("Kayden"));
  hosting::add_to_waitlist(String::from("Shane"));
}

// mod back_of_house {
//   pub struct Breakfast {
//     pub toast: String,
//     seasonal_fruit: String,
//   }

//   impl Breakfast {
//     pub fn summer(toast: &str) -> Breakfast {
//       Breakfast {
//         toast: String::from(toast),
//         seasonal_fruit: String::from("peaches"),
//       }
//     }
//   }
// }

// pub fn eat_at_restaraunt() {
//   let mut meal = back_of_house::Breakfast::summer("Wheat");

//   meal.toast = String::from("Rye");
//   println!("I would like {} toast, please.", meal.toast);

//   meal.toast = String::from("Butter");
//   println!("Sorry!, make that {} toast, please!", meal.toast);

//   // The next lines won't compile if we uncomment it; we're not allowed
//   // to see or modify the seasonal fruit that comes with the meal

//   // println!("Today\'s seasonal fruit is {}", meal.seasonal_fruit);
//   // meal.seasonal_fruit = String::from("blueberries");
// }
