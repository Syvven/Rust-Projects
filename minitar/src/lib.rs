use std::path::Path;
use std::error::Error;
use tar::Header;
use std::os::unix::fs::MetadataExt;
use std::fs::{self, OpenOptions};
use std::io::{BufReader, BufWriter, Write, BufRead};
use std::{env, process};
use std::ffi::{OsStr, CStr};

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

pub fn init_header(filename: &str) -> Result<tar::Header, Box<dyn Error>> {
    let metadata = fs::metadata(filename)?;
    let mut header = Header::new_ustar();
    
    header.set_metadata(&metadata);
    header.set_path(filename)?;

    let dev = metadata.dev();
    let major = (dev & 0xfff00) >> 8;
    let minor = (dev & 0x000ff) | ((dev >> 12) & 0xfff00);
    header.set_device_major(major.try_into()?)?;
    header.set_device_minor(minor.try_into()?)?;

    let pwd = unsafe { libc::getpwuid(metadata.uid()) };
    if pwd.is_null() {
        return Err("libc::getpwuid failed.".into());
    }
    let c_str: &CStr = unsafe { CStr::from_ptr((*pwd).pw_name) };
    let str_slice: &str = c_str.to_str()?;
    header.set_username(str_slice)?;


    let grp = unsafe { libc::getgrgid(metadata.gid()) };
    if grp.is_null() {
        return Err("libc::getgrgid failed.".into());
    }
    let c_str: &CStr = unsafe { CStr::from_ptr((*grp).gr_name) };
    let str_slice: &str = c_str.to_str()?;
    header.set_groupname(str_slice)?;
    
    Ok(header)
}

pub fn remove_trailing_zeros(config: &Config) -> Result<(), Box<dyn Error>> {
    let archive = OpenOptions::new()
        .write(true)
        .open(&config.archive_name)?;
    
    archive.set_len(archive.metadata()?.len()-1024)?;
    Ok(())
}

pub fn archiving_helper(config: &Config, create: bool) -> Result<(), Box<dyn Error>> {
    let archive = if create {
        OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&config.archive_name)?
    } else {
        OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(&config.archive_name)?
    };

    let mut writer = BufWriter::new(&archive);
    
    for file in config.files.iter() {
        let f = OpenOptions::new()
            .read(true)
            .open(file)?;

        let header = init_header(file)?;

        writer.write_all(header.as_bytes())?;

        let mut reader = BufReader::new(f);
        let mut length = 1;
        let mut total_len = 0;

        while length > 0 {
            let buffer = reader.fill_buf()?;
            
            writer.write(buffer)?;

            length = buffer.len();
            total_len += length;

            reader.consume(length);
        }

        let pad: usize = 512 - (total_len % 512);
        let zeros = vec![0; pad];
        writer.write_all(&zeros)?;
    }

    let zeros = vec![0; 1024];
    writer.write_all(&zeros)?;

    Ok(())
}

pub fn create_archive(config: &Config) -> Result<(), Box<dyn Error>> {
    archiving_helper(config, true)?;

    Ok(())
}

pub fn append_to_archive(config: &Config) -> Result<(), Box<dyn Error>> {
    remove_trailing_zeros(config)?;
    archiving_helper(config, false)?;

    Ok(())
}

pub fn get_archive_files(config: &Config) -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub fn print_archive(config: &Config) -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub fn update_archive(config: &Config) -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub fn extract_from_archive(config: &Config) -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub fn minitar_main(config: Config) -> Result<(), Box<dyn Error>> {
    match &config.command[..] {
        "c" => create_archive(&config)?,
        "a" => append_to_archive(&config)?,
        "t" => print_archive(&config)?,
        "u" => update_archive(&config)?,
        "x" => extract_from_archive(&config)?,
        _ => {
            eprintln!("Unexpected Command: Exiting");
            process::exit(1);
        },
    };

    Ok(())
} 