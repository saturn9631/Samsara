use std::fmt::Formatter;
use std::fmt::Display;
use std::fmt::Result;

pub struct Parser {
    characters: Vec<char>,
    ast_stage_1: Vec<Tokens>,
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
            ast_stage_1: Vec::new(),
            //ast_stage_2: Vec::new(),
        }
    }
    fn generate_stage_1(&mut self) {
        let mut first_space = true; //To stop multiple pushes of blank words. 
        let mut nested = 0;
        for character in self.characters {
            if character == '(' {
                self.ast_stage_1.push(Tokens::Block(1, Vec::new()));
                first_space = true;
                nested += 1
            } else if character == ')' {
                first_space = true;
                nested -= 1
            } else if character == '[' {
                self.ast_stage_1.push(Tokens::Block(1, Vec::new()));
                nested += 1
            } else if character == ']' {
                first_space = true;
                nested -= 1
            } else if character == '{' {
                self.ast_stage_1.push(Tokens::Block(1, Vec::new()));
                nested += 1
            } else if character == '}' {
                first_space = true;
                nested -= 1
            } else if character == ' ' || character == '\n' || character == '\t' {
                if first_space {
                    self.ast_stage_1.push(Tokens::Word(String::new()));
                    first_space = false;
                }
            } else {
                let mut possible_container = self.ast_stage_1.last_mut();
                match possible_target {
                    Some(target) => {
                        target.push(character);
                    }
                    None => {
                        println!("An error occured accessing the latatst token in the list.");
                    }
                }
            }
        }
        if nested != 0 {
            if nested > 0 {
                println!("You need to close some blocks");
            } else {
                println!("You have too many block closers");
            }
        }
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

    pub fn print_tokens(&self) {
        let mut index: u8 = 0;
        for token in &self.tokens {
            println!("Token {index}: {token}");
            index += 1;
        }
    }
    
    pub fn print_stage_1(&self) {
    }

    pub fn print_stage_2(&self) {
    }
} /*impl Display for Parser {
}*/

enum Tokens {
    Word(String),
    Block(Int, Vec<Tokens>),
} impl Tokens {

    fn push(&mut self, character: char) {
        match &self {
            Tokens::Word(identifier) => {
                identifier.push(character);
            }
            Tokens::Block(block_type, mut inside) => {
                inside.push(character)
            }
        }
    }
} impl Display for Tokens {
    fn fmt(&self, &mut formatter: Formatter) -> Result {
        write!(formatter, "", self)
    }
