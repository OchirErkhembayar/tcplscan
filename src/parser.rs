use std::process;

use crate::{error, token::TokenType, tokenizer::Token};

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Stmt {
    pub kind: StmtType,
    pub line: usize,
}

impl Stmt {
    fn new(kind: StmtType, line: usize) -> Self {
        Self { kind, line }
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum StmtType {
    If,
    Elseif,
    Function,
    For,
    Foreach,
    Switch { case_count: usize },
    Match { case_count: usize },
}
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
    pub fn new(tokens: &'a [Token]) -> Self {
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
        let token = match self.tokens.get(0) {
            Some(token) => token,
            None => return None,
        };
        self.advance();
        Some(token)
    }

    fn next_token(&mut self) -> &Token {
        self.next_token_opt().unwrap_or_else(|| {
            eprintln!("ERROR: Expected token.");
            process::exit(1);
        })
    }

    fn consume(&mut self, expected: TokenType) {
        let next_token = self.next_token();
        if next_token.token_type != expected {
            error(
                format!("Expected {:?}, got {}", expected, next_token.lexeme).as_str(),
                next_token.line,
            );
            process::exit(1);
        }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Stmt;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_stmt()
    }
}

impl<'a> Parser<'a> {
    fn next_stmt(&mut self) -> Option<Stmt> {
        let token = match self.next_token_opt() {
            Some(token) => token,
            None => return None,
        };

        let token = match token.token_type {
            TokenType::If => Stmt::new(StmtType::If, token.line),
            TokenType::Elseif => Stmt::new(StmtType::Elseif, token.line),
            TokenType::For => Stmt::new(StmtType::For, token.line),
            TokenType::Foreach => Stmt::new(StmtType::Foreach, token.line),
            TokenType::Function => Stmt::new(StmtType::Function, token.line),
            _ => return self.next_stmt(),
        };

        Some(token)
    }
}
