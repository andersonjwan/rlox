pub enum TokenKind {
    // single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    ForwardSlash,
    Star,

    // one or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // literals
    Identifier,
    String,
    Number,

    // keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    EndOfFile,
}

pub struct Position(pub usize, pub usize);

pub struct Token {
    kind: TokenKind,
    lexeme: Option<String>,
    pos: Position,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: Option<String>, pos: Position) -> Token {
        Token { kind, lexeme, pos }
    }
}
