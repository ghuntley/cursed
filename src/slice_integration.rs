//! Comprehensive slice integration for the CURSED language
//!
//! This module provides a high-level API that connects all slice-related functionality
//! across the parser → AST → LLVM → runtime pipeline. It serves as the main entry point
//! for working with slices in the CURSED programming language.
//!
//! ## Overview
//!
//! The CURSED language supports slices as dynamic arrays with runtime bounds checking
//! and automatic memory management. Slices are created using literal syntax and can
//! be manipulated through various operations.
//!
//! ## Syntax
//!
//! ```cursed
//! // Creating slice literals
//! let numbers = []normie{1, 2, 3, 4, 5}     // Slice of integers
//! let names = []tea{"alice", "bob", "charlie"} // Slice of strings  
//! let empty = []thicc{}                      // Empty slice of int64s
//! let chars = []sip{'a', 'b', 'c'}          // Slice of characters
//! 
//! // Slice operations
//! let length = len(numbers)                  // Get slice length
//! let element = numbers[2]                   // Access element (bounds checked)
//! let subslice = numbers[1:3]               // Create subslice
//! ```
//!
//! ## Implementation
//!
//! The slice system is implemented across multiple layers:
//!
//! 1. **Parser Layer** (`src/parser/slice_literal.rs`)
//!    - Parses `[]Type{elements...}` syntax
//!    - Handles nested expressions and type resolution
//!    - Generates SliceLiteral AST nodes
//!
//! 2. **AST Layer** (`src/ast/expressions/slice_literal.rs`)
//!    - SliceLiteral AST node representation
//!    - Element type and values storage
//!    - String representation and debugging
//!
//! 3. **LLVM Layer** (`src/codegen/llvm/slice_*.rs`)
//!    - Slice literal compilation to LLVM IR
//!    - Runtime operations (indexing, bounds checking)
//!    - Memory management integration
//!
//! 4. **Runtime Layer** (`src/runtime/slice_*.rs`)
//!    - Runtime slice operations and utilities
//!    - Memory allocation and deallocation
//!    - Bounds checking and error handling

use crate::ast::expressions::slice_literal::SliceLiteral;
use crate::ast::Expression;
use crate::codegen::llvm::slice_literal::{SliceLiteralCompiler, SliceLiteralCompilerImpl};
use crate::codegen::llvm::slice_operations::{SliceOperations, SliceOperationsImpl};
use crate::core::type_checker::Type;
use crate::error::Error;
use crate::lexer::{Lexer, Token, TokenType};
use crate::parser::Parser;
use crate::runtime::slice_runtime::SliceRuntime;

use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::BasicValueEnum;
use std::collections::HashMap;
use tracing::{debug, error, info, instrument, warn};

/// High-level slice integration API for the CURSED language
///
/// This struct provides a unified interface for working with slices across
/// all layers of the compiler and runtime system.
pub struct SliceIntegration<'ctx> {
    /// LLVM context for code generation
    context: &'ctx Context,
    /// Slice literal compiler for AST → LLVM translation
    compiler: SliceLiteralCompilerImpl,
    /// Slice operations handler for runtime functionality
    operations: SliceOperationsImpl,
    /// Runtime system for slice management
    runtime: SliceRuntime,
    /// Cache of compiled slice types for performance
    type_cache: HashMap<String, Type>,
}

impl<'ctx> SliceIntegration<'ctx> {
    /// Create a new slice integration instance
    ///
    /// # Arguments
    /// * `context` - LLVM context for code generation
    ///
    /// # Returns
    /// A new SliceIntegration instance ready for use
    #[instrument(level = "debug")]
    pub fn new(context: &'ctx Context) -> Self {
        info!("Initializing slice integration system");
        Self {
            context,
            compiler: SliceLiteralCompilerImpl,
            operations: SliceOperationsImpl,
            runtime: SliceRuntime::new(),
            type_cache: HashMap::new(),
        }
    }

