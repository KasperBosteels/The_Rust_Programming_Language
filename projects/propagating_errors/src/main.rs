use std::fs::File;
use std::io::{self, Read};

fn main() -> Result<(), Box <dyn Error>> {
    let greeting_file =File::open("hello.txt")?;
    Ok(())
}

fn read_username_from_file()  ->  Result<String, io::Error> {
    let mut username = String::new();
    let mut username_file = File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn last_char_of_first_line(text:&str) ->  Option<char> {
    text.lines().next()?.chars().last()
}