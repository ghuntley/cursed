#[cfg(test)]
mod tests {
    use crate::lexer::{Lexer, Token};
    use proptest::prelude::*;
    
    #[test]
    fn test_next_token() {
        let input = r#"vibe main;

yeet "vibez";

slay main() {
    vibez.spill("Hello, World!");  fr fr This is a comment
    
    sus name tea = "bestie";
    vibez.spillf("Hey %s, what's good?", name);

    lowkey 1 < 2 {
        vibez.spill("This is based!");
    } highkey {
        vibez.spill("This is sus!");
    }
}
"#;

        let expected = vec![
            Token::Vibe,
            Token::Identifier("main".to_string()),
            Token::Semicolon,
            Token::Yeet,
            Token::String("vibez".to_string()),
            Token::Semicolon,
            Token::Slay,
            Token::Identifier("main".to_string()),
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::Identifier("vibez".to_string()),
            Token::Dot,
            Token::Identifier("spill".to_string()),
            Token::LParen,
            Token::String("Hello, World!".to_string()),
            Token::RParen,
            Token::Semicolon,
            Token::LineComment,
            Token::Sus,
            Token::Identifier("name".to_string()),
            Token::Tea,
            Token::Assign,
            Token::String("bestie".to_string()),
            Token::Semicolon,
            Token::Identifier("vibez".to_string()),
            Token::Dot,
            Token::Identifier("spillf".to_string()),
            Token::LParen,
            Token::String("Hey %s, what's good?".to_string()),
            Token::Comma,
            Token::Identifier("name".to_string()),
            Token::RParen,
            Token::Semicolon,
            Token::Lowkey,
            Token::Int(1),
            Token::Lt,
            Token::Int(2),
            Token::LBrace,
            Token::Identifier("vibez".to_string()),
            Token::Dot,
            Token::Identifier("spill".to_string()),
            Token::LParen,
            Token::String("This is based!".to_string()),
            Token::RParen,
            Token::Semicolon,
            Token::RBrace,
            Token::Highkey,
            Token::LBrace,
            Token::Identifier("vibez".to_string()),
            Token::Dot,
            Token::Identifier("spill".to_string()),
            Token::LParen,
            Token::String("This is sus!".to_string()),
            Token::RParen,
            Token::Semicolon,
            Token::RBrace,
            Token::RBrace,
            Token::Eof,
        ];

        let mut lexer = Lexer::new(input);
        
        for expected_token in expected {
            let token = lexer.next_token().unwrap();
            assert_eq!(token, expected_token);
        }
    }
    
    #[test]
    fn test_operators() {
        let input = r#"+ - * / < > == != <= >= = !
"#;

        let expected = vec![
            Token::Plus,
            Token::Minus,
            Token::Asterisk,
            Token::Slash,
            Token::Lt,
            Token::Gt,
            Token::Eq,
            Token::NotEq,
            Token::LtEq,
            Token::GtEq,
            Token::Assign,
            Token::Bang,
            Token::Eof,
        ];

        let mut lexer = Lexer::new(input);
        
        for expected_token in expected {
            let token = lexer.next_token().unwrap();
            assert_eq!(token, expected_token);
        }
    }
    
