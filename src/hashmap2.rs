use console::style;
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

fn main() {

    let start = Instant::now();
    let mut my_vec = vec![];
    // run expensive operation
    for i in 0..200000 {
        my_vec.push("ABCDE..");
    }
    let duration = start.elapsed().as_secs_f64();
    println!("Time elapsed is: {}, vec.len: {}", duration, my_vec.len());

    let ss1 = String::from("Hello, ");
    let ss2 = String::from("world!");
    let ss4 = format!("{} {}", ss1, ss2);
    println!("\ns4: {}\n", style(ss4).bold().cyan());

    let s1 = String::from("chris");
    let h = &s1[0..2];
    println!("\nslice: {}\n", style(h).bold().cyan());

    let hindu_string = String::from("नमस्ते");
    for c in hindu_string.chars() {
        println!("char: {}", c);
    }
    for b in hindu_string.bytes() {
        println!("byte: {}", b);
    }

    let teams = vec![String::from("Lions"), String::from("Rams")];

    // * Hash Maps

    let teams = vec![400, 599];
    let scores = vec![10, 20];

    let score_map: HashMap<_, _> = teams.into_iter().zip(scores.into_iter()).collect();

    println!("score_map: {:?}", score_map);


    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    scores.entry(String::from("Blue")).or_insert(80);
    scores.entry(String::from("Green")).or_insert(5);
    // println!("scores: {:?}", scores);
    // let team_name = String::from("Blue");
    // let score = scores.get(&team_name);
    // println!("score: {:?}", score);

    // match score {
    //     Some(&s) => println!("Score for {} team is {}", team_name, &s),
    //     None => println!("Not a valid team name!"),
    // }

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let text = "hello world wonderful world";

    // : HashMap<&str, &str>
    let mut hm = HashMap::new();
    let text_col = text.split_whitespace();
    // println!("{:?}", text_col);
    for word in text_col {
        println!("{}", word);
        let count = hm.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", hm);
}
