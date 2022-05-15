use std::env;
use std::fs;

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static srt> {
        if args.len() < 3 {
            return Err("usage: cargo run <word> <filename>")
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Probelm parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("Failed to read file to string.");

    println!("WIth text:\n{}", contents);
}
