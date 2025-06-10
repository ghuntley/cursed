use cursed::ast::dot_expression::DotExpression;
use cursed::ast::Identifier;
use cursed::ast::traits::Expression;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::ExpressionCompilation;
use cursed::codegen::llvm::PropertyAccessCompilation;
use cursed::codegen::llvm::VariableHandling;
use cursed::lexer::token::Token;
use cursed::lexer::TokenType;

use inkwell::context::Context;
use std::path::PathBuf;

// Tests for property access (field access) in the LLVM code generator


#[test]
fn test_property_access() {
    // TODO: Implement test
    assert!(true);
})
    generator.add_variable_with_type(test_obj, struct_ptr, struct_type.into().unwrap();", fixed")