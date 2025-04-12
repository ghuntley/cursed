use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::core::type_checker::Type;
use cursed::error::Error;

// Helper function to create a parser from input string
fn parse_type_from_string(input: &str) -> Result<Type, Error> {
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer).unwrap();
    parser.parse_type()
}

#[test]
fn test_channel_type_parsing() {
    // Instead of full program, just test the channel type parsing
    let channel_type = parse_type_from_string("dm<normie>").unwrap();
    
    // Verify it's a channel type with int element
    match channel_type {
        Type::Channel(elem_type) => {
            assert_eq!(*elem_type, Type::Normie);
        },
        _ => panic!("Expected channel type, got {:?}", channel_type),
    }
}

#[test]
fn test_generic_type_parsing() {
    // Test parsing a generic type
    let box_type = parse_type_from_string("Box[normie]").unwrap();
    
    // Verify it's a Box with int element
    match box_type {
        Type::Struct(name, type_args) => {
            assert_eq!(name, "Box");
            assert_eq!(type_args.len(), 1);
            assert_eq!(*type_args[0], Type::Normie);
        },
        _ => panic!("Expected struct type, got {:?}", box_type),
    }
}

#[test]
fn test_multi_param_generic_type_parsing() {
    // Test parsing a generic type with multiple type parameters
    let pair_type = parse_type_from_string("Pair[tea, normie]").unwrap();
    
    // Verify it's a Pair with string and int elements
    match pair_type {
        Type::Struct(name, type_args) => {
            assert_eq!(name, "Pair");
            assert_eq!(type_args.len(), 2);
            assert_eq!(*type_args[0], Type::Tea);
            assert_eq!(*type_args[1], Type::Normie);
        },
        _ => panic!("Expected struct type, got {:?}", pair_type),
    }
}

#[test]
fn test_nested_generic_type_parsing() {
    // Test parsing a nested generic type
    let box_type = parse_type_from_string("Box[Pair[tea, normie]]").unwrap();
    
    // Verify it's a Box containing a Pair
    match box_type {
        Type::Struct(name, type_args) => {
            assert_eq!(name, "Box");
            assert_eq!(type_args.len(), 1);
            
            // Check the inner Pair type
            match &*type_args[0] {
                Type::Struct(inner_name, inner_args) => {
                    assert_eq!(inner_name, "Pair");
                    assert_eq!(inner_args.len(), 2);
                    assert_eq!(*inner_args[0], Type::Tea);
                    assert_eq!(*inner_args[1], Type::Normie);
                },
                _ => panic!("Expected Pair type, got {:?}", type_args[0]),
            }
        },
        _ => panic!("Expected Box type, got {:?}", box_type),
    }
}