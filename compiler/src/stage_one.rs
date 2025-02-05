pub struct Scentence {
    tokens: Vec<Token>;
} impl Scentence {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new();
        }
    }
}
#[derive(Debug)]
enum Token {
    Word(String),
    Block(Int, Scentences),
} impl Tokens {
    fn push(&mut self, character: char) {
        match &self {
            Tokens::Word(identifier) => {
                identifier.push(character);
            }
            Tokens::Block(block_type, mut body) => {
                body.last_mut().push(character);
            }
        }
    }

    fn seperate(&mut self) {
    }
} /* impl Display for Tokens {
    fn fmt(&self, &formatter: Formatter<str>) -> Result {
        write!(formatter, "", self)
    }
} */
