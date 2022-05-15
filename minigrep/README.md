# minigrep

## Functionality

This minigrep program was created while following along with the rust programming book mentioned in the parent directory README. It takes two command line arguments: a query and a filename. The program then searches the file for the specified query and outputs every line that contains the query. Files to be searched are located in the top level of the directory. 

## Usage

Install rust and cargo.

Download the `minigrep` folder.

cd into the `minigrep` folder:

`> cd minigrep`

Type into the command line:

`> cargo run <query> <filename>`

where query is a word and filename is the name and extension of a file that exists in the top level of the `minigrep` directory.

Lines that contain the query will be printed to the terminal.
