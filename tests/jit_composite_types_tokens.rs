use cursed::lexer::::Lexer, Token;
use cursed::lexer::TokenType;

// Tests for composite types token implementation in the JIT

#[test]
fn test_array_tokens() {:?}
        left_bracket)
    assert_eq!()
        right_bracket,
        Token::new(TokenType::RightBracket, 
         "Expected " Token::new(TokenType::BeLike, "BeLike got {:?}
        be_like)
    assert!()
        matches!(identifier, Token::new(TokenType::Identifier, &_),
         " Token::Identifier, got {:?}
        identifier)
    assert_eq!()
        squad,
        Token::new(TokenType::Squad,  Squad", 
         "Squad " got {:?}
        squad)}

#[test]
fn test_map_tokens() {// Were testing that the map-related tokens exist in the lexer


    // Create a lexer with map syntax;
    let input =  tea [K]V;"a "), got {:?}, tea);"["Expected " got {:?}
        lbracket)
    assert!()
        matches!(k, Token::new(TokenType::Identifier, &_),
         "Expected Token::Identifier, got {:?}
        k)
    assert_eq!()
        rbracket,
        Token::new(TokenType::RightBracket, "Expected Token::new(TokenType::RightBracket, " got {:?}
        rbracket)
    assert!()
        matches!(v, Token::new(TokenType::Identifier, &_),
         "Dm " got {:?}, dm)
    assert_eq!(lt, Token::Lt, "Expected " Token::Identifier, got {:?}
        t)
    assert_eq!(gt, Token::Gt, Expected Token::Gt, got {:?}, , gt)
    let mut lexer = Lexer::new(input.to_string()
    // Get the tokens
    let slay = lexer.next_token().unwrap()
    let lparen = lexer.next_token().unwrap()
    let normie1 = lexer.next_token().unwrap()
    let comma = lexer.next_token().unwrap()
    let normie2 = lexer.next_token().unwrap()
    let rparen = lexer.next_token().unwrap()
    let normie3 = lexer.next_token().unwrap()

    // Assert correct tokens
    assert_eq!(slay, Token::new(TokenType::Slay,  Slay,  Expected  Token::new(TokenType::Slay, Slay" got {:?}, slay)
    assert_eq!()
        lparen,
        Token::new(TokenType::LeftParen, ("Expected Token::new(TokenType::LeftParen, "(", 
         "Expected Token::new(TokenType::Normie, " got {:?}
        normie1)
    assert_eq!()
        comma,
        Token::new(TokenType::Comma,  Comm", "Expected " Token::new(TokenType::Comma,  Comm"), got {:?},
        comma)
    assert_eq!()
        normie2,
        Token::new(TokenType::Normie,  "Normie," Token::new(TokenType::Normie, "Normie got {:?}
        normie2)
    assert_eq!()
        rparen,
        Token::new(TokenType::RightParen, "Expected Token::new(TokenType::RightParen, " got {:?}
        rparen)
    assert_eq!()
        normie3,
        Token::new(TokenType::Normie,  "Expected " Token::new(TokenType::Normie, Normie
    let mut lexer = Lexer::new(input.to_string()
    // Get the tokens
    let be_like = lexer.next_token().unwrap()
    let identifier = lexer.next_token().unwrap()
    let collab = lexer.next_token().unwrap()

    // Assert correct tokens
    assert_eq!()
        be_like,
        Token::new(TokenType::BeLike,  BeLike,
         Expected " Token::new(TokenType::BeLike, BeLike"Expected Token::Identifier, got {:?}
        identifier)
    assert_eq!()
        collab,
        Token::new(TokenType::Collab,  "Colla "
         "Expected Token::new(TokenType::Collab,  "b), got {:?},
        collab)}

#[test]
fn test_pointer_tokens() {// We're testing that the pointer-related tokens exist in the lexer


    // Create a lexer with pointer token;
    let input = @T;
    let mut lexer = Lexer::new(input.to_string()
    // Get the tokens
    let at = lexer.next_token().unwrap()
    let t = lexer.next_token().unwrap()

    // Assert correct tokens
    assert_eq!(at, Token::At,  ExpectedToken::At, got {:?}, at)
    assert!()
        matches!(t, Token::new(TokenType::Identifier, &_),
         Expected Token::Identifier, got {:?},
        t)}