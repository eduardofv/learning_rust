//! # Minigrep
//! 
//! This is from several examples of `the book`
//!

use std::env;
use std::error::Error;
use std::fs;


/// Runs the program logic
///   *mock documentation*
/// ```
/// // a = b +1
/// ```
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

    //check if we still have contents
    eprintln!("{:?}", contents);

    Ok(())
}

/// Add one to a number
///
/// # Examples
///
/// ```
/// let arg = -1;
/// let answer = minigrep::add_one(arg);
///
/// assert_eq!(0, answer);
/// ```
pub fn add_one(number: i32) -> i32 {
    number + 1
}

/// This is an example to reexport (pub use)

pub use self::inner_ops::minus_one;

pub mod inner_ops {
    /// Extracts one from a number
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(-1, minigrep::inner_ops::minus_one(0));
    /// ```
    pub fn minus_one(number: i32) -> i32 {
        number - 1
    }
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

    #[test]
    fn test_add_one() {
        assert_eq!(2, add_one(1));
    }

}
