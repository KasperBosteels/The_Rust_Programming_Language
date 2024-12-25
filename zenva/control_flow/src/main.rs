use core::num;

fn main() {
    let number = 30;

    if number % 5 == 0 && number % 3 == 0 {
        println!("{} is a triqunit", number);
    } else if number % 6 == 0 && number % 4 == 0 {
        println!("{} is a hexaquad", number);
    } else {
        print!("{} is just another number", number);
    }

    let is_weekend = true;

    let activity = if is_weekend { "go hiking" } else { "go work" };

    println!("{}", activity);

    let arr = [10, 20, 60, 40, 50];
    for elem in arr {
        println!("{}", elem);
    }

    let mut countdown = 10;

    while countdown > 0 {
        println!("countdown {}", countdown);
        countdown -= 1;
    }
    println!("lift off");

    let mut index = 1;

    loop {
        index += 1;

        println!("index {}", index);

        if index == 100 {
            println!("max index reached");
            break;
        }
    }
    println!("fibonacci");
    let mut counter = 2;
    let mut last_values = (0, 1, 0);
    println!("{}", last_values.0);
    println!("{}", last_values.1);

    while counter < 10 {
        last_values.2 = last_values.0 + last_values.1;

        println!("{}", last_values.2);

        last_values.0 = last_values.1;
        last_values.1 = last_values.2;

        counter += 1;
    }

    println!("average temp");
    let mut temps = (0.0, [70.3, 75.6, 72.1, 68.3, 74.11, 78.0, 73.5]);

    for temp in temps.1 {
        temps.0 += temp;
    }
    temps.0 = temps.0 / temps.1.len() as f32;

    println!("{}", temps.0);
}
