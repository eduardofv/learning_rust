use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    //dbg!(args);
    println!("q: {}", config.query);
    println!("path: {}", config.path);

    let contents = fs::read_to_string(config.path)
        .expect("Should have been able to read the file");

    println!("Text:\n{contents}");
}


struct Config {
    query: String,
    path: String,
}


fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let path = args[2].clone();

    let config = Config { query, path };
    config
}
