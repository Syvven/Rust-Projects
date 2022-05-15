use std::env;
use std::process;

// import the crate from lib.rs
use minigrep::Config;

// main sets up the logic of the program
fn main() {
    // setup the config struct with the arguments and handle errors
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Probelm parsing arguments: {}", err);
        process::exit(1);
    });

    // run the main code of the program and handle errors
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
