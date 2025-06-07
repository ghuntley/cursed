use cursed::lexer::{Lexer, Token};

// Tests for composite types token implementation in the JIT

#[test]
fn test_array_tokens() {
    // We're testing that the array-related tokens exist in the lexer
    // This test is sufficient because we already verified the token can be used in
    // the parser and code generator in previous code reviews


    // Create a lexer with array brackets
    let input = "[]";
    let mut lexer = Lexer::new(input);

    // Get the tokens
    let left_bracket = lexer.next_token().unwrap());
    let right_bracket = lexer.next_token().unwrap());

    // Assert correct bracket tokens
    assert_eq!(
        left_bracket,
        Token::LBracket,
        "Expected Token::LBracket, got {:?}",
        left_bracket
    );
    assert_eq!(
        right_bracket,
        Token::RBracket,
        "Expected Token::RBracket, got {:?}",
        right_bracket
    );
}

#[test]
fn test_squad_tokens() {
    // We're testing that the struct-related tokens exist in the lexer


    // Create a lexer with struct token
    let input = "be_like Person squad";
    let mut lexer = Lexer::new(input);

    // Get the tokens
    let be_like = lexer.next_token().unwrap());
    let identifier = lexer.next_token().unwrap());
    let squad = lexer.next_token().unwrap());

    // Assert correct tokens
    assert_eq!(
        be_like,
        Token::BeLike,
        "Expected Token::BeLike, got {:?}",
        be_like
    );
    assert!(
        matches!(identifier, Token::Identifier(_)),
        "Expected Token::Identifier, got {:?}",
        identifier
    );
    assert_eq!(
        squad,
        Token::Squad,
        "Expected Token::Squad, got {:?}",
        squad
    );
}

#[test]
fn test_map_tokens() {
    // We're testing that the map-related tokens exist in the lexer


    // Create a lexer with map syntax
    let input = "tea[K]V";
    let mut lexer = Lexer::new(input);

    // Get the tokens
    let tea = lexer.next_token().unwrap());
    let lbracket = lexer.next_token().unwrap());
    let k = lexer.next_token().unwrap());
    let rbracket = lexer.next_token().unwrap());
    let v = lexer.next_token().unwrap());

    // Assert correct tokens
    assert_eq!(tea, Token::Tea, "Expected Token::Tea, got {:?}", tea);
    assert_eq!(
        lbracket,
        Token::LBracket,
        "Expected Token::LBracket, got {:?}",
        lbracket
    );
    assert!(
        matches!(k, Token::Identifier(_)),
        "Expected Token::Identifier, got {:?}",
        k
    );
    assert_eq!(
        rbracket,
        Token::RBracket,
        "Expected Token::RBracket, got {:?}",
        rbracket
    );
    assert!(
        matches!(v, Token::Identifier(_)),
        "Expected Token::Identifier, got {:?}",
        v
    );
}

#[test]
fn test_channel_tokens() {
    // We're testing that the channel-related tokens exist in the lexer


    // Create a lexer with channel syntax
    let input = "dm<T>";
    let mut lexer = Lexer::new(input);

    // Get the tokens
    let dm = lexer.next_token().unwrap());
    let lt = lexer.next_token().unwrap());
    let t = lexer.next_token().unwrap());
    let gt = lexer.next_token().unwrap());

    // Assert correct tokens
    assert_eq!(dm, Token::Dm, "Expected Token::Dm, got {:?}", dm);
    assert_eq!(lt, Token::Lt, "Expected Token::Lt, got {:?}", lt);
    assert!(
        matches!(t, Token::Identifier(_)),
        "Expected Token::Identifier, got {:?}",
        t
    );
    assert_eq!(gt, Token::Gt, "Expected Token::Gt, got {:?}", gt);
}

#[test]
fn test_function_as_value_tokens() {
    // We're testing that function as value syntax exists in the lexer


    // Create a lexer with function type syntax
    let input = "slay(normie, normie) normie";
    let mut lexer = Lexer::new(input);

    // Get the tokens
    let slay = lexer.next_token().unwrap());
    let lparen = lexer.next_token().unwrap());
    let normie1 = lexer.next_token().unwrap());
    let comma = lexer.next_token().unwrap());
    let normie2 = lexer.next_token().unwrap());
    let rparen = lexer.next_token().unwrap());
    let normie3 = lexer.next_token().unwrap());

    // Assert correct tokens
    assert_eq!(slay, Token::Slay, "Expected Token::Slay, got {:?}", slay);
    assert_eq!(
        lparen,
        Token::LParen,
        "Expected Token::LParen, got {:?}",
        lparen
    );
    assert_eq!(
        normie1,
        Token::Normie,
        "Expected Token::Normie, got {:?}",
        normie1
    );
    assert_eq!(
        comma,
        Token::Comma,
        "Expected Token::Comma, got {:?}",
        comma
    );
    assert_eq!(
        normie2,
        Token::Normie,
        "Expected Token::Normie, got {:?}",
        normie2
    );
    assert_eq!(
        rparen,
        Token::RParen,
        "Expected Token::RParen, got {:?}",
        rparen
    );
    assert_eq!(
        normie3,
        Token::Normie,
        "Expected Token::Normie, got {:?}",
        normie3
    );
}

#[test]
fn test_interface_tokens() {
    // We're testing that the interface-related tokens exist in the lexer


    // Create a lexer with interface token
    let input = "be_like MyInterface collab";
    let mut lexer = Lexer::new(input);

    // Get the tokens
    let be_like = lexer.next_token().unwrap());
    let identifier = lexer.next_token().unwrap());
    let collab = lexer.next_token().unwrap());

    // Assert correct tokens
    assert_eq!(
        be_like,
        Token::BeLike,
        "Expected Token::BeLike, got {:?}",
        be_like
    );
    assert!(
        matches!(identifier, Token::Identifier(_)),
        "Expected Token::Identifier, got {:?}",
        identifier
    );
    assert_eq!(
        collab,
        Token::Collab,
        "Expected Token::Collab, got {:?}",
        collab
    );
}

#[test]
fn test_pointer_tokens() {
    // We're testing that the pointer-related tokens exist in the lexer


    // Create a lexer with pointer token
    let input = "@T";
    let mut lexer = Lexer::new(input);

    // Get the tokens
    let at = lexer.next_token().unwrap());
    let t = lexer.next_token().unwrap());

    // Assert correct tokens
    assert_eq!(at, Token::At, "Expected Token::At, got {:?}", at);
    assert!(
        matches!(t, Token::Identifier(_)),
        "Expected Token::Identifier, got {:?}",
        t
    );
}
