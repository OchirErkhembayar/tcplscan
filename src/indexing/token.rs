use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::indexing::{parser::Visibility, tokenizer::Token};

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, Keyword> = {
        let mut map = HashMap::new();
        map.insert("if", Keyword::If);
        map.insert("elseif", Keyword::Elseif);
        map.insert("for", Keyword::For);
        map.insert("foreach", Keyword::Foreach);
        map.insert("match", Keyword::Match);
        map.insert("switch", Keyword::Switch);
        map.insert("while", Keyword::While);
        map.insert("case", Keyword::Case);
        map.insert("namespace", Keyword::Namespace);
        map.insert("class", Keyword::Class);
        map.insert("function", Keyword::Function);
        map.insert("throw", Keyword::Throw);
        map.insert("catch", Keyword::Catch);
        map.insert("extends", Keyword::Extends);
        map.insert("abstract", Keyword::Abstract);
        map.insert("use", Keyword::Use);
        map.insert("as", Keyword::As);
        map.insert("public", Keyword::Visibility(Visibility::Public));
        map.insert("private", Keyword::Visibility(Visibility::Private));
        map.insert("protected", Keyword::Visibility(Visibility::Protected));
        map.insert("const", Keyword::Const);
        map.insert("static", Keyword::Static);
        map.insert("iterable", Keyword::Iterable);
        map.insert("string", Keyword::String);
        map.insert("array", Keyword::Array);
        map.insert("int", Keyword::Int);
        map.insert("float", Keyword::Float);
        map.insert("bool", Keyword::Bool);
        map.insert("self", Keyword::MySelf);
        map.insert("void", Keyword::Void);
        map.insert("readonly", Keyword::Readonly);
        map.insert("mixed", Keyword::Mixed);
        map.insert("true", Keyword::Bool);
        map.insert("false", Keyword::Bool);
        map.insert("implements", Keyword::Implements);
        map.insert("trait", Keyword::Trait);
        map
    };
}

lazy_static! {
    static ref BUILT_IN_DATA_TYPES: HashMap<&'static str, Keyword> = {
        let mut map = HashMap::new();
        map.insert("string", Keyword::String);
        map.insert("array", Keyword::Array);
        map.insert("int", Keyword::Int);
        map.insert("float", Keyword::Float);
        map.insert("bool", Keyword::Bool);
        map.insert("self", Keyword::MySelf);
        map.insert("void", Keyword::Void);
        map.insert("readonly", Keyword::Readonly);
        map.insert("iterable", Keyword::Iterable);
        map.insert("static", Keyword::Static);
        map.insert("mixed", Keyword::Mixed);
        map.insert("true", Keyword::Bool);
        map.insert("false", Keyword::Bool);
        map
    };
}

pub fn match_keyword(token: &Token) -> Option<Keyword> {
    if token.token_type != TokenType::Identifier {
        return None;
    }
    KEYWORDS.get(token.lexeme.as_str()).copied()
}

pub fn match_data_type(token: &Token) -> Option<Keyword> {
    if token.token_type != TokenType::Identifier {
        return None;
    }
    BUILT_IN_DATA_TYPES.get(token.lexeme.as_str()).copied()
}

#[derive(Debug, PartialEq, Hash, Eq, Copy, Clone)]
pub enum Keyword {
    If,
    Elseif,
    For,
    Foreach,
    Match,
    Switch,
    While,
    Case,
    Namespace,
    Class,
    Function,
    Throw,
    Catch,
    Extends,
    Abstract,
    Use,
    As,
    Visibility(Visibility),
    Const,
    Static,
    Iterable,
    String,
    Array,
    Int,
    Float,
    Bool,
    MySelf,
    Void,
    Readonly,
    Mixed,
    Implements,
    Trait,
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
    // BitwiseAnd,
    Hash,
    Reference,
    Modulo,
    AtSign,
    BinaryNegation,

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
    // BackSlash,
    OrOperator,
    AndOperator,
    HereDoc,
    FatArrow,
    ThinArrow,
    ColonColon,

    // Literals
    Identifier,
    String,
    // Integer,
    Number, // Because I don't care about int/float for calculating complexity. Hope I don't regret

            /*
            // Keywords
            And,
            Or,
            Xor,
            Class,
            This,
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
            */
}
