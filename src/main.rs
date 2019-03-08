use std::env;
use std::process;
use grrs::Config;

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

    if let Err(e) = grrs::run(config) {
        println!("Application Error: {}", e);
        process::exit(1)
    }
}