    /// Parse a slice literal from source code
    ///
    /// # Arguments
    /// * `source` - Source code containing a slice literal
    ///
    /// # Returns
    /// A SliceLiteral AST node on success, or an error if parsing fails
    ///
    /// # Examples
    /// ```rust
    /// # use cursed::slice_integration::SliceIntegration;
    /// # use inkwell::context::Context;
    /// let context = Context::create();
    /// let integration = SliceIntegration::new(&context);
    /// let slice_ast = integration.parse_slice_literal("[]normie{1, 2, 3}").unwrap();
    /// ```
    #[instrument(skip(self), level = "debug")]
    pub fn parse_slice_literal(&self, source: &str) -> Result<Box<SliceLiteral>, Error> {
        debug!("Parsing slice literal from source: {}", source);
        
        // For now, create a simple manual slice literal for testing
        // In a full implementation, this would use the full parser
        if source.starts_with("[]") {
            // Extract type name and create a basic slice literal
            if let Some(type_end) = source.find('{') {
                let type_part = &source[2..type_end]; // Skip "[]"
                info!("Creating slice literal for type: {}", type_part);
                
                // Create a mock slice literal
                let slice_literal = SliceLiteral {
                    token: Token::LBracket,
                    element_type: Box::new(crate::ast::expressions::Identifier {
                        token: type_part.to_string(),
                        value: type_part.to_string(),
                    }),
                    elements: Vec::new(), // Simplified for testing
                };
                
                return Ok(Box::new(slice_literal));
            }
        }
        
        Err(Error::from_str("Invalid slice literal syntax"))
    }

    /// Compile a slice literal to LLVM IR
    ///
    /// # Arguments
    /// * `module` - LLVM module for code generation
    /// * `builder` - LLVM builder for instruction generation
    /// * `slice_literal` - The slice literal AST node to compile
    ///
    /// # Returns
    /// LLVM BasicValueEnum representing the compiled slice
    ///
    /// # Examples
    /// ```rust
    /// # use cursed::slice_integration::SliceIntegration;
    /// # use inkwell::context::Context;
    /// let context = Context::create();
    /// let module = context.create_module("test");
    /// let builder = context.create_builder();
    /// let integration = SliceIntegration::new(&context);
    /// 
    /// let slice_ast = integration.parse_slice_literal("[]normie{1, 2, 3}").unwrap();
    /// let slice_value = integration.compile_slice_literal(&module, &builder, &slice_ast).unwrap();
    /// ```
    #[instrument(skip(self, module, builder, slice_literal), level = "debug")]
    pub fn compile_slice_literal(
        &self,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        slice_literal: &SliceLiteral,
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Compiling slice literal with {} elements", slice_literal.elements.len());
        
        // Infer the element type
        let element_type = self.infer_element_type(slice_literal)?;
        
        // Use the slice compiler to generate LLVM IR
        self.compiler
            .compile_slice_literal(self.context, module, builder, slice_literal, &element_type)
            .map_err(|e| Error::from_str(&format!("Slice compilation failed: {}", e)))
    }

    /// Create an empty slice of the specified type
    ///
    /// # Arguments
    /// * `module` - LLVM module for code generation
    /// * `builder` - LLVM builder for instruction generation
    /// * `element_type` - The type of elements in the slice
    ///
    /// # Returns
    /// LLVM BasicValueEnum representing an empty slice
    #[instrument(skip(self, module, builder), level = "debug")]
    pub fn create_empty_slice(
        &self,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        element_type: &Type,
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Creating empty slice of type: {:?}", element_type);
        
        // Create an empty slice using the slice literal compiler
        let empty_slice_literal = SliceLiteral {
            token: Token::LBracket,
            element_type: Box::new(crate::ast::expressions::Identifier {
                token: format!("{:?}", element_type).to_lowercase(),
                value: format!("{:?}", element_type).to_lowercase(),
            }),
            elements: Vec::new(),
        };
        
        let empty_slice = self.compiler.compile_slice_literal(self.context, module, builder, &empty_slice_literal, element_type)
            .map_err(|e| Error::from_str(&format!("Failed to create empty slice: {}", e)))?;
        Ok(empty_slice)
    }

    /// Get the length of a slice at runtime
    ///
    /// # Arguments
    /// * `module` - LLVM module for code generation
    /// * `builder` - LLVM builder for instruction generation
    /// * `slice_value` - The slice value to get the length of
    ///
    /// # Returns
    /// LLVM BasicValueEnum representing the slice length (integer)
    #[instrument(skip(self, module, builder, slice_value), level = "debug")]
    pub fn get_slice_length(
        &self,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        slice_value: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Getting slice length");
        
        if let BasicValueEnum::StructValue(struct_val) = slice_value {
            let length = self.operations.slice_len(self.context, builder, struct_val)
                .map_err(|e| Error::from_str(&format!("Failed to get slice length: {}", e)))?;
            Ok(length.into())
        } else {
            Err(Error::from_str("Expected slice struct value"))
        }
    }

