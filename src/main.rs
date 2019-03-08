use std::env;
use std::error::Error;
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

    //let filepath = fs::canonicalize(&config.filename);
    //println!("path (cannonicalized) {:?}", filepath);

    if let Err(e) = run(config) {
        println!("Application Error: {}", e);
        process::exit(1)
    }
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

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", file_contents);
    Ok(())
}
