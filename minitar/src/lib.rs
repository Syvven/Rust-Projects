use std::path::Path;
use std::error::Error;
use tar::Header;
use std::os::unix::fs::MetadataExt;
use std::fs::{self, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Write, BufRead, Seek, SeekFrom};
use std::{env, process, slice, mem};
use std::ffi::{OsStr, CStr};

// config struct to hold the passed in arguments
#[derive(Debug)]
pub struct Config {
    pub command: String,
    pub archive_name: String,
    pub files: Vec<String>,
}

// config member functions
impl Config {
    // constructor for config
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // valid commands for modifying tar files
        let valids = vec!["c", "a", "t", "u", "x"];

        args.next();

        let err_msg = "Usage: cargo run c|a|t|u|x ARCHIVE_NAME.tar [FILE...]";

        // extract the command and error handle
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

        // extract the archive name and error handle
        let archive_name = match args.next() {
            Some(arg) => arg,
            None => return Err(err_msg),
        };

        // get every file that follows
        let files = args.collect();
        
        // make sure that the extension of the archive file is .tar
        match Path::new(&archive_name).extension().and_then(OsStr::to_str) {
            Some(ext) => {
                if ext != "tar" {
                    return Err(err_msg);
                }
            },
            None => return Err(err_msg),
        };

        // return the constructed struct
        Ok(Config { 
            command, 
            archive_name, 
            files 
        })
    }
}

// helper function for creating Tar Headers
pub fn init_header(filename: &str) -> Result<tar::Header, Box<dyn Error>> {
    // obtain the metadata of the specified filename
    let metadata = fs::metadata(filename)?;

    // set up a template header using the tar::Header crate
    let mut header = Header::new_ustar();
    
    // set the metadata
    header.set_metadata(&metadata);
    header.set_path(filename)?;

    // obtain major and minor device numbers
    let dev = metadata.dev();
    let major = (dev & 0xfff00) >> 8;
    let minor = (dev & 0x000ff) | ((dev >> 12) & 0xfff00);
    header.set_device_major(major.try_into()?)?;
    header.set_device_minor(minor.try_into()?)?;

    // obtain username
    let pwd = unsafe { libc::getpwuid(metadata.uid()) };
    if pwd.is_null() {
        return Err("libc::getpwuid failed.".into());
    }
    let c_str: &CStr = unsafe { CStr::from_ptr((*pwd).pw_name) };
    let str_slice: &str = c_str.to_str()?;
    header.set_username(str_slice)?;

    // obtain group username
    let grp = unsafe { libc::getgrgid(metadata.gid()) };
    if grp.is_null() {
        return Err("libc::getgrgid failed.".into());
    }
    let c_str: &CStr = unsafe { CStr::from_ptr((*grp).gr_name) };
    let str_slice: &str = c_str.to_str()?;
    header.set_groupname(str_slice)?;
    
    // return completed header
    Ok(header)
}

// helper function for appending to the archive
// removes the trailing zeros present in every archive
pub fn remove_trailing_zeros(config: &Config) -> Result<(), Box<dyn Error>> {
    // opens archive file
    let archive = OpenOptions::new()
        .write(true)
        .open(&config.archive_name)?;
    
    // sets the length to truncate the file
    archive.set_len(archive.metadata()?.len()-1024)?;
    Ok(())
}

// helper for archiving files
pub fn archiving_helper(config: &Config, create: bool) -> Result<(), Box<dyn Error>> {
    // checks if we are creating or appendings
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
    
    // iterates through the specified files
    // opens each file
    for file in config.files.iter() {
        let f = OpenOptions::new()
            .read(true)
            .open(file)?;

        let header = init_header(file)?;

        // writes the header to the archive file first
        writer.write_all(header.as_bytes())?;

        let mut reader = BufReader::new(f);
        let mut length = 1;
        let mut total_len = 0;

        // reads in from the specified file
        // and writes what was read into the archive file
        while length > 0 {
            let buffer = reader.fill_buf()?;
            
            writer.write(buffer)?;

            length = buffer.len();
            total_len += length;

            reader.consume(length);
        }

        // writes remaining pad zeros to be devisible by 512
        let pad: usize = 512 - (total_len % 512);
        let zeros = vec![0; pad];
        writer.write_all(&zeros)?;
    }

    // adds in the 1024 zeros at the end of the file
    let zeros = vec![0; 1024];
    writer.write_all(&zeros)?;

    Ok(())
}

