//! Code generation for user-defined types

use inkwell::types::{BasicTypeEnum, StructType};
use inkwell::values::BasicValueEnum;
use std::collections::HashMap;
use crate::ast::declarations::{StructDeclaration, FieldDefinition};
use crate::ast::traits::Expression;
use crate::error::Error;
use super::context::LlvmCodeGenerator;
use crate::ast::declarations::{StructStatement, FunctionStatement};
use crate::ast::expressions::Identifier;
use crate::ast::expressions::struct_expr::{StructLiteral, StructFieldAccess};
use crate::ast::expressions::method_expr::MethodCall;

impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Declare a struct type in LLVM
    pub fn declare_struct_type(
        &self,
        struct_decl: &StructDeclaration
    ) -> Result<StructType<'ctx>, Error> {
        // Create the struct type
        let struct_name = &struct_decl.name.value;
        let struct_type = self.context.opaque_struct_type(struct_name);
        
        // Collect field types
        let mut field_types = Vec::new();
        for field in &struct_decl.fields {
            let field_type = self.type_to_llvm_type(&field.type_name)?;
            field_types.push(field_type);
        }
        
        // Set the body of the struct type
        struct_type.set_body(&field_types, false);
        
        Ok(struct_type)
    }
    
    /// Convert a Cursed type name to an LLVM type
    pub fn type_to_llvm_type(
        &self,
        type_name: &str
    ) -> Result<BasicTypeEnum<'ctx>, Error> {
        match type_name {
            "normie" => Ok(self.context.i32_type().into()),
            "thicc" => Ok(self.context.i64_type().into()),
            "smol" => Ok(self.context.i8_type().into()),
            "mid" => Ok(self.context.i16_type().into()),
            "snack" => Ok(self.context.f32_type().into()),
            "meal" => Ok(self.context.f64_type().into()),
            "lit" => Ok(self.context.bool_type().into()),
            "tea" => {
                // String is represented as pointer to string struct
                // which contains length and data
                Ok(self.context.i8_type().ptr_type(inkwell::AddressSpace::Generic).into())
            },
            "byte" => Ok(self.context.i8_type().into()),
            "rune" => Ok(self.context.i32_type().into()),
            // Handle pointers
            type_name if type_name.starts_with('@') => {
                let inner_type = &type_name[1..]; // Remove @ prefix
                let inner_llvm_type = self.type_to_llvm_type(inner_type)?;
                Ok(inner_llvm_type.ptr_type(inkwell::AddressSpace::Generic).into())
            },
            // Handle user-defined types
            _ => {
                // Look up the struct type
                if let Some(struct_type) = self.context.get_struct_type(type_name) {
                    Ok(struct_type.into())
                } else {
                    Err(Error::CodegenError(format!("Unknown type: {}", type_name)))
                }
            }
        }
    }
    
    /// Compile a struct literal
    pub fn compile_struct_literal(
        &self,
        struct_name: &str,
        field_values: &HashMap<String, Box<dyn Expression>>
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Look up the struct type
        let struct_type = match self.context.get_struct_type(struct_name) {
            Some(t) => t,
            None => return Err(Error::CodegenError(format!("Unknown struct type: {}", struct_name)))
        };
        
        // Create a new struct instance
        let struct_instance = self.builder.build_alloca(struct_type, "struct_instance")
            .map_err(|e| Error::CodegenError(format!("Failed to allocate struct instance: {}", e)))?;
        
        // Compile each field and store it in the struct
        for (i, (field_name, field_expr)) in field_values.iter().enumerate() {
            let field_value = self.compile_expression(&**field_expr)?;
            
            // Get a pointer to the field
            let field_ptr = self.builder.build_struct_gep(struct_instance, i as u32, &format!("field.{}.ptr", field_name))
                .map_err(|e| Error::CodegenError(format!("Failed to get field GEP: {}", e)))?;
            
            // Store the value in the field
            self.builder.build_store(field_ptr, field_value)
                .map_err(|e| Error::CodegenError(format!("Failed to store field value: {}", e)))?;
        }
        
        // Load and return the struct
        Ok(self.builder.build_load(struct_type, struct_instance, "struct_load")
            .map_err(|e| Error::CodegenError(format!("Failed to load struct: {}", e)))?
            .into())
    }
}