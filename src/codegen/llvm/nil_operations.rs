//! Comprehensive nil representation and operations in LLVM
//!
//! This module provides a unified system for handling nil (cap) values across
//! all nullable types in the CURSED language. It implements:
//! 
//! - Consistent nil representation for all nullable types
//! - Nil literal compilation
//! - Nil comparison operations (== nil, != nil)
//! - Runtime nil checking and validation
//! - Integration with garbage collector for nil handling
//! - Memory-safe nil operations

use crate::core::type_checker::Type;
use crate::error::Error;
use crate::ast::expressions::NilLiteral;
use inkwell::types::{BasicType, BasicTypeEnum};
use inkwell::values::{BasicValue, BasicValueEnum, IntValue, PointerValue};
use inkwell::{AddressSpace, IntPredicate};
use super::LlvmCodeGenerator;
use super::zero_values::ZeroValueGeneration;
use tracing::{debug, instrument, warn, info};
use std::collections::HashMap;

/// Trait for comprehensive nil operations in LLVM code generation
pub trait NilOperations<'ctx> {
    /// Compile a nil literal to the appropriate LLVM representation
    fn compile_nil_literal(&mut self, nil_literal: &NilLiteral, expected_type: Option<&Type>) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Create a nil value for a specific type
    fn create_nil_value_for_type(&self, cursed_type: &Type) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Check if a value is nil (== nil comparison)
    fn compile_is_nil_check(&mut self, value: BasicValueEnum<'ctx>, value_type: &Type) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Check if a value is not nil (!= nil comparison)
    fn compile_is_not_nil_check(&mut self, value: BasicValueEnum<'ctx>, value_type: &Type) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Create a nil pointer for a specific pointee type
    fn create_nil_pointer_typed(&self, pointee_type: BasicTypeEnum<'ctx>) -> BasicValueEnum<'ctx>;
    
    /// Create a nil interface value with proper type information
    fn create_nil_interface_typed(&self, interface_name: Option<&str>) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Create a nil slice value with proper element type
    fn create_nil_slice_typed(&self, element_type: &Type) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Create a nil map value with proper key/value types
    fn create_nil_map_typed(&self, key_type: &Type, value_type: &Type) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Create a nil channel value with proper element type
    fn create_nil_channel_typed(&self, element_type: &Type) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Create a nil function value with proper signature
    fn create_nil_function_typed(&self, param_types: &[Type], return_type: &Type) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Check if a type can be nil
    fn type_can_be_nil(&self, cursed_type: &Type) -> bool;
    
    /// Get the nil representation size for a type
    fn get_nil_representation_size(&self, cursed_type: &Type) -> usize;
    
    /// Runtime nil validation for garbage collection integration
    fn validate_nil_for_gc(&self, value: BasicValueEnum<'ctx>, value_type: &Type) -> Result<bool, Error>;
}

impl<'ctx> NilOperations<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self), level = "debug")]
    fn compile_nil_literal(&mut self, nil_literal: &NilLiteral, expected_type: Option<&Type>) -> Result<BasicValueEnum<'ctx>, Error> {
        info!("Compiling nil literal with expected type: {:?}", expected_type);
        
        match expected_type {
            Some(ty) => {
                if !self.type_can_be_nil(ty) {
                    return Err(Error::from_str(&format!(
                    "Type {:?} cannot be nil. Only pointers, slices, maps, channels, functions, and interfaces can be nil.", 
                    ty
                    )));
                }
                self.create_nil_value_for_type(ty)
            },
            None => {
                // Default to a null pointer when type is unknown
                warn!("Compiling nil literal without type context, defaulting to null pointer");
                let ptr_type = self.context().i8_type().ptr_type(AddressSpace::default());
                Ok(ptr_type.const_null().into())
            }
        }
    }

    #[instrument(skip(self), level = "debug")]
    fn create_nil_value_for_type(&self, cursed_type: &Type) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Creating nil value for type: {:?}", cursed_type);
        
        match cursed_type {
            // Pointer types - null pointer
            Type::Pointer(target_type) => {
                let target_zero = <Self as ZeroValueGeneration<'ctx>>::create_zero_value(self, target_type)?;
                let target_llvm_type = target_zero.get_type();
                let ptr_type = target_llvm_type.ptr_type(AddressSpace::default());
                Ok(ptr_type.const_null().into())
            },
            
            // Slice types - nil slice (ptr=null, len=0, cap=0)
            Type::Slice(element_type) => self.create_nil_slice_typed(element_type),
            
            // Map types - nil map (null pointer to runtime map)
            Type::Map(key_type, value_type) => self.create_nil_map_typed(key_type, value_type),
            
            // Channel types - nil channel (null pointer to runtime channel)
            Type::Channel(element_type) => self.create_nil_channel_typed(element_type),
            
            // Function types - nil function (null function pointer)
            Type::Function(param_types, return_type) => {
                let param_types_deref: Vec<Type> = param_types.iter().map(|t| (**t).clone()).collect();
                self.create_nil_function_typed(&param_types_deref, return_type)
            },
            
            // Interface types - nil interface (null data and type pointers)
            Type::Interface(name, _type_params) => self.create_nil_interface_typed(Some(name)),
            
            _ => Err(Error::from_str(&format!("Type {:?} cannot be nil", cursed_type)))
        }
    }

    #[instrument(skip(self), level = "debug")]
    fn compile_is_nil_check(&mut self, value: BasicValueEnum<'ctx>, value_type: &Type) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Compiling nil check for type: {:?}", value_type);
        
        match value_type {
            // Pointer types - check if pointer is null
            Type::Pointer(_) => {
                if let BasicValueEnum::PointerValue(ptr_val) = value {
                    let null_ptr = ptr_val.get_type().const_null();
                    Ok(self.builder().build_int_compare(
                        IntPredicate::EQ,
                        self.builder().build_ptr_to_int(ptr_val, self.context().i64_type(), "ptr_int")?,
                        self.builder().build_ptr_to_int(null_ptr, self.context().i64_type(), "null_int")?,
                        "is_nil_ptr"
                    )?.into())
                } else {
                    Err(Error::from_str("Expected pointer value for pointer nil check"))
                }
            },
            
            // Slice types - check if data pointer is null
            Type::Slice(_) => {
                if let BasicValueEnum::StructValue(slice_val) = value {
                    let data_ptr = self.builder().build_extract_value(slice_val, 0, "slice_data")?
                        .into_pointer_value();
                    let null_ptr = data_ptr.get_type().const_null();
                    Ok(self.builder().build_int_compare(
                        IntPredicate::EQ,
                        self.builder().build_ptr_to_int(data_ptr, self.context().i64_type(), "slice_ptr_int")?,
                        self.builder().build_ptr_to_int(null_ptr, self.context().i64_type(), "slice_null_int")?,
                        "is_nil_slice"
                    )?.into())
                } else {
                    Err(Error::from_str("Expected struct value for slice nil check"))
                }
            },
            
            // Map, channel, and function types - check if pointer is null
            Type::Map(_, _) | Type::Channel(_) | Type::Function(_, _) => {
                if let BasicValueEnum::PointerValue(ptr_val) = value {
                    let null_ptr = ptr_val.get_type().const_null();
                    Ok(self.builder().build_int_compare(
                        IntPredicate::EQ,
                        self.builder().build_ptr_to_int(ptr_val, self.context().i64_type(), "ptr_int")?,
                        self.builder().build_ptr_to_int(null_ptr, self.context().i64_type(), "null_int")?,
                        "is_nil_ptr"
                    )?.into())
                } else {
                    Err(Error::from_str("Expected pointer value for nil check"))
                }
            },
            
            // Interface types - check if both data and type pointers are null
            Type::Interface(_, _) => {
                if let BasicValueEnum::StructValue(interface_val) = value {
                    let data_ptr = self.builder().build_extract_value(interface_val, 0, "interface_data")?
                        .into_pointer_value();
                    let type_ptr = self.builder().build_extract_value(interface_val, 1, "interface_type")?
                        .into_pointer_value();
                    
                    let data_null = data_ptr.get_type().const_null();
                    let type_null = type_ptr.get_type().const_null();
                    
                    let data_is_null = self.builder().build_int_compare(
                        IntPredicate::EQ,
                        self.builder().build_ptr_to_int(data_ptr, self.context().i64_type(), "data_ptr_int")?,
                        self.builder().build_ptr_to_int(data_null, self.context().i64_type(), "data_null_int")?,
                        "data_is_null"
                    )?;
                    
                    let type_is_null = self.builder().build_int_compare(
                        IntPredicate::EQ,
                        self.builder().build_ptr_to_int(type_ptr, self.context().i64_type(), "type_ptr_int")?,
                        self.builder().build_ptr_to_int(type_null, self.context().i64_type(), "type_null_int")?,
                        "type_is_null"
                    )?;
                    
                    // Interface is nil if both data and type pointers are null
                    Ok(self.builder().build_and(data_is_null, type_is_null, "is_nil_interface")?.into())
                } else {
                    Err(Error::from_str("Expected struct value for interface nil check"))
                }
            },
            
            _ => Err(Error::from_str(&format!("Type {:?} cannot be checked for nil", value_type)))
        }
    }

    fn compile_is_not_nil_check(&mut self, value: BasicValueEnum<'ctx>, value_type: &Type) -> Result<BasicValueEnum<'ctx>, Error> {
        let is_nil = self.compile_is_nil_check(value, value_type)?;
        if let BasicValueEnum::IntValue(nil_bool) = is_nil {
            Ok(self.builder().build_not(nil_bool, "is_not_nil")?.into())
        } else {
            Err(Error::from_str("Expected boolean result from nil check"))
        }
    }

    fn create_nil_pointer_typed(&self, pointee_type: BasicTypeEnum<'ctx>) -> BasicValueEnum<'ctx> {
        pointee_type.ptr_type(AddressSpace::default()).const_null().into()
    }

    #[instrument(skip(self), level = "debug")]
    fn create_nil_interface_typed(&self, interface_name: Option<&str>) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Creating nil interface for: {:?}", interface_name);
        
        let interface_struct = self.context().struct_type(&[
            self.context().i8_type().ptr_type(AddressSpace::default()).into(), // data pointer
            self.context().i8_type().ptr_type(AddressSpace::default()).into(), // type info pointer
        ], false);
        
        let null_data = self.context().i8_type().ptr_type(AddressSpace::default()).const_null();
        let null_type_info = self.context().i8_type().ptr_type(AddressSpace::default()).const_null();
        
        Ok(interface_struct.const_named_struct(&[
            null_data.into(),
            null_type_info.into(),
        ]).into())
    }

    #[instrument(skip(self), level = "debug")]
    fn create_nil_slice_typed(&self, element_type: &Type) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Creating nil slice for element type: {:?}", element_type);
        
        let slice_struct = self.context().struct_type(&[
            self.context().i8_type().ptr_type(AddressSpace::default()).into(), // data pointer
            self.context().i64_type().into(), // length
            self.context().i64_type().into(), // capacity
        ], false);
        
        let null_ptr = self.context().i8_type().ptr_type(AddressSpace::default()).const_null();
        let zero_len = self.context().i64_type().const_zero();
        let zero_cap = self.context().i64_type().const_zero();
        
        Ok(slice_struct.const_named_struct(&[
            null_ptr.into(),
            zero_len.into(),
            zero_cap.into(),
        ]).into())
    }

    #[instrument(skip(self), level = "debug")]
    fn create_nil_map_typed(&self, key_type: &Type, value_type: &Type) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Creating nil map for key type: {:?}, value type: {:?}", key_type, value_type);
        
        // Map is represented as a pointer to runtime map structure
        let map_ptr_type = self.context().i8_type().ptr_type(AddressSpace::default());
        Ok(map_ptr_type.const_null().into())
    }

    #[instrument(skip(self), level = "debug")]
    fn create_nil_channel_typed(&self, element_type: &Type) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Creating nil channel for element type: {:?}", element_type);
        
        // Channel is represented as a pointer to runtime channel structure
        let channel_ptr_type = self.context().i8_type().ptr_type(AddressSpace::default());
        Ok(channel_ptr_type.const_null().into())
    }

    #[instrument(skip(self), level = "debug")]
    fn create_nil_function_typed(&self, param_types: &[Type], return_type: &Type) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Creating nil function with {} parameters, return type: {:?}", param_types.len(), return_type);
        
        // Function is represented as a function pointer
        let func_ptr_type = self.context().i8_type().ptr_type(AddressSpace::default());
        Ok(func_ptr_type.const_null().into())
    }

    fn type_can_be_nil(&self, cursed_type: &Type) -> bool {
        match cursed_type {
            Type::Pointer(_) | Type::Slice(_) | Type::Map(_, _) | 
            Type::Channel(_) | Type::Function(_, _) | Type::Interface(_, _) => true,
            _ => false,
        }
    }

    fn get_nil_representation_size(&self, cursed_type: &Type) -> usize {
        match cursed_type {
            Type::Pointer(_) => 8,              // 64-bit pointer
            Type::Slice(_) => 24,               // {ptr, len, cap} = 8 + 8 + 8
            Type::Map(_, _) => 8,               // pointer to runtime map
            Type::Channel(_) => 8,              // pointer to runtime channel
            Type::Function(_, _) => 8,          // function pointer
            Type::Interface(_, _) => 16,        // {data_ptr, type_ptr} = 8 + 8
            _ => 0,                             // non-nullable types
        }
    }

    #[instrument(skip(self), level = "debug")]
    fn validate_nil_for_gc(&self, value: BasicValueEnum<'ctx>, value_type: &Type) -> Result<bool, Error> {
        debug!("Validating nil value for GC integration, type: {:?}", value_type);
        
        // For GC purposes, nil values should not be tracked as they don't reference any heap objects
        match value_type {
            Type::Pointer(_) | Type::Slice(_) | Type::Map(_, _) | 
            Type::Channel(_) | Type::Function(_, _) | Type::Interface(_, _) => {
                // These types can be nil and when nil, they don't reference any GC objects
                Ok(true)
            },
            _ => {
                // Non-nullable types are never nil
                Ok(false)
            }
        }
    }
}

