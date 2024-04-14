#[cfg(test)]
mod tests {
    use crate::lexer::Token;
    use crate::Lexer;

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
