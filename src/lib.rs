use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &file_contents)
    } else {
        search_case_insensitive(&config.query, &file_contents)
    };

    for line in results {
        println!("matching line: {}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut matches = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            matches.push(line);
        }
    }

    matches
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let insensitive_query = query.to_lowercase();
    let mut matches = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&insensitive_query) {
            matches.push(line);
        }
    }

    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "toot";
        let contents = "\
Rust:
Safe n Fast
Productive for bloots
bloot's are worth a toot";

        assert_eq!(vec!["bloot's are worth a toot"], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "blOOt";
        let contents = "\
Rust:
Safe n Fast
Productive for bloots
bloot's are worth a toot";

        assert_eq!(
            vec!["Productive for bloots", "bloot's are worth a toot"],
            search_case_insensitive(query, contents)
        );
    }
}
