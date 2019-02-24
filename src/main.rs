use exitfailure::ExitFailure;
use failure::ResultExt;
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

// ExitFailure is from a crate to help handle errors in main, under the hood it uses the failure
// crate
fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    // Show the args given
    println!("pattern: {}", args.pattern);
    // Show the path input
    println!("path: {:?}", args.path);
    // Get the full path name (returns a result)
    println!("path (cannonicalized) {:?}", fs::canonicalize(&args.path));

    // Attempt to find the file, updat the ExitFailure with context for the error
    let file_result = File::open(args.path)
        .with_context(|args_path| format!("Couldn't read file `{:?}`", args_path))?;
    // Unwrap (here's where we throw the error) and return the file result into the Buffered reader

    let f = BufReader::new(file_result);

    for line in f.lines() {
        let line_contents = line.unwrap();
        if line_contents.contains(&args.pattern) {
            println!("{:?}", line_contents);
        }
    }

    Ok(())
}
