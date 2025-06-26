// Make std depend on kernel
//use std;

use ast::*;
use ast::syntax::*;
use ast::tokenizer::AST_Stage_One;
mod ast;

fn main() {
    let test = String::from("Hi, I am a choochoo, train. \n I run forever carring the souls to the afterlife. \n I love my job as the soul-train.\n");
    let token_ast = AST_Stage_One::new(test);
}

