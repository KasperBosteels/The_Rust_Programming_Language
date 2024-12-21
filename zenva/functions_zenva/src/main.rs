fn main() {
    let name = String::from("Kasper {}");
    greet_user(name);

    let sum = calculate_sum(11, 12);
    println!("{}", sum);

    let studen_scores = [85, 90, 78, 92, 88];

    for s in studen_scores {
        println!("{}", get_letter_grade(s));
    }
}

fn calculate_sum(a: i32, b: i32) -> i32 {
    a + b
}

fn greet_user(name: String) {
    println!("hi {}", name);
}

fn get_letter_grade(sscore: i32) -> String {
    let mut score = String::from("F");

    if sscore > 90 {
        score = String::from("A");
    } else if sscore > 80 {
        score = String::from("B");
    } else if sscore > 70 {
        score = String::from("C");
    } else if sscore > 60 {
        score = String::from("D");
    }
    score
}
