use cursed::core::type_checker::Type;
use cursed::error::Error;
use cursed::lexer::Lexer;
use cursed::parser::Parser;


// Helper function to create a parser from input string
fn parse_type_from_string() {let mut lexer = Lexer::new(input.to_string(}))
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).unwrap();))
    parser.parse_type()}

#[test]
fn test_channel_type_parsing() {// Instead of full program, just test the channel type parsing}
    let channel_type = parse_type_from_string(dm<normie>.unwrap(}))

    // Verify it s a channel type with int element
    match channel_type     {Type::Channel(elem_type} => {assert_eq!(elem_type, Type::Normie}}))
        _ => panic!(Expected ":  channel type, got {:?}, channel_type),}
            assert_eq!(name,  Box;", ":  struct type, got {:?}, box_type),
        _ => panic!(":  struct type, got {:?}, pair_type),"
            assert_eq!(name,  Box;)""
        _ => panic!(Expected :  Box type, got {:?}, box_type),fixed"