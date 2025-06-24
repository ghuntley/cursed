// Enhanced error propagation compiler for CURSED language
//
// This module provides advanced LLVM IR generation for the `?` operator
// with context tracking and sophisticated error handling.

use crate::ast::expressions::ErrorPropagation;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::error::{CursedError, Error, SourceLocation};

use tracing::{debug, instrument};

/// Enhanced error propagation compilation trait (non-conflicting)
pub trait EnhancedErrorPropagationCompiler {
    /// Compile error propagation expression with enhanced context
    fn compile_enhanced_error_propagation(&mut self, expr: &ErrorPropagation) -> Result<(), Error>;
}

/// Error propagation context
#[derive(Debug, Clone)]
pub struct ErrorPropagationContext {
    /// Source location of the propagation
    pub source_location: SourceLocation,
    /// Whether this is in tail position
    pub is_tail_position: bool,
    /// Function context
    pub function_context: Option<String>,
}

impl ErrorPropagationContext {
    pub fn new(location: SourceLocation) -> Self {
        Self {
            source_location: location,
            is_tail_position: false,
            function_context: None,
        }
    }
    
    pub fn with_tail_position(mut self, is_tail: bool) -> Self {
        self.is_tail_position = is_tail;
        self
    }
    
    pub fn with_function_context(mut self, context: String) -> Self {
        self.function_context = Some(context);
        self
    }
}

impl EnhancedErrorPropagationCompiler for LlvmCodeGenerator {
    #[instrument(skip(self, expr))]
    fn compile_enhanced_error_propagation(&mut self, expr: &ErrorPropagation) -> Result<(), Error> {
        debug!("Compiling enhanced error propagation");
        
        let context = ErrorPropagationContext::new(SourceLocation::new(1, 1))
            .with_tail_position(false);
            
        self.compile_error_propagation_enhanced(expr, &context)
    }
}

impl LlvmCodeGenerator {
    /// Enhanced error propagation compilation with context
    fn compile_error_propagation_enhanced(
        &mut self,
        expr: &ErrorPropagation,
        context: &ErrorPropagationContext,
    ) -> Result<(), Error> {
        // Compile the inner expression
        let inner_ir = self.compile_expression_to_string(expr.expression.as_ref())
            .map_err(|e| CursedError::code_generation_error(e.to_string(), None, None))?;
        
        // Generate enhanced error propagation IR
        let temp_name = format!("%enhanced_error_prop_{}", self.next_temp_id());
        let ir = format!(
            "  {} = call i8* @cursed_enhanced_error_propagation(i8* {}, i32 {}, i32 {})",
            temp_name, 
            inner_ir,
            context.source_location.line,
            context.source_location.column
        );
        
        Ok(format!("{}\n{}", inner_ir, ir))
    }
}

// FFI functions for error propagation runtime support
extern "C" {
    /// Enhanced error propagation with context
    fn cursed_enhanced_error_propagation(
        value: *const u8,
        line: u32,
        column: u32,
    ) -> *mut u8;
    
    /// Propagate error through the runtime system
    fn cursed_error_propagation(error_value: *const u8, line: u32, column: u32);
    
    /// Trigger panic for unhandled error propagation
    fn cursed_error_propagation_panic(message: *const u8);
    
    /// Initialize error propagation runtime
    fn cursed_error_propagation_init();
    
    /// Cleanup error propagation runtime
    fn cursed_error_propagation_cleanup();
}
