use std::env;
use std::error::Error;
use std::fs;


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.path)?;

    eprintln!("Text:\n{contents}");

    //println!("Results:");

    let results = if config.ignore_case {
        eprintln!("Searching case insensitive");
        search_case_insensitive(&config.query, &contents)
    } else {
        eprintln!("Searching case sensitive");
        search(&config.query, &contents)
    };

    for result in results {
        println!("{}", result);
    }

    eprintln!("{:?}", contents);

    Ok(())
}


//Chap12
/*
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
*/

//Chap13
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query_low = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query_low) {
            results.push(line);
        }
    }

    results
}


pub struct Config {
    pub query: String,
    pub path: String,
    pub ignore_case: bool,
}

impl Config {

    //Chap 12
    /*
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, path, ignore_case })
    }
    */

    //Chap13
    pub fn build(
        mut args: impl Iterator<Item = String>,
        ) -> Result<Config, &'static str> {
        
        //ignore program name
        args.next();
        
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("No query string"),
        };

        let path = match args.next() {
            Some(arg) => arg,
            None => return Err("No file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, path, ignore_case })
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
            
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_sensitive_search() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";
            
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive_search() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";
            
        assert_eq!(
            vec!["safe, fast, productive.", "Duct tape"], 
            search_case_insensitive(query, contents));
    }

}
