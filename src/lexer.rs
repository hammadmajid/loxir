#[derive(Debug)]
#[derive(PartialEq)]
pub struct Token {
    pub lexeme: String,
    pub kind: TokenKind,
}

#[derive(Debug, PartialEq)]
enum TokenKind {
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
    Identifier,
    String,
    Number,

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

mod utils {
    pub fn generate_error_msg(line: usize, column: usize) -> String {
        "[".to_string() + &line.to_string() + ":" + &column.to_string() + "]"
    }
}
