use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguements: {}", err);
        process::exit(1);
    });

    println!("pattern: {}", config.query);
    println!("path: {:?}", config.filename);

    let filepath = fs::canonicalize(&config.filename);
    println!("path (cannonicalized) {:?}", filepath);

    let file_contents =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file");
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
