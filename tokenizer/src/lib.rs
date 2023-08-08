#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
}

impl Token {
    fn new(token_type: TokenType) -> Self {
        Self { token_type }
    }
}

#[derive(Debug, PartialEq, Hash, Eq)]
pub enum TokenType {
    Match,
    If,
    Else,
    Condition,
    Logical,
    While,
    Switch,
    Equality,
}

pub struct Tokenizer<'a> {
    pub code: &'a [char],
}

impl<'a> Tokenizer<'a> {
    pub fn new(code: &'a [char]) -> Self {
        Self { code }
    }

    fn trim_whitespace(&mut self) {
        while self.code.len() > 0 && self.code[0].is_whitespace() {
            self.code = &self.code[1..];
        }
    }

    fn next_lexeme(&mut self) -> Option<String> {
        self.trim_whitespace();
        if self.code.len() == 0 {
            return None;
        }

        let mut counter = 0;

        while self
            .code
            .get(counter)
            .is_some_and(|char| !char.is_whitespace())
        {
            counter += 1;
        }

        let lexeme = &self.code[..counter];
        self.code = &self.code[counter..];

        Some(lexeme.iter().collect::<String>())
    }

    fn next_token(&mut self) -> Option<Token> {
        let mut token = None;

        while token.is_none() {
            let lexeme = match self.next_lexeme() {
                Some(lexeme) => lexeme,
                None => return None,
            };
            token = match lexeme.as_str() {
                "if" => Some(Token::new(TokenType::If)),
                "else" => Some(Token::new(TokenType::Else)),
                "&&" | "||" => Some(Token::new(TokenType::Logical)),
                "while" => Some(Token::new(TokenType::While)),
                "switch" => Some(Token::new(TokenType::Switch)),
                "===" | "==" | "!==" | "!=" | ">" | "<" | ">=" | "<=" => {
                    Some(Token::new(TokenType::Equality))
                }
                _ => None,
            };
        }

        token
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_string() {
        let code = " \
            if ($something) {
                echo 'Hello, 'World!\n';
            } else { \
                echo 'Bye, bye, World!\n';
            }
            
            if (true && (false || true)) {
                echo 'True!\n';
            } else {
                echo 'False!\n';
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
            "
        .chars()
        .collect::<Vec<_>>();
        let mut tokenizer = Tokenizer::new(&code);

        assert_eq!(Token::new(TokenType::If), tokenizer.next().unwrap());
        assert_eq!(Token::new(TokenType::Else), tokenizer.next().unwrap());
        assert_eq!(Token::new(TokenType::If), tokenizer.next().unwrap());
        assert_eq!(Token::new(TokenType::Logical), tokenizer.next().unwrap());
        assert_eq!(Token::new(TokenType::Logical), tokenizer.next().unwrap());
        assert_eq!(Token::new(TokenType::Else), tokenizer.next().unwrap());
        assert_eq!(Token::new(TokenType::While), tokenizer.next().unwrap());
        assert_eq!(Token::new(TokenType::Switch), tokenizer.next().unwrap());
        assert_eq!(None, tokenizer.next());
    }
}
