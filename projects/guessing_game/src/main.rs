use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the correct number!");
    loop {
    println!("Please input your guess:");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: i32 = match guess
        .trim()
        .parse(){
        Ok(num) =>  num,
        Err(e) => continue,
    };
        if gues  < 1 || gues > 100 {
            println!("The secret nuymber will be between 1 and 100.");
            continue;
        }
    println!("You guessed: {guess}");

    match guess.cmp(&secret_number){
        Ordering::Less => println!("You guessed too small!"),
        Ordering::Greater => println!("You guessed too high!"),
        Ordering::Equal => {
            println!("You guessed right!");
            break;
            }
        }
    }
}