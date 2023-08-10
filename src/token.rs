use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut map = HashMap::new();
        map.insert("and", TokenType::And);
        map.insert("or", TokenType::Or);
        map.insert("xor", TokenType::Xor);
        map.insert("class", TokenType::Class);
        map.insert("else", TokenType::Else);
        map.insert("false", TokenType::False);
        map.insert("function", TokenType::Function);
        map.insert("for", TokenType::For);
        map.insert("foreach", TokenType::Foreach);
        map.insert("if", TokenType::If);
        map.insert("elseif", TokenType::Elseif);
        map.insert("while", TokenType::While);
        map.insert("match", TokenType::Match);
        map.insert("switch", TokenType::Switch);
        map.insert("declare", TokenType::Declare);
        map.insert("strict_types", TokenType::StrictTypes);
        map.insert("namespace", TokenType::Namespace);
        map.insert("class", TokenType::Class);
        map.insert("case", TokenType::Case);
        map.insert("default", TokenType::Default);
        map
    };
}

pub fn match_keyword(keyword: &str) -> Option<TokenType> {
    KEYWORDS.get(keyword).copied()
}

#[derive(Debug, PartialEq, Hash, Eq, Copy, Clone)]
pub enum TokenType {
    PhpTag,

    // Single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Question,
    Colon,
    Pipe,
    BitwiseAnd,
    Hash,
    Reference,
    Modulo,
    AtSign,

    // One or two... or three... character tokens
    Bang,
    BangEqual,
    BangEqualEqual,
    Equal,
    EqualEqual,
    EqualEqualEqual, // Why!!!
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    BackSlash,
    OrOperator,
    AndOperator,
    HereDoc,
    FatArrow,

    // Literals
    Identifier,
    String,
    Integer,
    Number, // Because I don't care about int/float for calculating complexity. Hope I don't regret

    // Keywords
    And,
    Or,
    Xor,
    Class,
    Else, // Care
    False,
    Function, // Care
    For,      // Care
    Foreach,  // Care
    If,       // Care
    Elseif,   // Care
    While,    // Care
    Match,    // Care
    Switch,   // Care
    Case,     // Care
    Break,
    Try,     // Care
    Catch,   // Care
    Finally, // Care
    Const,
    Return,
    Throw, // Care
    New,
    Clone,
    EndSwitch,
    Final,
    Include,
    Readonly,
    Use,
    Yield,
    YieldFrom, // Ugh
    Abstract,
    Callable,
    Do, // Care
    Declare,
    EndDeclare,
    EndWhile,
    Global,
    IncludeOnce,
    Continue,
    Echo,
    EndFor,
    Fn,
    Goto,
    InstanceOf,
    Private,
    Trait,
    EndForeach,
    InsteadOf,
    Protected,
    As,
    Default,
    Extends,
    Implements,
    Interface,
    Static,
    StrictTypes,
    Namespace,

    // Function keywords
    Die,
    Empty,
    Isset,
    List,
    Print,
    Eval,
    Array,
    Exit,
    Unset,
    Require,
    RequireOnce,

    Eof,
}
