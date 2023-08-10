use std::process;

use crate::{token::TokenType, tokenizer::Token};

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
    Throw,
    Catch,
    Switch { case_count: usize },
    Match { case_count: usize },
}

pub struct Parser<'a> {
    tokens: &'a [Token],
    pub stmts: Vec<Stmt>,
    brackets: Vec<TokenType>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self {
            tokens,
            stmts: Vec::new(),
            brackets: Vec::new(),
        }
    }

    fn closing_bracket(&mut self, token_type: TokenType) {
        let top = self.brackets.pop().unwrap_or_else(|| {
            eprintln!("Unmatched opening bracket: {:?}", token_type);
            process::exit(1);
        });
        match top {
            TokenType::LeftParen => {
                if token_type != TokenType::RightParen {
                    eprintln!("Unmatched closing bracket: {:?}", token_type);
                    process::exit(1);
                }
            }
            TokenType::LeftBrace => {
                if token_type != TokenType::RightBrace {
                    eprintln!("Unmatched closing bracket: {:?}", token_type);
                    process::exit(1);
                }
            }
            _ => {
                panic!("This shouldn't happen :P");
            }
        }
    }

    fn advance(&mut self) {
        if let Some(token) = self.tokens.get(0) {
            match token.token_type {
                TokenType::LeftParen => self.brackets.push(TokenType::LeftParen),
                TokenType::LeftBrace => self.brackets.push(TokenType::LeftBrace),
                TokenType::RightParen => self.closing_bracket(TokenType::RightParen),
                TokenType::RightBrace => self.closing_bracket(TokenType::RightBrace),
                _ => (),
            }
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
}

impl<'a> Parser<'a> {
    pub fn parse(&mut self) {
        while let Some(token) = self.next_token_opt() {
            let stmt = match token.token_type {
                TokenType::If => Stmt::new(StmtType::If, token.line),
                TokenType::Elseif => Stmt::new(StmtType::Elseif, token.line),
                TokenType::For => Stmt::new(StmtType::For, token.line),
                TokenType::Foreach => Stmt::new(StmtType::Foreach, token.line),
                TokenType::Function => Stmt::new(StmtType::Function, token.line),
                TokenType::Throw => Stmt::new(StmtType::Throw, token.line),
                TokenType::Catch => Stmt::new(StmtType::Catch, token.line),
                TokenType::Switch => {
                    let line = token.line;
                    self.switch_stmt(line)
                }
                TokenType::Match => {
                    let line = token.line;
                    self.match_stmt(line)
                }
                _ => continue,
            };
            self.stmts.push(stmt);
        }
    }

    fn switch_stmt(&mut self, line: usize) -> Stmt {
        let mut case_count = 0;
        let depth = self.brackets.len();
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
                TokenType::Throw => {
                    let line = token.line;
                    self.stmts.push(Stmt::new(StmtType::Throw, line));
                }
                TokenType::Catch => {
                    let line = token.line;
                    self.stmts.push(Stmt::new(StmtType::Catch, line));
                }
                TokenType::RightBrace => {
                    if self.brackets.len() == depth {
                        break;
                    }
                }
                _ => continue,
            }
        }
        Stmt::new(StmtType::Switch { case_count }, line)
    }

    fn match_stmt(&mut self, line: usize) -> Stmt {
        let mut case_count = 0;
        let depth = self.brackets.len();
        loop {
            let token = self.next_token_opt().unwrap_or_else(|| {
                eprintln!("Unterminated match statement");
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
                TokenType::Throw => {
                    let line = token.line;
                    self.stmts.push(Stmt::new(StmtType::Throw, line));
                }
                TokenType::Catch => {
                    let line = token.line;
                    self.stmts.push(Stmt::new(StmtType::Catch, line));
                }
                TokenType::RightBrace => {
                    if self.brackets.len() == depth {
                        break;
                    }
                }
                _ => continue,
            }
        }
        Stmt::new(StmtType::Match { case_count }, line)
    }
}
