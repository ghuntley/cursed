//! Pointer operations for LLVM code generation
//! 
//! This module implements pointer operations including address-of operations,
//! pointer dereferencing, and memory operations for the LLVM code generator.

use inkwell::values::{BasicValueEnum, PointerValue};
use inkwell::AddressSpace;

use crate::ast::expressions::Identifier;
use crate::ast::pointer::types::PointerType;
use crate::ast::pointer::operations::PointerDereference;
use crate::ast::traits::Expression;
use inkwell::types::BasicTypeEnum;
use crate::error::Error;

use super::context::LlvmCodeGenerator;
use super::expression::ExpressionCompilation;

/// Trait for pointer operations
pub trait PointerOperations<'ctx> {
    /// Get the address of an expression (equivalent to & operator in C)
    fn get_address_of(&mut self, expr: &dyn Expression) -> Result<PointerValue<'ctx>, Error>;
    
    /// Compile a pointer type expression (&var)
    fn compile_pointer_type(&mut self, ptr_type: &PointerType) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Compile a pointer dereference expression (*ptr)
    fn compile_pointer_dereference(&mut self, deref: &PointerDereference) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Load a value from a pointer
    fn load_from_pointer(&mut self, ptr: PointerValue<'ctx>, name: &str) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Store a value to a pointer
    fn store_to_pointer(&mut self, ptr: PointerValue<'ctx>, value: BasicValueEnum<'ctx>) -> Result<(), Error>;
    
    /// Create a null pointer of the given type
    fn create_null_pointer(&mut self, type_name: &str) -> Result<PointerValue<'ctx>, Error>;
}

