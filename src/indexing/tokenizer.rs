use std::process;

use crate::indexing::token::TokenType;
use crate::error;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub lexeme: String,
}

impl Token {
    fn new(token_type: TokenType, line: usize, lexeme: String) -> Self {
        Self {
            token_type,
            line,
            lexeme,
        }
    }
}

pub struct Tokenizer<'a> {
    pub code: &'a [char],
    pub line: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(code: &'a [char]) -> Self {
        Self { code, line: 1 }
    }

    fn advance(&mut self) {
        if let Some(c) = self.code.first() {
            if c == &'\n' {
                self.line += 1;
            }

            self.code = &self.code[1..];
        } else {
            error("Expected token", self.line);
        }
    }

    fn next_char_opt(&mut self) -> Option<char> {
        let char = match self.code.first() {
            Some(char) => char,
            None => return None,
        };
        self.advance();
        Some(*char)
    }

    fn next_char(&mut self) -> char {
        self.next_char_opt().unwrap_or_else(|| {
            error("Character missing", self.line);
            process::exit(1);
        })
    }

    fn peek(&self) -> Option<&char> {
        self.code.first()
    }

    fn peek_next(&self) -> Option<&char> {
        self.code.get(1)
    }

    fn match_char(&mut self, to_match: char) -> bool {
        if self.peek().is_some_and(|char| char == &to_match) {
            self.advance();
            return true;
        }
        false
    }

    fn make_token(&self, token_type: TokenType, lexeme: String) -> Token {
        Token::new(token_type, self.line, lexeme)
    }

    fn string(&mut self, quote_type: char) -> Token {
        let mut string = String::new();
        let mut escaped = false;
        while (!self.peek().is_some_and(|c| c == &quote_type) || escaped) && !self.code.is_empty() {
            let char = self.next_char();
            if char == '\\' {
                escaped = !escaped;
            } else {
                escaped = false;
            }
            string.push(char);
        }
        if self.code.is_empty() {
            error("Unterminated string", self.line);
        }
        self.advance();

        self.make_token(TokenType::String, string)
    }

    fn number(&mut self) -> Token {
        let mut string = String::new();
        while self.peek().is_some_and(|c| c.is_ascii_digit()) {
            string.push(self.next_char());
        }
        if self.peek().is_some_and(|c| c == &'.') {
            string.push(self.next_char());
            while self.peek().is_some_and(|c| c.is_ascii_digit()) {
                string.push(self.next_char());
            }
        }

        self.make_token(TokenType::Number, string)
    }

    fn identifier(&mut self, start: char) -> Token {
        let mut word = start.to_string();
        while self
            .peek()
            .is_some_and(|c| c.is_alphanumeric() || c == &'_' || c == &'\\')
        {
            word.push(self.next_char());
        }

        self.make_token(TokenType::Identifier, word)
    }

    fn here_doc(&mut self) -> Token {
        let mut title = Vec::new();
        let mut doc = String::new();
        if self.peek().is_some_and(|c| c == &'\'' || c == &'"') {
            let opening = self.next_char();
            while self.peek().is_some_and(|c| c != &opening) {
                title.push(self.next_char());
            }
            self.advance();
        } else {
            while self.peek().is_some_and(|c| c != &'\n') {
                title.push(self.next_char());
            }
        }
        self.advance();
        loop {
            let mut found = true;
            for (i, char) in title.iter().enumerate() {
                if &self.code[i] != char {
                    found = false;
                    break;
                }
            }
            if found {
                title.iter().for_each(|_| self.advance());
                break;
            } else {
                doc.push(self.next_char());
            }
        }

        self.make_token(TokenType::HereDoc, doc)
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.scan_token()
    }
}

