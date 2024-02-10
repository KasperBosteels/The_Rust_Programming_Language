use std::collections::HashMap;

fn main () {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"),5);

    let team_name = String::from("Blue");
    // get returns option<&V>
    // copied turns it into a &i32 into i32
    // unwrap_or sets default value
    let score = scores.get(&team_name).copied().unwrap_or(0);


    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}