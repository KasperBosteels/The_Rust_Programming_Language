use std::error::Error;
use std::fs;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.query_path)?;

    let results = if config.ignore_case {
        case_insencitive_search(&config.query,&content)
    } else {
        search(&config.query, &content)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub struct Config {
    pub path: String,
    pub query: String,
    pub query_path: String,
    pub ignore_case: bool,
}
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let path = args[0].clone();
        let query = args[1].clone();
        let query_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            path,
            query,
            query_path,
            ignore_case
        })
    }
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    
    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn case_insencitive_search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
        rust:
        safe, fast, productive.
        pick three. 
        Duct tape";
        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }

    #[test]
    fn one_result_case_incensitive() {
        let qeuery = "FROG";
        let content = "\
        rust:
        dog DOG frog frog frog frog
        halleluja";
        assert_eq!(
            vec!["dog DOG frog frog frog frog"],
            case_insencitive_search(qeuery, content)
        );
    }
}