    proptest! {
        #[test]
        fn doesnt_crash_on_arbitrary_input(s in "\\PC*") {
            let mut lexer = Lexer::new(&s);
            while let Ok(token) = lexer.next_token() {
                if token == Token::Eof {
                    break;
                }
            }
        }
        
        #[test]
        fn identifiers_are_lexed_correctly(s in "[a-zA-Z_][a-zA-Z0-9_]*") {
            let mut lexer = Lexer::new(&s);
            let token = lexer.next_token().unwrap();
            match token {
                Token::Identifier(id) => {
                    assert_eq!(id, s);
                },
                _ => match s.as_str() {
                    "vibe" => assert_eq!(token, Token::Vibe),
                    "yeet" => assert_eq!(token, Token::Yeet),
                    "slay" => assert_eq!(token, Token::Slay),
                    "sus" => assert_eq!(token, Token::Sus),
                    "facts" => assert_eq!(token, Token::Facts),
                    "lowkey" => assert_eq!(token, Token::Lowkey),
                    "highkey" => assert_eq!(token, Token::Highkey),
                    "bestie" => assert_eq!(token, Token::Bestie),
                    "periodt" => assert_eq!(token, Token::Periodt),
                    "vibe_check" => assert_eq!(token, Token::VibeCheck),
                    "mood" => assert_eq!(token, Token::Mood),
                    "basic" => assert_eq!(token, Token::Basic),
                    "ghosted" => assert_eq!(token, Token::Ghosted),
                    "simp" => assert_eq!(token, Token::Simp),
                    "be_like" => assert_eq!(token, Token::BeLike),
                    "squad" => assert_eq!(token, Token::Squad),
                    "collab" => assert_eq!(token, Token::Collab),
                    "tea" => assert_eq!(token, Token::Tea),
                    "dm" => assert_eq!(token, Token::Dm),
                    "stan" => assert_eq!(token, Token::Stan),
                    "flex" => assert_eq!(token, Token::Flex),
                    "later" => assert_eq!(token, Token::Later),
                    "yolo" => assert_eq!(token, Token::Yolo),
                    "based" => assert_eq!(token, Token::Based),
                    "cap" => assert_eq!(token, Token::Cap),
                    _ => panic!("Expected an identifier: {} but got {:?}", s, token),
                }
            }
        }
        
        #[test]
        fn integer_literals_are_lexed_correctly(n in 0..10000i64) {
            let s = n.to_string();
            let mut lexer = Lexer::new(&s);
            let token = lexer.next_token().unwrap();
            prop_assert_eq!(token, Token::Int(n));
        }
        
        #[test]
        fn float_literals_are_lexed_correctly(n in 0.0..10000.0) {
            let s = n.to_string();
            let mut lexer = Lexer::new(&s);
            let token = lexer.next_token().unwrap();
            match token {
                Token::Float(f) => {
                    let expected: f64 = s.parse().unwrap();
                    prop_assert!((f - expected).abs() < 0.000001);
                },
                _ => panic!("Expected a float: {} but got {:?}", s, token),
            }
        }
        
        #[test]
        fn string_literals_are_lexed_correctly(s in "[a-zA-Z0-9_\\s!@#$%^&*(),.?\":{}|<>]*") {
            let input = format!("\"{}\"", s);
            let mut lexer = Lexer::new(&input);
            let token = lexer.next_token().unwrap();
            prop_assert_eq!(token, Token::String(s));
        }
    }
    
    #[test]
    fn test_comments() {
        let input = r#"
        // This is a normal comment in most languages
        fr fr This is a comment in CURSED
        
        no cap
        This is a block comment in CURSED
        It can span multiple lines
        on god
        
        // Mixing comment styles
        slay comment() {
            fr fr Inline function comment
            yolo "comment"; // Not a comment
        }
        "#;
        
        // Expected tokens when parsing the input
        let expected = vec![
            // fr fr This is a comment in CURSED
            Token::LineComment,
            
            // no cap...on god (block comment)
            Token::BlockCommentStart,
            Token::BlockCommentEnd,
            
            // slay comment()...
            Token::Slay,
            Token::Identifier("comment".to_string()),
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            
            // fr fr Inline function comment
            Token::LineComment,
            
            // yolo "comment";
            Token::Yolo,
            Token::String("comment".to_string()),
            Token::Semicolon,
            
            Token::RBrace,
            Token::Eof,
        ];
        
        let mut lexer = Lexer::new(input);
        
        for expected_token in expected {
            let token = lexer.next_token().unwrap();
            assert_eq!(token, expected_token);
        }
    }
} 