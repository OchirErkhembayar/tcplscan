use crate::tokenizer::Token;

// Count the number of if and case labels
// elsif, if, function
#[derive(Debug)]
pub struct Cyclomatic {
    pub score: usize,
}

pub struct Complexity {
    cyclomatic: Cyclomatic,
}

pub struct Parser<'a> {
    tokens: &'a [Token],
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a [Token]) -> Self {
        Self { tokens }
    }

    fn advance(&mut self) {
        if let Some(_) = self.tokens.get(0) {
            self.tokens = &self.tokens[1..];
        } else {
            panic!("Fix this bug with advancing in parser");
        }
    }

    fn next_token_opt(&mut self) -> Option<&Token> {
        let char = match self.tokens.get(0) {
            Some(char) => char,
            None => return None,
        };
        self.advance();
        Some(char)
    }
}

impl<'_> Iterator for Parser<'_> {
    type Item = Complexity;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token_opt() 
    }
}

impl<'a> Parser<'a> {
    fn parse_next()
}
