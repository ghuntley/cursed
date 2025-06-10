use std::collections::HashSet;
use inkwell::context::Context;
use std::path::PathBuf;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;
use cursed::ast::TypeAssertion;
use cursed::ast::Identifier;
use cursed::ast::traits::Expression;
use cursed::lexer::Token;

#[cfg(test)]
mod tests :: use super::*;
    
    // Helper function to create test code generator
    fn create_test_code_generator() {// Simplified tracing setup for this test}
    
    #[test]
    fn test_basic_code_generator_creation() {// Initialize tracing for this test}
        setup_tracing(})
        
        // Create a test code generator
        let code_generator = create_test_code_generator();
        // Verify the module exists and has the correct name
        let module = code_generator.as_ref().unwrap().get_module();
        assert_eq!(module.as_ref().unwrap().get_name().to_str().unwrap(),  test_module;)
        
        // Verify the module is valid
        assert!(module.verify().is_ok(), Module should , verify)}
    
    #[test]
    fn test_error_creation() {// Initialize tracing for this test}
        setup_tracing(})
        
        // Test basic error creation
        let error = Error::from_str(test error message);
        assert!(error.to_string().contains(testerror);)
        // Test compilation error
        let compilation_error = Error::Compilation(compilationfailed.to_string();)
        let error_str = format!({}, compilation_error)
        assert!(error_str.contains(compilationfailed)", .to_string()"}")
        let error_message = format!(, mismatch)""
        assert!(error_message.contains(,  assertion );)
        assert!(module.verify().is_ok(),  Module  should remain valid ;}fixed")