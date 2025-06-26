// Make std depend on kernel
//use std;
use std::fmt::{Debug, Formatter};
pub struct AST_Stage_One {
    tokens: Vec<Vec<String>>,
} impl AST_Stage_One {
    pub fn new(input: String) -> Self {
        let lines: Vec<&str> = input.split('\n').collect();
        let mut tokens = Vec::new();
        for line in lines {
            let words: Vec<String> = line.split(' ').map(|x| String::from(x)).collect();
            tokens.push(words);
        }
        Self {
            tokens,
        }
    }
}
