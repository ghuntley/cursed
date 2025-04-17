//! LLVM code generation for struct field type inference in the CURSED language.
//!
//! This module handles the analysis of struct field types when they are not explicitly
//! declared, allowing for a more ergonomic struct initialization syntax.
//!
//! Key responsibilities include:
//! - Type inference for struct fields during initialization
//! - Type coercion when assigning values to struct fields
//! - Type compatibility checking between fields and their initializers
//! - Registering inferred field types with the struct type system
//! - Supporting nested struct type inference
//!
//! This allows for flexibility in struct instantiation while maintaining type safety.

use crate::ast::expressions::struct_expr::{StructLiteral, KeyValuePair};
use crate::ast::traits::Expression;
use crate::Error;
use crate::codegen::llvm::ExpressionCompilation;
use crate::lexer::token::Token;
use inkwell::types::{BasicTypeEnum, StructType};
use inkwell::values::BasicValueEnum;
use std::collections::HashMap;

/// Trait for struct field type inference in LLVM code generation
pub trait StructFieldInference<'ctx> {
    /// Compile a struct literal expression with type inference
    fn compile_struct_literal(&mut self, struct_literal: &StructLiteral) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Register a struct type with the code generator for later use
    fn register_struct_type(&mut self, struct_name: &str, struct_type: StructType<'ctx>) -> Result<(), Error>;
    
    /// Check if a field value type is compatible with a field type and perform coercion if needed
    fn check_field_type_compatibility(
        &mut self, 
        field_name: &str,
        field_type: BasicTypeEnum<'ctx>, 
        field_value: BasicValueEnum<'ctx>
    ) -> Result<BasicValueEnum<'ctx>, Error>;
}

impl<'ctx> StructFieldInference<'ctx> for crate::codegen::llvm::LlvmCodeGenerator<'ctx> {
    /// Compile a struct literal expression with type inference
    fn compile_struct_literal(&mut self, struct_literal: &StructLiteral) -> Result<BasicValueEnum<'ctx>, Error> {
        let struct_name = &struct_literal.struct_name;
        
        // Get the struct type if it exists
        let struct_type = match self.get_struct_type(&self.current_package_name(), struct_name) {
            Some(ty) => ty,
            None => return Err(Error::Compilation(format!("Unknown struct type: {}", struct_name))),
        };
        
        // Check that we have the correct number of fields
        let expected_field_count = struct_type.get_field_types().len();
        if struct_literal.fields.len() != expected_field_count {
            return Err(Error::Compilation(format!(
                "Struct literal for '{}' has {} fields, but {} were expected",
                struct_name, struct_literal.fields.len(), expected_field_count
            )));
        }
        
        // Collect field names and their expected types
        let mut field_types: HashMap<String, (usize, BasicTypeEnum<'ctx>)> = HashMap::new();
        for (idx, field_type) in struct_type.get_field_types().iter().enumerate() {
            // We don't have field names in LLVM types, so we'll need a mapping
            // For now, assume fields are in the same order as the struct definition
            let field_name = match &struct_literal.fields.get(idx) {
                Some(field) => field.key.value.clone(),
                None => return Err(Error::Compilation(format!("Missing field at index {}", idx))),
            };
            field_types.insert(field_name, (idx, *field_type));
        }
        
        // Allocate memory for the struct
        let struct_ptr = self.builder().build_alloca(struct_type, &format!("struct_{}_instance", struct_name)).unwrap();
        
        // Initialize each field with the provided value, performing type coercion if necessary
        for field in &struct_literal.fields {
            let field_name = &field.key.value;
            
            // Find the field index and type
            let (field_idx, field_type) = match field_types.get(field_name) {
                Some(info) => *info,
                None => return Err(Error::Compilation(format!("Unknown field: {}", field_name))),
            };
            
            // Compile the field value expression
            let field_value = self.compile_expression(field.value.as_ref())?;
            
            // Check type compatibility and perform coercion if needed
            let coerced_value = self.check_field_type_compatibility(field_name, field_type, field_value)?;
            
            // Get pointer to the field
            let field_ptr = unsafe {
                self.builder().build_struct_gep(
                    struct_type,
                    struct_ptr,
                    field_idx as u32,
                    &format!("field_{}_ptr", field_name)
                ).unwrap()
            };
            
            // Store the value in the field
            self.builder().build_store(field_ptr, coerced_value).unwrap();
        }
        
        // Return the struct pointer as a BasicValueEnum
        Ok(struct_ptr.into())
    }
    
    /// Register a struct type with the code generator
    fn register_struct_type(&mut self, struct_name: &str, struct_type: StructType<'ctx>) -> Result<(), Error> {
        let package_name = self.current_package_name().to_string();
        
        // Ensure we have an entry for this package
        self.struct_types.entry(package_name)
            .or_insert_with(HashMap::new)
            .insert(struct_name.to_string(), struct_type);
        
        Ok(())
    }
    
    /// Check if a field value type is compatible with a field type and perform coercion if needed
    fn check_field_type_compatibility(
        &mut self, 
        field_name: &str,
        field_type: BasicTypeEnum<'ctx>, 
        field_value: BasicValueEnum<'ctx>
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Handle type coercion for numeric types
        match (field_type, field_value) {
            // Integer to float coercion
            (BasicTypeEnum::FloatType(dest_type), BasicValueEnum::IntValue(int_val)) => {
                let float_val = self.builder()
                    .build_signed_int_to_float(int_val, dest_type, &format!("coerce_{}_int_to_float", field_name)).unwrap();
                Ok(float_val.into())
            },
            
            // Float to integer coercion (potentially lossy, might want to warn)
            (BasicTypeEnum::IntType(dest_type), BasicValueEnum::FloatValue(float_val)) => {
                let int_val = self.builder()
                    .build_float_to_signed_int(float_val, dest_type, &format!("coerce_{}_float_to_int", field_name)).unwrap();
                Ok(int_val.into())
            },
            
            // Direct match - no coercion needed
            (dest_type, value) if self.types_are_compatible(dest_type, value) => {
                Ok(value)
            },
            
            // Incompatible types
            (dest_type, value) => {
                Err(Error::Compilation(format!(
                    "Type mismatch for field '{}': expected {:?}, got {:?}",
                    field_name, dest_type, value
                )))
            }
        }
    }
}

impl<'ctx> crate::codegen::llvm::LlvmCodeGenerator<'ctx> {
    /// Check if a value type is compatible with a destination type
    fn types_are_compatible(&self, dest_type: BasicTypeEnum<'ctx>, value: BasicValueEnum<'ctx>) -> bool {
        match (dest_type, value) {
            (BasicTypeEnum::IntType(_), BasicValueEnum::IntValue(_)) => true,
            (BasicTypeEnum::FloatType(_), BasicValueEnum::FloatValue(_)) => true,
            (BasicTypeEnum::PointerType(_), BasicValueEnum::PointerValue(_)) => true,
            (BasicTypeEnum::StructType(_), BasicValueEnum::StructValue(_)) => true,
            (BasicTypeEnum::ArrayType(_), BasicValueEnum::ArrayValue(_)) => true,
            _ => false,
        }
    }
}