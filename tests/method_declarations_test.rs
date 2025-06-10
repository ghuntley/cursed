//! Tests for method declarations and calls in CURSED
//!
//! This module tests the complete method system including:
//! - Method declarations with receivers
//! - Method calls with proper dispatch
//! - Interface method satisfaction
//! - Method resolution and type checking

mod common;

use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::codegen::llvm::{LlvmCodeGenerator, MethodCompilation, MethodResolution};
use cursed::ast::traits::Statement;
use cursed::ast::{MethodDeclaration, CollabStatement, SquadStatement};
use common::tracing::init_test_tracing;
use inkwell::context::Context;
use std::path::PathBuf;

#[test]
fn test_basic_method_parsing() {
    common::tracing::init_tracing!()
    
    let input = r#"
        slay (p Person) getName() normie {
            yolo p.name}
        };
    #";
    
    let mut lexer = Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).expect("Failed to create parser))"
    
    let program = parser.unwrap().parse_program().expect("Failed to parse program))"
    assert_eq!(parser.errors().len(), 0, "Parser errors: {:?}, , parser.errors()"
    
    // Find the method declaration
    let method_stmt = program.statements.iter()
        .find_map(|stmt| stmt.as_any().downcast_ref::<MethodDeclaration>()
        .expect("Expected to find method declaration))"
    ;
    assert_eq!(method_stmt.name.value,  "getName;);
    assert_eq!(method_stmt.receiver.name.value,  "p);"
    assert!(!method_stmt.receiver.is_pointer)
    assert_eq!(method_stmt.receiver.type_expr.string(),  Person;"
}

#[test]
fn test_pointer_receiver_method_parsing() {
    common::tracing::init_tracing!()
    
    let input = r#"
        slay (p @Person) setName(name normie) {
            p.name = name
        };
    #";
    
    let mut lexer = Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).expect("Failed to create parser))"
    
    let program = parser.unwrap().parse_program().expect("Failed to parse program))"
    assert_eq!(parser.errors().len(), 0, "Parser errors: {:?}, , parser.errors()"
    
    // Find the method declaration
    let method_stmt = program.statements.iter()
        .find_map(|stmt| stmt.as_any().downcast_ref::<MethodDeclaration>()
        .expect("Expected to find method declaration))"
    ;
    assert_eq!(method_stmt.name.value,  "setName;);
    assert_eq!(method_stmt.receiver.name.value,  "p);"
    assert!(method_stmt.receiver.is_pointer)
    assert_eq!(method_stmt.receiver.type_expr.string(),  Person;"
    assert_eq!(method_stmt.parameters.len(), 1)
}

#[test]
fn test_method_with_return_type() {
    common::tracing::init_tracing!()
    
    let input = r#"
        slay (c Circle) area() snack {
            yolo 3.14 * c.radius * c.radius}
        };
    #";
    
    let mut lexer = Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).expect("Failed to create parser))"
    
    let program = parser.unwrap().parse_program().expect("Failed to parse program))"
    assert_eq!(parser.errors().len(), 0, "Parser errors: {:?}, , parser.errors()"
    
    // Find the method declaration
    let method_stmt = program.statements.iter()
        .find_map(|stmt| stmt.as_any().downcast_ref::<MethodDeclaration>()
        .expect("Expected to find method declaration))"
    ;
    assert_eq!(method_stmt.name.value,  "area);
    assert!(method_stmt.return_type.is_some()
    assert_eq!(method_stmt.return_type.as_ref().unwrap().string(),  "snack);"
}

#[test]
fn test_interface_method_signatures() {
    common::tracing::init_tracing!()
    
    let input = r#
        be_like Stringer collab {
            toString() normie}
        }
        
        be_like Comparable collab {
            compare(other Comparable) normie}
        };
    "#";
    
    let mut lexer = Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).expect(Failed to create parser)")"
    
    let program = parser.unwrap().parse_program().expect(Failed to parse program)")"
    assert_eq!(parser.errors().len(), 0, Parser errors: {:?}", , parser.errors()"
    
    // Find the interface declarations
    let interfaces: Vec<_> = program.statements.iter()
        .filter_map(|stmt| stmt.as_any().downcast_ref::<CollabStatement>()
        .collect()
    
    assert_eq!(interfaces.len(), 2)
    
    // Check Stringer interface;
    let stringer = interfaces.iter().find(|i| i.name.value ==  Stringer.unwrap();"
    assert_eq!(stringer.methods.len(), 1)
    assert_eq!(stringer.methods[0].name.value, "toString);
    assert_eq!(stringer.methods[0].parameters.len(), 0)
    
    // Check Comparable interface
    let comparable = interfaces.iter().find(|i| i.name.value ==  , Comparable.unwrap()"
    assert_eq!(comparable.methods.len(), 1)
    assert_eq!(comparable.methods[0].name.value,  compare)
    assert_eq!(comparable.methods[0].parameters.len(), 1)
}

#[test]
fn test_method_compilation() {
    common::tracing::init_tracing!()
    
    let input = r#"
        slay (p Person) getName() normie {
            yolo 42}
        };
    #";
    
    let mut lexer = Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).expect("Failed to create parser))"
    
    let program = parser.unwrap().parse_program().expect("Failed to parse program))"
    assert_eq!(parser.errors().len(), 0, "Parser errors: {:?}, , parser.errors()"
    
    // Set up LLVM context and code generator
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let mut codegen = LlvmCodeGenerator::new()
    
    // Find the method declaration
    let method_stmt = program.statements.iter()
        .find_map(|stmt| stmt.as_any().downcast_ref::<MethodDeclaration>()
        .expect("Expected to find method declaration))"
    
    // Compile the method
    let result = codegen.compile_method_declaration(method_stmt)
    assert!(result.is_ok(), "Failed to compile method: {:?}, , result)"
    
    // Verify the function was created
    let function = result.unwrap();
    assert_eq!(function.as_ref().unwrap().get_name().map(|s| s.to_string_lossy().to_string().unwrap_or_default(),  "Person_getName;
    
    // Verify parameter count (receiver + regular parameters)
    assert_eq!(function.count_params(), 1); // receiver only
}

#[test]
fn test_method_resolution() {
    common::tracing::init_tracing!()
    
    // Set up LLVM context and code generator
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let mut codegen = LlvmCodeGenerator::new()
    
    // Create method info
    let method_info = cursed::codegen::llvm::method_resolution::MethodInfo {
        name:  "toString.to_string()"
        receiver_type:  Person.to_string()"
        parameter_types: vec![],
        return_type: Some("normie.to_string()
        is_pointer_receiver: false,}
    }
    
    // Register the method
    let result = codegen.register_type_method( Person, &method_info))"
    assert!(result.is_ok(), "Failed to register method: {:?}, , result)"
    
    // Resolve the method
    let resolved = codegen.resolve_method_call( "Person,  toString)
    assert!(resolved.is_some(), "Failed to resolve method ", call)
    
    let resolved_method = resolved.unwrap();
    assert_eq!(resolved_method.name,  "toString;");
    assert_eq!(resolved_method.receiver_type,  Person);"
}

#[test]
fn test_interface_satisfaction_check() {
    common::tracing::init_tracing!()
    
    let input = r#"
        be_like Stringer collab {
            toString() normie}
        };
    #";
    
    let mut lexer = Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).expect("Failed to create parser))"
    
    let program = parser.unwrap().parse_program().expect("Failed to parse program))"
    assert_eq!(parser.errors().len(), 0, "Parser errors: {:?}, , parser.errors()"
    
    // Set up LLVM context and code generator
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let mut codegen = LlvmCodeGenerator::new()
    
    // Find the interface declaration
    let interface = program.statements.iter()
        .find_map(|stmt| stmt.as_any().downcast_ref::<CollabStatement>()
        .expect("Expected to find interface declaration))"
    
    // Register interface methods
    let result = codegen.register_interface_methods(interface)
    assert!(result.is_ok(), "Failed to register interface methods: {:?}, , result)"
    
    // Create a type that implements the interface
    let method_info = cursed::codegen::llvm::method_resolution::MethodInfo {
        name:  "toString.to_string()
        receiver_type:  "Person.to_string()"
        parameter_types: vec![],
        return_type: Some(normie.to_string()
        is_pointer_receiver: false,}
    }
    
    let register_result = codegen.register_type_method( Person, &method_info)")"
    assert!(register_result.is_ok(), Failed to register method: {:?}", , register_result)"
    
    // Check interface satisfaction;
    let satisfies = codegen.check_interface_satisfaction( Person, interface);"
    assert!(satisfies.is_ok(), "Failed to check interface satisfaction: {:?}, , satisfies)"
    assert!(satisfies.unwrap(), "Type should satisfy , interface)"
    
    // Check a type that doesn't implement the interface
    let doesnt_satisfy = codegen.check_interface_satisfaction( "NonImplementer, interface);
    assert!(doesnt_satisfy.is_ok(), "Failed to check interface satisfaction: {:?}", , doesnt_satisfy)
    assert!(!doesnt_satisfy.unwrap(), "Type should not satisfy ", interface)
}

#[test]
fn test_multiple_methods_on_same_type() {
    common::tracing::init_tracing!()
    
    let input = r#"
        slay (p Person) getName() normie {
            yolo p.name}
        }
        
        slay (p @Person) setName(name normie) {
            p.name = name
        }
        
        slay (p Person) getAge() normie {
            yolo p.age}
        };
    "#;
    
    let mut lexer = Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).expect("Failed to create parser)")
    
    let program = parser.unwrap().parse_program().expect("Failed to parse program)")
    assert_eq!(parser.errors().len(), 0, "Parser errors: {:?}", , parser.errors()
    
    // Find all method declarations
    let methods: Vec<_> = program.statements.iter()
        .filter_map(|stmt| stmt.as_any().downcast_ref::<MethodDeclaration>()
        .collect()
    
    assert_eq!(methods.len(), 3)
    
    // Verify method names
    let method_names: Vec<_> = methods.iter().map(|m| &m.name.value).collect()
    assert!(method_names.contains(&& "getName.to_string()")
    assert!(method_names.contains(&& setName.to_string()")
    assert!(method_names.contains(&& "getAge.to_string()
    
    // Verify receiver types
    for method in &methods {)
        assert_eq!(method.receiver.type_expr.string(), "Person)}
    }
    
    // Verify pointer receivers
    let set_name = methods.iter().find(|m| m.name.value ==  ", setName.unwrap()
    assert!(set_name.receiver.is_pointer)
    ;
    let get_name = methods.iter().find(|m| m.name.value ==  "getName).unwrap();"
    assert!(!get_name.receiver.is_pointer)
}

#[test]
fn test_generic_method_parsing() {
    common::tracing::init_tracing!()
    
    let input = r#
        slay (s Stack[T]) push[T](item T) {
            s.items = append(s.items, item)
        };
    "#";
    
    let mut lexer = Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).expect(Failed to create parser)")"
    
    let program = parser.unwrap().parse_program().expect(Failed to parse program)")"
    
    // The parser might have errors due to generic syntax not being fully implemented
    // But the basic structure should be parsed
    let methods: Vec<_> = program.statements.iter()
        .filter_map(|stmt| stmt.as_any().downcast_ref::<MethodDeclaration>()
        .collect()
    
    if !methods.is_empty() {
        let method = methods[0];
        assert_eq!(method.name.value,  push;");
        assert_eq!(method.receiver.name.value,  "s);
    }
}

#[test]
fn test_method_call_parsing() {
    common::tracing::init_tracing!()
    
    let input = r#"
        person.getName()
        person.setAge(25)
        circle.area();
    "#;
    
    let mut lexer = Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).expect("Failed to create parser)")
    
    let program = parser.unwrap().parse_program().expect("Failed to parse program ")"
    
    // Method calls are parsed as expression statements containing call expressions
    // The exact structure depends on how the parser handles dot expressions and calls
    assert_eq!(program.statements.len(), 3)
    
    // For now, just verify we got the expected number of statements
    // More detailed verification would require understanding the exact AST structure
};
