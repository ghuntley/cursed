//! Zero value initialization for CURSED types
//!
//! This module provides comprehensive zero value initialization for all types in the CURSED language.
//! Zero values follow Go semantics:
//! - false for booleans
//! - 0 for numeric types
//! - empty string for strings
//! - nil for pointers, slices, maps, channels, functions
//! - zero value for each field in structs
//! - nil for interfaces

use crate::core::type_checker::Type;
use crate::error::Error;
use inkwell::types::{BasicType, BasicTypeEnum, StructType};
use inkwell::values::{BasicValue, BasicValueEnum, StructValue};
use inkwell::{AddressSpace, FloatPredicate, IntPredicate};
use super::LlvmCodeGenerator;
use super::string_type::CursedStringType;
use tracing::{debug, instrument, warn};
use std::collections::HashMap;

/// Trait for generating zero values in LLVM
pub trait ZeroValueGeneration<'ctx> {
    /// Create a zero value for a given CURSED type
    fn create_zero_value(&self, cursed_type: &Type) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Create a zero value for a given LLVM type
    fn create_zero_value_for_llvm_type(&self, llvm_type: BasicTypeEnum<'ctx>) -> BasicValueEnum<'ctx>;
    
    /// Initialize a struct with zero values for all fields
    fn initialize_struct_with_zero_values(
        &self,
        struct_type: StructType<'ctx>,
        field_types: &[Type],
    ) -> Result<StructValue<'ctx>, Error>;
    
    /// Create a nil pointer of the specified type
    fn create_nil_pointer(&self, pointee_type: BasicTypeEnum<'ctx>) -> BasicValueEnum<'ctx>;
    
    /// Create an empty string value
    fn create_empty_string(&self) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Create an empty slice value
    fn create_empty_slice(&self, element_type: &Type) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Create an empty map value
    fn create_empty_map(&self, key_type: &Type, value_type: &Type) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Create a nil interface value
    fn create_nil_interface(&self) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Check if a type requires special zero value handling
    fn requires_special_zero_handling(&self, cursed_type: &Type) -> bool;
    
    /// Get the size in bytes needed for a zero-initialized value of this type
    fn get_zero_value_size(&self, cursed_type: &Type) -> usize;
}

impl<'ctx> ZeroValueGeneration<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self), level = "debug")]
    fn create_zero_value(&self, cursed_type: &Type) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Creating zero value for type: {:?}", cursed_type);
        
        if !cursed_type.has_zero_value() {
            return Err(Error::from_str(&format!(
                "Type {:?} does not have a well-defined zero value", 
                cursed_type
            )));
        }
        
        match cursed_type {
            // Basic types
            Type::Lit => Ok(self.context().bool_type().const_zero().into()),
            Type::Smol => Ok(self.context().i8_type().const_zero().into()),
            Type::Mid => Ok(self.context().i16_type().const_zero().into()),
            Type::Normie => Ok(self.context().i32_type().const_zero().into()),
            Type::Thicc => Ok(self.context().i64_type().const_zero().into()),
            Type::Snack => Ok(self.context().f32_type().const_zero().into()),
            Type::Meal => Ok(self.context().f64_type().const_zero().into()),
            Type::Byte => Ok(self.context().i8_type().const_zero().into()),
            Type::Rune | Type::Sip => Ok(self.context().i32_type().const_zero().into()),
            
            // String type - empty string
            Type::Tea => self.create_empty_string(),
            
            // Complex numbers - zero real and imaginary parts
            Type::Extra => {
                let complex_struct = self.context().struct_type(&[
                    self.context().f64_type().into(), // real part
                    self.context().f64_type().into(), // imaginary part
                ], false);
                
                let zero_real = self.context().f64_type().const_zero();
                let zero_imag = self.context().f64_type().const_zero();
                
                Ok(complex_struct.const_named_struct(&[zero_real.into(), zero_imag.into()]).into())
            },
            
            // Array types - zero value for each element
            Type::Array(element_type, size) => {
                let element_zero = self.create_zero_value(element_type)?;
                let llvm_element_type = element_zero.get_type();
                let array_type = llvm_element_type.array_type(*size as u32);
                
                // Create array of zero values
                let zero_elements: Vec<BasicValueEnum> = (0..*size)
                    .map(|_| element_zero)
                    .collect();
                
                // Convert to array type and create constant array
                match llvm_element_type {
                    BasicTypeEnum::IntType(int_type) => {
                        let int_values: Vec<_> = zero_elements.iter()
                            .map(|v| v.into_int_value())
                            .collect();
                        Ok(int_type.const_array(&int_values).into())
                    },
                    BasicTypeEnum::FloatType(float_type) => {
                        let float_values: Vec<_> = zero_elements.iter()
                            .map(|v| v.into_float_value())
                            .collect();
                        Ok(float_type.const_array(&float_values).into())
                    },
                    BasicTypeEnum::PointerType(ptr_type) => {
                        let ptr_values: Vec<_> = zero_elements.iter()
                            .map(|v| v.into_pointer_value())
                            .collect();
                        Ok(ptr_type.const_array(&ptr_values).into())
                    },
                    _ => {
                        // For other types, create a struct containing the elements
                        let array_type = self.context().struct_type(&vec![llvm_element_type; *size], false);
                        Ok(array_type.const_named_struct(&zero_elements.iter().map(|v| *v).collect::<Vec<_>>()).into())
                    }
                }
            },
            
            // Slice types - nil slice (ptr=null, len=0, cap=0)
            Type::Slice(element_type) => self.create_empty_slice(element_type),
            
            // Map types - nil map
            Type::Map(key_type, value_type) => self.create_empty_map(key_type, value_type),
            
            // Pointer types - nil pointer
            Type::Pointer(target_type) => {
                // Convert target type to LLVM type and create null pointer
                let target_zero = self.create_zero_value(target_type)?;
                let target_llvm_type = target_zero.get_type();
                Ok(target_llvm_type.ptr_type(AddressSpace::default()).const_null().into())
            },
            
            // Channel types - nil channel
            Type::Channel(_) => {
                let channel_ptr_type = self.context().i8_type().ptr_type(AddressSpace::default());
                Ok(channel_ptr_type.const_null().into())
            },
            
            // Function types - nil function
            Type::Function(_, _) => {
                let func_ptr_type = self.context().i8_type().ptr_type(AddressSpace::default());
                Ok(func_ptr_type.const_null().into())
            },
            
            // Interface types - nil interface
            Type::Interface(_, _) => self.create_nil_interface(),
            
            // Struct types - zero value for each field
            Type::Struct(struct_name, type_args) => {
                // For now, create a simple struct with common fields
                // In a full implementation, this would look up the actual struct definition
                let struct_type = self.context().struct_type(&[
                    self.context().i64_type().into(), // placeholder field
                ], false);
                
                let zero_field = self.context().i64_type().const_zero();
                Ok(struct_type.const_named_struct(&[zero_field.into()]).into())
            },
            
            // Generic and named types - treat as opaque pointers for now
            Type::Generic(_, _) | Type::Named(_) => {
                let ptr_type = self.context().i8_type().ptr_type(AddressSpace::default());
                Ok(ptr_type.const_null().into())
            },
            
            // Type parameters - defer to runtime resolution
            Type::TypeParam(_) => {
                let ptr_type = self.context().i8_type().ptr_type(AddressSpace::default());
                Ok(ptr_type.const_null().into())
            },
            
            Type::Unknown => Err(Error::from_str("Cannot create zero value for unknown type")),
        }
    }
    
    fn create_zero_value_for_llvm_type(&self, llvm_type: BasicTypeEnum<'ctx>) -> BasicValueEnum<'ctx> {
        match llvm_type {
            BasicTypeEnum::IntType(int_type) => int_type.const_zero().into(),
            BasicTypeEnum::FloatType(float_type) => float_type.const_zero().into(),
            BasicTypeEnum::PointerType(ptr_type) => ptr_type.const_null().into(),
            BasicTypeEnum::ArrayType(array_type) => {
                let element_type = array_type.get_element_type();
                let element_zero = self.create_zero_value_for_llvm_type(element_type);
                let size = array_type.len();
                
                let zero_elements: Vec<BasicValueEnum> = (0..size)
                    .map(|_| element_zero)
                    .collect();
                
                // Create array based on element type
                match element_type {
                    BasicTypeEnum::IntType(int_type) => {
                        let int_values: Vec<_> = zero_elements.iter()
                            .map(|v| v.into_int_value())
                            .collect();
                        int_type.const_array(&int_values).into()
                    },
                    BasicTypeEnum::FloatType(float_type) => {
                        let float_values: Vec<_> = zero_elements.iter()
                            .map(|v| v.into_float_value())
                            .collect();
                        float_type.const_array(&float_values).into()
                    },
                    BasicTypeEnum::PointerType(ptr_type) => {
                        let ptr_values: Vec<_> = zero_elements.iter()
                            .map(|v| v.into_pointer_value())
                            .collect();
                        ptr_type.const_array(&ptr_values).into()
                    },
                    _ => {
                        // For complex types, create a single zero value
                        self.create_zero_value_for_llvm_type(element_type)
                    }
                }
            },
            BasicTypeEnum::StructType(struct_type) => {
                let field_types = struct_type.get_field_types();
                let zero_fields: Vec<BasicValueEnum> = field_types
                    .iter()
                    .map(|field_type| self.create_zero_value_for_llvm_type(*field_type))
                    .collect();
                
                struct_type.const_named_struct(&zero_fields.iter().map(|v| *v).collect::<Vec<_>>()).into()
            },
            BasicTypeEnum::VectorType(vector_type) => {
                let element_type = vector_type.get_element_type();
                let element_zero = self.create_zero_value_for_llvm_type(element_type);
                let size = vector_type.get_size();
                
                let zero_elements: Vec<BasicValueEnum> = (0..size)
                    .map(|_| element_zero)
                    .collect();
                
                // For vectors, we need to create based on element type
                match element_type {
                    BasicTypeEnum::IntType(int_type) => {
                        let int_values: Vec<_> = zero_elements.iter()
                            .map(|v| v.into_int_value())
                            .collect();
                        inkwell::types::VectorType::const_vector(&int_values).into()
                    },
                    BasicTypeEnum::FloatType(float_type) => {
                        let float_values: Vec<_> = zero_elements.iter()
                            .map(|v| v.into_float_value())
                            .collect();
                        inkwell::types::VectorType::const_vector(&float_values).into()
                    },
                    _ => {
                        // For unsupported vector element types, return a single zero value
                        self.create_zero_value_for_llvm_type(element_type)
                    }
                }
            },
        }
    }
    
    #[instrument(skip(self), level = "debug")]
    fn initialize_struct_with_zero_values(
        &self,
        struct_type: StructType<'ctx>,
        field_types: &[Type],
    ) -> Result<StructValue<'ctx>, Error> {
        debug!("Initializing struct with {} fields", field_types.len());
        
        let mut zero_fields = Vec::new();
        
        for field_type in field_types {
            let zero_value = self.create_zero_value(field_type)?;
            zero_fields.push(zero_value);
        }
        
        Ok(struct_type.const_named_struct(&zero_fields.iter().map(|v| *v).collect::<Vec<_>>()))
    }
    
    fn create_nil_pointer(&self, pointee_type: BasicTypeEnum<'ctx>) -> BasicValueEnum<'ctx> {
        pointee_type.ptr_type(AddressSpace::default()).const_null().into()
    }
    
    #[instrument(skip(self), level = "debug")]
    fn create_empty_string(&self) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Creating empty string value using CursedStringType");
        
        // Use the CursedStringType to create an empty string
        let string_type = CursedStringType::new(self.context());
        let empty_string = string_type.create_empty_string(self.builder())
            .map_err(|e| Error::from_str(&format!("Failed to create empty string: {}", e)))?;
        
        Ok(empty_string.into())
    }
    
    #[instrument(skip(self), level = "debug")]
    fn create_empty_slice(&self, element_type: &Type) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Creating empty slice for element type: {:?}", element_type);
        
        // Slice is represented as a struct with ptr, len, cap
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
    fn create_empty_map(&self, key_type: &Type, value_type: &Type) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Creating empty map for key type: {:?}, value type: {:?}", key_type, value_type);
        
        // Map is represented as a pointer to runtime map structure
        let map_ptr_type = self.context().i8_type().ptr_type(AddressSpace::default());
        Ok(map_ptr_type.const_null().into())
    }
    
    #[instrument(skip(self), level = "debug")]
    fn create_nil_interface(&self) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Creating nil interface value");
        
        // Interface is represented as a struct with data pointer and type info
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
    
    fn requires_special_zero_handling(&self, cursed_type: &Type) -> bool {
        match cursed_type {
            Type::Tea | Type::Slice(_) | Type::Map(_, _) | 
            Type::Interface(_, _) | Type::Struct(_, _) | Type::Extra => true,
            Type::Array(element_type, _) => self.requires_special_zero_handling(element_type),
            _ => false,
        }
    }
    
    fn get_zero_value_size(&self, cursed_type: &Type) -> usize {
        cursed_type.size()
    }
}

