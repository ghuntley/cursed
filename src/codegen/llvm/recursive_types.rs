//! LLVM code generation for recursive types in the CURSED language.
//!
//! This module handles the generation of LLVM IR for recursive type definitions,
//! including forward declarations, opaque types, and proper memory layout.

use crate::codegen::llvm::LlvmCodeGenerator;
use crate::core::type_checker::Type;
use crate::error::Error;
use inkwell::types::{BasicTypeEnum, StructType, BasicType};
use inkwell::AddressSpace;
use std::collections::HashMap;
use tracing::{debug, info, warn};
use crate::codegen::llvm::function_monomorphization::FunctionMonomorphization;

/// Trait for LLVM code generation of recursive types
pub trait RecursiveTypeLLVM<'ctx> {
    /// Create a forward declaration for a recursive type
    fn create_forward_declaration(&mut self, type_name: &str) -> Result<StructType<'ctx>, Error>;
    
    /// Generate LLVM type for a recursive type definition
    fn generate_recursive_type(&mut self, type_name: &str, type_def: &Type) -> Result<BasicTypeEnum<'ctx>, Error>;
    
    /// Resolve forward declared types with their actual definitions
    fn resolve_forward_declarations(&mut self) -> Result<(), Error>;
    
    /// Check if a type is opaque (forward declared but not yet defined)
    fn is_opaque_type(&self, type_name: &str) -> bool;
    
    /// Set the body of a recursive struct type
    fn set_recursive_struct_body(&mut self, type_name: &str, field_types: &[BasicTypeEnum<'ctx>]) -> Result<(), Error>;
    
    /// Generate proper memory layout for recursive structures
    fn generate_recursive_memory_layout(&mut self, type_name: &str, type_def: &Type) -> Result<BasicTypeEnum<'ctx>, Error>;
}

impl<'ctx> RecursiveTypeLLVM<'ctx> for LlvmCodeGenerator<'ctx> {
    /// Create a forward declaration for a recursive type
    fn create_forward_declaration(&mut self, type_name: &str) -> Result<StructType<'ctx>, Error> {
        debug!(type_name = %type_name, "Creating forward declaration");
        
        // Create an opaque struct type
        let opaque_type = self.context().opaque_struct_type(type_name);
        
        // Register it in our type system
        let package_name_str = self.current_package_name().to_string();
        // Direct registration to avoid private method access issues
        if let Some(package_structs) = self.struct_types.get_mut(&package_name_str) {
            package_structs.insert(type_name.to_string(), opaque_type);
        } else {
            let mut new_package = HashMap::new();
            new_package.insert(type_name.to_string(), opaque_type);
            self.struct_types.insert(package_name_str, new_package);
        }
        
        Ok(opaque_type)
    }
    
    /// Generate LLVM type for a recursive type definition
    fn generate_recursive_type(&mut self, type_name: &str, type_def: &Type) -> Result<BasicTypeEnum<'ctx>, Error> {
        debug!(type_name = %type_name, "Generating recursive type");
        
        match type_def {
            Type::Struct(name, field_types) => {
                // Check if we already have a forward declaration
                let struct_type = if let Some(existing_type) = self.get_struct_type(&self.current_package_name(), name) {
                    existing_type
                } else {
                    // Create a new opaque type
                    self.create_forward_declaration(name)?
                };
                
                // Generate field types
                let mut llvm_field_types = Vec::new();
                for field_type in field_types {
                    let llvm_field_type = self.generate_recursive_field_type(field_type)?;
                    llvm_field_types.push(llvm_field_type);
                }
                
                // Set the body of the struct if it's not already set
                if struct_type.count_fields() == 0 && !llvm_field_types.is_empty() {
                    struct_type.set_body(&llvm_field_types, false);
                }
                
                Ok(BasicTypeEnum::StructType(struct_type))
            }
            Type::Pointer(inner_type) => {
                // For recursive pointers, we need to handle the inner type carefully
                let inner_llvm_type = self.generate_recursive_type_reference(inner_type)?;
                let pointer_type = inner_llvm_type.ptr_type(AddressSpace::default());
                Ok(BasicTypeEnum::PointerType(pointer_type))
            }
            _ => {
                // For non-recursive types, use standard generation
                self.type_to_llvm_type(type_def)
            }
        }
    }
    
    /// Resolve forward declared types with their actual definitions
    fn resolve_forward_declarations(&mut self) -> Result<(), Error> {
        info!("Resolving forward declarations");
        
        // This method is called after all types have been processed
        // We don't need to do anything special here since LLVM handles
        // opaque types automatically once their bodies are set
        
        Ok(())
    }
    
    /// Check if a type is opaque (forward declared but not yet defined)
    fn is_opaque_type(&self, type_name: &str) -> bool {
        if let Some(struct_type) = self.get_struct_type(&self.current_package_name(), type_name) {
            // A struct is opaque if it has no fields defined
            struct_type.count_fields() == 0
        } else {
            false
        }
    }
    
    /// Set the body of a recursive struct type
    fn set_recursive_struct_body(&mut self, type_name: &str, field_types: &[BasicTypeEnum<'ctx>]) -> Result<(), Error> {
        debug!(type_name = %type_name, field_count = field_types.len(), "Setting recursive struct body");
        
        if let Some(struct_type) = self.get_struct_type(&self.current_package_name(), type_name) {
            // Only set body if not already set
            if struct_type.count_fields() == 0 {
                struct_type.set_body(field_types, false);
                debug!(type_name = %type_name, "Set struct body successfully");
            } else {
                debug!(type_name = %type_name, "Struct body already set");
            }
            Ok(())
        } else {
            Err(Error::from_str(&format!("Struct type not found: {}", type_name)))
        }
    }
    
    /// Generate proper memory layout for recursive structures
    fn generate_recursive_memory_layout(&mut self, type_name: &str, type_def: &Type) -> Result<BasicTypeEnum<'ctx>, Error> {
        debug!(type_name = %type_name, "Generating recursive memory layout");
        
        match type_def {
            Type::Struct(name, field_types) => {
                // For recursive structs, we need to ensure proper alignment and size
                let mut llvm_field_types = Vec::new();
                
                for field_type in field_types {
                    let llvm_field_type = self.generate_recursive_field_type(field_type)?;
                    llvm_field_types.push(llvm_field_type);
                }
                
                // Get or create the struct type
                let struct_type = if let Some(existing) = self.get_struct_type(&self.current_package_name(), name) {
                    existing
                } else {
                    self.create_forward_declaration(name)?
                };
                
                // Set the body with proper memory layout
                if struct_type.count_fields() == 0 && !llvm_field_types.is_empty() {
                    struct_type.set_body(&llvm_field_types, false);
                }
                
                Ok(BasicTypeEnum::StructType(struct_type))
            }
            _ => self.generate_recursive_type(type_name, type_def),
        }
    }
}

impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Generate LLVM type for a field in a recursive structure
    fn generate_recursive_field_type(&mut self, field_type: &Type) -> Result<BasicTypeEnum<'ctx>, Error> {
        match field_type {
            Type::Pointer(inner) => {
                // For pointer fields, we might need forward declarations
                match inner.as_ref() {
                    Type::Named(type_name) => {
                        // Check if this is a recursive reference
                        if self.is_opaque_type(type_name) || !self.get_struct_type(&self.current_package_name(), type_name).is_some() {
                            // Create forward declaration if needed
                            let forward_type = self.create_forward_declaration(type_name)?;
                            let pointer_type = forward_type.ptr_type(AddressSpace::default());
                            Ok(BasicTypeEnum::PointerType(pointer_type))
                        } else {
                            // Type is already defined
                            let inner_type = self.type_to_llvm_type(inner)?;
                            let pointer_type = inner_type.ptr_type(AddressSpace::default());
                            Ok(BasicTypeEnum::PointerType(pointer_type))
                        }
                    }
                    _ => {
                        // Regular pointer to non-named type
                        let inner_type = self.type_to_llvm_type(inner)?;
                        let pointer_type = inner_type.ptr_type(AddressSpace::default());
                        Ok(BasicTypeEnum::PointerType(pointer_type))
                    }
                }
            }
            Type::Named(type_name) => {
                // For named types, check if they're recursive
                if self.is_opaque_type(type_name) {
                    // Create forward declaration
                    let forward_type = self.create_forward_declaration(type_name)?;
                    Ok(BasicTypeEnum::StructType(forward_type))
                } else if let Some(struct_type) = self.get_struct_type(&self.current_package_name(), type_name) {
                    // Use existing type
                    Ok(BasicTypeEnum::StructType(struct_type))
                } else {
                    // Might be a primitive type or error
                    self.type_to_llvm_type(field_type)
                }
            }
            _ => {
                // For other types, use standard generation
                self.type_to_llvm_type(field_type)
            }
        }
    }
    
    /// Generate type reference for recursive types (handles cycles)
    fn generate_recursive_type_reference(&mut self, type_def: &Type) -> Result<BasicTypeEnum<'ctx>, Error> {
        match type_def {
            Type::Named(type_name) => {
                // For named types in recursive contexts, always use/create forward declaration
                if let Some(struct_type) = self.get_struct_type(&self.current_package_name(), type_name) {
                    Ok(BasicTypeEnum::StructType(struct_type))
                } else {
                    let forward_type = self.create_forward_declaration(type_name)?;
                    Ok(BasicTypeEnum::StructType(forward_type))
                }
            }
            Type::Struct(name, _) => {
                // Similar to named types
                if let Some(struct_type) = self.get_struct_type(&self.current_package_name(), name) {
                    Ok(BasicTypeEnum::StructType(struct_type))
                } else {
                    let forward_type = self.create_forward_declaration(name)?;
                    Ok(BasicTypeEnum::StructType(forward_type))
                }
            }
            _ => {
                // For non-struct types, use standard generation
                self.type_to_llvm_type(type_def)
            }
        }
    }
    
    /// Process a program with recursive types
    pub fn process_recursive_types(&mut self, recursive_types: &HashMap<String, crate::core::recursive_types::RecursiveType>) -> Result<(), Error> {
        info!(type_count = recursive_types.len(), "Processing recursive types");
        
        // First pass: create forward declarations for all recursive types
        for (type_name, recursive_type) in recursive_types {
            if recursive_type.is_directly_recursive() {
                debug!(type_name = %type_name, "Creating forward declaration for recursive type");
                self.create_forward_declaration(type_name)?;
            }
        }
        
        // Second pass: generate actual type definitions
        for (type_name, recursive_type) in recursive_types {
            debug!(type_name = %type_name, "Generating LLVM type for recursive type");
            self.generate_recursive_type(type_name, &recursive_type.definition)?;
        }
        
        // Third pass: ensure all forward declarations are resolved
        self.resolve_forward_declarations()?;
        
        info!("Finished processing recursive types");
        Ok(())
    }
}

