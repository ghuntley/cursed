use cursed::lexer::{Lexer, Token};


#[test]
fn test_generic_syntax_lexing() {
    let input = "be_like Stack[T] squad { items []T; }";
    let mut lexer = Lexer::new(input);

    // Expected token sequence for generic type declaration
    let expected_tokens = vec![
        Token::BeLike,                     // "be_like"
        Token::Identifier("Stack".into()), // "Stack"
        Token::LBracket,                   // "["
        Token::Identifier("T".into()),     // "T"
        Token::RBracket,                   // "]"
        Token::Squad,                      // "squad"
        Token::LBrace,                     // "{"
        Token::Identifier("items".into()), // "items"
        Token::LBracket,                   // "["
        Token::RBracket,                   // "]"
        Token::Identifier("T".into()),     // "T"
        Token::Semicolon,                  // ";"
        Token::RBrace,                     // "}"
        Token::Eof,                        // End of file
    ];

    // Test tokens one by one
    for expected in expected_tokens {
        let token = lexer.next_token().unwrap();
        assert_eq!(token, expected, "Expected {:?}, got {:?}", expected, token);
    }

    // Test another generic example with multiple type parameters
    let input = "be_like Pair[A, B] squad { first A; second B; }";
    let mut lexer = Lexer::new(input);

    // Expected token sequence for a generic type with multiple parameters
    let expected_tokens = vec![
        Token::BeLike,                      // "be_like"
        Token::Identifier("Pair".into()),   // "Pair"
        Token::LBracket,                    // "["
        Token::Identifier("A".into()),      // "A"
        Token::Comma,                       // ","
        Token::Identifier("B".into()),      // "B"
        Token::RBracket,                    // "]"
        Token::Squad,                       // "squad"
        Token::LBrace,                      // "{"
        Token::Identifier("first".into()),  // "first"
        Token::Identifier("A".into()),      // "A"
        Token::Semicolon,                   // ";"
        Token::Identifier("second".into()), // "second"
        Token::Identifier("B".into()),      // "B"
        Token::Semicolon,                   // ";"
        Token::RBrace,                      // "}"
        Token::Eof,                         // End of file
    ];

    // Test tokens one by one
    for expected in expected_tokens {
        let token = lexer.next_token().unwrap();
        assert_eq!(token, expected, "Expected {:?}, got {:?}", expected, token);
    }

    // Test generic function syntax
    let input = "slay map[K, V](key K) V { yolo values[key]; }";
    let mut lexer = Lexer::new(input);

    // Expected token sequence for a generic function
    let expected_tokens = vec![
        Token::Slay,                        // "slay"
        Token::Identifier("map".into()),    // "map"
        Token::LBracket,                    // "["
        Token::Identifier("K".into()),      // "K"
        Token::Comma,                       // ","
        Token::Identifier("V".into()),      // "V"
        Token::RBracket,                    // "]"
        Token::LParen,                      // "("
        Token::Identifier("key".into()),    // "key"
        Token::Identifier("K".into()),      // "K"
        Token::RParen,                      // ")"
        Token::Identifier("V".into()),      // "V"
        Token::LBrace,                      // "{"
        Token::Yolo,                        // "yolo"
        Token::Identifier("values".into()), // "values"
        Token::LBracket,                    // "["
        Token::Identifier("key".into()),    // "key"
        Token::RBracket,                    // "]"
        Token::Semicolon,                   // ";"
        Token::RBrace,                      // "}"
        Token::Eof,                         // End of file
    ];

    // Test tokens one by one
    for expected in expected_tokens {
        let token = lexer.next_token().unwrap();
        assert_eq!(token, expected, "Expected {:?}, got {:?}", expected, token);
    }
}
