//! Builder helpers for LLVM code generation
//! This module handles building LLVM IR instructions

use inkwell::values::{BasicValueEnum, FunctionValue, PointerValue};
use inkwell::types::{BasicTypeEnum, BasicType, BasicMetadataTypeEnum};
use super::context::LlvmCodeGenerator;

impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Create a local variable in the current function
    pub fn create_local_variable(
        &mut self,
        name: &str,
        llvm_type: BasicTypeEnum<'ctx>,
        initial_value: Option<BasicValueEnum<'ctx>>
    ) -> Result<PointerValue<'ctx>, String> {
        if !self.in_function() {
            return Err("Cannot create local variable outside of function".to_string());
        }
        
        // Allocate memory for the variable
        let alloca = self.create_entry_block_alloca(llvm_type, name);
        
        // Store the initial value if provided
        if let Some(value) = initial_value {
            if value.get_type() != llvm_type {
                return Err(format!(
                    "Type mismatch: variable '{}' has type {:?} but got value of type {:?}",
                    name, llvm_type, value.get_type()
                ));
            }
            
            self.builder.build_store(alloca, value).unwrap();
        }
        
        // Register the variable
        self.variables.insert(name.to_string(), (alloca, llvm_type));
        
        Ok(alloca)
    }
    
    /// Load a variable value from a pointer
    pub fn load_variable(&self, name: &str, ptr: PointerValue<'ctx>, ty: BasicTypeEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, String> {
        let value = match ty {
            BasicTypeEnum::IntType(int_type) => {
                self.builder.build_load(int_type, ptr, name).unwrap()
            },
            BasicTypeEnum::FloatType(float_type) => {
                self.builder.build_load(float_type, ptr, name).unwrap()
            },
            BasicTypeEnum::PointerType(ptr_type) => {
                self.builder.build_load(ptr_type, ptr, name).unwrap()
            },
            BasicTypeEnum::StructType(struct_type) => {
                self.builder.build_load(struct_type, ptr, name).unwrap()
            },
            BasicTypeEnum::ArrayType(array_type) => {
                self.builder.build_load(array_type, ptr, name).unwrap()
            },
            BasicTypeEnum::VectorType(vector_type) => {
                self.builder.build_load(vector_type, ptr, name).unwrap()
            },
        };
        
        Ok(value)
    }
    
    /// Create a new function
    pub fn create_function(
        &mut self,
        name: &str,
        param_types: &[BasicTypeEnum<'ctx>],
        return_type: BasicTypeEnum<'ctx>,
        is_variadic: bool
    ) -> Result<FunctionValue<'ctx>, String> {
        // Check if the function already exists
        if let Some(existing) = self.module.get_function(name) {
            return Ok(existing);
        }
        
        // Convert parameter types to metadata types
        let metadata_param_types: Vec<BasicMetadataTypeEnum<'ctx>> = param_types.iter()
            .map(|&ty| ty.into())
            .collect();
        
        // Create function type
        let fn_type = return_type.fn_type(&metadata_param_types, is_variadic);
        
        // Add the function to the module
        let function = self.module.add_function(name, fn_type, None);
        
        // Register the function
        self.functions.insert(name.to_string(), function);
        
        Ok(function)
    }
}