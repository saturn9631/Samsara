// Make std depend on kernel
//use std;

use tokenizer;

struct AST_Stage_Two {
} impl AST_Stage_Two {
}

enum AST2_Element {
    Root(Vec<AST_Element>),
    Branch(String, Vec<AST_Element::Leaf>),
    Leaf(String),
} impl AST2_Element {
    fn from(oldTree: AST_Stage_One) -> Self {
    }
}
