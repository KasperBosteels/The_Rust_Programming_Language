use std::collections::HashMap;

fn main() {
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

    let field_name = String::from("Favorite COlor");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name,field_value);
    //owner for name and value is now map origin is now invalid.

    overwriting_a_value();

    adding_value_if_not_present();

    updating_value_based_on_old_value();
}

fn overwriting_a_value() {

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 11);
}

fn adding_value_if_not_present () {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(11);
    scores.entry(String::from("Blue")).or_insert(20);

    println!("{:?}", scores)
}

fn updating_value_based_on_old_value () {
    let text = "hellow world wonderfull world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}