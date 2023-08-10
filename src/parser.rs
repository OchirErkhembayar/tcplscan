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
    pub stmts: Vec<Stmt>,
    bracket_stack: Vec<char>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, stmts: Vec::new(), bracket_stack: Vec::new() }
    }

    fn advance(&mut self) {
        if let Some(_) = self.tokens.get(0) {
            self.tokens = &self.tokens[1..];
        } else {
            panic!("Fix this bug with advancing in parser");
        }
    }
    
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(0)
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

    fn add_bracket(&mut self) {

    }
}

impl<'a> Parser<'a> {
    pub fn parse(&mut self) {
        loop {
            let token = match self.next_token_opt() {
                Some(token) => token,
                None => break,
            };

            let stmt = match token.token_type {
                TokenType::If => Stmt::new(StmtType::If, token.line),
                TokenType::Elseif => Stmt::new(StmtType::Elseif, token.line),
                TokenType::For => Stmt::new(StmtType::For, token.line),
                TokenType::Foreach => Stmt::new(StmtType::Foreach, token.line),
                TokenType::Function => Stmt::new(StmtType::Function, token.line),
                TokenType::Switch => {
                    let line = token.line;
                    self.switch_stmt(line)
                }
                _ => continue,
            };
            self.stmts.push(stmt);
        }
    }

    fn switch_stmt(&mut self, line: usize) -> Stmt {
        // Look for next token and if you find case then keep looking for more case
        // If you find default or another statement then stop
        let mut case_count = 0;
        loop {
            let token = self.next_token_opt().unwrap_or_else(|| {
                eprintln!("Unterminated switch statement");
                process::exit(1);
            });

            match token.token_type {
                TokenType::Case => case_count += 1,
                TokenType::Default => break,
                TokenType::Match => {
                    let line = token.line;
                    let stmt = self.match_stmt(line);
                    self.stmts.push(stmt);
                }
                TokenType::Switch => {
                    let line = token.line;
                    let stmt = self.switch_stmt(line);
                    self.stmts.push(stmt);
                }
                TokenType::If => {
                    let line = token.line;
                    self.stmts.push(Stmt::new(StmtType::If, line));
                }
                TokenType::Elseif => {
                    let line = token.line;
                    self.stmts.push(Stmt::new(StmtType::Elseif, line));
                }
                TokenType::For => {
                    let line = token.line;
                    self.stmts.push(Stmt::new(StmtType::For, line));
                }
                TokenType::Foreach => {
                    let line = token.line;
                    self.stmts.push(Stmt::new(StmtType::Foreach, line));
                }
                _ => {
                    // Figure out how to tell that switch ended when there is no default
                    // Create a stack of brackets/parens and use the depth to tell when it ends
                    // Then we can remove the default arm of this match
                    continue;
                },
            }
        }
        Stmt::new(StmtType::Switch { case_count }, line)
    }

    fn match_stmt(&mut self, line: usize) -> Stmt {
        let mut case_count = 0;
        loop {
            let token = self.next_token_opt().unwrap_or_else(|| {
                eprintln!("Unterminated switch statement");
                process::exit(1);
            });

            match token.token_type {
                TokenType::FatArrow => case_count += 1,
                TokenType::Default => break,
                TokenType::Match => {
                    let line = token.line;
                    self.match_stmt(line);
                }
                TokenType::Switch => {
                    let line = token.line;
                    let stmt = self.switch_stmt(line);
                    self.stmts.push(stmt);
                }
                TokenType::If => {
                    let line = token.line;
                    self.stmts.push(Stmt::new(StmtType::If, line));
                }
                TokenType::Elseif => {
                    let line = token.line;
                    self.stmts.push(Stmt::new(StmtType::Elseif, line));
                }
                TokenType::For => {
                    let line = token.line;
                    self.stmts.push(Stmt::new(StmtType::For, line));
                }
                TokenType::Foreach => {
                    let line = token.line;
                    self.stmts.push(Stmt::new(StmtType::Foreach, line));
                }
                _ => continue,
            }
        }
        Stmt::new(StmtType::Match { case_count }, line)
    }
}
