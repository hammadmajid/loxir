use crate::lexer::LexerError;
use crate::lexer::Token;
use crate::Lexer;

use std::collections::HashMap;

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