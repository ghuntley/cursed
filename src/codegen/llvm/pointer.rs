//! LLVM code generation for pointer operations

use inkwell::values::BasicValueEnum;
use inkwell::types::{BasicTypeEnum, BasicType};
use crate::ast::{PointerType, PointerDereference};
use crate::ast::expressions::Identifier;
use super::context::LlvmCodeGenerator;

impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Compile pointer type expressions
    pub fn compile_pointer_type(&mut self, pointer_type: &PointerType) -> Result<BasicValueEnum<'ctx>, String> {
        // Handle pointer type expressions (@T)
        // Create a null pointer of the specified type
        // First get the target type
        if let Some(target_ident) = pointer_type.target_type.as_any().downcast_ref::<Identifier>() {
            let type_name = &target_ident.value;
            // Get the LLVM type for the target type name
            let llvm_type: BasicTypeEnum<'ctx> = match type_name.as_str() {
                "normie" => self.context.i32_type().into(),
                "smol" => self.context.i8_type().into(),
                "mid" => self.context.i16_type().into(),
                "thicc" => self.context.i64_type().into(),
                "snack" => self.context.f32_type().into(),
                "meal" => self.context.f64_type().into(),
                "tea" => self.context.i8_type().ptr_type(inkwell::AddressSpace::default()).into(), // String type (pointer to i8)
                "lit" => self.context.bool_type().into(),
                "byte" => self.context.i8_type().into(),
                "rune" => self.context.i32_type().into(),
                _ => {
                    // Check if it's a struct type
                    if let Some(struct_type) = self.get_struct_type(&self.current_package_name, type_name) {
                        struct_type.into()
                    } else {
                        return Err(format!("Unknown type name: {}", type_name));
                    }
                }
            };
            let ptr_type = llvm_type.ptr_type(inkwell::AddressSpace::default());
            return Ok(ptr_type.const_null().into());
        } else {
            return Err(format!("Unsupported target type: {}", pointer_type.target_type.string()));
        }
    }
    
    /// Compile pointer dereference expressions
    pub fn compile_pointer_dereference(&mut self, pointer_deref: &PointerDereference) -> Result<BasicValueEnum<'ctx>, String> {
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
                return Err(format!("Cannot take address of unknown variable: {}", var_name));
            }
        }
        
        // Handle pointer dereference expressions (@ptr)
        // First, get the pointer value
        let ptr_val = self.compile_expression(pointer_deref.pointer.as_ref())?;
        
        println!("DEBUG: Attempting to dereference value of type: {:?}", ptr_val.get_type());
        
        if !ptr_val.is_pointer_value() {
            println!("DEBUG: Value is not a pointer: {:?}", ptr_val);
            return Err(format!("Cannot dereference non-pointer value"));
        }
        
        let ptr = ptr_val.into_pointer_value();
        
        // Try to infer the pointee type from the pointer type
        // For i32 pointers
        if ptr.get_type() == self.context.i32_type().ptr_type(inkwell::AddressSpace::default()) {
            let loaded_value = match self.builder.build_load(self.context.i32_type(), ptr, "deref_int") {
                Ok(val) => val,
                Err(e) => return Err(format!("Failed to load i32 value: {}", e))
            };
            return Ok(loaded_value);
        }
        // For i64 pointers
        else if ptr.get_type() == self.context.i64_type().ptr_type(inkwell::AddressSpace::default()) {
            let loaded_value = match self.builder.build_load(self.context.i64_type(), ptr, "deref_int64") {
                Ok(val) => val,
                Err(e) => return Err(format!("Failed to load i64 value: {}", e))
            };
            return Ok(loaded_value);
        }
        // For f32 pointers
        else if ptr.get_type() == self.context.f32_type().ptr_type(inkwell::AddressSpace::default()) {
            let loaded_value = match self.builder.build_load(self.context.f32_type(), ptr, "deref_float") {
                Ok(val) => val,
                Err(e) => return Err(format!("Failed to load f32 value: {}", e))
            };
            return Ok(loaded_value);
        }
        // For f64 pointers
        else if ptr.get_type() == self.context.f64_type().ptr_type(inkwell::AddressSpace::default()) {
            let loaded_value = match self.builder.build_load(self.context.f64_type(), ptr, "deref_double") {
                Ok(val) => val,
                Err(e) => return Err(format!("Failed to load f64 value: {}", e))
            };
            return Ok(loaded_value);
        }
        
        // If we couldn't determine the pointer type, try with the most common one first (normie - i32)
        let loaded_value = match self.builder.build_load(self.context.i32_type(), ptr, "deref_int_fallback") {
            Ok(val) => return Ok(val),
            Err(_) => {
                // Try the next most common type (thicc - i64)
                match self.builder.build_load(self.context.i64_type(), ptr, "deref_int64_fallback") {
                    Ok(val) => return Ok(val),
                    Err(_) => {
                        // Try float types
                        match self.builder.build_load(self.context.f32_type(), ptr, "deref_float_fallback") {
                            Ok(val) => return Ok(val),
                            Err(_) => {
                                match self.builder.build_load(self.context.f64_type(), ptr, "deref_double_fallback") {
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
        return Err(format!("Failed to dereference pointer: unsupported type"));
    }
}