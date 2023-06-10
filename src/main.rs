use minigrep::Configuration;
use std::{env, process};

fn main() {
    let arguments: Vec<String> = env::args().collect();

    let configuration = Configuration::new(&arguments).unwrap_or_else(|error| {
        eprintln!("problem parsing arguments: {}", error);

        process::exit(1);
    });

    println!("searching for {}", configuration.query);
    println!("in file {}", configuration.filename);

    if let Err(error) = minigrep::run(configuration) {
        eprintln!("application error: {}", error);

        process::exit(1);
    }
}
