use std::fmt::Formatter;
use std::fmt::Display;
use std::fmt::Result;

mod stage_one;
use stage_one::Scentences;

pub struct Parser {
    characters: Vec<char>,
    ast_stage_1: Scentences,
    //ast_stage_2: Vec<>,
} impl Parser {
    pub fn new(lines: Vec<String>) -> Self {
        let mut pieces: Vec<char> = Vec::new();
        for line in lines {
            for character in line.chars() {
                pieces.push(character);
            }
        }
        Self {
            characters: pieces,
            ast_stage_1: Scentences::new(),
            //ast_stage_2: Vec::new(),
        }
    }
    fn generate_stage_1(&mut self) {
    }

    fn generate_stage_2(&mut self) {
    }

    pub fn print_characters(&self) {
        let mut index: u8 = 0;
        for character in &self.characters {
            println!("Character {index}: {character}");
            index += 1;
        }
    }

    pub fn print_words(&self) {
    }

    pub fn print_stage_2(&self) {
    }
} /*impl Display for Parser {
}*/

