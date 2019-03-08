use grrs::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguements: {}", err);
        process::exit(1);
    });

    if let Err(e) = grrs::run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(1)
    }
}
