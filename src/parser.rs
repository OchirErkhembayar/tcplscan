use std::process;

use crate::{token::{TokenType, match_keyword, Keyword}, tokenizer::Token};

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
    Switch { 
        case_count: usize,
        stmts: Vec<Stmt>,
    },
    Match { case_count: usize },
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Class {
    pub name: String,
    pub functions: Vec<Function>
}

impl Class {
    fn new() -> Self {
        Self {
            name: String::new(),
            functions: Vec::new(),
        }
    }

    fn push_str(&mut self, name: &str) {
        self.name.push_str(name);
    }

    fn add_fn(&mut self, function: Function) {
        self.functions.push(function);
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Function {
    pub stmts: Vec<Stmt>,
    pub name: String,
    pub params: usize,
    pub return_type: Option<String>,
}

impl Function {
    fn new(name: String, stmts: Vec<Stmt>, params: usize, return_type: Option<String>) -> Self {
        Self { 
            name, 
            stmts,
            params,
            return_type,
        }
    }
}

pub struct Parser<'a> {
    tokens: &'a [Token],
    pub stmts: Vec<Stmt>,
    brackets: Vec<TokenType>,
    pub class: Class,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self {
            tokens,
            stmts: Vec::new(),
            brackets: Vec::new(),
            class: Class::new(),
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

    fn next_token(&mut self) -> &Token {
        self.next_token_opt().unwrap_or_else(|| {
            eprintln!("Expected token. Found none.");
            process::exit(1);
        })
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(0)
    }
}

impl<'a> Parser<'a> {
    pub fn parse(&mut self) {
        while let Some(token) = self.next_token_opt() {
            let stmt = match token.token_type {
                TokenType::Identifier => {
                    if let Some(token_type) = match_keyword(token.lexeme.as_str()) {
                        match token_type {
                            Keyword::Namespace => {
                                let namespace = self.next_token().lexeme.clone();
                                self.class.push_str(namespace.as_str());
                                continue;
                            }
                            Keyword::Class => {
                                let name = self.next_token().lexeme.clone();
                                self.class.push_str("\\");
                                self.class.push_str(name.as_str());
                                continue;
                            }
                            Keyword::Function => {
                                println!("Starting a function {:?}", self.peek());
                                let name = self.next_token().lexeme.clone();
                                let mut params = 0;
                                while self.peek().is_some_and(|t| t.token_type != TokenType::RightParen) {
                                    if self.next_token().lexeme.starts_with("$") {
                                        params += 1;
                                    }
                                }
                                self.advance();
                                let return_type = if self.peek().is_some_and(|t| t.token_type == TokenType::Colon) {
                                    self.advance();
                                    Some(self.next_token().lexeme.clone())
                                } else {
                                    None
                                };
                                let depth = self.brackets.len();
                                println!("Depth: {depth}");
                                println!("Next token: {:?}", self.peek());
                                self.advance();
                                println!("Depth after: {}", self.brackets.len());
                                let mut stmts = Vec::new();
                                while depth != self.brackets.len() {
                                    if let Some(stmt) = self.parse_stmt() {
                                        stmts.push(stmt);
                                    }
                                }
                                println!("Parsed stmts: {:?}", stmts);
                                self.class.add_fn(Function::new(name, stmts, params, return_type));
                                continue;
                            }
                            _ => Stmt::new(StmtType::For, token.line),
                        }
                    } else {
                        continue;
                    }
                }
                _ => continue,
            };
            self.stmts.push(stmt);
        }
    }

    fn parse_stmt(&mut self) -> Option<Stmt> {
        loop {
            let token = self.next_token();
            println!("Token inside function: {:?}", token);
            let keyword = match match_keyword(token.lexeme.as_str()) {
                Some(keyword) => keyword,
                None => return None,
            };
            let line = token.line;
            return Some(match keyword {
                Keyword::If => Stmt::new(StmtType::If, line),
                Keyword::Elseif => Stmt::new(StmtType::Elseif, line),
                Keyword::For => Stmt::new(StmtType::For, line),
                Keyword::Foreach => Stmt::new(StmtType::Foreach, line),
                Keyword::Switch => self.switch_stmt(line),
                Keyword::Match => self.match_stmt(line),
                _ => {
                    println!("Todo {:?}", token);
                    todo!();
                },
            });
        }
    }

    fn switch_stmt(&mut self, line: usize) -> Stmt {
        let mut case_count = 0;
        let depth = self.brackets.len();
        let mut stmts = Vec::new();
        loop {
            let token = self.next_token_opt().unwrap_or_else(|| {
                eprintln!("Unterminated switch statement");
                process::exit(1);
            });

            match token.token_type {
                TokenType::Identifier => {
                    let keyword = match match_keyword(token.lexeme.as_str()) {
                        Some(keyword) => keyword,
                        None => continue,
                    };
                    stmts.push(match keyword {
                        Keyword::Case => {
                            case_count += 1;
                            continue;
                        }
                        Keyword::If => Stmt::new(StmtType::If, token.line),
                        Keyword::Elseif => Stmt::new(StmtType::Elseif, token.line),
                        Keyword::For => Stmt::new(StmtType::For, token.line),
                        Keyword::Foreach => Stmt::new(StmtType::Foreach, token.line),
                        Keyword::Match => {
                            let line = token.line;
                            self.match_stmt(line)
                        }
                        Keyword::Switch => {
                            let line = token.line;
                            self.switch_stmt(line)
                        }
                        _ => {
                            println!("Todo {:?}", token);
                            todo!();
                        },
                    });
                }
                TokenType::RightBrace => {
                    if self.brackets.len() == depth {
                        break;
                    }
                }
                _ => continue,
            }
        }
        Stmt::new(StmtType::Switch { case_count, stmts }, line)
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
