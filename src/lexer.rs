use std::collections::HashMap;

pub struct Lexer {
    chars: Vec<char>,
    line_idx: usize,
    col_idx: usize,
    read_idx: usize,
    pub has_error: bool,
    pub errors: Vec<LexerError>,
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        Lexer {
            chars: source.chars().collect(),
            line_idx: 0,
            col_idx: 0,
            read_idx: 0,
            has_error: false,
            errors: vec![],
        }
    }

    pub fn scan(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        loop {
            if self.peek_next().is_none() {
                break;
            }

            match self.peek() {
                // Single-character tokens.
                ' ' => {
                    // ignore whitespace
                    self.consume();
                }
                '\n' => {
                    self.line_idx += 1;
                    self.col_idx = 0;
                    self.consume()
                }
                '\0' => {
                    tokens.push(Token {
                        lexeme: self.peek().to_string(),
                        kind: TokenKind::Eof,
                    });
                    self.consume();
                }
                ';' => {
                    tokens.push(Token {
                        lexeme: self.peek().to_string(),
                        kind: TokenKind::Semicolon,
                    });
                    self.consume();
                }
                '+' => {
                    tokens.push(Token {
                        lexeme: self.peek().to_string(),
                        kind: TokenKind::Plus,
                    });
                    self.consume();
                }
                '-' => {
                    tokens.push(Token {
                        lexeme: self.peek().to_string(),
                        kind: TokenKind::Minus,
                    });
                    self.consume();
                }
                '*' => {
                    tokens.push(Token {
                        lexeme: self.peek().to_string(),
                        kind: TokenKind::Star,
                    });
                    self.consume();
                }
                ',' => {
                    tokens.push(Token {
                        lexeme: self.peek().to_string(),
                        kind: TokenKind::Comma,
                    });
                    self.consume();
                }
                '.' => {
                    tokens.push(Token {
                        lexeme: self.peek().to_string(),
                        kind: TokenKind::Dot,
                    });
                    self.consume();
                }
                '(' => {
                    tokens.push(Token {
                        lexeme: self.peek().to_string(),
                        kind: TokenKind::LeftParen,
                    });
                    self.consume();
                }
                ')' => {
                    tokens.push(Token {
                        lexeme: self.peek().to_string(),
                        kind: TokenKind::RightParen,
                    });
                    self.consume();
                }
                '{' => {
                    tokens.push(Token {
                        lexeme: self.peek().to_string(),
                        kind: TokenKind::LeftBrace,
                    });
                    self.consume();
                }
                '}' => {
                    tokens.push(Token {
                        lexeme: self.peek().to_string(),
                        kind: TokenKind::RightBrace,
                    });
                    self.consume();
                }
                '/' => {
                    self.consume(); // Consume the first '/'
                    if self.peek() == '/' {
                        // Single-line comment
                        while self.peek() != '\n' && self.peek() != '\0' {
                            self.consume();
                        }
                        self.line_idx += 1;
                        self.col_idx = 0;
                    } else {
                        // Not a comment, so it's a slash token
                        tokens.push(Token {
                            lexeme: '/'.to_string(),
                            kind: TokenKind::Slash,
                        });
                    }
                }

                // Single or multi-character tokens
                '!' => {
                    if self.peek() == '=' {
                        self.consume();
                        tokens.push(Token { lexeme: "!=".to_string(), kind: TokenKind::BangEqual })
                    } else {
                        self.consume();
                        tokens.push(Token { lexeme: "!".to_string(), kind: TokenKind::Bang })
                    }
                }
                '=' => {
                    self.consume();
                    if self.peek() == '=' {
                        self.consume();
                        tokens.push(Token { lexeme: "==".to_string(), kind: TokenKind::EqualEqual })
                    } else if self.peek() == '>' {
                        self.consume();
                        tokens.push(Token { lexeme: "=>".to_string(), kind: TokenKind::GreaterEqual })
                    } else if self.peek() == '<' {
                        self.consume();
                        tokens.push(Token { lexeme: "=<".to_string(), kind: TokenKind::LessEqual })
                    } else {
                        self.consume();
                        tokens.push(Token { lexeme: "=".to_string(), kind: TokenKind::Equal })
                    }
                }
                '<' => {
                    self.consume();
                    tokens.push(Token { lexeme: "<".to_string(), kind: TokenKind::Less })
                }
                '>' => {
                    self.consume();
                    tokens.push(Token { lexeme: "<".to_string(), kind: TokenKind::Greater })
                }
                // String literal
                '"' => {
                    self.consume_string_literal(&mut tokens);
                }

                // Identifiers, keywords and unknown tokens
                _ => {
                    if self.peek().is_ascii_alphabetic() || self.peek() == '_' {
                        self.consume_keyword_or_literal(&mut tokens);
                    } else if self.peek().is_ascii_digit() {
                        self.consume_number(&mut tokens);
                    } else {
                        self.has_error = true;
                        self.errors.push(LexerError {
                            err_msg: utils::generate_error_msg(self.line_idx, self.col_idx, LexerErrorKind::UnknownToken, self.peek()),
                            kind: LexerErrorKind::UnknownToken,
                        });
                        self.consume();
                    }
                }
            }
        }

        tokens
    }

    fn consume_string_literal(&mut self, tokens: &mut Vec<Token>) {
        self.consume();
        let mut buffer = String::new();
        while self.peek() != '"' {
            buffer.push(self.peek());
            self.consume();

            if self.peek() == '\0' {
                self.errors.push(LexerError {
                    err_msg: utils::generate_error_msg(self.line_idx, self.col_idx, LexerErrorKind::UnterminatedString, '\0'),
                    kind: LexerErrorKind::UnterminatedString,
                });
                self.consume();
                break;
            } else if self.peek() == '\n' {
                self.consume();
                self.line_idx += 1;
            }
        }
        self.consume();

        tokens.push(Token { lexeme: buffer, kind: TokenKind::String })
    }

    fn consume_number(&mut self, tokens: &mut Vec<Token>) {
        let mut buffer = String::from(self.peek());
        self.consume();
        while self.peek().is_ascii_digit() {
            buffer.push(self.peek());
            self.consume();
        }
        // Check for a decimal point
        if self.peek() == '.' {
            // Ensure the dot is not the last character
            if self.peek_next().is_some() && self.peek_next().unwrap().is_ascii_digit() {
                buffer.push(self.peek());
                self.consume();
                // Consume digits after the dot
                while self.peek().is_ascii_digit() {
                    buffer.push(self.peek());
                    self.consume();
                }
            }
        }
        tokens.push(Token { lexeme: buffer, kind: TokenKind::Number });
    }

    fn consume_keyword_or_literal(&mut self, tokens: &mut Vec<Token>) {
        let mut buffer = String::from(self.peek());
        self.consume();
        while self.peek() != ' ' && self.peek() != '\n' && self.peek() != '\0' {
            if !self.peek().is_ascii_alphabetic() && !self.peek().is_ascii_digit() && self.peek() != '_' {
                break;
            }
            buffer.push(self.peek());
            self.consume();
        }
        tokens.push(utils::match_literal_or_keyword(buffer));
    }


    fn consume(&mut self) {
        self.read_idx += 1;
        self.col_idx += 1;
    }

    fn peek(&self) -> char {
        self.chars[self.read_idx]
    }

    fn peek_next(&self) -> Option<char> {
        if self.read_idx + 1 < self.chars.len() {
            Some(self.chars[self.read_idx])
        } else {
            None
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct LexerError {
    pub err_msg: String,
    kind: LexerErrorKind,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum LexerErrorKind {
    UnknownToken,
    UnterminatedString,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Token {
    pub lexeme: String,
    pub kind: TokenKind,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenKind {
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
    use super::*;

    pub fn generate_error_msg(line: usize, column: usize, kind: LexerErrorKind, token: char) -> String {
        let mut error_map: HashMap<LexerErrorKind, &str> = HashMap::new();

        let binding = format!("Unknown token found {}", token);
        error_map.insert(LexerErrorKind::UnknownToken, &binding);
        error_map.insert(LexerErrorKind::UnterminatedString, "Unterminated string");

        let msg = error_map.get(&kind);

        match msg {
            None => { unimplemented!() }
            Some(msg) => {
                format!("[{}:{}] {}", line, column, msg)
            }
        }
    }

    pub fn match_literal_or_keyword(lexeme: String) -> Token {
        let mut keywords_map: HashMap<String, TokenKind> = HashMap::new();

        // Populate the keywords_map with all the keywords and their corresponding TokenKind values
        keywords_map.insert(String::from("and"), TokenKind::And);
        keywords_map.insert(String::from("class"), TokenKind::Class);
        keywords_map.insert(String::from("else"), TokenKind::Else);
        keywords_map.insert(String::from("false"), TokenKind::False);
        keywords_map.insert(String::from("fun"), TokenKind::Fun);
        keywords_map.insert(String::from("for"), TokenKind::For);
        keywords_map.insert(String::from("if"), TokenKind::If);
        keywords_map.insert(String::from("nil"), TokenKind::Nil);
        keywords_map.insert(String::from("or"), TokenKind::Or);
        keywords_map.insert(String::from("print"), TokenKind::Print);
        keywords_map.insert(String::from("return"), TokenKind::Return);
        keywords_map.insert(String::from("super"), TokenKind::Super);
        keywords_map.insert(String::from("this"), TokenKind::This);
        keywords_map.insert(String::from("true"), TokenKind::True);
        keywords_map.insert(String::from("var"), TokenKind::Var);
        keywords_map.insert(String::from("while"), TokenKind::While);

        let found = keywords_map.get(&lexeme);

        match found {
            None => {
                Token {
                    lexeme,
                    kind: TokenKind::Identifier,
                }
            }
            Some(kind) => {
                Token {
                    lexeme,
                    kind: *kind,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whitespace_and_comments() {
        let source = "var x = 10; // This is a comment\0";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.scan();

        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[4].kind, TokenKind::Semicolon);
    }

    #[test]
    fn test_unknown_token_error() {
        let source = "var x = 10; @\0";
        let mut lexer = Lexer::new(source.to_string());
        let _tokens = lexer.scan();

        assert!(lexer.has_error);
        assert_eq!(lexer.errors[0].kind, LexerErrorKind::UnknownToken);
        assert_eq!(lexer.errors[0].err_msg, "[0:12] Unknown token found @");
    }

    #[test]
    fn test_literals() {
        let source = "var name = \"John Doe\"; var age = 30;\0";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.scan();

        assert_eq!(tokens.len(), 10);
        assert_eq!(tokens[3].kind, TokenKind::String);
        assert_eq!(tokens[8].kind, TokenKind::Number);
    }

    #[test]
    fn test_keywords_and_identifiers() {
        let source = "var myVar = true;\0";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.scan();

        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0].kind, TokenKind::Var);
        assert_eq!(tokens[1].kind, TokenKind::Identifier);
        assert_eq!(tokens[3].kind, TokenKind::True);
    }
}
