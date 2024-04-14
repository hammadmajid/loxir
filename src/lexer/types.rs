pub struct Lexer {
    pub(crate) chars: Vec<char>,
    pub(crate) line_idx: usize,
    pub(crate) col_idx: usize,
    pub(crate) read_idx: usize,
    pub has_error: bool,
    pub errors: Vec<String>,
}

#[derive(Debug, PartialEq)]
#[derive(Clone)]
pub enum Token {
    // Single-character tokens.
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
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier(String),
    String(String),
    Number(String),

    // Keywords.
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

    Eof,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum LexerError {
    UnknownToken,
    UnterminatedString,
    UnterminatedMultilineComment,
}