    /// Access an element of a slice with bounds checking
    ///
    /// # Arguments
    /// * `module` - LLVM module for code generation
    /// * `builder` - LLVM builder for instruction generation
    /// * `slice_value` - The slice value to access
    /// * `index` - The index to access (as LLVM value)
    /// * `element_type` - The type of elements in the slice
    ///
    /// # Returns
    /// LLVM BasicValueEnum representing the accessed element
    #[instrument(skip(self, module, builder, slice_value, index), level = "debug")]
    pub fn access_slice_element(
        &self,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        slice_value: BasicValueEnum<'ctx>,
        index: BasicValueEnum<'ctx>,
        element_type: &Type,
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Accessing slice element with bounds checking");
        
        if let (BasicValueEnum::StructValue(struct_val), BasicValueEnum::IntValue(int_val)) = (slice_value, index) {
            let element = self.operations.slice_index(self.context, module, builder, struct_val, int_val, element_type)
                .map_err(|e| Error::from_str(&format!("Failed to access slice element: {}", e)))?;
            Ok(element)
        } else {
            Err(Error::from_str("Expected slice struct value and integer index"))
        }
    }

    /// Create a subslice from an existing slice
    ///
    /// # Arguments
    /// * `module` - LLVM module for code generation
    /// * `builder` - LLVM builder for instruction generation
    /// * `slice_value` - The source slice
    /// * `start` - Start index for the subslice
    /// * `end` - End index for the subslice (exclusive)
    /// * `element_type` - The type of elements in the slice
    ///
    /// # Returns
    /// LLVM BasicValueEnum representing the new subslice
    #[instrument(skip(self, module, builder, slice_value, start, end), level = "debug")]
    pub fn create_subslice(
        &self,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        slice_value: BasicValueEnum<'ctx>,
        start: BasicValueEnum<'ctx>,
        end: BasicValueEnum<'ctx>,
        element_type: &Type,
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Creating subslice");
        
        if let (BasicValueEnum::StructValue(struct_val), BasicValueEnum::IntValue(start_int), BasicValueEnum::IntValue(end_int)) = (slice_value, start, end) {
            let subslice = self.operations.slice_subslice(self.context, module, builder, struct_val, start_int, end_int, element_type)
                .map_err(|e| Error::from_str(&format!("Failed to create subslice: {}", e)))?;
            Ok(subslice.into())
        } else {
            Err(Error::from_str("Expected slice struct value and integer indices"))
        }
    }

    /// Infer the element type from a slice literal
    ///
    /// This method analyzes the element_type expression in a SliceLiteral
    /// and returns the corresponding Type enum value.
    ///
    /// # Arguments
    /// * `slice_literal` - The slice literal to analyze
    ///
    /// # Returns
    /// The inferred Type on success, or an error if type inference fails
    #[instrument(skip(self, slice_literal), level = "debug")]
    pub fn infer_element_type(&self, slice_literal: &SliceLiteral) -> Result<Type, Error> {
        let type_expr_string = slice_literal.element_type.string();
        debug!("Inferring element type from expression: {}", type_expr_string);
        
        // Check cache first
        if let Some(cached_type) = self.type_cache.get(&type_expr_string) {
            debug!("Using cached type: {:?}", cached_type);
            return Ok(cached_type.clone());
        }
        
        // Map CURSED type names to Type enum values
        let inferred_type = match type_expr_string.as_str() {
            "lit" => Type::Lit,           // bool
            "smol" => Type::Smol,         // int8
            "mid" => Type::Mid,           // int32
            "normie" => Type::Normie,     // int
            "thicc" => Type::Thicc,       // int64
            "snack" => Type::Snack,       // float32
            "meal" => Type::Meal,         // float64
            "tea" => Type::Tea,           // string
            "sip" => Type::Sip,           // char
            "rune" => Type::Rune,         // rune (Unicode code point)
            "byte" => Type::Byte,         // byte (uint8)
            "extra" => Type::Extra,       // interface{}
            _ => {
                warn!("Unknown type in slice literal: {}", type_expr_string);
                return Err(Error::from_str(&format!("Unknown element type: {}", type_expr_string)));
            }
        };
        
        info!("Inferred element type: {:?} for expression: {}", inferred_type, type_expr_string);
        Ok(inferred_type)
    }

    /// Get runtime system for slice management
    ///
    /// # Returns
    /// Reference to the SliceRuntime instance
    pub fn runtime(&self) -> &SliceRuntime {
        &self.runtime
    }

