use std::env;
use std::process;

// bring the crate containing most logic into scope
use minitar::Config;

fn main() {
    // initialize the config struct based on the input arguments
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    // run the main function 
    // propogated errors are handled here
    if let Err(e) = minitar::minitar_main(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
