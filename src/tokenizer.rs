use std::process;

use crate::{error, token::{TokenType, match_keyword}};

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
}

impl Token {
    fn new(token_type: TokenType, line: usize) -> Self {
        Self { token_type, line }
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
        if let Some(c) = self.code.get(0) {
            if c == &'\n' {
                self.line += 1;
            }

            self.code = &self.code[1..];
        } else {
            error("Expected token", self.line);
        }
    }

    fn next_char(&mut self) -> Option<char> {
        let char = match self.code.get(0) {
            Some(char) => char,
            None => return None,
        };
        self.advance();
        Some(*char)
    }

    fn peek(&self) -> Option<&char> {
        self.code.get(0)
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

    fn make_token(&self, token_type: TokenType) -> Token {
        Token::new(token_type, self.line)
    }

    fn variable(&mut self) -> Token {
        while self.peek().is_some_and(|c| c.is_alphanumeric() || c == &'_') {
            self.advance();
        }

        self.make_token(TokenType::Identifier)
    }
    
    fn string(&mut self, quote_type: char) -> Token {
        println!("String! {}{}{}{} line: {}", self.code[0], self.code[1], self.code[2], self.code[3], self.line);
        let mut previous = *self.peek().unwrap(); // Fix this
        while !self.code.is_empty() && !(previous != '\\' && self.peek().is_some_and(|c| c == &quote_type)) {
            previous = self.next_char().unwrap();
        }
        if self.code.is_empty() {
            error("Unterminated string", self.line);
        }
        self.advance();

        self.make_token(TokenType::String)
    }

    fn number(&mut self) -> Token {
        while self.peek().is_some_and(|c| c.is_ascii_digit()) {
            self.advance();
        }
        if self.peek().is_some_and(|c| c == &'.') {
            self.advance();
            while self.peek().is_some_and(|c| c.is_ascii_digit()) {
                self.advance();
            }
        }

        self.make_token(TokenType::Number)
    }

    fn identifier(&mut self, start: char) -> Token {
        let mut word = start.to_string();
        while self.peek().is_some_and(|c| c.is_alphanumeric() || c == &'_') {
            word.push(self.next_char().unwrap());
        }

        self.make_token(match_keyword(word.as_str()).unwrap_or_else(|| TokenType::Identifier))
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
        let char = match self.next_char() {
            Some(char) => char,
            None => return None,
        };
        let token = match char {
            ' ' | '\r' | '\t' => return self.scan_token(),
            '\n' => {
                self.line += 1;
                return self.scan_token();
            }
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '[' => self.make_token(TokenType::LeftBracket),
            ']' => self.make_token(TokenType::RightBracket),
            ',' => self.make_token(TokenType::Comma),
            '.' => self.make_token(TokenType::Dot),
            '-' => self.make_token(TokenType::Minus),
            '+' => self.make_token(TokenType::Plus),
            ';' => self.make_token(TokenType::Semicolon),
            '#' => self.make_token(TokenType::Hash),
            '/' => {
                println!("Started a slash line: {}, char: {}", self.line, self.code[0]);
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
                    println!("Second slash: {}, char: {}", self.line, self.code[0]);
                    while self.peek().is_some_and(|c| c != &'\n') {
                        println!("{}", self.code[0]);
                        self.advance();
                    }
                    self.advance();
                    println!("Finished slashes: {}, char: {} peek {} is new line: {}", self.line, self.code[0], self.code[1], self.code[0] == '\n');
                    if self.code.is_empty() {
                        return None;
                    }
                    return self.scan_token();
                }
                self.make_token(TokenType::Slash)
            }
            '\\' => self.make_token(TokenType::BackSlash),
            '*' => self.make_token(TokenType::Star),
            '?' => self.make_token(TokenType::Question),
            ':' => self.make_token(TokenType::Colon),
            '!' => {
                if self.match_char('=') {
                    if self.match_char('=') {
                        self.make_token(TokenType::BangEqualEqual)
                    } else {
                        self.make_token(TokenType::BangEqual)
                    }
                } else {
                    self.make_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.match_char('=') {
                    if self.match_char('=') {
                        self.make_token(TokenType::EqualEqualEqual)
                    } else {
                        self.make_token(TokenType::EqualEqual)
                    }
                } else {
                    self.make_token(TokenType::Equal)
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.make_token(TokenType::GreaterEqual)
                } else {
                    self.make_token(TokenType::Greater)
                }
            }
            '<' => {
                if self.match_char('?') && self.match_char('p') && self.match_char('h') && self.match_char('p') {
                    return Some(self.make_token(TokenType::PhpTag));
                }
                if self.match_char('=') {
                    self.make_token(TokenType::LessEqual)
                } else {
                    self.make_token(TokenType::Less)
                }
            }
            '|' => {
                if self.match_char('|') {
                    self.make_token(TokenType::OrOperator)
                } else {
                    self.make_token(TokenType::Pipe)
                }
            }
            '&' => {
                if self.match_char('&') {
                    self.make_token(TokenType::AndOperator)
                } else {
                    self.make_token(TokenType::Reference)
                }
            }
            '%' => self.make_token(TokenType::Modulo),
            '"' => self.string('"'),
            '\'' => self.string('\''),
            '0'..='9' => self.number(),
            '_' | 'a'..='z' | 'A'..='Z' => self.identifier(char),
            '$' => self.variable(),
            _ => {
                println!("{char} line {}, peek next 2: {}{}", self.line, self.peek().unwrap(), self.peek_next().unwrap());
                todo!();
            },
        };

        Some(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_string() {
        let code = " \
                if elseif while switch match
            "
        .chars()
        .collect::<Vec<_>>();
        let mut tokenizer = Tokenizer::new(&code);

        assert_eq!(Token::new(TokenType::If, 1), tokenizer.next().unwrap());
        assert_eq!(Token::new(TokenType::Elseif, 1), tokenizer.next().unwrap());
        assert_eq!(Token::new(TokenType::While, 1), tokenizer.next().unwrap());
        assert_eq!(Token::new(TokenType::Switch, 1), tokenizer.next().unwrap());
        assert_eq!(Token::new(TokenType::Match, 1), tokenizer.next().unwrap());
        assert_eq!(None, tokenizer.next());
    }

    #[test]
    fn test_scan_tokens_method() {
        let code = " \
            if (true && (false || true)) {
                echo 'True!\n';
            } elseif (true) {
                echo 'False!\n';
            } else {
                echo 'Else!\n';
            }

            while (true) {
                $var = 'Foo';
            }

            switch ($i) {
                case 0:
                    echo '$i = 0';
                    break;
                case 1:
                    echo '$i = 1';
                    break;
            }

            match ($var) {
                1 => 3,
                2 => 4,
            };
            "
        .chars()
        .collect::<Vec<_>>();

        let tokens: Vec<Token> = Tokenizer::new(&code).collect();

        assert_eq!(
            Vec::from([
                Token::new(TokenType::If, 1),
                Token::new(TokenType::Elseif, 1),
                Token::new(TokenType::While, 1),
                Token::new(TokenType::Switch, 1),
                Token::new(TokenType::Match, 1),
            ]),
            tokens,
        );
    }
}
