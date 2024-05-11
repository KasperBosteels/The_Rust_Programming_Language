use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_args(&args);

    println!("Searching for: {}",config.query);
    println!("In file: {}",config.query_path);



    let file_contents = fs::read_to_string(config.query_path)
    .expect("Should have been able to read the file");

    println!("With text:\n\r{file_contents}");
}

struct Config {
    path:String,
    query:String,
    query_path:String,
}


fn parse_args(args:&[String]) -> Config {
    let path = args[0].clone();
    let query = args[1].clone();
    let query_path = args[2].clone();
    Config{
        path,query,query_path
    }
}