/// Extension trait for nil operations integration with existing code generator
pub trait NilOperationsExtension<'ctx> {
    /// Integrate nil operations into expression compilation
    fn compile_nil_expression(&mut self, nil_literal: &NilLiteral, expected_type: Option<&Type>) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Compile nil comparison in binary operations
    fn compile_nil_comparison(&mut self, left: BasicValueEnum<'ctx>, right: BasicValueEnum<'ctx>, 
                              left_type: &Type, right_type: &Type, operator: &str) -> Result<BasicValueEnum<'ctx>, Error>;
}

impl<'ctx> NilOperationsExtension<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self), level = "debug")]
    fn compile_nil_expression(&mut self, nil_literal: &NilLiteral, expected_type: Option<&Type>) -> Result<BasicValueEnum<'ctx>, Error> {
        self.compile_nil_literal(nil_literal, expected_type)
    }

    #[instrument(skip(self), level = "debug")]
    fn compile_nil_comparison(&mut self, left: BasicValueEnum<'ctx>, right: BasicValueEnum<'ctx>, 
                              left_type: &Type, right_type: &Type, operator: &str) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Compiling nil comparison: {:?} {} {:?}", left_type, operator, right_type);
        
        match operator {
            "==" => {
                // Check if one operand is nil and the other is a nullable type
                if self.type_can_be_nil(left_type) && right_type == &Type::Unknown {
                    // right is nil literal, check if left is nil
                    self.compile_is_nil_check(left, left_type)
                } else if left_type == &Type::Unknown && self.type_can_be_nil(right_type) {
                    // left is nil literal, check if right is nil
                    self.compile_is_nil_check(right, right_type)
                } else {
                    Err(Error::from_str(&format!("Invalid nil comparison between {:?} and {:?}", left_type, right_type)))
                }
            },
            "!=" => {
                // Check if one operand is nil and the other is a nullable type
                if self.type_can_be_nil(left_type) && right_type == &Type::Unknown {
                    // right is nil literal, check if left is not nil
                    self.compile_is_not_nil_check(left, left_type)
                } else if left_type == &Type::Unknown && self.type_can_be_nil(right_type) {
                    // left is nil literal, check if right is not nil
                    self.compile_is_not_nil_check(right, right_type)
                } else {
                    Err(Error::from_str(&format!("Invalid nil comparison between {:?} and {:?}", left_type, right_type)))
                }
            },
            _ => Err(Error::from_str(&format!("Unsupported nil comparison operator: {}", operator)))
        }
    }
}

