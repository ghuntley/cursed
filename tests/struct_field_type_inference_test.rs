//! Tests for struct field type inference in the LLVM code generator

use cursed::ast::expressions::identifiers::Identifier;
use cursed::ast::expressions::literals::{IntegerLiteral, FloatLiteral, StringLiteral, BooleanLiteral};
use cursed::ast::expressions::special::StructInstantiation;
use cursed::ast::statements::block::BlockStatement;
use cursed::ast::statements::ExpressionStatement;
use cursed::ast::statements::declarations::LetStatement;
use cursed::ast::traits::{Expression, Statement};
use cursed::Type;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::{ExpressionCompilation, StatementCompilation};
use cursed::lexer::token::Token;
use cursed::lexer::TokenType;
use inkwell::context::Context;
use inkwell::values::BasicValueEnum;
use std::path::PathBuf;
use std::collections::HashMap;

#[test]
#[ignore = "Struct field type inference not yet implemented"]
fn test_struct_field_type_inference() {
    let context = Context::create();
    let mut generator = LlvmCodeGenerator::new(&context, "test_struct_field_type_inference", PathBuf::from("test_struct_inference.csd"));

    // Create a struct instantiation with fields but without explicit type annotations
    let mut fields = HashMap::new();
    
    // Integer field
    fields.insert("id".to_string(), Box::new(IntegerLiteral {
        token: Token::new(TokenType::Int, "42").token_literal(),
        value: 42,
    }) as Box<dyn Expression>);
    
    // String field
    fields.insert("name".to_string(), Box::new(StringLiteral {
        token: Token::new(TokenType::String, "\"Alice\"").token_literal(),
        value: "Alice".to_string(),
    }) as Box<dyn Expression>);
    
    // Boolean field
    fields.insert("active".to_string(), Box::new(BooleanLiteral {
        token: Token::new(TokenType::True, "true").token_literal(),
        value: true,
    }) as Box<dyn Expression>);

    // Create struct instantiation expression
    let struct_inst = StructInstantiation {
        token: Token::new(TokenType::LBrace, "{").token_literal(),
        struct_type: Box::new(Identifier {
            token: Token::new(TokenType::Identifier, "User").token_literal(),
            value: "User".to_string(),
        }),
        fields,
        type_arguments: vec![],
    };
    
    // Create a variable to hold the struct instance
    let let_stmt = LetStatement {
        token: Token::new(TokenType::Sus, "sus").token_literal(),
        name: Identifier {
            token: Token::new(TokenType::Identifier, "user").token_literal(),
            value: "user".to_string(),
        },
        type_annotation: None, // No explicit type annotation
        value: Some(Box::new(struct_inst)),
    };
    
    // Try to compile the statement - this should register the struct type
    // with inferred field types
    let result = generator.compile_statement(&let_stmt);
    assert!(result.is_ok(), "Failed to compile struct instantiation: {:?}", result.err());
    
    // Now check if the struct's fields were properly registered with inferred types
    // We need to access the type checker's internal state for this
    // In a real implementation, we would expose a method to query field types
    
    // For now, we'll use property access to verify field types were correctly inferred
    let id_access = generator.compile_property_access("user", "id");
    assert!(id_access.is_ok(), "Failed to access 'id' field: {:?}", id_access.err());
    assert!(id_access.unwrap().is_int_value(), "'id' field should be inferred as integer");
    
    let name_access = generator.compile_property_access("user", "name");
    assert!(name_access.is_ok(), "Failed to access 'name' field: {:?}", name_access.err());
    // String should be a pointer
    assert!(name_access.unwrap().is_pointer_value(), "'name' field should be inferred as string (pointer)");
    
    let active_access = generator.compile_property_access("user", "active");
    assert!(active_access.is_ok(), "Failed to access 'active' field: {:?}", active_access.err());
    assert!(active_access.unwrap().is_int_value(), "'active' field should be inferred as boolean (i1)");
}

