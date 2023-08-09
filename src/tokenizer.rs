use std::process;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: Keyword,
}

impl Token {
    fn new(token_type: Keyword) -> Self {
        Self { token_type }
    }
}

#[derive(Debug, PartialEq, Hash, Eq)]
pub enum Keyword {
    Match { case_count: usize },
    If,
    Elseif,
    While,
    Switch { case_count: usize },
    Function,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
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

    fn next_lexeme_opt(&mut self) -> Option<String> {
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

    fn next_lexeme(&mut self) -> String {
        self.next_lexeme_opt().unwrap_or_else(|| {
            eprintln!("ERROR: syntax error");
            process::exit(1);
        })
    }

    fn next_token(&mut self) -> Option<Token> {
        let mut token = None;

        while token.is_none() {
            let lexeme = match self.next_lexeme_opt() {
                Some(lexeme) => lexeme,
                None => return None,
            };
            token = match lexeme.as_str() {
                "if" => Some(Token::new(Keyword::If)),
                "elseif" => Some(Token::new(Keyword::Elseif)),
                "while" => Some(Token::new(Keyword::While)),
                "switch" => Some(Token::new(Keyword::Switch { case_count: 1 })),
                "match" => Some(Token::new(Keyword::Match { case_count: 1 })),
                "function" => Some(Token::new(Keyword::Function)),
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
            } else {
                echo 'Lol\n';
            }
            
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
        let mut tokenizer = Tokenizer::new(&code);

        assert_eq!(Token::new(Keyword::If), tokenizer.next().unwrap());
        assert_eq!(Token::new(Keyword::If), tokenizer.next().unwrap());
        assert_eq!(Token::new(Keyword::Elseif), tokenizer.next().unwrap());
        assert_eq!(Token::new(Keyword::While), tokenizer.next().unwrap());
        assert_eq!(
            Token::new(Keyword::Switch { case_count: 1 }),
            tokenizer.next().unwrap(),
        );
        assert_eq!(
            Token::new(Keyword::Match { case_count: 1 }),
            tokenizer.next().unwrap(),
        );
        assert_eq!(None, tokenizer.next());
    }
}
