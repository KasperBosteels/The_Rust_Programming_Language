use chrono::prelude::*;
use std::{
    fmt::Error, io::{self, Write}, process
};

fn main() {
    let local: DateTime<Local> = Local::now();
    let day_of_month = local.day();
    let month = local.month();
    let year = local.year();
    let current_hour = local.hour();
    let minutes = local.minute();
    println!("date: {day_of_month}/{month}/{year}\r\ntime: {current_hour}:{minutes}");
    println!("| time|\t-     | code |");
    if let Err(e) = calculate_code(&day_of_month, &current_hour) {
        eprintln!("Application error: {e}");
        process::exit(1)
    }

    let mut input = String::new();
    println!("Press enter to exit");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
}

fn calculate_code(day: &u32, hour: &u32) -> Result<(), Error> {
    let hour_plus_one = hour.to_owned() + 1;
    let mut start_hour = hour_plus_one;
    if hour_plus_one > 5 {
        start_hour -= 5;
    }
    for iteration in 0..10 {
        let mut hour_to_calculate = start_hour + iteration;
        if hour_to_calculate > 24 || hour_to_calculate == 0{
            hour_to_calculate = 1;
        }
        let hour_with_date = (day + hour_to_calculate) + 2;
        let reversed_numbers = switch_numbers(&hour_with_date);
        let mut first_number_diff = reversed_numbers
            .to_string()
            .chars()    
            .next()
            .and_then(|c| c.to_digit(10))
            .unwrap()
            - 1;
        if first_number_diff < 10 {
            first_number_diff = format!("0{}", first_number_diff).parse().unwrap();
        }
        let last_number_sum = reversed_numbers
            .to_string()
            .chars()
            .last()
            .and_then(|c| c.to_digit(10))
            .unwrap()
            + 1;
        let text_to_print = format!(
            "{}{}{}",
            first_number_diff, reversed_numbers, last_number_sum
        );
        let hour_to_print = hour_to_calculate - 1;
        if hour_to_calculate == hour_plus_one {
            println!(" {hour_to_print}:00\t-   ==>\t{text_to_print} <== current");
        } else {
            println!(" {hour_to_print}:00\t-\t{text_to_print}");
        }
    }
    Ok(())
}

fn switch_numbers(number: &u32) -> u32 {
    number
        .to_string()
        .chars()
        .rev()
        .collect::<String>()
        .parse()
        .unwrap()
}
