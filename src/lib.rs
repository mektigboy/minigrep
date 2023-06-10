use std::{env, error::Error, fs};

pub fn run(configuration: Configuration) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(configuration.filename)?;

    let results = if configuration.case_sensitive {
        search(&configuration.query, &contents)
    } else {
        search_case_insensitive(&configuration.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub struct Configuration {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Configuration {
    pub fn new(arguments: &[String]) -> Result<Configuration, &str> {
        if arguments.len() < 3 {
            return Err("not enough arguments");
        }

        let query = arguments[1].clone();
        let filename = arguments[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Configuration {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
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
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
