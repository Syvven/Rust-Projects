use std::fs;
use std::env;
use std::error::Error;

// config struct that holds important information
// related to the workings of the minigrep program
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

// member functions
impl Config {
    // creates a new instance of the Config struct with the input values
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string."),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name."),
        };

        // see if the case sensitive env variable is set
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

// search function if case sensistive is not set
pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    // sets the query to lowercase
    let query = query.to_lowercase();

    // gets the lines that contain the query
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

// search function for if case sensitive is set
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents   
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

// main run function
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // read the file into a string
    // is this good practive? what if big file?
    let contents = fs::read_to_string(config.filename)?;

    // call corresponding search function
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    // print each line containing the query
    for line in results {
        println!("{}", line);
    }

    // return success
    Ok(())
}


// tests for the functions
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
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