// helper function for reading in header structs from the tar file
pub fn read_header<R: Read>(reader: &mut BufReader<R>) -> Result<Header, Box<dyn Error>> {
    // zeros out the header struct
    let mut header: Header = unsafe { mem::zeroed() };
    // gets the size of the Header type
    let header_size = mem::size_of::<Header>();
    unsafe {
        // slices...I don't really know what is going on here tbh
        let header_slice = slice::from_raw_parts_mut(&mut header as *mut _ as *mut u8, header_size);
        reader.read_exact(header_slice)?;
    }
    
    // return read Header
    Ok(header)
}

// function to get every file header present in the archive file
pub fn get_archive_files(config: &Config) -> Result<Vec<Header>, Box<dyn Error>> {
    let archive = OpenOptions::new()
        .read(true)
        .open(&config.archive_name)?;

    // makes sure there is a file in the archive
    if archive.metadata()?.len() < 512 {
        return Ok(vec![])
    }

    let mut reader = BufReader::new(&archive);
    
    // set up needed things
    let zeros: &[u8;512] = &[0; 512];
    let mut files = vec![];

    // read in the first header
    let mut header = read_header(&mut reader)?;

    // loop until the header read in is the zero bytes at the end
    while header.as_bytes() != zeros {
        // get size and pad zeros
        let size = header.size()?;
        let num_offset = size + (512 - (size % 512));  
        // seek to the start of the next header
        reader.seek(SeekFrom::Current(num_offset.try_into()?))?;

        // push the recently read in header to the vector
        files.push(header);

        // read in the next header
        header = read_header(&mut reader)?;
    } 

    // return vector of headers present
    Ok(files)
}

// starting point for creating archive
pub fn create_archive(config: &Config) -> Result<(), Box<dyn Error>> {
    archiving_helper(config, true)?;
    Ok(())
}

// starting point for appending to archive
pub fn append_to_archive(config: &Config) -> Result<(), Box<dyn Error>> {
    remove_trailing_zeros(config)?;
    archiving_helper(config, false)?;
    Ok(())
}

// starting point for printing archive
// does the actual printing as well
pub fn print_archive(config: &Config) -> Result<(), Box<dyn Error>> {
    let files = get_archive_files(config)?;
    for file in files.iter() {
        println!("{:?}", file.path()?);
    }
    Ok(())
}

// starting point for updating archive
// checks that the specified files are in the archive
// and appends them if they all are
pub fn update_archive(config: &Config) -> Result<(), Box<dyn Error>> {
    let files = get_archive_files(config)?;
    for file in config.files.iter() {
        let mut exist = false;
        for ofile in files.iter() {
            let path = ofile.path()?.into_owned().into_os_string().into_string().unwrap();
            if path == *file {
                exist = true;
                break;
            }
        }
        if !exist {
            return Err(format!("Update {}: file does not exist in archive", file).into())
        }
    }

    append_to_archive(config)?;
    Ok(())
}

// gets the currently present archive files using get_archive_files
// for each header, it creates or truncates the specified file
// and reads in the data from the archive in 512 byte blocks
// the data is then written into the newly created file
pub fn extract_from_archive(config: &Config) -> Result<(), Box<dyn Error>> {
    let files = get_archive_files(config)?;
    let archive = OpenOptions::new()
        .read(true)
        .open(&config.archive_name)?;

    if archive.metadata()?.len() < 512 {
        return Ok(())
    }

    let mut reader = BufReader::new(&archive);

    reader.seek(SeekFrom::Current(512.try_into()?))?;

    let mut buf = vec![0u8; 512];
    for header in files.iter() {
        // obtain information
        let size = header.size()?;
        let num_blocks = size / 512;
        let num_leftover: usize = (size % 512).try_into()?;
        let num_to_skip: usize = 512 - num_leftover;

        let path = header.path()?.into_owned().into_os_string().into_string().unwrap();

        let f = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path)?;
        
        let mut writer = BufWriter::new(&f);
        // iterate through blocks to read and write
        for _i in 0..num_blocks {
            reader.read_exact(&mut buf)?;
            writer.write_all(&buf)?;
        }

        // write in the leftover stuff
        let mut secbuf = vec![0u8; num_leftover];
        reader.read_exact(&mut secbuf)?;
        writer.write_all(&secbuf)?;

        // seek to next block of data
        reader.seek(SeekFrom::Current((512+num_to_skip).try_into()?))?;
    } 
    Ok(())
}

// main function for breaking up the command logic
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