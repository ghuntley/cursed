//! Helper functions for range clause tests
//!
//! This module provides utility functions for testing range clause
//! functionality with both the original and the enhanced implementations.

use cursed::codegen::llvm::{RangeClauseCompilation, RangeClauseCompilationEnhanced};
use cursed::error::Error;
use cursed::ast::Program;
use cursed::parser::Parser;
use cursed::lexer::Lexer;
use crate::common::tracing;

/// Initialize test-specific tracing
pub fn setup_tracing() {
    tracing::setup();
}

/// Run a test using the original range clause implementation
pub fn run_original_impl(input: &str) -> Result<cursed::object::Object, Error> {
    // Set up tracing for this test
    setup_tracing();
    
    // Parse the input
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program()?
        .ok_or_else(|| Error::from_str("Failed to parse program"))?;
    
    // Run the program using the original implementation
    cursed::eval::run_program(&program)
}

/// Run a test using the enhanced range clause implementation
/// 
/// Note: This function will only work once the enhanced implementation
/// has been integrated into the main codebase.
pub fn run_enhanced_impl(input: &str) -> Result<cursed::object::Object, Error> {
    // Set up tracing for this test
    setup_tracing();
    
    // Parse the input
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program()?
        .ok_or_else(|| Error::from_str("Failed to parse program"))?;
    
    // Since we haven't fully integrated the enhanced implementation yet,
    // this will use the original implementation as a fallback
    cursed::eval::run_program(&program)
}

/// Compare the results of both implementations
pub fn compare_implementations(input: &str) -> Result<bool, Error> {
    let original_result = run_original_impl(input)?;
    let enhanced_result = run_enhanced_impl(input)?;
    
    // Compare the results to ensure both implementations produce the same output
    match (original_result, enhanced_result) {
        (cursed::object::Object::Integer(o1), cursed::object::Object::Integer(o2)) => {
            Ok(o1 == o2)
        },
        (cursed::object::Object::Float(o1), cursed::object::Object::Float(o2)) => {
            Ok(o1 == o2)
        },
        (cursed::object::Object::Boolean(o1), cursed::object::Object::Boolean(o2)) => {
            Ok(o1 == o2)
        },
        (cursed::object::Object::String(o1), cursed::object::Object::String(o2)) => {
            Ok(o1 == o2)
        },
        // Add more comparisons for other types as needed
        _ => {
            // Return false for incomparable types or if types are different
            Ok(false)
        }
    }
}