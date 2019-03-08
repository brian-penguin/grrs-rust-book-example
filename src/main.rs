use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, filename) = parse_config(&args);
    // This is the fully qualified file system path
    let filepath = fs::canonicalize(&filename);

    println!("pattern: {}", query);
    println!("path: {:?}", filename);
    println!("path (cannonicalized) {:?}", filepath);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];
    (query, filename)
}
