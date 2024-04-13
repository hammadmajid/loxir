use std::collections::HashMap;

pub struct Lexer {
    chars: Vec<char>,
    line_idx: usize,
    col_idx: usize,
    read_idx: usize,
    pub has_error: bool,
    pub errors: Vec<String>,
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        Lexer {
            chars: source.chars().collect(),
            line_idx: 0,
            col_idx: 1,
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
                    tokens.push(Token::Eof);
                    self.consume();
                }
                ';' => {
                    tokens.push(Token::Semicolon);
                    self.consume();
                }
                '+' => {
                    tokens.push(Token::Plus);
                    self.consume();
                }
                '-' => {
                    tokens.push(Token::Minus);
                    self.consume();
                }
                '*' => {
                    tokens.push(Token::Star);
                    self.consume();
                }
                ',' => {
                    tokens.push(Token::Comma);
                    self.consume();
                }
                '.' => {
                    tokens.push(Token::Dot);
                    self.consume();
                }
                '(' => {
                    tokens.push(Token::LeftParen);
                    self.consume();
                }
                ')' => {
                    tokens.push(Token::RightParen);
                    self.consume();
                }
                '{' => {
                    tokens.push(Token::LeftBrace);
                    self.consume();
                }
                '}' => {
                    tokens.push(Token::RightBrace);
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
                        tokens.push(Token::Slash);
                    }
                }

                // Single or multi-character tokens
                '!' => {
                    if self.peek() == '=' {
                        self.consume();
                        tokens.push(Token::BangEqual)
                    } else {
                        self.consume();
                        tokens.push(Token::Bang)
                    }
                }
                '=' => {
                    self.consume();
                    if self.peek() == '=' {
                        self.consume();
                        tokens.push(Token::EqualEqual)
                    } else if self.peek() == '>' {
                        self.consume();
                        tokens.push(Token::GreaterEqual)
                    } else if self.peek() == '<' {
                        self.consume();
                        tokens.push(Token::LessEqual)
                    } else {
                        self.consume();
                        tokens.push(Token::Equal)
                    }
                }
                '<' => {
                    self.consume();
                    tokens.push(Token::Less)
                }
                '>' => {
                    self.consume();
                    tokens.push(Token::Greater)
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
                        self.errors.push(Lexer::generate_error_msg(self.line_idx, self.col_idx, LexerError::UnknownToken, self.peek()),
                        );
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
                self.errors.push(Lexer::generate_error_msg(self.line_idx, self.col_idx, LexerError::UnterminatedString, '\0'),
                );
                self.consume();
                break;
            } else if self.peek() == '\n' {
                self.consume();
                self.line_idx += 1;
            }
        }
        self.consume();

        tokens.push(Token::String(buffer))
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
        tokens.push(Token::Number(buffer));
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
        tokens.push(Lexer::match_literal_or_keyword(buffer));
    }

    fn generate_error_msg(line: usize, column: usize, kind: LexerError, token: char) -> String {
        let mut error_map: HashMap<LexerError, &str> = HashMap::new();

        let binding = format!("Unknown token found {}", token);
        error_map.insert(LexerError::UnknownToken, &binding);
        error_map.insert(LexerError::UnterminatedString, "Unterminated string");

        let msg = error_map.get(&kind);

        match msg {
            None => { unimplemented!() }
            Some(msg) => {
                format!("[{}:{}] {}", line, column, msg)
            }
        }
    }

    fn match_literal_or_keyword(lexeme: String) -> Token {
        let mut keywords_map: HashMap<String, Token> = HashMap::new();

        // Populate the keywords_map with all the keywords and their corresponding TokenKind values
        keywords_map.insert(String::from("and"), Token::And);
        keywords_map.insert(String::from("class"), Token::Class);
        keywords_map.insert(String::from("else"), Token::Else);
        keywords_map.insert(String::from("false"), Token::False);
        keywords_map.insert(String::from("fun"), Token::Fun);
        keywords_map.insert(String::from("for"), Token::For);
        keywords_map.insert(String::from("if"), Token::If);
        keywords_map.insert(String::from("nil"), Token::Nil);
        keywords_map.insert(String::from("or"), Token::Or);
        keywords_map.insert(String::from("print"), Token::Print);
        keywords_map.insert(String::from("return"), Token::Return);
        keywords_map.insert(String::from("super"), Token::Super);
        keywords_map.insert(String::from("this"), Token::This);
        keywords_map.insert(String::from("true"), Token::True);
        keywords_map.insert(String::from("var"), Token::Var);
        keywords_map.insert(String::from("while"), Token::While);

        let found = keywords_map.get(&lexeme);

        match found {
            None => {
                Token::Identifier(lexeme)
            }
            Some(kind) => {
                kind.clone()
            }
        }
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

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum LexerError {
    UnknownToken,
    UnterminatedString,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whitespace_and_comments() {
        let source = "var x = 10; // This is a comment\0";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.scan();

        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[4], Token::Semicolon);
    }

    #[test]
    fn test_unknown_token_error() {
        let source = "var x = 10; @\0";
        let mut lexer = Lexer::new(source.to_string());
        let _tokens = lexer.scan();

        assert!(lexer.has_error);
        assert_eq!(lexer.errors[0], "[0:12] Unknown token found @");
    }

    #[test]
    fn test_literals() {
        let source = "var name = \"John Doe\"; var age = 30;\0";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.scan();

        assert_eq!(tokens.len(), 10);
        assert_eq!(tokens[3], Token::String(String::from("John Doe")));
        assert_eq!(tokens[8], Token::Number(String::from("30")));
    }

    #[test]
    fn test_keywords_and_identifiers() {
        let source = "var myVar = true;\0";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.scan();

        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0], Token::Var);
        assert_eq!(tokens[1], Token::Identifier(String::from("myVar")));
        assert_eq!(tokens[3], Token::True);
    }

    #[test]
    fn test_multi_character_tokens() {
        let source = "!= == >= <= < >\0";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.scan();

        assert_eq!(tokens.len(), 6);
        assert_eq!(tokens[0], Token::BangEqual);
        assert_eq!(tokens[1], Token::EqualEqual);
        assert_eq!(tokens[2], Token::GreaterEqual);
        assert_eq!(tokens[3], Token::LessEqual);
        assert_eq!(tokens[4], Token::Less);
        assert_eq!(tokens[5], Token::Greater);
    }

    #[test]
    fn test_unterminated_string() {
        let source = "var name = \"John Doe;\0";
        let mut lexer = Lexer::new(source.to_string());
        let _tokens = lexer.scan();

        assert!(lexer.has_error);
        assert_eq!(lexer.errors[0], "[0:14] Unterminated string");
    }

    #[test]
    fn test_number_with_decimal() {
        let source = "var pi = 3.14159;\0";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.scan();

        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[3], Token::Number(String::from("3.14159")));
    }

    #[test]
    fn test_empty_string() {
        let source = "var empty = \"\";\0";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.scan();

        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[3], Token::String(String::from("")));
    }

    #[test]
    fn test_multiple_statements() {
        let source = "var x = 10; var y = 20; var z = 30;\0";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.scan();

        assert_eq!(tokens.len(), 15);
        assert_eq!(tokens[4], Token::Semicolon);
        assert_eq!(tokens[9], Token::Semicolon);
        assert_eq!(tokens[14], Token::Semicolon);
    }

    #[test]
    fn test_newline_handling() {
        let source = "var x = 10;\nvar y = 20;\nvar z = 30;\0";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.scan();

        assert_eq!(tokens.len(), 15);
        assert_eq!(tokens[4], Token::Semicolon);
        assert_eq!(tokens[9], Token::Semicolon);
        assert_eq!(tokens[14], Token::Semicolon);
    }

    #[test]
    fn test_complex_expression() {
        let source = "var result = (10 + 20) * 3 / 2;\0";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.scan();

        assert_eq!(tokens.len(), 15);
        assert_eq!(tokens[3], Token::Number(String::from("10")));
        assert_eq!(tokens[5], Token::Number(String::from("20")));
        assert_eq!(tokens[7], Token::Number(String::from("3")));
        assert_eq!(tokens[9], Token::Number(String::from("2")));
        assert_eq!(tokens[14], Token::Semicolon);
    }
}
