use std::fs::File;
use std::io::{BufReader, BufRead};

mod parser;
use crate::parser::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = open_file("../test/sources/main.yg");
    let mut lines: Vec<String> = Vec::new();
    for possible_line in reader.lines() {
        match possible_line {
            Ok(line) => {
                lines.push(line);
            }
            Err(_) => println!("There was an error reading this line."),
        }
    }
    let mut parser = Parser::new(lines);
    parser.print_characters();

    Ok(())
}

/*
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}
*/

fn open_file(file_path: &str) -> BufReader<File> {
    let potential_file = File::open(file_path);
    let file = match potential_file {
        Ok(successful_file) => successful_file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };

    let reader = BufReader::new(file);
    return reader;
}