#[test]
#[ignore = "Struct field type inference not yet implemented"]
fn test_nested_struct_field_type_inference() {
    let context = Context::create();
    let mut generator = LlvmCodeGenerator::new(&context, "test_nested_struct_inference", PathBuf::from("test_nested_struct_inference.csd"));

    // Create a struct instantiation with a nested struct
    let mut address_fields = HashMap::new();
    
    // Address fields
    address_fields.insert("street".to_string(), Box::new(StringLiteral {
        token: Token::new(TokenType::String, "\"123 Main St\"").token_literal(),
        value: "123 Main St".to_string(),
    }) as Box<dyn Expression>);
    
    address_fields.insert("city".to_string(), Box::new(StringLiteral {
        token: Token::new(TokenType::String, "\"Anytown\"").token_literal(),
        value: "Anytown".to_string(),
    }) as Box<dyn Expression>);
    
    address_fields.insert("zipcode".to_string(), Box::new(IntegerLiteral {
        token: Token::new(TokenType::Int, "12345").token_literal(),
        value: 12345,
    }) as Box<dyn Expression>);
    
    // Create address struct instantiation
    let address_inst = StructInstantiation {
        token: Token::new(TokenType::LBrace, "{").token_literal(),
        struct_type: Box::new(Identifier {
            token: Token::new(TokenType::Identifier, "Address").token_literal(),
            value: "Address".to_string(),
        }),
        fields: address_fields,
        type_arguments: vec![],
    };
    
    // Create user struct with nested address
    let mut user_fields = HashMap::new();
    
    user_fields.insert("id".to_string(), Box::new(IntegerLiteral {
        token: Token::new(TokenType::Int, "101").token_literal(),
        value: 101,
    }) as Box<dyn Expression>);
    
    user_fields.insert("name".to_string(), Box::new(StringLiteral {
        token: Token::new(TokenType::String, "\"Bob\"").token_literal(),
        value: "Bob".to_string(),
    }) as Box<dyn Expression>);
    
    user_fields.insert("address".to_string(), Box::new(address_inst) as Box<dyn Expression>);
    
    // Create user struct instantiation
    let user_inst = StructInstantiation {
        token: Token::new(TokenType::LBrace, "{").token_literal(),
        struct_type: Box::new(Identifier {
            token: Token::new(TokenType::Identifier, "User").token_literal(),
            value: "User".to_string(),
        }),
        fields: user_fields,
        type_arguments: vec![],
    };
    
    // Create a variable to hold the struct instance
    let let_stmt = LetStatement {
        token: Token::new(TokenType::Sus, "sus").token_literal(),
        name: Identifier {
            token: Token::new(TokenType::Identifier, "user").token_literal(),
            value: "user".to_string(),
        },
        type_annotation: None, // No explicit type annotation
        value: Some(Box::new(user_inst)),
    };
    
    // Try to compile the statement - this should register both struct types
    // with inferred field types for both User and Address
    let result = generator.compile_statement(&let_stmt);
    assert!(result.is_ok(), "Failed to compile nested struct instantiation: {:?}", result.err());
    
    // Now verify access to nested fields to ensure types were inferred correctly
    let address_access = generator.compile_nested_property_access("user", "address");
    assert!(address_access.is_ok(), "Failed to access 'address' field: {:?}", address_access.err());
    assert!(address_access.unwrap().is_pointer_value(), "'address' field should be a pointer to Address struct");
    
    let zipcode_access = generator.compile_nested_property_access("user.address", "zipcode");
    assert!(zipcode_access.is_ok(), "Failed to access 'zipcode' field: {:?}", zipcode_access.err());
    assert!(zipcode_access.unwrap().is_int_value(), "'zipcode' field should be inferred as integer");
}

