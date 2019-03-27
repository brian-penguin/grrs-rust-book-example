use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // The first arg is the name of the program (we should call next and ignore that )
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("No query string provided")
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("No query string provided")
        };

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

    if config.query.len() == 0 {
        println!("Please try again with valid query string");
        return Ok(());
    }

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
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
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

    fn contents() -> &'static str {
        "\
Rust:
Safe n Fast
Productive for bloots
bloot's are worth a toot"
    }

    #[test]
    fn one_result() {
        let query = "toot";
        let contents = contents();
        assert_eq!(vec!["bloot's are worth a toot"], search(query, contents));
    }

    #[test]
    fn multi_results() {
        let query = "bloot";
        let contents = contents();

        assert_eq!(
            vec!["Productive for bloots", "bloot's are worth a toot"],
            search(query, contents)
        );
    }

    #[test]
    fn no_results() {
        let query = "NOT FOUND";
        let contents = contents();

        let mut empty = vec![""];
        empty.clear();
        assert_eq!(empty, search(query, contents));
    }

    #[test]
    fn whitespace() {
        let query = " ";
        let contents = contents();

        assert_eq!(
            vec![
                "Safe n Fast",
                "Productive for bloots",
                "bloot's are worth a toot"
            ],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "blOOt";
        let contents = contents();

        assert_eq!(
            vec!["Productive for bloots", "bloot's are worth a toot"],
            search_case_insensitive(query, contents)
        );
    }
}
