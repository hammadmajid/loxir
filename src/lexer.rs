pub struct Lexer {
    chars: Vec<char>,
    line_idx: usize,
    col_idx: usize,
    pub has_error: bool,
    pub error: LexerError,
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        Lexer {
            chars: source.chars().collect(),
            line_idx: 0,
            col_idx: 0,
            has_error: false,
            error: LexerError { err_msg: "".to_string(), kind: LexerErrorKind::UnknownToken },
        }
    }

}

    fn consume(&mut self) {
        self.col_idx += 1;
    }

    fn peek(&self) -> char {
        self.chars[self.col_idx]
    }

    fn peek_next(&self) -> Option<char> {
        if self.col_idx + 1 < self.chars.len() {
            Some(self.chars[self.col_idx])
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct LexerError {
    pub err_msg: String,
    kind: LexerErrorKind,
}

#[derive(Debug, PartialEq)]
pub enum LexerErrorKind {
    UnknownToken,
    UnterminatedString,
}

// TODO: map each error kind with a error message using a hashmap

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whitespace_and_comments() {
        let source = "var x = 10; // This is a comment";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.scan().unwrap();

        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[4].kind, TokenKind::Semicolon);
    }

    #[test]
    fn test_unknown_token_error() {
        let source = "var x = 10; @";
        let mut lexer = Lexer::new(source.to_string());
        let result = lexer.scan();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind, LexerErrorKind::UnknownToken);
        assert_eq!(err.err_msg, "[1:14]");
    }

    #[test]
    fn test_literals() {
        let source = r#"var name = "John Doe"; var age = 30;"#;
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.scan().unwrap();

        assert_eq!(tokens.len(), 10);
        assert_eq!(tokens[4].kind, TokenKind::String);
        assert_eq!(tokens[9].kind, TokenKind::Number);
    }

    #[test]
    fn test_keywords_and_identifiers() {
        let source = "var myVar = true;";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.scan().unwrap();

        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0].kind, TokenKind::Var);
        assert_eq!(tokens[1].kind, TokenKind::Identifier);
        assert_eq!(tokens[3].kind, TokenKind::True);
    }
}
