//! LLVM code generation for struct types and struct instantiation

use inkwell::types::BasicTypeEnum;
use inkwell::values::{BasicValueEnum, BasicValue};
use crate::ast::{SquadStatement, BeLikeExpression};
use super::context::LlvmCodeGenerator;

impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Get a type for a struct based on its name
    /// If the struct doesn't exist yet, return None
    pub fn get_struct_type(&self, package_name: &str, struct_name: &str) -> Option<inkwell::types::StructType<'ctx>> {
        self.struct_types
            .get(package_name)
            .and_then(|pkg_structs| pkg_structs.get(struct_name))
            .copied()
    }
    
    /// Register a struct type with the code generator
    pub fn register_struct_type(&mut self, package_name: &str, struct_name: &str, struct_type: inkwell::types::StructType<'ctx>) {
        let pkg_structs = self.struct_types
            .entry(package_name.to_string())
            .or_insert_with(std::collections::HashMap::new);
            
        pkg_structs.insert(struct_name.to_string(), struct_type);
    }
    
    /// Compile a struct declaration
    pub fn compile_squad_statement(&mut self, squad_stmt: &SquadStatement) -> Result<(), String> {
        let struct_name = &squad_stmt.name.value;
        
        // Create a list of field types for the struct
        let mut field_types = Vec::new();
        let mut field_names = Vec::new();
        
        // Process each field
        for field in &squad_stmt.fields {
            let field_name = &field.name.value;
            let type_name = &field.type_name.value;
            
            // Get the LLVM type for this field
            let field_type = self.get_llvm_type_for_name(type_name)?;
            
            field_types.push(field_type);
            field_names.push(field_name.clone());
        }
        
        // Create the struct type
        let struct_type = self.context.struct_type(&field_types, false);
        
        // To avoid borrowing issues, clone the package name
        let package_name = self.current_package_name.clone();
        
        // Register the struct type
        self.register_struct_type(&package_name, struct_name, struct_type);
        
        // Debugging output
        println!("Compiled struct '{}' with {} fields", struct_name, field_types.len());
        
        Ok(())
    }
    
    /// Helper function to get LLVM type for a CURSED type name
    pub fn get_llvm_type_for_name(&self, type_name: &str) -> Result<BasicTypeEnum<'ctx>, String> {
        match type_name {
            "smol" => Ok(self.context.i8_type().into()),
            "mid" => Ok(self.context.i16_type().into()),  
            "normie" => Ok(self.context.i32_type().into()), 
            "thicc" => Ok(self.context.i64_type().into()),
            "snack" => Ok(self.context.f32_type().into()),
            "meal" => Ok(self.context.f64_type().into()),
            "lit" => Ok(self.context.bool_type().into()),
            "tea" => Ok(self.context.i8_type().ptr_type(inkwell::AddressSpace::default()).into()),
            "byte" => Ok(self.context.i8_type().into()),
            "rune" => Ok(self.context.i32_type().into()),
            _ => {
                // Check if it's a struct type
                if let Some(struct_type) = self.get_struct_type(&self.current_package_name, type_name) {
                    Ok(struct_type.ptr_type(inkwell::AddressSpace::default()).into())
                } else {
                    Err(format!("Unknown type: {}", type_name))
                }
            }
        }
    }
    
    /// Compile struct instantiation expression
    pub fn compile_struct_instantiation(&mut self, expr: &BeLikeExpression) -> Result<BasicValueEnum<'ctx>, String> {
        let struct_name = &expr.struct_name.value;
        
        // Get the struct type
        let package_name = self.current_package_name.clone();
        let struct_type = self.get_struct_type(&package_name, struct_name)
            .ok_or_else(|| format!("Unknown struct type: {}", struct_name))?;
            
        // Allocate memory for the struct
        let struct_ptr = self.builder.build_alloca(struct_type, &format!("{}_instance", struct_name)).unwrap();
        
        // Set each field value
        for (i, (field_name, field_value_expr)) in expr.fields.iter().enumerate() {
            // Compile the field value
            let field_value = self.compile_expression(field_value_expr.as_ref())?;
            
            // Calculate the pointer to the field
            let indices = [
                self.context.i32_type().const_int(0, false),
                self.context.i32_type().const_int(i as u64, false)
            ];
            
            // GEP to get pointer to the field
            let field_ptr = unsafe {
                self.builder.build_gep(
                    struct_type,
                    struct_ptr,
                    &indices,
                    &format!("{}_field_{}", struct_name, field_name)
                ).unwrap()
            };
            
            // Store the value in the field
            self.builder.build_store(field_ptr, field_value).unwrap();
        }
        
        // Return the pointer to the struct
        Ok(struct_ptr.as_basic_value_enum())
    }
}