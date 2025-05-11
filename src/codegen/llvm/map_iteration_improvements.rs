//! Improved map iteration support for range clauses
//!
//! This module enhances map iteration in range clauses by providing
//! accurate type determination for map keys and values, which allows
//! for proper memory allocation and access during iteration.

use crate::error::Error;
use crate::codegen::llvm::context::LlvmCodeGenerator;
use crate::codegen::llvm::pointer_type_extension::{PointerTypeExtension, BasicTypeEnumExtension};
use inkwell::basic_block::BasicBlock;
use inkwell::types::{BasicType, BasicTypeEnum};
use inkwell::values::{BasicValue, BasicValueEnum, IntValue, PointerValue};
use inkwell::AddressSpace;
use tracing::{debug, info, instrument};

/// Map iteration enhancement trait that improves type handling for map iterations
///
/// This trait adds methods to properly determine key and value types
/// from map structures, enabling more accurate memory allocation and
/// type-safe access to map elements during iteration.
pub trait MapIterationEnhancements<'ctx> {
    /// Determine the key type for a map based on its internal structure
    ///
    /// This method examines the map's type structure to extract the
    /// concrete key type, allowing for proper memory allocation and
    /// type-safe access to keys during iteration.
    fn determine_map_key_type(&self, map_value: BasicValueEnum<'ctx>) -> Result<BasicTypeEnum<'ctx>, Error>;

    /// Determine the value type for a map based on its internal structure
    ///
    /// This method examines the map's type structure to extract the
    /// concrete value type, allowing for proper memory allocation and
    /// type-safe access to values during iteration.
    fn determine_map_value_type(&self, map_value: BasicValueEnum<'ctx>) -> Result<BasicTypeEnum<'ctx>, Error>;
}

impl<'ctx> MapIterationEnhancements<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self, map_value), level = "debug")]
    fn determine_map_key_type(&self, map_value: BasicValueEnum<'ctx>) -> Result<BasicTypeEnum<'ctx>, Error> {
        debug!("Determining map key type");
        
        if !map_value.is_pointer_value() {
            return Err(Error::CodeGenError("Expected map to be a pointer type".to_string()));
        }
        
        let map_ptr = map_value.into_pointer_value();
let map_ptr_type = map_ptr.get_type();
// Use our extension trait for get_element_type
let pointee_type = map_ptr_type.get_element_type();
        
        // Maps in CURSED are typically implemented as a struct with fields for
        // key type, value type, buckets, etc. We need to analyze the struct type
        if let Some(struct_type) = pointee_type.as_struct_type() {
            let type_name = struct_type.get_name()
                .and_then(|n| n.to_str().ok())
                .unwrap_or("unknown");
            
            debug!("Map struct type name: {}", type_name);
            
            // Try to extract type parameters from the name
            // Common formats include Map<KeyType,ValueType> or Dictionary<K,V>
            if type_name.contains('<') && type_name.contains('>') {
                let start_idx = type_name.find('<').unwrap() + 1;
                let end_idx = type_name.rfind('>').unwrap();
                
                if start_idx < end_idx {
                    let type_params_str = &type_name[start_idx..end_idx];
                    let type_params: Vec<&str> = type_params_str.split(',').collect();
                    
                    if !type_params.is_empty() {
                        // First type parameter is typically the key type
                        let key_type_name = type_params[0].trim();
                        debug!("Extracted key type from name: {}", key_type_name);
                        
                        // Map the type name to an LLVM type
                        return self.map_type_name_to_llvm_type(key_type_name);
                    }
                }
            }
            
            // If we couldn't extract the type from the name, examine the struct fields
            // In many map implementations, there are type descriptors or metadata fields
            // that indicate the key and value types
            
            // For now, try to infer from the buckets or entries field if available
            if struct_type.get_field_types().len() > 0 {
                // Look for a buckets or entries field (often field 0 or 2)
                let buckets_field_indices = [0, 2];
                
                for &field_idx in &buckets_field_indices {
                    if field_idx < struct_type.get_field_types().len() {
                        let field_type = struct_type.get_field_types()[field_idx];
                        
                        // Buckets/entries field is typically a pointer to an array of entry structs
                        if let Some(ptr_type) = field_type.as_pointer_type() {
                            let element_type = ptr_type.get_element_type();
                            
                            // Try to extract key type from entry struct
                            if let Some(entry_struct) = element_type.as_struct_type() {
                                if entry_struct.get_field_types().len() > 0 {
                                    // First field in an entry is typically the key
                                    let key_field_type = entry_struct.get_field_types()[0];
                                    debug!("Inferred key type from map entry struct");
                                    return Ok(key_field_type);
                                }
                            } else if let Some(array_type) = element_type.as_array_type() {
                                // If it's an array, look at the element type
                                let array_element = array_type.get_element_type();
                                if let Some(entry_struct) = array_element.as_struct_type() {
                                    if entry_struct.get_field_types().len() > 0 {
                                        // First field in an entry is typically the key
                                        let key_field_type = entry_struct.get_field_types()[0];
                                        debug!("Inferred key type from map entry array");
                                        return Ok(key_field_type);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // If we couldn't determine the exact type, look for type metadata
        // in the map itself or in associated functions
        
        // For now, return a default type if we couldn't determine the key type
        // In a real implementation, we would have a more comprehensive type registry
        debug!("Using default tea (string) type for map key");
        Ok(self.context.i8_type().ptr_type(AddressSpace::default()).into())
    }

    #[instrument(skip(self, map_value), level = "debug")]
    fn determine_map_value_type(&self, map_value: BasicValueEnum<'ctx>) -> Result<BasicTypeEnum<'ctx>, Error> {
        debug!("Determining map value type");
        
        if !map_value.is_pointer_value() {
            return Err(Error::CodeGenError("Expected map to be a pointer type".to_string()));
        }
        
        let map_ptr = map_value.into_pointer_value();
        let map_ptr_type = map_ptr.get_type();
        let pointee_type = map_ptr_type.get_element_type();
        
        // Maps in CURSED are typically implemented as a struct with fields for
        // key type, value type, buckets, etc. We need to analyze the struct type
        if let Some(struct_type) = pointee_type.as_struct_type() {
            let type_name = struct_type.get_name()
                .and_then(|n| n.to_str().ok())
                .unwrap_or("unknown");
            
            debug!("Map struct type name: {}", type_name);
            
            // Try to extract type parameters from the name
            // Common formats include Map<KeyType,ValueType> or Dictionary<K,V>
            if type_name.contains('<') && type_name.contains('>') {
                let start_idx = type_name.find('<').unwrap() + 1;
                let end_idx = type_name.rfind('>').unwrap();
                
                if start_idx < end_idx {
                    let type_params_str = &type_name[start_idx..end_idx];
                    let type_params: Vec<&str> = type_params_str.split(',').collect();
                    
                    if type_params.len() > 1 {
                        // Second type parameter is typically the value type
                        let value_type_name = type_params[1].trim();
                        debug!("Extracted value type from name: {}", value_type_name);
                        
                        // Map the type name to an LLVM type
                        return self.map_type_name_to_llvm_type(value_type_name);
                    }
                }
            }
            
            // If we couldn't extract the type from the name, examine the struct fields
            // In many map implementations, there are type descriptors or metadata fields
            // that indicate the key and value types
            
            // For now, try to infer from the buckets or entries field if available
            if struct_type.get_field_types().len() > 0 {
                // Look for a buckets or entries field (often field 0 or 2)
                let buckets_field_indices = [0, 2];
                
                for &field_idx in &buckets_field_indices {
                    if field_idx < struct_type.get_field_types().len() {
                        let field_type = struct_type.get_field_types()[field_idx];
                        
                        // Buckets/entries field is typically a pointer to an array of entry structs
                        if let Some(ptr_type) = field_type.as_pointer_type() {
                            let element_type = ptr_type.get_element_type();
                            
                            // Try to extract value type from entry struct
                            if let Some(entry_struct) = element_type.as_struct_type() {
                                if entry_struct.get_field_types().len() > 1 {
                                    // Second field in an entry is typically the value
                                    let value_field_type = entry_struct.get_field_types()[1];
                                    debug!("Inferred value type from map entry struct");
                                    return Ok(value_field_type);
                                }
                            } else if let Some(array_type) = element_type.as_array_type() {
                                // If it's an array, look at the element type
                                let array_element = array_type.get_element_type();
                                if let Some(entry_struct) = array_element.as_struct_type() {
                                    if entry_struct.get_field_types().len() > 1 {
                                        // Second field in an entry is typically the value
                                        let value_field_type = entry_struct.get_field_types()[1];
                                        debug!("Inferred value type from map entry array");
                                        return Ok(value_field_type);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // If we couldn't determine the exact type, look for type metadata
        // in the map itself or in associated functions
        
        // For now, return a default type if we couldn't determine the value type
        // In a real implementation, we would have a more comprehensive type registry
        debug!("Using default thicc (i64) type for map value");
        Ok(self.context.i64_type().into())
    }
}

/// Private helper methods for the MapIterationEnhancements trait
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Map a type name string to an LLVM type
    fn map_type_name_to_llvm_type(&self, type_name: &str) -> Result<BasicTypeEnum<'ctx>, Error> {
        match type_name {
            // Basic primitive types
            "i8" | "byte" | "smol" => Ok(self.context.i8_type().into()),
            "i16" | "mid" => Ok(self.context.i16_type().into()),
            "i32" | "int" | "normie" => Ok(self.context.i32_type().into()),
            "i64" | "thicc" => Ok(self.context.i64_type().into()),
            "f32" | "float" | "snack" => Ok(self.context.f32_type().into()),
            "f64" | "double" | "meal" => Ok(self.context.f64_type().into()),
            "bool" | "lit" => Ok(self.context.bool_type().into()),
            
            // Character type
            "char" | "rune" | "sip" => Ok(self.context.i8_type().into()),
            
            // String type (pointer to i8)
            "string" | "str" | "tea" => {
                let ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
                Ok(ptr_type.into())
            },
            
            // Default for any other or unknown type
            _ => {
                debug!("Unknown type name: {}, using i64 as default", type_name);
                Ok(self.context.i64_type().into())
            }
        }
    }
}