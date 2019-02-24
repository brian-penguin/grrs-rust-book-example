use std::fs::{self, File};
use std::io::prelude::*;
use std::io::BufReader;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    // Show the args given
    println!("pattern: {}", args.pattern);
    // Show the path input
    println!("path: {:?}", args.path);
    // Get the full path name (returns a result)
    println!("path (cannonicalized) {:?}", fs::canonicalize(&args.path));

    let f = File::open(args.path).expect("Couldn't open file");
    let f = BufReader::new(f);

    for line in f.lines() {
        let line_contents = line.unwrap();
        if line_contents.contains(&args.pattern) {
            println!("{:?}", line_contents);
        }
    }
}
