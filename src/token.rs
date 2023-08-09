pub enum TokenType {
    // Single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Start,
    Question,
    Colon,

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
    For, // Care
    Foreach, // Care
    If, // Care
    Else, // Care
    Elseif, // Care
    Var,
    While, // Care
    Match, // Care
    Switch, // Care
    Case, // Care
    Break,
    Try, // Care
    Catch, // Care
    Finally, // Care
    Const,
    Return,
    Throw, // Care
    New,
    Clone,
    Die,
    Empty,
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
