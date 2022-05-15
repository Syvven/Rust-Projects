# minitar

## Functionality

Remake in Rust of the CSCI4061: Intro to Operating Systems minitar project that was originally in C. Source code to that project will not be included due to academic dishonesty concerns.

This program serves to replicate the functionality of the `tar` archiving utility. It utilizes file I/O and robust error handling in order to properly archive and extract files. Included are a couple files in the `test_files` directory to test the functionality on. 

## Usage

- First, make sure you have rust and cargo installed.
- Clone the repository and cd into the `minitar` directory.
- Create any files you would like to contain in the Archive, or use the test files in the `test_files` subdirectory.
- Run in the command line `> cargo clean` to make sure everything is ready to be run.
- Arguments to the program are structured as thus:

`> cargo run <command> <archive_name> <file1> <file2> etc...`

- Commands include:
  - c: creates archive of specified name and adds the specified files. If the archive already exists, it overwrites existing data. If any of the files don't exist, the program will error.
  - a: appends specified files to the specified archive. If the archive or any of the files do not exist, the program will error.
  - t: prints out the names of each file contained in the archive.
  - u: appends the specified files to the end of the specified archive. If any of the files do not exist in the archive, the program will error.
  - x: extracts the files from the specified archive. Overwrites existing files with same names. Errors if the archive name does not exist.
- <archive_name> must end in .tar
- None of the commands require files to be specified, however, nothing will be added if they are not specified.
- The exact path to the files must be specified and they will be created in the same directory when extracted.
