use std::path::PathBuf;
use std::sync::Arc;
use inkwell::context::Context;
use inkwell::module::Module;
use cursed::ast::expressions::{TypeAssertion, TypeAssertionQuestion};
use cursed::ast::expressions::Expression;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::interface_type_assertion_filesystem_integration::InterfaceTypeAssertionFilesystemIntegration;
use cursed::error::Error;
use cursed::error::SourceLocation;
use self::common::tracing;
use self::common::timing::Timer;

#[cfg(test)]
mod tests {
    
    
    
    #[path = "common/mod.rs"]
    mod common;
    
    // Initialize tracing for tests
    
    // Import the Timer utility for benchmarking
    
    // Macro to set up tracing
    macro_rules! init_tracing {
        () => {
            common::tracing::setup();
        };
    }
    
    #[test]
    fn test_source_location_creation() {
        init_tracing!();
        
        let ctx = Context::create();
        let module = Module::create("test", &ctx);
        let builder = ctx.create_builder();
        
        let mut code_gen = LlvmCodeGenerator::new(&ctx, module, builder);
        
        // Set up a source directory
        code_gen.set_source_directory("examples".to_string());
        
        // Create a source location
        let location = code_gen.create_enhanced_source_location(
            "examples/interface_type_assertion_filesystem_source_location.csd:167:22",
            "shape",
            "Circle"
        );
        
        assert!(location.is_some(), "Location should be created");
        
        let location = location.unwrap();
        assert_eq!(location.line, 167);
        assert_eq!(location.column, 22);
        assert!(location.file.is_some(), "File path should be resolved");
        assert_eq!(location.source_line, "shape.(Circle)?");
    }
    
    #[test]
    fn test_context_line_range() {
        init_tracing!();
        
        let ctx = Context::create();
        let module = Module::create("test", &ctx);
        let builder = ctx.create_builder();
        
        let code_gen = LlvmCodeGenerator::new(&ctx, module, builder);
        
        // Test middle of file
        let (start, end) = code_gen.get_context_line_range(50, 100);
        assert!(start > 0);
        assert!(end < 100);
        assert!(end > 50);
        assert!(start <= 50);
        
        // Test near beginning
        let (start, end) = code_gen.get_context_line_range(2, 100);
        assert_eq!(start, 0);
        
        // Test near end
        let (start, end) = code_gen.get_context_line_range(99, 100);
        assert_eq!(end, 99);
    }
    
    #[test]
    fn test_enhance_error_with_source_context() {
        init_tracing!();
        
        let ctx = Context::create();
        let module = Module::create("test", &ctx);
        let builder = ctx.create_builder();
        
        let mut code_gen = LlvmCodeGenerator::new(&ctx, module, builder);
        
        // Set up a source directory
        code_gen.set_source_directory("examples".to_string());
        
        // Create a source location
        let location = SourceLocation {
            line: 167,
            column: 22,
            file: Some("examples/interface_type_assertion_filesystem_source_location.csd".to_string()),
            source_line: "shape.(Circle)?".to_string(),
        };
        
        // Enhance an error message
        let error_message = "Type assertion failed".to_string();
        let enhanced = code_gen.enhance_error_with_source_context(error_message, &location);
        
        // Should contain the original error and context information
        assert!(enhanced.contains("Type assertion failed"));
        assert!(enhanced.contains("Source context"));
        assert!(enhanced.contains(">"));
        assert!(enhanced.contains("^"));
    }
    
    #[test]
    fn test_resolve_source_path() {
        init_tracing!();
        
        let ctx = Context::create();
        let module = Module::create("test", &ctx);
        let builder = ctx.create_builder();
        
        let mut code_gen = LlvmCodeGenerator::new(&ctx, module, builder);
        
        // Set up a source directory
        code_gen.set_source_directory("examples".to_string());
        
        // Test resolving a file path
        let path = code_gen.resolve_source_path(Some("interface_type_assertion_filesystem_source_location.csd");
        
        assert!(path.is_some(), "Path should be resolved");
        let path = path.unwrap();
        assert!(path.exists(), "Resolved path should exist");
        assert!(path.to_string_lossy().contains("interface_type_assertion_filesystem_source_location.csd"));
    }
}