#[test]
#[ignore = "Array type inference not yet implemented"]
fn test_array_literal_type_inference() {
    let context = Context::create();
    let mut generator = LlvmCodeGenerator::new(&context, "test_array_inference", PathBuf::from("test_array_inference.csd"));

    // Create an array literal with element type inference
    // In this case, all elements are integers
    let elements = vec![
        Box::new(IntegerLiteral {
            token: Token::new(TokenType::Int, "1").token_literal(),
            value: 1,
        }) as Box<dyn Expression>,
        Box::new(IntegerLiteral {
            token: Token::new(TokenType::Int, "2").token_literal(),
            value: 2,
        }) as Box<dyn Expression>,
        Box::new(IntegerLiteral {
            token: Token::new(TokenType::Int, "3").token_literal(),
            value: 3,
        }) as Box<dyn Expression>,
    ];
    
    // Create array literal
    let array_lit = cursed::ast::expressions::ArrayLiteral {
        token: Token::new(TokenType::LBracket, "[").token_literal(),
        elements,
    };
    
    // Create a variable to hold the array
    let let_stmt = LetStatement {
        token: Token::new(TokenType::Sus, "sus").token_literal(),
        name: Identifier {
            token: Token::new(TokenType::Identifier, "numbers").token_literal(),
            value: "numbers".to_string(),
        },
        type_annotation: None, // No explicit type annotation
        value: Some(Box::new(array_lit)),
    };
    
    // Try to compile the statement
    let result = generator.compile_statement(&let_stmt);
    assert!(result.is_ok(), "Failed to compile array literal: {:?}", result.err());
    
    // Verify the element type was correctly inferred
    let element_access = generator.compile_index_access("numbers", 0);
    assert!(element_access.is_ok(), "Failed to access array element: {:?}", element_access.err());
    assert!(element_access.unwrap().is_int_value(), "Array element should be inferred as integer");
}

#[test]
#[ignore = "Map type inference not yet implemented"]
fn test_map_literal_type_inference() {
    let context = Context::create();
    let mut generator = LlvmCodeGenerator::new(&context, "test_map_inference", PathBuf::from("test_map_inference.csd"));

    // Create a map literal with key-value type inference
    // In this case, string keys and integer values
    let mut pairs = vec![];
    
    // Add key-value pairs
    pairs.push((
        Box::new(StringLiteral {
            token: Token::new(TokenType::String, "\"one\"").token_literal(),
            value: "one".to_string(),
        }) as Box<dyn Expression>,
        Box::new(IntegerLiteral {
            token: Token::new(TokenType::Int, "1").token_literal(),
            value: 1,
        }) as Box<dyn Expression>,
    ));
    
    pairs.push((
        Box::new(StringLiteral {
            token: Token::new(TokenType::String, "\"two\"").token_literal(),
            value: "two".to_string(),
        }) as Box<dyn Expression>,
        Box::new(IntegerLiteral {
            token: Token::new(TokenType::Int, "2").token_literal(),
            value: 2,
        }) as Box<dyn Expression>,
    ));
    
    // Create hash literal
    let hash_lit = cursed::ast::expressions::HashLiteral {
        token: Token::new(TokenType::LBrace, "{").token_literal(),
        pairs,
    };
    
    // Create a variable to hold the map
    let let_stmt = LetStatement {
        token: Token::new(TokenType::Sus, "sus").token_literal(),
        name: Identifier {
            token: Token::new(TokenType::Identifier, "numbers").token_literal(),
            value: "numbers".to_string(),
        },
        type_annotation: None, // No explicit type annotation
        value: Some(Box::new(hash_lit)),
    };
    
    // Try to compile the statement
    let result = generator.compile_statement(&let_stmt);
    assert!(result.is_ok(), "Failed to compile map literal: {:?}", result.err());
    
    // Verify the key-value types were correctly inferred
    let value_access = generator.compile_map_access("numbers", "one");
    assert!(value_access.is_ok(), "Failed to access map value: {:?}", value_access.err());
    assert!(value_access.unwrap().is_int_value(), "Map value should be inferred as integer");
}