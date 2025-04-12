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
use super::context::LlvmCodeGenerator;

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
        // First, we need to initialize the hash map runtime
        self.init_hash_functions();
        
        // Get the create_hashmap function
        let create_fn = self.module.get_function("create_hashmap").ok_or_else(|| 
            "create_hashmap function not found".to_string()
        )?;
        
        // Call the function to create an empty hashmap
        let hashmap = self.builder.build_call(
            create_fn,
            &[],
            "new_hashmap"
        ).unwrap();
        
        let hashmap_ptr = hashmap.try_as_basic_value().left().unwrap();
        
        // For each pair, add it to the hashmap
        for (key, value) in &hash_lit.pairs {
            // Compile the key and value
            let key_val = self.compile_expression(key.as_ref())?;
            let value_val = self.compile_expression(value.as_ref())?;
            
            // Get the insert function
            let insert_fn = self.module.get_function("hashmap_insert").ok_or_else(|| 
                "hashmap_insert function not found".to_string()
            )?;
            
            // Convert key and value to void pointers if necessary
            let key_ptr = if key_val.is_pointer_value() {
                key_val.into_pointer_value()
            } else {
                // Store the key in a temporary alloca
                let key_type = key_val.get_type();
                let key_alloca = self.builder.build_alloca(key_type, "key_temp").unwrap();
                self.builder.build_store(key_alloca, key_val).unwrap();
                key_alloca
            };
            
            let value_ptr = if value_val.is_pointer_value() {
                value_val.into_pointer_value()
            } else {
                // Store the value in a temporary alloca
                let value_type = value_val.get_type();
                let value_alloca = self.builder.build_alloca(value_type, "value_temp").unwrap();
                self.builder.build_store(value_alloca, value_val).unwrap();
                value_alloca
            };
            
            // Cast to void pointers
            let void_ptr_type = self.context.i8_type().ptr_type(inkwell::AddressSpace::default());
            let key_void_ptr = self.builder.build_bitcast(key_ptr, void_ptr_type, "key_void_ptr").unwrap();
            let value_void_ptr = self.builder.build_bitcast(value_ptr, void_ptr_type, "value_void_ptr").unwrap();
            
            // Call the insert function
            self.builder.build_call(
                insert_fn,
                &[hashmap_ptr.into(), key_void_ptr.into(), value_void_ptr.into()],
                "insert_result"
            ).unwrap();
        }
        
        // Return the hashmap pointer
        Ok(hashmap_ptr)
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