use std::env;

use minigrep::Configuration;
fn main() {
    let arguments: Vec<String> = env::args().collect();
    let configuration = Configuration::new(&arguments);

    println!("File: {}", configuration.filename);
    println!("Query: {}", configuration.query);

    minigrep::run(configuration);
}