impl<'ctx> PointerOperations<'ctx> for LlvmCodeGenerator<'ctx> {
    fn get_address_of(&mut self, expr: &dyn Expression) -> Result<PointerValue<'ctx>, Error> {
        let any = expr.as_any();
        
        if let Some(ident) = any.downcast_ref::<Identifier>() {
            // Case 1: Getting address of a variable
            use crate::codegen::llvm::variables::VariableHandling;
            if let Some(var_entry) = self.lookup_variable(&ident.value) {
                // Return the variable's address (which is already a pointer)
                return Ok(var_entry);
            }
            
            // Check if it's a global variable
            if let Some(global) = self.module().get_global(&ident.value) {
                return Ok(global.as_pointer_value());
            }
            
            return Err(Error::codegen(format!("Cannot find variable to take address of: {}", ident.value)));
        }
        
        // Case 2: Need to evaluate the expression, store it, and then take its address
        let value = self.compile_expression(expr)?;
        
        // Create a temporary alloca to store the value
        let temp_name = format!("addr_temp_{}", self.string_literal_counter);
        self.string_literal_counter += 1;
        
        // Create an allocation in the entry block for better optimization potential
        let ptr_type = self.context().i8_type().ptr_type(inkwell::AddressSpace::default());
        // For matching on type
        use inkwell::types::BasicTypeEnum;
        
        let alloca = match value.get_type() {
            BasicTypeEnum::IntType(int_type) => {
                self.create_entry_block_alloca(int_type, &temp_name)
            },
            BasicTypeEnum::FloatType(float_type) => {
                self.create_entry_block_alloca(float_type, &temp_name)
            },
            BasicTypeEnum::PointerType(ptr_type) => {
                self.create_entry_block_alloca(ptr_type, &temp_name)
            },
            BasicTypeEnum::StructType(struct_type) => {
                self.create_entry_block_alloca(struct_type, &temp_name)
            },
            _ => return Err(Error::codegen(format!("Unsupported type for address-of operation")))
        };
        
        // Store the value in the temporary location
        self.store_to_pointer(alloca, value)?;
        
        Ok(alloca)
    }
    
    fn compile_pointer_type(&mut self, ptr_type: &PointerType) -> Result<BasicValueEnum<'ctx>, Error> {
        // Get the address of the target expression
        let ptr = self.get_address_of(&*ptr_type.target_type)?;
        Ok(ptr.into())
    }
    
    fn compile_pointer_dereference(&mut self, deref: &PointerDereference) -> Result<BasicValueEnum<'ctx>, Error> {
        // Compile the pointer expression
        let ptr_value = self.compile_expression(&*deref.pointer)?;
        
        // Make sure it's a pointer
        if !ptr_value.is_pointer_value() {
            return Err(Error::codegen(format!("Cannot dereference non-pointer value")));
        }
        
        let ptr = ptr_value.into_pointer_value();
        
        // Load the value from the pointer
        let name = format!("deref_{}", self.string_literal_counter);
        self.string_literal_counter += 1;
        
        // Check for null pointer
        // Call this implementation directly to avoid the ambiguity
        let is_null = ptr_value.is_pointer_value() && ptr_value.into_pointer_value().is_null();
        if is_null {
            // Handle null pointer dereference by returning a default value
            return Ok(self.context().i32_type().const_zero().into());
        }
        
        self.load_from_pointer(ptr, &name)
    }
    
    fn load_from_pointer(&mut self, ptr: PointerValue<'ctx>, name: &str) -> Result<BasicValueEnum<'ctx>, Error> {
        // Check for null pointer
        let i8_null = self.context().i8_type().ptr_type(inkwell::AddressSpace::default()).const_null();
        
        // Create basic blocks for comparison and loading
        let function = self.current_function().ok_or_else(|| Error::codegen("No current function for pointer load".to_string()))?;
        let current_block = self.builder().get_insert_block().ok_or_else(|| Error::codegen("No current block".to_string()))?;
        let load_block = self.context().append_basic_block(function, &format!("{}_load", name));
        let null_block = self.context().append_basic_block(function, &format!("{}_null", name));
        let merge_block = self.context().append_basic_block(function, &format!("{}_merge", name));
        
        // Compare pointer with null
        let ptr_as_i8 = self.builder().build_pointer_cast(
            ptr, 
            self.context().i8_type().ptr_type(inkwell::AddressSpace::default()),
            &format!("{}_as_i8_ptr", name)
        ).map_err(|e| Error::codegen(format!("Failed to cast pointer: {}", e)))?;
        
        let cmp = self.builder().build_int_compare(
            inkwell::IntPredicate::EQ,
            ptr_as_i8, // Already a pointer value
            i8_null,
            &format!("{}_null_check", name)
        ).map_err(|e| Error::codegen(format!("Failed to compare pointer with null: {}", e)))?;
        
        self.builder().build_conditional_branch(cmp, null_block, load_block)
            .map_err(|e| Error::codegen(format!("Failed to build branch: {}", e)))?;
        
        // Build the load block
        self.builder().position_at_end(load_block);
        
        // Get the pointee type from the pointer
        // Alternative implementation to handle get_element_type
        // For LLVM pointer types, use a different approach
        let pointee_type = match ptr.get_type() {
            _ => { // Assume it's a pointer for now
                // Just use i8 as a fallback pointee type for simplicity
                self.context().i8_type().into()
            }
        };
        
        // Load the value
        let load_result = self.builder().build_load(pointee_type, ptr, name)
            .map_err(|e| Error::codegen(format!("Failed to load from pointer: {}", e)))?;
        
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| Error::codegen(format!("Failed to build branch: {}", e)))?;
        
        // Build the null block
        self.builder().position_at_end(null_block);
        
        // For null pointers, return a default value based on the type
        // For matching on type
        use inkwell::types::BasicTypeEnum;
        
        let default_value: BasicValueEnum<'ctx> = match pointee_type {
            BasicTypeEnum::IntType(int_type) => int_type.const_zero().into(),
            BasicTypeEnum::FloatType(float_type) => float_type.const_zero().into(),
            BasicTypeEnum::PointerType(ptr_type) => ptr_type.const_null().into(),
            BasicTypeEnum::StructType(_) => {
                // Use a dummy integer for structs
                self.context().i32_type().const_zero().into()
            },
            _ => self.context().i32_type().const_zero().into()
        };
        
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| Error::codegen(format!("Failed to build branch: {}", e)))?;
        
        // Build the merge block
        self.builder().position_at_end(merge_block);
        
        // Create a PHI node to merge results
        let phi = self.builder().build_phi(load_result.get_type(), &format!("{}_phi", name))
            .map_err(|e| Error::codegen(format!("Failed to build phi node: {}", e)))?;
        
        phi.add_incoming(&[(&load_result, load_block), (&default_value, null_block)]);
        
        let result = phi.as_basic_value();
        
        Ok(result)
    }
    
    fn store_to_pointer(&mut self, ptr: PointerValue<'ctx>, value: BasicValueEnum<'ctx>) -> Result<(), Error> {
        // Check for null pointer
        let i8_null = self.context().i8_type().ptr_type(inkwell::AddressSpace::default()).const_null();
        
        // Create basic blocks for comparison and storing
        let function = self.current_function().ok_or_else(|| Error::codegen("No current function for pointer store".to_string()))?;
        let current_block = self.builder().get_insert_block().ok_or_else(|| Error::codegen("No current block".to_string()))?;
        let store_block = self.context().append_basic_block(function, "ptr_store");
        let null_block = self.context().append_basic_block(function, "ptr_null");
        let merge_block = self.context().append_basic_block(function, "ptr_merge");
        
        // Compare pointer with null
        let ptr_as_i8 = self.builder().build_pointer_cast(
            ptr, 
            self.context().i8_type().ptr_type(inkwell::AddressSpace::default()),
            "ptr_as_i8_ptr"
        ).map_err(|e| Error::codegen(format!("Failed to cast pointer: {}", e)))?;
        
        let cmp = self.builder().build_int_compare(
            inkwell::IntPredicate::EQ,
            ptr_as_i8, // Already a pointer value
            i8_null,
            "ptr_null_check"
        ).map_err(|e| Error::codegen(format!("Failed to compare pointer with null: {}", e)))?;
        
        self.builder().build_conditional_branch(cmp, null_block, store_block)
            .map_err(|e| Error::codegen(format!("Failed to build branch: {}", e)))?;
        
        // Build the store block
        self.builder().position_at_end(store_block);
        
        // Store the value
        self.builder().build_store(ptr, value)
            .map_err(|e| Error::codegen(format!("Failed to store to pointer: {}", e)))?;
        
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| Error::codegen(format!("Failed to build branch: {}", e)))?;
        
        // Build the null block - just emit warning and continue
        self.builder().position_at_end(null_block);
        
        // Skip the store and continue
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| Error::codegen(format!("Failed to build branch: {}", e)))?;
        
        // Continue from the merge block
        self.builder().position_at_end(merge_block);
        
        Ok(())
    }
    
    fn create_null_pointer(&mut self, type_name: &str) -> Result<PointerValue<'ctx>, Error> {
        // Create a null pointer of the appropriate type
        match type_name {
            "normie" => Ok(self.context().i32_type().ptr_type(inkwell::AddressSpace::default()).const_null()),
            "thicc" => Ok(self.context().i64_type().ptr_type(inkwell::AddressSpace::default()).const_null()),
            "smol" => Ok(self.context().i8_type().ptr_type(inkwell::AddressSpace::default()).const_null()),
            "mid" => Ok(self.context().i16_type().ptr_type(inkwell::AddressSpace::default()).const_null()),
            "snack" => Ok(self.context().f32_type().ptr_type(inkwell::AddressSpace::default()).const_null()),
            "meal" => Ok(self.context().f64_type().ptr_type(inkwell::AddressSpace::default()).const_null()),
            "bool" => Ok(self.context().bool_type().ptr_type(inkwell::AddressSpace::default()).const_null()),
            "char" => Ok(self.context().i8_type().ptr_type(inkwell::AddressSpace::default()).const_null()),
            "void" => Ok(self.context().i8_type().ptr_type(inkwell::AddressSpace::default()).const_null()),
            _ => {
                // Check if it's a struct type
                if let Some(struct_type) = self.get_struct_type(&self.current_package_name, type_name) {
                    Ok(struct_type.ptr_type(inkwell::AddressSpace::default()).const_null())
                } else {
                    Err(Error::codegen(format!("Unsupported type for null pointer: {}", type_name)))
                }
            }
        }
    }
}

// Legacy compatibility methods
impl<'ctx> LlvmCodeGenerator<'ctx> {
    pub fn get_address_of(&mut self, expr: &dyn Expression) -> Result<PointerValue<'ctx>, Error> {
        <Self as PointerOperations<'ctx>>::get_address_of(self, expr)
    }
    
    pub fn compile_pointer_type(&mut self, ptr_type: &PointerType) -> Result<BasicValueEnum<'ctx>, Error> {
        <Self as PointerOperations<'ctx>>::compile_pointer_type(self, ptr_type)
    }
    
    pub fn compile_pointer_dereference(&mut self, deref: &PointerDereference) -> Result<BasicValueEnum<'ctx>, Error> {
        <Self as PointerOperations<'ctx>>::compile_pointer_dereference(self, deref)
    }
    
    pub fn is_null_pointer(&self, ptr: BasicValueEnum<'ctx>) -> bool {
        if let BasicValueEnum::PointerValue(ptr_val) = ptr {
            // Compare with null pointer of the same type
            let null_ptr = ptr_val.get_type().const_null();
            ptr_val == null_ptr
        } else {
            false
        }
    }
} 