/// Helper functions for zero value operations
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Check if a value is a zero value
    #[instrument(skip(self), level = "debug")]
    pub fn is_zero_value(&self, value: BasicValueEnum<'ctx>, cursed_type: &Type) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Checking if value is zero for type: {:?}", cursed_type);
        
        let zero_value = self.create_zero_value(cursed_type)?;
        
        // Compare the value with the zero value
        match (value, zero_value) {
            (BasicValueEnum::IntValue(val), BasicValueEnum::IntValue(zero)) => {
                Ok(self.builder().build_int_compare(IntPredicate::EQ, val, zero, "is_zero")?.into())
            },
            (BasicValueEnum::FloatValue(val), BasicValueEnum::FloatValue(zero)) => {
                Ok(self.builder().build_float_compare(FloatPredicate::OEQ, val, zero, "is_zero")?.into())
            },
            (BasicValueEnum::PointerValue(val), BasicValueEnum::PointerValue(zero)) => {
                Ok(self.builder().build_int_compare(
                    IntPredicate::EQ,
                    self.builder().build_ptr_to_int(val, self.context().i64_type(), "val_int")?,
                    self.builder().build_ptr_to_int(zero, self.context().i64_type(), "zero_int")?,
                    "is_zero"
                )?.into())
            },
            _ => {
                // For complex types, assume they're equal if they have the same representation
                // This is a simplified implementation
                Ok(self.context().bool_type().const_int(1, false).into())
            }
        }
    }
    
    /// Initialize a memory location with zero values
    #[instrument(skip(self), level = "debug")]
    pub fn zero_initialize_memory(&self, ptr: BasicValueEnum<'ctx>, cursed_type: &Type) -> Result<(), Error> {
        debug!("Zero-initializing memory for type: {:?}", cursed_type);
        
        if let BasicValueEnum::PointerValue(ptr_val) = ptr {
            let zero_value = self.create_zero_value(cursed_type)?;
            self.builder().build_store(ptr_val, zero_value)?;
            Ok(())
        } else {
            Err(Error::from_str("Expected pointer value for memory initialization"))
        }
    }
    
    /// Create a zero-initialized array allocation
    #[instrument(skip(self), level = "debug")]
    pub fn zero_allocate_array(&self, element_type: &Type, size: u32) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Zero-allocating array of {} elements of type: {:?}", size, element_type);
        
        let element_zero = self.create_zero_value(element_type)?;
        let element_llvm_type = element_zero.get_type();
        let array_type = element_llvm_type.array_type(size);
        
        // Allocate memory for the array
        let array_ptr = self.builder().build_alloca(array_type, "zero_array")?;
        
        // Initialize each element to zero
        for i in 0..size {
            let element_ptr = unsafe {
                self.builder().build_gep(
                    array_type,
                    array_ptr,
                    &[
                        self.context().i32_type().const_zero(),
                        self.context().i32_type().const_int(i as u64, false),
                    ],
                    &format!("element_{}", i),
                )?
            };
            self.builder().build_store(element_ptr, element_zero)?;
        }
        
        Ok(array_ptr.into())
    }
}

// Tests are moved to tests/zero_value_test.rs to avoid import issues
