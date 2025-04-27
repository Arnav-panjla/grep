use std::{fs, result};
use std::error::Error;
use std::env;

pub fn run(config : Config)-> Result<(), Box<dyn Error>> {        
    let content = fs::read_to_string(config.path)?;

    let result = if config.is_case_sensitive {
        search(&config.query, &content)
    } else {
        search_case_insensitive(&config.query, &content)
    };

    for line in result {
        println!("{line}");
    }
    Ok(())
} 

pub struct Config { 
    query : String, 
    path : String, 
    is_case_sensitive : bool,
} 

impl Config { 
    pub fn build(args: & Vec<String>) -> Result<Config, &'static str> { 
            if args.len() < 3 {return Err("Not enough arguments");} 
            let query  = args[1].clone(); 
            let path = args[2].clone(); 
            let is_case_sensitive = env::var("CASE_INSENSITIVE").is_ok();
        Ok(Config {query, path, is_case_sensitive}) 
} 
} 

pub fn search<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}