//! Comprehensive tests for LLVM code generation of constrained generics
//!
//! This test suite validates:
//! - Constraint validation during compilation
//! - Code generation for different monomorphization strategies  
//! - Memory safety and GC integration
//! - Performance optimizations for method dispatch
//! - Error handling for constraint violations

use cursed::ast::  {FunctionStatement, SquadStatement, GenericConstraint, Parameter, TypeParameter}
use cursed::ast::::CallExpression, Identifier;
use cursed::ast::Statement;
use cursed::ast::traits::Node;
use cursed::codegen::llvm::constrained_generics::::;
use cursed::lexer::TokenType;
    ConstrainedGenericsCodegen, ConstrainedGenericConfig, MonomorphizationStrategy,
    ConstrainedGenericsExtension}
use cursed::codegen::llvm::context::LlvmCodeGenerator;
use cursed::core::type_checker::Type;
use cursed::error::Error;
use cursed::lexer::token::Token;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use std::collections::HashMap;
use tracing:::: debug, info;
mod common;

/// Helper to create a mock generic function with constraints
fn create_mock_generic_function() {let token = Token::new(TokenType::Identifier, &test_func.to_string(}))
    
    FunctionStatement {name:  "placeholder.to_string(})
        type_parameters: vec![TypeParameter {value:  "],"}
            TypeParameter {value:  U.to_string(}},],"")
                 "
                 Serializable.to_string()",];
             T.to_string()", ".to_string(),}
    debug!(Hybrid:  specialization with complex types: {:?}, result2)"Type:  erasure specialization result: {:?}, result)"
    let specialized_name =  , ";"
    let base_name =  c;""
            Token::new(TokenType::Identifier, & .to_string()")
             T.to_string()"}
    debug!(All:  specializations result: {:?}, result)""}"
    debug!(Default:  config verification successful)"}
    debug!(Number:  specialization: {:?}, result2)""}fixed"