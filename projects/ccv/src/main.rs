use std::{ fmt::Error, process};
use chrono::prelude::*;

fn main() {
    let local: DateTime<Local> = Local::now();
    let day_of_month = local.day();
    let current_hour = local.hour() + 1;
    
    if let Err(e) =  calculate_code(&day_of_month, &current_hour) {
        eprintln!("Application error: {e}");
        process::exit(1)
    }
}

fn calculate_code(day:&u32, hour:&u32) -> Result<(), Error> {
    let hour = hour.to_owned();
    print!("{hour}");
    let start_hour = hour - 5;
    
    for iteration in 0..10 {
        let hour_to_calculate = start_hour + iteration;
        let hour_with_date = (day + hour_to_calculate) + 2;
        let reversed_numbers = switch_numbers(&hour_with_date);
        let first_number_diff = reversed_numbers.to_string().chars().next().and_then(|c| c.to_digit(10)).unwrap() - 1;
        let last_number_sum = reversed_numbers.to_string().chars().last().and_then(|c| c.to_digit(10)).unwrap() + 2;

        if hour_to_calculate == hour {
            println!( "{first_number_diff} {reversed_numbers} {last_number_sum} <== current time");
        }else{
            println!( "{first_number_diff} {reversed_numbers} {last_number_sum}");
        }
    }   
    Ok(())
}

fn switch_numbers(number:&u32) -> u32 {
    number.to_string().chars().rev().collect::<String>().parse().unwrap()
}
