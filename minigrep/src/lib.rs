use std::error::Error;
use std::env;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_new_few_args() {
        let args = vec![String::from("target/debug/minigrep")];
        let result = Config::new(&args);
        assert!(result.is_err());
        assert_eq!(result.err(), Some("Not enough arguments provided"));
    }

    #[test]
    fn config_new_with_right_args() {
        let application = String::from("target/debug/minigrep");
        let query = String::from("hello");
        let filename = String::from("poem.txt");
        let args = vec![application, query, filename];
        let result = Config::new(&args);
        assert!(result.is_ok());

        let config = result.unwrap();
        assert_eq!(config.query, String::from("hello"));
        assert_eq!(config.filename, String::from("poem.txt"));
    }

    #[test]
    fn run_ok() {
        let config = Config {
            query: String::from("hello"),
            filename: String::from("poem.txt"),
            case_sensitive: true,
        };
        let result = run(config);
        assert!(result.is_ok());
    }

    #[test]
    fn run_with_non_existent_file() {
        let config = Config {
            query: String::from("hello"),
            filename: String::from("non_existent.txt"),
            case_sensitive: true,
        };
        let result = run(config);
        assert!(result.is_err());
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            search(query, contents),
            vec!["safe, fast, productive."]
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            search_case_insensitive(query, contents),
            vec!["Rust:", "Trust me."]
        );
    }
}