impl<'a> Tokenizer<'a> {
    fn scan_token(&mut self) -> Option<Token> {
        let char = match self.next_char_opt() {
            Some(char) => char,
            None => return None,
        };
        let token = match char {
            ' ' | '\r' | '\t' => return self.scan_token(),
            '\n' => return self.scan_token(),
            '{' => self.make_token(TokenType::LeftBrace, "{".to_string()),
            '}' => self.make_token(TokenType::RightBrace, "}".to_string()),
            '(' => self.make_token(TokenType::LeftParen, "(".to_string()),
            ')' => self.make_token(TokenType::RightParen, ")".to_string()),
            '[' => self.make_token(TokenType::LeftBracket, "[".to_string()),
            ']' => self.make_token(TokenType::RightBracket, "]".to_string()),
            ',' => self.make_token(TokenType::Comma, ",".to_string()),
            '.' => self.make_token(TokenType::Dot, ".".to_string()),
            '-' => {
                if self.peek().is_some_and(|c| c == &'>') {
                    self.advance();
                    self.make_token(TokenType::ThinArrow, "->".to_string())
                } else {
                    self.make_token(TokenType::Minus, "-".to_string())
                }
            }
            '+' => self.make_token(TokenType::Plus, "+".to_string()),
            ';' => self.make_token(TokenType::Semicolon, ";".to_string()),
            '#' => self.make_token(TokenType::Hash, "#".to_string()),
            '/' => {
                if self.match_char('*') {
                    while !(self.code.is_empty()
                        || (self.peek().is_some_and(|c| c == &'*')
                            && self.peek_next().is_some_and(|c| c == &'/')))
                    {
                        self.advance();
                    }
                    if self.code.len() < 2 {
                        error("Unterminated block comment", self.line);
                    }
                    self.advance();
                    self.advance();
                    return self.scan_token();
                } else if self.match_char('/') {
                    while self.peek().is_some_and(|c| c != &'\n') {
                        self.advance();
                    }
                    self.advance();
                    return self.scan_token();
                }
                self.make_token(TokenType::Slash, "/".to_string())
            }
            '*' => self.make_token(TokenType::Star, "*".to_string()),
            '?' => self.make_token(TokenType::Question, "?".to_string()),
            ':' => {
                if self.peek().is_some_and(|c| c == &':') {
                    self.advance();
                    self.make_token(TokenType::ColonColon, "::".to_string())
                } else {
                    self.make_token(TokenType::Colon, ":".to_string())
                }
            }
            '!' => {
                if self.match_char('=') {
                    if self.match_char('=') {
                        self.make_token(TokenType::BangEqualEqual, "!==".to_string())
                    } else {
                        self.make_token(TokenType::BangEqual, "!=".to_string())
                    }
                } else {
                    self.make_token(TokenType::Bang, "!".to_string())
                }
            }
            '=' => {
                if self.match_char('=') {
                    if self.match_char('=') {
                        self.make_token(TokenType::EqualEqualEqual, "===".to_string())
                    } else {
                        self.make_token(TokenType::EqualEqual, "==".to_string())
                    }
                } else if self.match_char('>') {
                    self.make_token(TokenType::FatArrow, "=>".to_string())
                } else {
                    self.make_token(TokenType::Equal, "=".to_string())
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.make_token(TokenType::GreaterEqual, ">=".to_string())
                } else {
                    self.make_token(TokenType::Greater, ">".to_string())
                }
            }
            '<' => {
                if self.match_char('<') && self.match_char('<') {
                    self.here_doc()
                } else if self.match_char('?')
                    && self.match_char('p')
                    && self.match_char('h')
                    && self.match_char('p')
                {
                    self.make_token(TokenType::PhpTag, "<?php".to_string())
                } else if self.match_char('=') {
                    self.make_token(TokenType::LessEqual, "<=".to_string())
                } else {
                    self.make_token(TokenType::Less, "<".to_string())
                }
            }
            '|' => {
                if self.match_char('|') {
                    self.make_token(TokenType::OrOperator, "||".to_string())
                } else {
                    self.make_token(TokenType::Pipe, "|".to_string())
                }
            }
            '&' => {
                if self.match_char('&') {
                    self.make_token(TokenType::AndOperator, "&&".to_string())
                } else {
                    self.make_token(TokenType::Reference, "&".to_string())
                }
            }
            '%' => self.make_token(TokenType::Modulo, "%".to_string()),
            '"' => self.string('"'),
            '\'' => self.string('\''),
            '0'..='9' => self.number(),
            '@' => self.make_token(TokenType::AtSign, "@".to_string()),
            '_' | 'a'..='z' | 'A'..='Z' | '$' | '\\' => self.identifier(char),
            _ => {
                println!(
                    "{char} line {}, peek next 2: {}{}",
                    self.line,
                    self.peek().unwrap(),
                    self.peek_next().unwrap()
                );
                todo!();
            }
        };

        Some(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string() {
        let code = " \
            'string';
            \"String with a 'string' inside\";
        "
        .chars()
        .collect::<Vec<_>>();
        let tokens: Vec<Token> = Tokenizer::new(&code).collect();

        assert_eq!(
            vec![
                Token::new(TokenType::String, 1, "string".to_string()),
                Token::new(TokenType::Semicolon, 1, ";".to_string()),
                Token::new(
                    TokenType::String,
                    2,
                    "String with a 'string' inside".to_string()
                ),
                Token::new(TokenType::Semicolon, 2, ";".to_string()),
            ],
            tokens
        );
    }
}
