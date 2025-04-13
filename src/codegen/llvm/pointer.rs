//! LLVM code generation for pointer operations
//!
//! This module implements pointer operations for the CURSED language,
//! including address-of (@var) and dereference (@ptr) operations.

use inkwell::values::BasicValueEnum;
use inkwell::types::{BasicTypeEnum, BasicType};
use crate::ast::{PointerType, PointerDereference};
use crate::ast::expressions::Identifier;
use crate::error::Error;
use super::context::LlvmCodeGenerator;
use super::pointer_ops::PointerOperations as PointerOps;

/// Trait for pointer operations in LLVM code generation
pub trait PointerOperationsLegacy<'ctx> {
    /// Compile pointer type expressions (type declarations for pointers)
    fn compile_pointer_type(&mut self, pointer_type: &PointerType) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Compile pointer dereference expressions (@ptr)
    /// Includes handling address-of expressions (@var)
    fn compile_pointer_dereference(&mut self, pointer_deref: &PointerDereference) -> Result<BasicValueEnum<'ctx>, Error>;
}

impl<'ctx> PointerOperationsLegacy<'ctx> for LlvmCodeGenerator<'ctx> {
    fn compile_pointer_type(&mut self, pointer_type: &PointerType) -> Result<BasicValueEnum<'ctx>, Error> {
        // Handle pointer type expressions (@T)
        // Create a null pointer of the specified type
        // First get the target type
        if let Some(target_ident) = pointer_type.target_type.as_any().downcast_ref::<Identifier>() {
            let type_name = &target_ident.value;
            // Get the LLVM type for the target type name
            let llvm_type: BasicTypeEnum<'ctx> = match type_name.as_str() {
                "normie" => self.context().i32_type().into(),
                "smol" => self.context().i8_type().into(),
                "mid" => self.context().i16_type().into(),
                "thicc" => self.context().i64_type().into(),
                "snack" => self.context().f32_type().into(),
                "meal" => self.context().f64_type().into(),
                "tea" => self.context().i8_type().ptr_type(inkwell::AddressSpace::default()).into(), // String type (pointer to i8)
                "lit" => self.context().bool_type().into(),
                "byte" => self.context().i8_type().into(),
                "rune" => self.context().i32_type().into(),
                _ => {
                    // Check if it's a struct type
                    if let Some(struct_type) = self.get_struct_type(&self.current_package_name(), type_name) {
                        struct_type.into()
                    } else {
                        return Err(Error::from_str(&format!("Unknown type name: {}", type_name)));
                    }
                }
            };
            let ptr_type = llvm_type.ptr_type(inkwell::AddressSpace::default());
            return Ok(ptr_type.const_null().into());
        } else {
            return Err(Error::from_str(&format!("Unsupported target type: {}", pointer_type.target_type.string())));
        }
    }
    
    fn compile_pointer_dereference(&mut self, pointer_deref: &PointerDereference) -> Result<BasicValueEnum<'ctx>, Error> {
        // Check if this is actually an address-of operation (@var)
        if let Some(ident) = pointer_deref.pointer.as_any().downcast_ref::<Identifier>() {
            // This is an address-of operation (@var)
            let var_name = &ident.value;
            println!("DEBUG: Address-of operation for variable: {}", var_name);
            
            // Find the variable in our symbol table
            if let Some((ptr, _)) = self.variables.get(var_name) {
                // Return the pointer directly - this is the address of the variable
                println!("DEBUG: Found variable {} in symbol table, returning pointer", var_name);
                return Ok((*ptr).into());
            } else {
                println!("DEBUG: Variable {} not found in symbol table", var_name);
                // Debug print all variables in the symbol table
                println!("DEBUG: Variables in symbol table: {:?}", self.variables.keys().collect::<Vec<_>>());
                return Err(Error::from_str(&format!("Cannot take address of unknown variable: {}", var_name)));
            }
        }
        
        // Handle pointer dereference expressions (@ptr)
        // First, get the pointer value
        let expr_result = self.compile_expression_internal(pointer_deref.pointer.as_ref())
            .map_err(|e| Error::from_str(&e))?;
        
        println!("DEBUG: Attempting to dereference value of type: {:?}", expr_result.get_type());
        
        if !expr_result.is_pointer_value() {
            println!("DEBUG: Value is not a pointer: {:?}", expr_result);
            return Err(Error::from_str("Cannot dereference non-pointer value"));
        }
        
        let ptr = expr_result.into_pointer_value();
        
        // Try to infer the pointee type from the pointer type
        // For i32 pointers
        if ptr.get_type() == self.context().i32_type().ptr_type(inkwell::AddressSpace::default()) {
            let loaded_value = match self.builder().build_load(self.context().i32_type(), ptr, "deref_int") {
                Ok(val) => val,
                Err(e) => return Err(Error::from_str(&format!("Failed to load i32 value: {}", e)))
            };
            return Ok(loaded_value);
        }
        // For i64 pointers
        else if ptr.get_type() == self.context().i64_type().ptr_type(inkwell::AddressSpace::default()) {
            let loaded_value = match self.builder().build_load(self.context().i64_type(), ptr, "deref_int64") {
                Ok(val) => val,
                Err(e) => return Err(Error::from_str(&format!("Failed to load i64 value: {}", e)))
            };
            return Ok(loaded_value);
        }
        // For f32 pointers
        else if ptr.get_type() == self.context().f32_type().ptr_type(inkwell::AddressSpace::default()) {
            let loaded_value = match self.builder().build_load(self.context().f32_type(), ptr, "deref_float") {
                Ok(val) => val,
                Err(e) => return Err(Error::from_str(&format!("Failed to load f32 value: {}", e)))
            };
            return Ok(loaded_value);
        }
        // For f64 pointers
        else if ptr.get_type() == self.context().f64_type().ptr_type(inkwell::AddressSpace::default()) {
            let loaded_value = match self.builder().build_load(self.context().f64_type(), ptr, "deref_double") {
                Ok(val) => val,
                Err(e) => return Err(Error::from_str(&format!("Failed to load f64 value: {}", e)))
            };
            return Ok(loaded_value);
        }
        
        // If we couldn't determine the pointer type, try with the most common one first (normie - i32)
        let loaded_value = match self.builder().build_load(self.context().i32_type(), ptr, "deref_int_fallback") {
            Ok(val) => return Ok(val),
            Err(_) => {
                // Try the next most common type (thicc - i64)
                match self.builder().build_load(self.context().i64_type(), ptr, "deref_int64_fallback") {
                    Ok(val) => return Ok(val),
                    Err(_) => {
                        // Try float types
                        match self.builder().build_load(self.context().f32_type(), ptr, "deref_float_fallback") {
                            Ok(val) => return Ok(val),
                            Err(_) => {
                                match self.builder().build_load(self.context().f64_type(), ptr, "deref_double_fallback") {
                                    Ok(val) => return Ok(val),
                                    Err(_) => {}
                                }
                            }
                        }
                    }
                }
            }
        };
        
        // If we get here, we couldn't load the value
        return Err(Error::from_str("Failed to dereference pointer: unsupported type"));
    }
}

