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
    println!("{hour}");
    let mut start_hour = hour;
    if hour > 5 {
     start_hour -= 5;
    }
    for iteration in 0..10 {
        let hour_to_calculate = start_hour + iteration;
        let hour_with_date = (day + hour_to_calculate) + 2;
        let reversed_numbers = switch_numbers(&hour_with_date);
        let mut first_number_diff = reversed_numbers.to_string().chars().next().and_then(|c| c.to_digit(10)).unwrap() - 1;
        if first_number_diff < 10 {
            first_number_diff =  format!("0{}",first_number_diff).parse().unwrap();
        }
        let last_number_sum = reversed_numbers.to_string().chars().last().and_then(|c| c.to_digit(10)).unwrap() + 2;
        let text_to_print =format!("{}{}{}",first_number_diff,reversed_numbers,last_number_sum);
        if hour_to_calculate == hour {
            println!("{text_to_print} <== current time" );
        }else{
            println!("{text_to_print}");
        }
    }   
    Ok(())
}

fn switch_numbers(number:&u32) -> u32 {
    number.to_string().chars().rev().collect::<String>().parse().unwrap()
}
