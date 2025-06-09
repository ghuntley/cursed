//! LLVM code generation for hash map operations in the CURSED language.
//!
//! This module provides functionality for translating CURSED hash map (dictionary)
//! operations into LLVM IR. It handles hash map literals (creating maps with initial
//! key-value pairs) and defines the interface for hash map runtime operations.
//!
//! The implementation provides:
//! - Creation of hash maps with key-value pairs
//! - Declaration of runtime functions for hash map operations
//! - Conversion between CURSED values and their hash map representation
//!
//! Hash maps in CURSED are similar to maps in Go, providing an unordered collection
//! of key-value pairs where keys must be hashable types. They are implemented as
//! runtime data structures with a C-style API for operations.

use inkwell::values::BasicValueEnum;
use crate::ast::HashLiteral;
use crate::ast::expressions::IndexExpression;
use crate::core::type_checker::Type;
use crate::error_enhanced::CursedError;
use super::context::LlvmCodeGenerator;
use super::map_operations::{MapOperations, create_map_operations};
use super::expression::ExpressionCompilation;
use tracing::{debug, info, warn};

impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Compiles a hash map literal expression to LLVM IR.
    ///
    /// This method translates a CURSED hash literal (like `{"key": value, 42: "answer"}`) 
    /// into LLVM IR instructions that create and initialize a hash map. It performs
    /// the following steps:
    ///
    /// 1. Initializes the hash map runtime functions if not already done
    /// 2. Calls the runtime function to create an empty hash map
    /// 3. For each key-value pair in the literal:
    ///    a. Compiles the key and value expressions
    ///    b. Converts them to a format suitable for the hash map (void pointers)
    ///    c. Calls the insert function to add the pair to the hash map
    /// 4. Returns a pointer to the created hash map
    ///
    /// # Arguments
    ///
    /// * `hash_lit` - The AST node representing the hash literal
    ///
    /// # Returns
    ///
    /// * `Result<BasicValueEnum, String>` - A pointer to the created hash map, or an error message
    pub fn compile_hash_literal(&mut self, hash_lit: &HashLiteral) -> Result<BasicValueEnum<'ctx>, String> {
        info!("Compiling hash literal with {} pairs", hash_lit.pairs.len());

        // TODO: Infer key and value types from the literal pairs
        // For now, use generic types - this needs proper type inference
        let key_type = Type::Tea; // Default assumption
        let value_type = Type::Thicc;  // Default assumption

        let map_ops = create_map_operations();

        // Compile key-value pairs
        let mut compiled_pairs = Vec::new();
        for (key_expr, value_expr) in &hash_lit.pairs {
            let key_val = self.compile_expression(key_expr.as_ref())
                .map_err(|e| format!("Failed to compile key expression: {}", e))?;
            let value_val = self.compile_expression(value_expr.as_ref())
                .map_err(|e| format!("Failed to compile value expression: {}", e))?;
            compiled_pairs.push((key_val, value_val));
        }

        // Create map literal using new map operations
        let map_struct = map_ops
            .create_map_literal(
                &self.context,
                &self.module,
                &self.builder,
                &compiled_pairs,
                &key_type,
                &value_type,
            )
            .map_err(|e| format!("Failed to create map literal: {}", e))?;

        debug!("Hash literal compilation completed successfully");
        Ok(map_struct.into())
    }

    /// Compiles a map indexing expression (map[key])
    pub fn compile_map_index(&mut self, index_expr: &IndexExpression, map_type: &Type) -> Result<BasicValueEnum<'ctx>, String> {
        info!("Compiling map index expression");

        // Extract key and value types from map type
        let (key_type, value_type) = match map_type {
            Type::Map(key, value) => (key.as_ref(), value.as_ref()),
            _ => return Err("Index expression not on a map type".to_string()),
        };

        // Compile the map expression
        let map_val = self.compile_expression(&*index_expr.left)
            .map_err(|e| format!("Failed to compile map expression: {}", e))?;
        let map_struct = map_val.into_struct_value();

        // Compile the key expression
        let key_val = self.compile_expression(&*index_expr.index)
            .map_err(|e| format!("Failed to compile key expression: {}", e))?;

        let map_ops = create_map_operations();

        // Get value from map
        let result = map_ops
            .map_get(
                &self.context,
                &self.module,
                &self.builder,
                map_struct,
                key_val,
                key_type,
                value_type,
            )
            .map_err(|e| format!("Failed to get value from map: {}", e))?;

        debug!("Map index compilation completed successfully");
        Ok(result)
    }

    /// Compiles a map assignment expression (map[key] = value)
    pub fn compile_map_assignment(
        &mut self,
        index_expr: &IndexExpression,
        value_expr: &dyn crate::ast::Expression,
        map_type: &Type,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        info!("Compiling map assignment expression");

        // Extract key and value types from map type
        let (key_type, value_type) = match map_type {
            Type::Map(key, value) => (key.as_ref(), value.as_ref()),
            _ => return Err("Assignment not on a map type".to_string()),
        };

        // Compile the map expression
        let map_val = self.compile_expression(&*index_expr.left)
            .map_err(|e| format!("Failed to compile map expression: {}", e))?;
        let map_struct = map_val.into_struct_value();

        // Compile the key and value expressions
        let key_val = self.compile_expression(&*index_expr.index)
            .map_err(|e| format!("Failed to compile key expression: {}", e))?;
        let value_val = self.compile_expression(value_expr)
            .map_err(|e| format!("Failed to compile value expression: {}", e))?;

        let map_ops = create_map_operations();

        // Set value in map
        let updated_map = map_ops
            .map_set(
                &self.context,
                &self.module,
                &self.builder,
                map_struct,
                key_val,
                value_val,
                key_type,
                value_type,
            )
            .map_err(|e| format!("Failed to set value in map: {}", e))?;

        debug!("Map assignment compilation completed successfully");
        Ok(updated_map.into())
    }
    
    /// Initializes the runtime functions needed for hash map operations.
    ///
    /// This method declares the external functions that implement hash map operations.
    /// These functions would typically be provided by a runtime library that's linked
    /// with the compiled CURSED program. The functions include:
    ///
    /// - `create_hashmap`: Creates a new empty hash map
    /// - `hashmap_insert`: Inserts a key-value pair into a hash map
    /// - `hashmap_get`: Retrieves a value for a key from a hash map
    /// - `hashmap_remove`: Removes a key-value pair from a hash map
    /// - `hashmap_has_key`: Checks if a key exists in a hash map
    /// - `hashmap_size`: Returns the number of key-value pairs in a hash map
    ///
    /// The method is idempotent - it only initializes the functions if they haven't
    /// been initialized already.
    fn init_hash_functions(&mut self) {
        // Skip initialization if we've already done it
        if self.module.get_function("create_hashmap").is_some() {
            return;
        }
        
        // Void pointer type
        let void_ptr = self.context.i8_type().ptr_type(inkwell::AddressSpace::default());
        
        // Create a new hashmap
        let create_hashmap_type = void_ptr.fn_type(&[], false);
        self.module.add_function("create_hashmap", create_hashmap_type, None);
        
        // Insert a key-value pair
        let insert_type = self.context.void_type().fn_type(&[void_ptr.into(), void_ptr.into(), void_ptr.into()], false);
        self.module.add_function("hashmap_insert", insert_type, None);
        
        // Get a value by key
        let get_type = void_ptr.fn_type(&[void_ptr.into(), void_ptr.into()], false);
        self.module.add_function("hashmap_get", get_type, None);
        
        // Remove a key-value pair
        let remove_type = self.context.void_type().fn_type(&[void_ptr.into(), void_ptr.into()], false);
        self.module.add_function("hashmap_remove", remove_type, None);
        
        // Check if a key exists
        let has_key_type = self.context.bool_type().fn_type(&[void_ptr.into(), void_ptr.into()], false);
        self.module.add_function("hashmap_has_key", has_key_type, None);
        
        // Get hashmap size
        let size_type = self.context.i64_type().fn_type(&[void_ptr.into()], false);
        self.module.add_function("hashmap_size", size_type, None);
    }
}