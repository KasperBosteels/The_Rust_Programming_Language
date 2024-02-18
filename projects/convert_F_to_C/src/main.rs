use std::io;

fn main() {
    let mut input_as_text = String::new();
   
    
    loop {
        io::stdin()
        .read_line(&mut input_as_text)
        .expect("Failed to read line");


        let input_in_f: f64 = match input_as_text
        .trim()
        .parse()
    {
        Ok(num) => num,
        Err(_) => 12.11,
    };
        let output_in_c:f64 = ((input_in_f - 32.)*5.)/9.;
        println!("{} Farenheit is {} in Celcius.", input_in_f, output_in_c);
        println!("Do you want to start again? (y)");
        let mut answer= String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");
        if answer.trim() != "y"{
            println!("{answer}");
            break;
        }
    }
}
