use std::{
    collections::VecDeque,
    fmt::{self, Display, Formatter},
    process,
};

use crate::{
    token::{match_data_type, match_keyword, Keyword, TokenType},
    tokenizer::Token,
};

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct Stmt {
    pub kind: StmtType,
    pub line: usize,
}

impl Stmt {
    fn new(kind: StmtType, line: usize) -> Self {
        Self { kind, line }
    }

    fn complexity(&self) -> usize {
        let kind = self.kind.clone();
        match kind {
            StmtType::Match { case_count } => case_count,
            StmtType::Switch { case_count, stmts } => {
                let mut sum = 0;
                sum += case_count;
                for stmt in stmts.iter() {
                    sum += stmt.complexity();
                }
                sum
            }
            _ => 1,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum StmtType {
    If,
    Elseif,
    For,
    Foreach,
    Throw,
    Catch,
    Switch { case_count: usize, stmts: Vec<Stmt> },
    Match { case_count: usize },
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Class {
    pub name: String,
    pub functions: Vec<Function>,
    pub extends: Option<String>,
    pub is_abstract: bool,
    pub dependencies: Vec<String>,
}

impl Class {
    fn new() -> Self {
        Self {
            name: String::new(),
            functions: Vec::new(),
            extends: None,
            is_abstract: false,
            dependencies: Vec::new(),
        }
    }

    fn add_fn(&mut self, function: Function) {
        if let Some(return_type) = &function.return_type {
            if return_type.chars().nth(0).unwrap().is_uppercase() {
                if !self.dependencies.contains(return_type) {
                    self.dependencies.push(return_type.to_owned());
                }
            }
        }

        self.functions.push(function);
    }

    pub fn highest_complexity_function(&self) -> usize {
        if self.functions.is_empty() {
            return 0;
        }
        let mut max = 0;
        for function in self.functions.iter() {
            max = std::cmp::max(max, function.complexity());
        }
        max
    }

    pub fn average_complexity(&self) -> f64 {
        if self.functions.is_empty() {
            return 0.0;
        }
        let mut sum = 0.0;
        let mut len = 0.0;
        self.functions
            .iter()
            .filter(|f| f.name != "__construct")
            .for_each(|f| {
                sum += f.complexity() as f64;
                len += 1.0;
            });
        if len == 0.0 {
            len += 1.0;
        }
        sum / len
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Visibility {
    Public,
    Private,
    Protected,
}

impl Display for Visibility {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Public => "public",
                Self::Private => "private",
                Self::Protected => "protected",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Function {
    pub stmts: Vec<Stmt>,
    pub name: String,
    pub params: usize,
    pub return_type: Option<String>,
    pub visibility: Visibility,
    pub is_abstract: bool,
}

impl Function {
    fn new(
        name: String,
        stmts: Vec<Stmt>,
        params: usize,
        return_type: Option<String>,
        visibility: Visibility,
        is_abstract: bool,
    ) -> Self {
        Self {
            name,
            stmts,
            params,
            return_type,
            visibility,
            is_abstract,
        }
    }

    pub fn complexity(&self) -> usize {
        let mut sum = 0;
        for stmt in self.stmts.iter() {
            sum += stmt.complexity();
        }
        sum + 1
    }
}

pub struct Parser {
    tokens: VecDeque<Token>,
    brackets: VecDeque<TokenType>,
    pub classes: Vec<Class>,
    namespace: String,
    uses: Vec<String>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            tokens: VecDeque::new(),
            brackets: VecDeque::new(),
            classes: Vec::new(),
            namespace: String::new(),
            uses: Vec::new(),
        }
    }

    fn closing_bracket(&mut self, token_type: TokenType) {
        let top = self.brackets.pop_back().unwrap_or_else(|| {
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

    fn next_token_opt(&mut self) -> Option<Token> {
        let token = self.tokens.pop_front()?;
        match token.token_type {
            TokenType::LeftParen => self.brackets.push_back(TokenType::LeftParen),
            TokenType::LeftBrace => self.brackets.push_back(TokenType::LeftBrace),
            TokenType::RightParen => self.closing_bracket(TokenType::RightParen),
            TokenType::RightBrace => self.closing_bracket(TokenType::RightBrace),
            _ => (),
        }
        Some(token)
    }

    fn next_token(&mut self) -> Token {
        self.next_token_opt().unwrap_or_else(|| {
            eprintln!("Expected token. Found none.");
            process::exit(1);
        })
    }

    fn next_matches_token_types(&self, token_types: &[TokenType]) -> bool {
        self.tokens
            .front()
            .is_some_and(|t| token_types.contains(&t.token_type))
    }

    fn next_matches_keywords(&self, keywords: &[Keyword]) -> bool {
        self.tokens
            .front()
            .is_some_and(|t| match_keyword(t).is_some_and(|kw| keywords.contains(&kw)))
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.front()
    }

    fn synchronize(&mut self) {
        while self
            .peek()
            .is_some_and(|t| t.token_type != TokenType::Semicolon)
        {
            self.next_token();
        }
        self.next_token();
    }
}

impl Parser {
    pub fn parse_file(&mut self, tokens: VecDeque<Token>) -> Class {
        let mut class = Class::new();
        self.tokens = tokens;
        self.brackets.clear();
        self.uses.clear();
        while let Some(token) = self.next_token_opt() {
            match token.token_type {
                TokenType::Identifier => {
                    if let Some(token_type) = match_keyword(&token) {
                        match token_type {
                            Keyword::Namespace => {
                                self.namespace = self.next_token().lexeme;
                                continue;
                            }
                            Keyword::Use => {
                                let name = self.next_token().lexeme;
                                if self.next_matches_keywords(&[Keyword::As]) {
                                    self.next_token();
                                    let alias = self.next_token().lexeme;
                                    let mut split: Vec<_> = name.split('\\').collect();
                                    split.pop();
                                    split.push(alias.as_str());
                                    self.uses.push(split.join("\\"));
                                }
                                self.uses.push(name);
                                continue;
                            }
                            Keyword::Abstract => {
                                class.is_abstract = true;
                                self.next_token();
                                return self.class(true);
                            }
                            Keyword::Class => {
                                // Todo
                                // 1. class name
                                // 2. properties
                                //    - private/public/protected/ NOT GIVEN (public)
                                //    - readonly?
                                //    - static?
                                //    - type
                                //    - name
                                //    ; semicolon
                                // 3. constructor -> potentially more promoted properties
                                // 4. Methods
                                return self.class(false);
                            }
                            _ => continue,
                        }
                    } else {
                        // Bit of a hack to get past things like Foo::class or $this->match() which was messing
                        // things up
                        if self.next_matches_token_types(&[
                            TokenType::ColonColon,
                            TokenType::ThinArrow,
                        ]) {
                            self.next_token();
                            self.next_token();
                        }
                        continue;
                    }
                }
                _ => continue,
            };
        }
        class
    }

    fn class(&mut self, is_abstract: bool) -> Class {
        let mut class = Class::new();
        class.is_abstract = is_abstract;
        class.name.push_str(self.namespace.as_str());
        class.name.push('\\');
        class.name.push_str(self.next_token().lexeme.as_str());
        if self.next_matches_keywords(&[Keyword::Extends]) {
            self.next_token();
            let extends = self.next_token();
            class.extends = Some(self.find_type(extends));
        }
        self.next_token();
        while !self.brackets.is_empty() {
            self.visibility(&mut class);
        }
        class
    }

    fn visibility(&mut self, class: &mut Class) {
        let token = self.next_token();
        if let Some(keyword) = match_keyword(&token) {
            match keyword {
                Keyword::Abstract => self.visibility(class),
                Keyword::Visibility(visibility) => {
                    let token = self.next_token();
                    if let Some(_) = match_keyword(&token) {
                        self.match_keyword(class, visibility, token);
                    }
                }
                Keyword::Use => (),
                _ => self.match_keyword(class, Visibility::Public, token),
            };
        }
    }

    fn match_keyword(&mut self, class: &mut Class, visibility: Visibility, token: Token) {
        let keyword = match match_keyword(&token) {
            Some(keyword) => keyword,
            None => return,
        };
        match keyword {
            Keyword::Const => self.synchronize(),
            Keyword::Function => class.add_fn(self.function(visibility)),
            Keyword::Readonly => {
                let token = self.next_token();
                match self.dependency(token) {
                    Some(dependency) => class.dependencies.push(dependency),
                    None => return,
                }
            }
            Keyword::Static => {
                let token = self.next_token();
                let keyword = match match_keyword(&token) {
                    Some(keyword) => keyword,
                    None => {
                        // it's a type
                        if match_data_type(&token).is_some() {
                            // Ignore built in data types
                            return;
                        }
                        if token.lexeme.starts_with('$') {
                            // static $foo;
                            return;
                        }
                        class.dependencies.push(self.find_type(token));
                        return;
                    }
                };
                if keyword == Keyword::Function {
                    class.add_fn(self.function(Visibility::Public));
                    return;
                }
                // ignore built in data types, not a dependency
                if match_data_type(&token).is_some() {
                    return;
                }
                class.dependencies.push(self.find_type(token));
            }
            _ => match self.dependency(token) {
                Some(dependency) => class.dependencies.push(dependency),
                None => self.synchronize(),
            },
        }
    }

    fn dependency(&mut self, token: Token) -> Option<String> {
        if token.lexeme.starts_with('$') {
            return None;
        }
        if match_data_type(&token).is_some() {
            return None;
        }
        Some(self.find_type(token))
    }

    fn function(&mut self, visibility: Visibility) -> Function {
        let name = self.next_token().lexeme;
        let mut params = 0;
        while !self.next_matches_token_types(&[TokenType::RightParen]) {
            if self.next_token().lexeme.starts_with('$') {
                params += 1;
            }
        }
        self.next_token();
        let return_type = if self.next_matches_token_types(&[TokenType::Colon]) {
            self.next_token();
            let mut return_token = self.next_token();
            // TODO handle this
            if return_token.token_type == TokenType::Question {
                return_token = self.next_token();
            }
            Some(self.find_type(return_token))
        } else {
            None
        };
        let depth = self.brackets.len();
        let token = self.next_token();
        if token.token_type == TokenType::Semicolon {
            return Function::new(name, Vec::new(), params, return_type, visibility, true);
        }
        let mut stmts = Vec::new();
        while depth != self.brackets.len() {
            if let Some(stmt) = self.parse_stmt() {
                stmts.push(stmt);
            }
        }
        Function::new(name, stmts, params, return_type, visibility, false)
    }

    fn find_type(&mut self, type_token: Token) -> String {
        if match_data_type(&type_token).is_some() {
            // Check if it's a built in data type
            return type_token.lexeme;
        }
        if type_token.lexeme.starts_with('\\') {
            return type_token.lexeme;
        }
        let mut return_type = String::new();
        for use_stmt in self.uses.iter() {
            let ending = use_stmt.split('\\').last().expect("Empty use statement");
            if type_token.lexeme.as_str() == ending {
                return_type.push_str(use_stmt.as_str());
                break;
            }
        }
        if return_type.is_empty() {
            return_type.push_str(self.namespace.as_str());
            return_type.push('\\');
            return_type.push_str(type_token.lexeme.as_str());
        }
        return_type
    }

    fn parse_stmt(&mut self) -> Option<Stmt> {
        let token = self.next_token();
        if token.token_type != TokenType::Identifier {
            // hack to get over things like $this->match()
            if vec![TokenType::ColonColon, TokenType::ThinArrow].contains(&token.token_type) {
                self.next_token();
                self.next_token();
                self.next_token();
            }
            return None;
        }
        let keyword = match match_keyword(&token) {
            Some(keyword) => keyword,
            None => return None,
        };
        let line = token.line;
        self.create_statement(keyword, line)
    }

    fn create_statement(&mut self, keyword: Keyword, line: usize) -> Option<Stmt> {
        Some(match keyword {
            Keyword::If => Stmt::new(StmtType::If, line),
            Keyword::Elseif => Stmt::new(StmtType::Elseif, line),
            Keyword::For => Stmt::new(StmtType::For, line),
            Keyword::Foreach => Stmt::new(StmtType::Foreach, line),
            Keyword::Switch => self.switch_stmt(line),
            Keyword::Match => self.match_stmt(line),
            Keyword::Throw => Stmt::new(StmtType::Throw, line),
            Keyword::Catch => Stmt::new(StmtType::Catch, line),
            _ => return None,
        })
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
                    let keyword = match match_keyword(&token) {
                        Some(keyword) => keyword,
                        None => continue,
                    };
                    stmts.push(match keyword {
                        Keyword::Case => {
                            case_count += 1;
                            continue;
                        }
                        _ => {
                            let line = token.line;
                            match self.create_statement(keyword, line) {
                                Some(stmt) => stmt,
                                None => continue,
                            }
                        }
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
