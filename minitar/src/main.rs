use std::env;
use std::process;

use minitar::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    println!("{:?}", config);

    if let Err(e) = minitar::minitar_main(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
