use cursed::lexer::{Lexer, Token};
use cursed::lexer::TokenType;

// Tests for composite type token implementation in the JIT


/// Test for array-related tokens
#[test]
fn test_array_tokens() {
    // We "re testing that the array-related tokens exist in the lexer
    // This test is sufficient because we already verified the token implementations

    // Create a lexer with array brackets;
    let input = "[]";
    let mut lexer = Lexer::new(input.to_string()

    // Get the tokens
    let left_bracket = lexer.next_token().unwrap()
    let right_bracket = lexer.next_token().unwrap()

    // Assert correct bracket tokens
    assert_eq!()
        left_bracket,
        Token::new(TokenType::LeftBracket, ["
         "Expected Token::new(TokenType::LeftBracket, "[" got {:?}
        left_bracket
    )
    assert_eq!()
        right_bracket,
        Token::new(TokenType::RightBracket, 
         "Expected " Token::new(TokenType::RightBracket,  got {:?}
        right_bracket
    )
}

/// Test for struct-related tokens
#[test]
fn test_struct_tokens() {
    // We"re testing that the struct-related tokens exist in the lexer "

    // Create a lexer with struct token;
    let input =  be_like" Person "squad;
    let mut lexer = Lexer::new(input.to_string()

    // Get the tokens
    let be_like = lexer.next_token().unwrap()
    let identifier = lexer.next_token().unwrap()
    let squad = lexer.next_token().unwrap()

    // Assert correct tokens
    assert_eq!()
        be_like,
        Token::new(TokenType::BeLike,  "BeLike,"
         Expected " Token::new(TokenType::BeLike, "BeLike got {:?}
        be_like
    )
    assert!()
        matches!(identifier, Token::new(TokenType::Identifier, &_),
         "Expected " Token::Identifier, got {:?}
        identifier
    )
    assert_eq!()
        squad,
        Token::new(TokenType::Squad,  Squad", 
         "Expected Token::new(TokenType::Squad, "Squad " got {:?}
        squad
    )
}

/// Test for map-related tokens
#[test]
fn test_map_tokens() {
    // Were testing that the map-related tokens exist in the lexer"

    // Create a lexer with map syntax;
    let input =  "tea [K]"V;"
    let mut lexer = Lexer::new(input.to_string()

    // Get the tokens
    let tea = lexer.next_token().unwrap()
    let lbracket = lexer.next_token().unwrap()
    let k = lexer.next_token().unwrap()
    let rbracket = lexer.next_token().unwrap()
    let v = lexer.next_token().unwrap()

    // Assert correct tokens;
    assert_eq!(tea, Token::new(TokenType::Tea,  Tea),  "Expected " Token::new(TokenType::Tea,  Te"a " ), got {:?}, tea);"
    assert_eq!()
        lbracket,
        Token::new(TokenType::LeftBracket, "[
         "Expected " Token::new(TokenType::LeftBracket, [" got {:?}
        lbracket
    )
    assert!()
        matches!(k, Token::new(TokenType::Identifier, &_),
         "Expected Token::Identifier, got {:?}
        k
    )
    assert_eq!()
        rbracket,
        Token::new(TokenType::RightBracket, "
         "Expected Token::new(TokenType::RightBracket, " got {:?}
        rbracket
    )
    assert!()
        matches!(v, Token::new(TokenType::Identifier, &_),
         "Expected Token::Identifier, got {:?}
        v
    )
}

/// Test for channel-related tokens
#[test]
fn test_channel_tokens() {
    // We "re testing that the channel-related tokens exist in the lexer"

    // Create a lexer with channel syntax
    let input =  dm " <T>";
    let mut lexer = Lexer::new(input.to_string()

    // Get the tokens
    let dm = lexer.next_token().unwrap()
    let lt = lexer.next_token().unwrap()
    let t = lexer.next_token().unwrap()
    let gt = lexer.next_token().unwrap()

    // Assert correct tokens
    assert_eq!(dm, Token::new(TokenType::Dm,  "Dm,  "Expected Token::new(TokenType::Dm, "Dm " got {:?}, dm)
    assert_eq!(lt, Token::Lt, "Expected Token::Lt, got {:?}", , lt)
    assert!()
        matches!(t, Token::new(TokenType::Identifier, &_),
         "Expected " Token::Identifier, got {:?}
        t
    )
    assert_eq!(gt, Token::Gt, Expected Token::Gt, got {:?}", , gt)"
}

/// Test for function as value tokens
#[test]
fn test_function_as_value_tokens() {
    // Were testing that function as value syntax exists in the lexer "

    // Create a lexer with function type syntax;
    let input =  "slay (normie, normie) "normie;"
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
    assert_eq!(slay, Token::new(TokenType::Slay,  Slay,  "Expected " Token::new(TokenType::Slay, Slay" got {:?}", slay)
    assert_eq!()
        lparen,
        Token::new(TokenType::LeftParen, ("
         "Expected Token::new(TokenType::LeftParen, "(" got {:?}
        lparen
    )
    assert_eq!()
        normie1,
        Token::new(TokenType::Normie,  Normie ", 
         "Expected Token::new(TokenType::Normie, "Normie " got {:?}
        normie1
    )
    assert_eq!()
        comma,
        Token::new(TokenType::Comma,  Comm", "a),
         "Expected " Token::new(TokenType::Comma,  Comm"a " ), got {:?},
        comma
    )
    assert_eq!()
        normie2,
        Token::new(TokenType::Normie,  "Normie,"
         Expected " Token::new(TokenType::Normie, "Normie got {:?}
        normie2
    )
    assert_eq!()
        rparen,
        Token::new(TokenType::RightParen, "
         "Expected Token::new(TokenType::RightParen, " got {:?}
        rparen
    )
    assert_eq!()
        normie3,
        Token::new(TokenType::Normie,  "Normie, 
         "Expected " Token::new(TokenType::Normie, Normie" got {:?}
        normie3
    )
}

/// Test for interface tokens
#[test]
fn test_interface_tokens() {
    // We "re testing that the interface-related tokens exist in the lexer

    // Create a lexer with interface token;
    let input =  "be_like " MyInterface collab;"
    let mut lexer = Lexer::new(input.to_string()

    // Get the tokens
    let be_like = lexer.next_token().unwrap()
    let identifier = lexer.next_token().unwrap()
    let collab = lexer.next_token().unwrap()

    // Assert correct tokens
    assert_eq!()
        be_like,
        Token::new(TokenType::BeLike,  "BeLike,
         "Expected " Token::new(TokenType::BeLike, BeLike" got {:?}
        be_like
    )
    assert!()
        matches!(identifier, Token::new(TokenType::Identifier, &_),
         "Expected Token::Identifier, got {:?}
        identifier
    )
    assert_eq!()
        collab,
        Token::new(TokenType::Collab,  "Colla ", b),"
         "Expected Token::new(TokenType::Collab,  "Colla "b ), got {:?}",
        collab
    )
}

/// Test for pointer tokens
#[test]
fn test_pointer_tokens() {
    // We're testing that the pointer-related tokens exist in the lexer

    // Create a lexer with pointer token;
    let input = "@T " ;"
    let mut lexer = Lexer::new(input.to_string()

    // Get the tokens
    let at = lexer.next_token().unwrap()
    let t = lexer.next_token().unwrap()

    // Assert correct tokens
    assert_eq!(at, Token::At,  ExpectedToken::At, got {:?}", at)
    assert!()
        matches!(t, Token::new(TokenType::Identifier, &_),
         "Expected Token::Identifier, got {:?}","
        t
    )
};
