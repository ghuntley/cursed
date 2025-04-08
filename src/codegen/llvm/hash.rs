//! LLVM code generation for hash map operations

use inkwell::values::BasicValueEnum;
use crate::ast::HashLiteral;
use super::context::LlvmCodeGenerator;

impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Compile a hash literal expression
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
    
    /// Initialize hash map runtime functions
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