/// Helper functions for nil operation utilities
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Check if an LLVM value represents a nil value
    #[instrument(skip(self), level = "debug")]
    pub fn is_llvm_value_nil(&self, value: BasicValueEnum<'ctx>, value_type: &Type) -> Result<bool, Error> {
        debug!("Checking if LLVM value is nil for type: {:?}", value_type);
        
        match value_type {
            Type::Pointer(_) => {
                if let BasicValueEnum::PointerValue(ptr_val) = value {
                    Ok(ptr_val.is_null())
                } else {
                    Ok(false)
                }
            },
            Type::Slice(_) => {
                if let BasicValueEnum::StructValue(_) = value {
                    // For slices, we'd need to extract and check the data pointer
                    // This is a compile-time check, so we can't easily determine this
                    Ok(false)
                } else {
                    Ok(false)
                }
            },
            Type::Map(_, _) | Type::Channel(_) | Type::Function(_, _) => {
                if let BasicValueEnum::PointerValue(ptr_val) = value {
                    Ok(ptr_val.is_null())
                } else {
                    Ok(false)
                }
            },
            Type::Interface(_, _) => {
                // For interfaces, we'd need to check both pointers
                // This is complex for compile-time checking
                Ok(false)
            },
            _ => Ok(false), // non-nullable types are never nil
        }
    }
    
    /// Create a typed nil value with proper LLVM type annotation
    #[instrument(skip(self), level = "debug")]
    pub fn create_typed_nil_value(&self, cursed_type: &Type) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Creating typed nil value for: {:?}", cursed_type);
        self.create_nil_value_for_type(cursed_type)
    }
}