/// Extension methods for working with recursive types in LLVM generation
pub trait RecursiveTypeExtensions<'ctx> {
    /// Check if a type definition contains recursive references
    fn contains_recursive_references(&self, type_def: &Type, type_name: &str) -> bool;
    
    /// Get the dependency chain for a recursive type
    fn get_dependency_chain(&self, type_def: &Type) -> Vec<String>;
    
    /// Validate recursive type for LLVM generation
    fn validate_recursive_type(&self, type_def: &Type) -> Result<(), Error>;
}

impl<'ctx> RecursiveTypeExtensions<'ctx> for LlvmCodeGenerator<'ctx> {
    /// Check if a type definition contains recursive references
    fn contains_recursive_references(&self, type_def: &Type, type_name: &str) -> bool {
        match type_def {
            Type::Named(name) => name == type_name,
            Type::Struct(name, field_types) => {
                if name == type_name {
                    return true;
                }
                field_types.iter().any(|field| self.contains_recursive_references(field, type_name))
            }
            Type::Pointer(inner) => self.contains_recursive_references(inner, type_name),
            Type::Array(inner, _) => self.contains_recursive_references(inner, type_name),
            Type::Slice(inner) => self.contains_recursive_references(inner, type_name),
            Type::Map(key, value) => {
                self.contains_recursive_references(key, type_name) ||
                self.contains_recursive_references(value, type_name)
            }
            Type::Function(params, return_type) => {
                params.iter().any(|param| self.contains_recursive_references(param, type_name)) ||
                self.contains_recursive_references(return_type, type_name)
            }
            Type::Channel(inner) => self.contains_recursive_references(inner, type_name),
            Type::Interface(name, type_args) => {
                if name == type_name {
                    return true;
                }
                type_args.iter().any(|arg| self.contains_recursive_references(arg, type_name))
            }
            Type::Generic(name, type_args) => {
                if name == type_name {
                    return true;
                }
                type_args.iter().any(|arg| self.contains_recursive_references(arg, type_name))
            }
            _ => false,
        }
    }
    
    /// Get the dependency chain for a recursive type
    fn get_dependency_chain(&self, type_def: &Type) -> Vec<String> {
        let mut dependencies = Vec::new();
        self.collect_dependencies(type_def, &mut dependencies);
        dependencies
    }
    
    /// Validate recursive type for LLVM generation
    fn validate_recursive_type(&self, type_def: &Type) -> Result<(), Error> {
        match type_def {
            Type::Struct(_, field_types) => {
                // Check that recursive references are through pointers
                for field_type in field_types {
                    self.validate_field_type(field_type)?;
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Collect dependencies recursively
    fn collect_dependencies(&self, type_def: &Type, dependencies: &mut Vec<String>) {
        match type_def {
            Type::Named(name) => {
                if !dependencies.contains(name) {
                    dependencies.push(name.clone());
                }
            }
            Type::Struct(name, field_types) => {
                if !dependencies.contains(name) {
                    dependencies.push(name.clone());
                }
                for field_type in field_types {
                    self.collect_dependencies(field_type, dependencies);
                }
            }
            Type::Pointer(inner) => self.collect_dependencies(inner, dependencies),
            Type::Array(inner, _) => self.collect_dependencies(inner, dependencies),
            Type::Slice(inner) => self.collect_dependencies(inner, dependencies),
            Type::Map(key, value) => {
                self.collect_dependencies(key, dependencies);
                self.collect_dependencies(value, dependencies);
            }
            Type::Function(params, return_type) => {
                for param in params {
                    self.collect_dependencies(param, dependencies);
                }
                self.collect_dependencies(return_type, dependencies);
            }
            Type::Channel(inner) => self.collect_dependencies(inner, dependencies),
            Type::Interface(name, type_args) => {
                if !dependencies.contains(name) {
                    dependencies.push(name.clone());
                }
                for arg in type_args {
                    self.collect_dependencies(arg, dependencies);
                }
            }
            Type::Generic(name, type_args) => {
                if !dependencies.contains(name) {
                    dependencies.push(name.clone());
                }
                for arg in type_args {
                    self.collect_dependencies(arg, dependencies);
                }
            }
            _ => {} // Primitive types have no dependencies
        }
    }
    
    /// Validate that a field type is safe for recursive structures
    fn validate_field_type(&self, field_type: &Type) -> Result<(), Error> {
        match field_type {
            Type::Struct(_, _) => {
                // Direct struct embedding might cause infinite size
                warn!("Direct struct embedding detected - consider using pointers for recursive references");
                Ok(())
            }
            Type::Pointer(_) => {
                // Pointers are always safe for recursion
                Ok(())
            }
            Type::Array(inner, _) => {
                // Arrays of structs might be problematic
                self.validate_field_type(inner)
            }
            Type::Slice(_) => {
                // Slices are safe (they're basically pointers)
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::llvm::LlvmCodeGenerator;
    use inkwell::context::Context;

    #[test]
    fn test_recursive_type_generation() {
        let context = Context::create();
        let module = context.create_module("test");
        let builder = context.create_builder();
        
        let mut codegen = LlvmCodeGenerator::new(&context, "test", std::path::PathBuf::from("test.csd"));
        
        // Test forward declaration
        let forward_type = codegen.create_forward_declaration("Node").unwrap();
        assert!(codegen.is_opaque_type("Node"));
        
        // Test setting body
        let field_types = vec![
            context.i32_type().into(), // value field
            forward_type.ptr_type(inkwell::AddressSpace::default()).into(), // next field
        ];
        
        codegen.set_recursive_struct_body("Node", &field_types).unwrap();
        assert!(!codegen.is_opaque_type("Node"));
    }
}