// Extension methods for LlvmCodeGenerator related to pointers
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Helper method to check if a value is a null pointer
    // Legacy version, kept for backward compatibility
    #[deprecated(note = "Use the implementation from pointer_ops module instead")]
    pub fn legacy_is_null_pointer(&self, ptr: BasicValueEnum<'ctx>) -> bool {
        if ptr.is_pointer_value() {
            let ptr_val = ptr.into_pointer_value();
            ptr_val.is_null()
        } else {
            false
        }
    }
    
    /// Compile an expression that accesses a field of a struct through a pointer
    pub fn compile_pointer_field_access(&mut self, 
                                      struct_ptr: inkwell::values::PointerValue<'ctx>,
                                      struct_type: inkwell::types::StructType<'ctx>,
                                      field_name: &str,
                                      field_index: u32) -> Result<BasicValueEnum<'ctx>, Error> {
        // Get a pointer to the field
        let field_ptr = self.builder().build_struct_gep(struct_type, struct_ptr, field_index, &format!("ptr_to_{}", field_name))
            .map_err(|e| Error::from_str(&format!("Failed to create GEP to field: {}", e)))?;
            
        // Get the field type
        let field_type = struct_type.get_field_type_at_index(field_index)
            .ok_or_else(|| Error::from_str(&format!("No field at index {} in struct", field_index)))?;
            
        // Load the field value
        let field_value = self.builder().build_load(field_type, field_ptr, field_name)
            .map_err(|e| Error::from_str(&format!("Failed to load field value: {}", e)))?;
            
        Ok(field_value)
    }
}

// Legacy pointer operations module
//
// This module provides backward compatibility for the pointer operations
// that were previously defined here but have been moved to pointer_ops.rs.
// It re-exports the trait and forwards method calls to the new implementation.

// Re-export the trait for backward compatibility
pub use super::pointer_ops::PointerOperations;
// We don't need to re-export this trait since it's already defined in this module

// The implementation is now in pointer_ops.rs, but we provide legacy methods here
impl<'ctx> LlvmCodeGenerator<'ctx> {
    pub fn get_address_of_var(&mut self, var_name: &str) -> Result<inkwell::values::PointerValue<'ctx>, Error> {
        // Import required trait
        use crate::codegen::llvm::variables::VariableHandling;
        if let Some(ptr) = self.lookup_variable(var_name) {
            Ok(ptr)
        } else {
            Err(Error::codegen(format!("Variable not found: {}", var_name)))
        }
    }
    
    pub fn dereference_pointer(&mut self, ptr: inkwell::values::PointerValue<'ctx>, name: &str) -> Result<BasicValueEnum<'ctx>, Error> {
        // Forward to the new implementation
        use crate::codegen::llvm::pointer_ops::PointerOperations;
        self.load_from_pointer(ptr, name)
    }
    
    // Legacy methods can still use the PointerOperations trait directly
}