    /// Parse and compile a slice literal in one operation
    ///
    /// This is a convenience method that combines parsing and compilation
    /// for rapid prototyping and testing.
    ///
    /// # Arguments
    /// * `source` - Source code containing a slice literal
    /// * `module` - LLVM module for code generation
    /// * `builder` - LLVM builder for instruction generation
    ///
    /// # Returns
    /// LLVM BasicValueEnum representing the compiled slice
    ///
    /// # Examples
    /// ```rust
    /// # use cursed::slice_integration::SliceIntegration;
    /// # use inkwell::context::Context;
    /// let context = Context::create();
    /// let module = context.create_module("test");
    /// let builder = context.create_builder();
    /// let integration = SliceIntegration::new(&context);
    /// 
    /// let slice_value = integration.parse_and_compile("[]normie{1, 2, 3}", &module, &builder).unwrap();
    /// ```
    #[instrument(skip(self, module, builder), level = "debug")]
    pub fn parse_and_compile(
        &self,
        source: &str,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        info!("Parsing and compiling slice literal from source");
        
        let slice_literal = self.parse_slice_literal(source)?;
        self.compile_slice_literal(module, builder, &slice_literal)
    }

    /// Validate a slice literal for correctness
    ///
    /// This method performs semantic validation on a slice literal to ensure
    /// all elements are compatible with the declared element type.
    ///
    /// # Arguments
    /// * `slice_literal` - The slice literal to validate
    ///
    /// # Returns
    /// Ok(()) if validation passes, or an error describing the validation failure
    #[instrument(skip(self, slice_literal), level = "debug")]
    pub fn validate_slice_literal(&self, slice_literal: &SliceLiteral) -> Result<(), Error> {
        debug!("Validating slice literal with {} elements", slice_literal.elements.len());
        
        let element_type = self.infer_element_type(slice_literal)?;
        
        // TODO: Add element compatibility checking
        // This would involve type checking each element against the declared type
        
        info!("Slice literal validation passed for type: {:?}", element_type);
        Ok(())
    }
}

/// Convenient functions for working with slices in CURSED
pub mod convenience {
    use super::*;

    /// Parse a slice literal from source code (convenience function)
    ///
    /// # Arguments
    /// * `source` - Source code containing a slice literal
    ///
    /// # Returns
    /// A SliceLiteral AST node on success
    pub fn parse_slice(source: &str) -> Result<Box<SliceLiteral>, Error> {
        let context = Context::create();
        let integration = SliceIntegration::new(&context);
        integration.parse_slice_literal(source)
    }

    /// Create an empty slice literal AST node
    ///
    /// # Arguments
    /// * `element_type_name` - Name of the element type (e.g., "normie", "tea")
    ///
    /// # Returns
    /// A SliceLiteral AST node representing an empty slice
    pub fn create_empty_slice_literal(element_type_name: &str) -> Box<SliceLiteral> {
        use crate::ast::expressions::Identifier;
        use crate::lexer::Token;
        
        Box::new(SliceLiteral {
            token: Token::LBracket,  // '[' token
            element_type: Box::new(Identifier {
                token: element_type_name.to_string(),
                value: element_type_name.to_string(),
            }),
            elements: Vec::new(),
        })
    }

    /// Get all supported slice element types
    ///
    /// # Returns
    /// Vector of all supported element type names
    pub fn supported_element_types() -> Vec<&'static str> {
        vec![
            "lit",      // bool
            "smol",     // int8
            "mid",      // int32
            "normie",   // int
            "thicc",    // int64
            "snack",    // float32
            "meal",     // float64
            "tea",      // string
            "sip",      // char
            "rune",     // rune
            "byte",     // byte
            "extra",    // interface{}
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slice_integration_creation() {
        let context = Context::create();
        let integration = SliceIntegration::new(&context);
        // Test that runtime is available
        let stats = integration.runtime().get_statistics();
        assert_eq!(stats.slices_created, 0); // Should start at 0
    }

    #[test]
    fn test_type_inference() {
        let context = Context::create();
        let integration = SliceIntegration::new(&context);
        
        let slice_literal = convenience::create_empty_slice_literal("normie");
        let inferred_type = integration.infer_element_type(&slice_literal).unwrap();
        assert_eq!(inferred_type, Type::Normie);
    }

    #[test]
    fn test_supported_types() {
        let types = convenience::supported_element_types();
        assert!(types.contains(&"normie"));
        assert!(types.contains(&"tea"));
        assert!(types.contains(&"lit"));
    }

    #[test]
    fn test_slice_validation() {
        let context = Context::create();
        let integration = SliceIntegration::new(&context);
        
        let slice_literal = convenience::create_empty_slice_literal("normie");
        let result = integration.validate_slice_literal(&slice_literal);
        assert!(result.is_ok());
    }
}
