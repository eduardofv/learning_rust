use std::env;
use std::process;

use minigrep::Config;

/// # Documentation for *main*
/// this
/// is 
/// code:
/// ```
/// a + b = 1
/// ```
fn main() {
    //Chapter 12
    /*
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {err}");
        process::exit(1);
    });
    */
    
    //Chapter 13
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {err}");
        process::exit(1);
    });

    //dbg!(args);
    eprintln!("q: {}", config.query);
    eprintln!("path: {}", config.path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application Error: {e}");
        process::exit(1);
    }
}
