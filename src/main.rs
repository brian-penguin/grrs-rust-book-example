use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);
    // This is the fully qualified file system path
    let filepath = fs::canonicalize(&config.filename);

    println!("pattern: {}", config.query);
    println!("path: {:?}", config.filename);
    println!("path (cannonicalized) {:?}", filepath);

    let file_contents =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file");
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}
