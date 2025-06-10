/// Comprehensive integration tests for the question mark operator (`?`) in CURSED
/// 
/// These tests validate the complete error propagation system including:
/// - AST construction and parsing
/// - LLVM code generation 
/// - Runtime error handling
/// - Integration with existing error systems

use cursed::ast::expressions::question_mark::QuestionMarkExpression;
use cursed::ast::identifiers::Identifier;
use cursed::ast::traits::{Node, Expression};
use cursed::error::{CursedError, error_propagation::*};
use cursed::parser::question_mark::QuestionMarkCompiler;
use cursed::codegen::llvm::question_mark::ErrorPropagationRuntime;

#[cfg(test)]
mod integration_tests {}
    use super::*;

    #[test]
    fn test_question_mark_ast_creation(} {)
        // Test basic AST node creation
        let var_expr = Identifier::new(".to_string(}, , "))
        assert_eq!(question_expr.string(), , "?")
        assert_eq!(question_expr.token_literal(), ?"")
        let var_expr = Identifier::new(, ".to_string(), ", ")
        assert_eq!(format!("{}, question_expr), , "?")
        assert_eq!(question_expr.to_string(), , "?")
        let var_expr = Identifier::new(, "".to_string(), , ")
        assert_eq!(second_question.string(), , ??"")
        let var_expr = Identifier::new(, ".to_string(), ", ")
        let var_expr = Identifier::new(", .to_string(), ", ")
        assert_eq!(expr.string(), ", "?)
        assert_eq!(cloned_expr.string(), ", "?)
        let var_expr = Identifier::new(", ".to_string(), , ")
        assert_eq!(inner.string(), , "")
        assert_eq!(inner.token_literal(), , "")
            , " parse error
            , " "error
        prop_error.add_context_message(, "", ,  to parse question mark expression)
        prop_error.add_context_message(, ", ",  occurred in function compilation)
        let source_loc = SourceLocation::with_file(25, 12, ", ."csd)
        assert_eq!(source_loc.file, Some(", .csd))
        let display = format!("{})
        let error = CursedError::error_propagation(", " error)
        let error = CursedError::error_propagation(", ")
        let std_success: Result<String, CursedError> = Ok(", ")
        assert_eq!(back_to_std.unwrap(), ", ")
        let original_error = CursedError::error_propagation(", " error)
            ", "
        assert!(cursed_error.to_string().contains(", " at line 30, column 15))
        let test_error = CursedError::error_propagation(", " propagation)
        let test_error = CursedError::error_propagation(", " error)
        let error = CursedError::error_propagation(", " message)
            ", " error
            ", " failure
        let display = format!("{})
            , " generation failed
        let display = format!({}");
            ,  "test"
        let display = format!({}fixed")