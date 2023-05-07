use std::fs;

pub struct Configuration {
    pub filename: String,
    pub query: String,
}

impl Configuration {
    pub fn new(arguments: &[String]) -> Configuration {
        let filename = arguments[1].clone();
        let query = arguments[2].clone();

        Configuration { filename, query }
    }
}

pub fn run(configuration: Configuration) {
    let content =
        fs::read_to_string(configuration.filename).expect("Something went wrong reading the file.");
    let found = search(&configuration.query, &content);

    for line in found {
        println!("{}", line);
    }
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
