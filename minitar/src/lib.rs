use std::env;
use std::process;
use std::path::Path;
use std::ffi::OsStr;
use std::error::Error;

#[derive(Debug)]
pub struct Config {
    pub command: String,
    pub archive_name: String,
    pub files: Vec<String>,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        let valids = vec!["c", "a", "t", "u", "x"];

        args.next();

        let err_msg = "Usage: cargo run c|a|t|u|x ARCHIVE_NAME.tar [FILE...]";

        let command = match args.next() {
            Some(arg) => {
                if !(valids.iter().any(|&s| s == arg)) {
                    return Err(err_msg);
                } else {
                    arg
                }
            },
            None => return Err(err_msg),
        };

        let archive_name = match args.next() {
            Some(arg) => arg,
            None => return Err(err_msg),
        };

        let files = args.collect();
        
        match Path::new(&archive_name).extension().and_then(OsStr::to_str) {
            Some(ext) => {
                if ext != "tar" {
                    return Err(err_msg);
                }
            },
            None => return Err(err_msg),
        };

        Ok(Config { 
            command, 
            archive_name, 
            files 
        })
    }
}

#[derive(Debug)]
pub struct TarHeader {
    pub name: String,
    pub mode: String,
    pub uid: String,
    pub gid: String,
    pub size: String,
    pub mtime: String,
    pub chksum: String,
    pub typeflag: String,
    pub linkname: String,
    pub magic: String,
    pub version: String,
    pub uname: String,
    pub gname: String,
    pub devmajor: String,
    pub devminor: String,
    pub prefix: String,
    pub padding: String,
}

impl TarHeader {
    pub fn new() -> TarHeader {

    }
}

pub fn create_archive(config: Config) -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub fn append_to_archive(config: Config) -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub fn get_archive_files(config: Config) -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub fn print_archive(config: Config) -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub fn update_archive(config: Config) -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub fn extract_from_archive(config: Config) -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub fn minitar_main(config: Config) -> Result<(), Box<dyn Error>> {
    match &config.command[..] {
        "c" => create_archive(config),
        "a" => append_to_archive(config),
        "t" => print_archive(config),
        "u" => update_archive(config),
        "x" => extract_from_archive(config),
        _ => {
            eprintln!("Unexpected Command: Exiting");
            process::exit(1);
        },
    };

    Ok(())
} 