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

        while let Some(character) = self.peek() {
            if self.peek_next().is_none() { break; }

            match character {
                ' ' | '\0' => { self.consume(); }
                '\n' => {
                    self.line_idx += 1;
                    self.col_idx = 0;
                    self.consume()
                }
                ';' => { tokens.push(self.consume_token(Token::Semicolon)); }
                '+' => { tokens.push(self.consume_token(Token::Plus)); }
                '-' => { tokens.push(self.consume_token(Token::Minus)); }
                '*' => { tokens.push(self.consume_token(Token::Star)); }
                ',' => { tokens.push(self.consume_token(Token::Comma)); }
                '.' => { tokens.push(self.consume_token(Token::Dot)); }
                '(' => { tokens.push(self.consume_token(Token::LeftParen)); }
                ')' => { tokens.push(self.consume_token(Token::RightParen)); }
                '{' => { tokens.push(self.consume_token(Token::LeftBrace)); }
                '}' => { tokens.push(self.consume_token(Token::RightBrace)); }
                '/' => { self.handle_slash(&mut tokens); }
                '!' => self.handle_one_or_two_character_tokens(&mut tokens, Token::Bang, Token::BangEqual),
                '=' => self.handle_one_or_two_character_tokens(&mut tokens, Token::Equal, Token::EqualEqual),
                '<' => self.handle_one_or_two_character_tokens(&mut tokens, Token::Less, Token::LessEqual),
                '>' => self.handle_one_or_two_character_tokens(&mut tokens, Token::Greater, Token::GreaterEqual),
                '"' => { self.consume_string_literal(&mut tokens); }
                '0'..='9' => { self.consume_number(&mut tokens); }
                'a'..='z' | 'A'..='Z' | '_' => { self.consume_keyword_or_literal(&mut tokens) }
                _ => {
                    self.has_error = true;
                    self.errors.push(self.generate_error_msg(LexerError::UnknownToken),
                    );
                    self.consume();
                }
            }
        }

        tokens.push(Token::Eof);

        tokens
    }

    fn handle_slash(&mut self, tokens: &mut Vec<Token>) {
        self.consume(); // Consume the first '/'
        if self.peek() == Some(&'/') {
            self.consume_single_line_comment();
        } else if self.peek() == Some(&'*') {
            self.consume_multi_line_comment();
        } else {
            // Not a comment, so it's a slash token
            tokens.push(self.consume_token(Token::Slash));
        }
    }

    fn consume_multi_line_comment(&mut self) {
        self.consume(); // Consume the '*'
        loop {
            match self.peek() {
                Some(&'*') => {
                    self.consume();
                    if self.peek() == Some(&'/') {
                        self.consume();
                        break;
                    }
                }
                Some(&'\n') => {
                    self.line_idx += 1;
                    self.consume();
                }
                Some(&'\0') => {
                    self.has_error = true;
                    self.errors.push(self.generate_error_msg(LexerError::UnterminatedMultilineComment));
                    break;
                }
                _ => self.consume(),
            }
        }
    }

    fn consume_single_line_comment(&mut self) {
        // Single-line comment
        while self.peek() != Some(&'\n') && self.peek() != Some(&'\0') {
            self.consume();
        }
        self.line_idx += 1;
        self.col_idx = 0;
    }

    fn handle_one_or_two_character_tokens(&mut self, tokens: &mut Vec<Token>, token_if_single: Token, token_if_double: Token) {
        self.consume();
        if self.peek() == Some(&'=') {
            tokens.push(self.consume_token(token_if_double));
        } else {
            tokens.push(self.consume_token(token_if_single));
        }
    }

    fn consume_string_literal(&mut self, tokens: &mut Vec<Token>) {
        self.consume();
        let mut buffer = String::new();
        while self.peek() != Some(&'"') {
            if let Some(c) = self.peek() {
                buffer.push(*c);
                self.consume();

                if self.peek() == Some(&'\0') {
                    self.has_error = true;
                    self.errors.push(self.generate_error_msg(LexerError::UnterminatedString));
                    self.consume();
                    break;
                } else if self.peek() == Some(&'\n') {
                    self.consume();
                    self.line_idx += 1;
                }
            } else {
                break;
            }
        }
        self.consume();

        tokens.push(Token::String(buffer))
    }

    fn consume_number(&mut self, tokens: &mut Vec<Token>) {
        let mut buffer = String::new();
        if let Some(c) = self.peek() {
            buffer.push(*c);
            self.consume();
        }
        while self.peek().map_or(false, |c| c.is_ascii_digit()) {
            if let Some(c) = self.peek() {
                buffer.push(*c);
                self.consume();
            }
        }
        // Check for a decimal point
        if self.peek() == Some(&'.') {
            // Ensure the dot is not the last character
            if self.peek_next().map_or(false, |c| c.is_ascii_digit()) {
                buffer.push(*self.peek().unwrap());
                self.consume();
                // Consume digits after the dot
                while self.peek().map_or(false, |c| c.is_ascii_digit()) {
                    buffer.push(*self.peek().unwrap());
                    self.consume();
                }
            }
        }
        tokens.push(Token::Number(buffer));
    }

    fn consume_keyword_or_literal(&mut self, tokens: &mut Vec<Token>) {
        let mut buffer = String::new();
        if let Some(c) = self.peek() {
            buffer.push(*c);
            self.consume();
        }
        while self.peek().map_or(false, |c| c.is_ascii_alphabetic() || c.is_ascii_digit() || *c == '_') {
            if let Some(c) = self.peek() {
                buffer.push(*c);
                self.consume();
            }
        }
        tokens.push(self.match_identifier_or_keyword(buffer));
    }

    fn generate_error_msg(&self, kind: LexerError) -> String {
        let mut error_map: HashMap<LexerError, String> = HashMap::new();

        let token_char = match self.peek() {
            Some(c) => c.to_string(),
            None => String::from("EOF"), // Assuming "EOF" for end of file
        };

        let binding = format!("Unknown token found {}", token_char);
        error_map.insert(LexerError::UnknownToken, binding);
        error_map.insert(LexerError::UnterminatedString, "Unterminated string".to_string());
        error_map.insert(LexerError::UnterminatedMultilineComment, "Unterminated multiline comment".to_string());

        let msg = error_map.get(&kind);

        match msg {
            None => {
                // This should not happen since all possible LexerError variants are covered
                "Internal error: Unknown lexer error type".to_string()
            }
            Some(msg) => {
                format!("[{}:{}] {}", self.line_idx, self.col_idx, msg)
            }
        }
    }

    fn match_identifier_or_keyword(&self, lexeme: String) -> Token {
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

    fn consume_token(&mut self, token: Token) -> Token {
        self.consume();
        token
    }

    fn peek(&self) -> Option<&char> { self.chars.get(self.read_idx) }

    fn peek_next(&self) -> Option<&char> { self.chars.get(self.read_idx + 1) }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum LexerError {
    UnknownToken,
    UnterminatedString,
    UnterminatedMultilineComment,
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

        assert_eq!(tokens.len(), 6);
        assert_eq!(tokens[4], Token::Semicolon);
    }

    #[test]
    fn test_unknown_token_error() {
        let source = "var x = 10; @\0";
        let mut lexer = Lexer::new(source.to_string());
        let _tokens = lexer.scan();

        assert!(lexer.has_error);
        assert_eq!(lexer.errors[0], "[0:13] Unknown token found @");
    }

    #[test]
    fn test_literals() {
        let source = "var name = \"John Doe\"; var age = 30;\0";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.scan();

        assert_eq!(tokens.len(), 11);
        assert_eq!(tokens[3], Token::String(String::from("John Doe")));
        assert_eq!(tokens[8], Token::Number(String::from("30")));
    }

    #[test]
    fn test_keywords_and_identifiers() {
        let source = "var myVar = true;\0";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.scan();

        assert_eq!(tokens.len(), 6);
        assert_eq!(tokens[0], Token::Var);
        assert_eq!(tokens[1], Token::Identifier(String::from("myVar")));
        assert_eq!(tokens[3], Token::True);
    }

    #[test]
    fn test_multi_character_tokens() {
        let source = "!= == >= <= < >\0";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.scan();

        assert_eq!(tokens.len(), 7);
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
        assert_eq!(lexer.errors[0], "[0:22] Unterminated string");
    }

    #[test]
    fn test_number_with_decimal() {
        let source = "var pi = 3.14159;\0";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.scan();

        assert_eq!(tokens.len(), 6);
        assert_eq!(tokens[3], Token::Number(String::from("3.14159")));
    }

    #[test]
    fn test_empty_string() {
        let source = "var empty = \"\";\0";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.scan();

        assert_eq!(tokens.len(), 6);
        assert_eq!(tokens[3], Token::String(String::from("")));
    }

    #[test]
    fn test_multiple_statements() {
        let source = "var x = 10; var y = 20; var z = 30;\0";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.scan();

        assert_eq!(tokens.len(), 16);
        assert_eq!(tokens[4], Token::Semicolon);
        assert_eq!(tokens[9], Token::Semicolon);
        assert_eq!(tokens[14], Token::Semicolon);
    }

    #[test]
    fn test_newline_handling() {
        let source = "var x = 10;\nvar y = 20;\nvar z = 30;\0";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.scan();

        assert_eq!(tokens.len(), 16);
        assert_eq!(tokens[4], Token::Semicolon);
        assert_eq!(tokens[9], Token::Semicolon);
        assert_eq!(tokens[14], Token::Semicolon);
    }

    #[test]
    fn test_complex_expression() {
        let source = "var result = (10 + 20) * 3 / 2;\0";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.scan();

        assert_eq!(tokens.len(), 14);
        assert_eq!(tokens[4], Token::Number(String::from("10")));
        assert_eq!(tokens[6], Token::Number(String::from("20")));
        assert_eq!(tokens[9], Token::Number(String::from("3")));
        assert_eq!(tokens[11], Token::Number(String::from("2")));
        assert_eq!(tokens[12], Token::Semicolon);
    }

    #[test]
    fn test_multiline_comment() {
        let source = "/* This is a\nmultiline comment in lox*/\0";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.scan();

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Eof);
    }

    #[test]
    fn test_unterminated_multiline_comment() {
        let source = "/* This is a\nmultiline comment in lox\0";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.scan();

        assert!(lexer.has_error);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Eof);
    }
}
