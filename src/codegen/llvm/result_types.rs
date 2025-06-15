//! Production-ready Result<T,E> and Option<T> type system for LLVM code generation
//!
//! This module provides comprehensive LLVM code generation for Result<T, E> and Option<T>
//! types with proper type-specific memory layouts, efficient tagged unions, and seamless
//! integration with the CURSED type system and error propagation mechanisms.

use crate::codegen::llvm::LlvmCodeGenerator;
use crate::error::CursedError;
use inkwell::types::{BasicTypeEnum, StructType, IntType, BasicType};
use inkwell::values::{BasicValueEnum, StructValue, IntValue, BasicValue};
use inkwell::IntPredicate;
use std::collections::HashMap;

/// Advanced trait for compiling Result and Option types with proper LLVM integration
pub trait ResultTypeCompiler {
    /// Generate LLVM struct type for Result<T, E>
    fn generate_result_type(
        &mut self,
        ok_type: BasicTypeEnum<'_>,
        err_type: BasicTypeEnum<'_>,
    ) -> Result<StructType<'_>, CursedError>;

    /// Generate LLVM struct type for Option<T>
    fn generate_option_type(
        &mut self,
        inner_type: BasicTypeEnum<'_>,
    ) -> Result<StructType<'_>, CursedError>;

    /// Create Result::Ok value with proper type handling
    fn create_result_ok(
        &mut self,
        ok_type: BasicTypeEnum<'_>,
        err_type: BasicTypeEnum<'_>,
        value: BasicValueEnum<'_>,
    ) -> Result<BasicValueEnum<'_>, CursedError>;

    /// Create Result::Err value with proper type handling
    fn create_result_err(
        &mut self,
        ok_type: BasicTypeEnum<'_>,
        err_type: BasicTypeEnum<'_>,
        error: BasicValueEnum<'_>,
    ) -> Result<BasicValueEnum<'_>, CursedError>;

    /// Create Option::Some value with proper type handling
    fn create_option_some(
        &mut self,
        inner_type: BasicTypeEnum<'_>,
        value: BasicValueEnum<'_>,
    ) -> Result<BasicValueEnum<'_>, CursedError>;

    /// Create Option::None value with proper type handling
    fn create_option_none(
        &mut self,
        inner_type: BasicTypeEnum<'_>,
    ) -> Result<BasicValueEnum<'_>, CursedError>;

    /// Check if Result is Ok
    fn is_result_ok(
        &mut self,
        result_value: BasicValueEnum<'_>,
    ) -> Result<IntValue<'_>, CursedError>;

    /// Check if Result is Err
    fn is_result_err(
        &mut self,
        result_value: BasicValueEnum<'_>,
    ) -> Result<IntValue<'_>, CursedError>;

    /// Check if Option is Some
    fn is_option_some(
        &mut self,
        option_value: BasicValueEnum<'_>,
    ) -> Result<IntValue<'_>, CursedError>;

    /// Check if Option is None
    fn is_option_none(
        &mut self,
        option_value: BasicValueEnum<'_>,
    ) -> Result<IntValue<'_>, CursedError>;

    /// Extract Ok value from Result (assumes Ok)
    fn extract_result_ok(
        &mut self,
        result_value: BasicValueEnum<'_>,
        ok_type: BasicTypeEnum<'_>,
    ) -> Result<BasicValueEnum<'_>, CursedError>;

    /// Extract Err value from Result (assumes Err)
    fn extract_result_err(
        &mut self,
        result_value: BasicValueEnum<'_>,
        err_type: BasicTypeEnum<'_>,
    ) -> Result<BasicValueEnum<'_>, CursedError>;

    /// Extract Some value from Option (assumes Some)
    fn extract_option_some(
        &mut self,
        option_value: BasicValueEnum<'_>,
        inner_type: BasicTypeEnum<'_>,
    ) -> Result<BasicValueEnum<'_>, CursedError>;

    /// Get LLVM representation string for debugging
    fn get_result_type_string(&self, ok_type: &str, err_type: &str) -> String;
    
    /// Get LLVM representation string for debugging
    fn get_option_type_string(&self, inner_type: &str) -> String;
}

/// Memory layout configuration for different type combinations
#[derive(Debug, Clone)]
pub struct TypeLayout {
    /// Total size in bytes
    pub size: usize,
    /// Memory alignment requirement
    pub alignment: usize,
    /// Discriminant size (typically 1 byte)
    pub discriminant_size: usize,
    /// Data payload size
    pub data_size: usize,
    /// Whether padding is needed
    pub needs_padding: bool,
}

/// Result type discriminant values
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResultDiscriminant {
    Ok = 0,
    Err = 1,
}

/// Option type discriminant values
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OptionDiscriminant {
    None = 0,
    Some = 1,
}

impl ResultTypeCompiler for LlvmCodeGenerator {
    fn generate_result_type(
        &mut self,
        ok_type: BasicTypeEnum<'_>,
        err_type: BasicTypeEnum<'_>,
    ) -> Result<StructType<'_>, CursedError> {
        let context = &*self.context;
        
        // Calculate the maximum size needed for the union
        let ok_size = self.get_type_size(ok_type)?;
        let err_size = self.get_type_size(err_type)?;
        let max_size = ok_size.max(err_size);
        
        // Create discriminant type (i8)
        let discriminant_type = context.i8_type();
        
        // For small types, use direct embedding
        if max_size <= 8 {
            let data_type = if max_size <= 1 {
                context.i8_type().into()
            } else if max_size <= 2 {
                context.i16_type().into()
            } else if max_size <= 4 {
                context.i32_type().into()
            } else {
                context.i64_type().into()
            };
            
            // Structure: { discriminant: i8, data: iN }
            let field_types = [discriminant_type.into(), data_type];
            Ok(context.struct_type(&field_types, false))
        } else {
            // For larger types, use a byte array to accommodate the largest type
            let data_array_type = context.i8_type().array_type(max_size as u32);
            
            // Structure: { discriminant: i8, data: [max_size x i8] }
            let field_types = [discriminant_type.into(), data_array_type.into()];
            Ok(context.struct_type(&field_types, false))
        }
    }

    fn generate_option_type(
        &mut self,
        inner_type: BasicTypeEnum<'_>,
    ) -> Result<StructType<'_>, CursedError> {
        let context = &*self.context;
        
        // Calculate the size needed for the inner type
        let inner_size = self.get_type_size(inner_type)?;
        
        // Create discriminant type (i8)
        let discriminant_type = context.i8_type();
        
        // For small types, use direct embedding
        if inner_size <= 8 {
            let data_type = if inner_size <= 1 {
                context.i8_type().into()
            } else if inner_size <= 2 {
                context.i16_type().into()
            } else if inner_size <= 4 {
                context.i32_type().into()
            } else {
                context.i64_type().into()
            };
            
            // Structure: { discriminant: i8, data: iN }
            let field_types = [discriminant_type.into(), data_type.into()];
            Ok(context.struct_type(&field_types, false))
        } else {
            // For larger types, use a byte array
            let data_array_type = context.i8_type().array_type(inner_size as u32);
            
            // Structure: { discriminant: i8, data: [inner_size x i8] }
            let field_types = [discriminant_type.into(), data_array_type.into()];
            Ok(context.struct_type(&field_types, false))
        }
    }

    fn create_result_ok(
        &mut self,
        ok_type: BasicTypeEnum<'_>,
        err_type: BasicTypeEnum<'_>,
        value: BasicValueEnum<'_>,
    ) -> Result<BasicValueEnum<'_>, CursedError> {
        let result_type = self.generate_result_type(ok_type, err_type)?;
        let builder = &self.builder.lock().unwrap();
        
        // Create discriminant value (0 for Ok)
        let discriminant = self.context.i8_type().const_int(ResultDiscriminant::Ok as u64, false);
        
        // Start with undefined struct value
        let mut result_value = result_type.get_undef();
        
        // Insert discriminant at index 0
        result_value = builder.build_insert_value(
            result_value,
            discriminant,
            0,
            "result_ok_discriminant"
        ).map_err(|e| CursedError::llvm_error(format!("Failed to insert discriminant: {}", e)))?
        .into_struct_value();

        // Convert and insert the value data
        let converted_data = self.convert_value_to_data_field(value, ok_type, &result_type)?;
        result_value = builder.build_insert_value(
            result_value,
            converted_data,
            1,
            "result_ok_data"
        ).map_err(|e| CursedError::llvm_error(format!("Failed to insert ok data: {}", e)))?
        .into_struct_value();

        Ok(result_value.into())
    }

    fn create_result_err(
        &mut self,
        ok_type: BasicTypeEnum<'_>,
        err_type: BasicTypeEnum<'_>,
        error: BasicValueEnum<'_>,
    ) -> Result<BasicValueEnum<'_>, CursedError> {
        let result_type = self.generate_result_type(ok_type, err_type)?;
        let builder = &self.builder.lock().unwrap();
        
        // Create discriminant value (1 for Err)
        let discriminant = self.context.i8_type().const_int(ResultDiscriminant::Err as u64, false);
        
        // Start with undefined struct value
        let mut result_value = result_type.get_undef();
        
        // Insert discriminant at index 0
        result_value = builder.build_insert_value(
            result_value,
            discriminant,
            0,
            "result_err_discriminant"
        ).map_err(|e| CursedError::llvm_error(format!("Failed to insert discriminant: {}", e)))?
        .into_struct_value();

        // Convert and insert the error data
        let converted_data = self.convert_value_to_data_field(error, err_type, &result_type)?;
        result_value = builder.build_insert_value(
            result_value,
            converted_data,
            1,
            "result_err_data"
        ).map_err(|e| CursedError::llvm_error(format!("Failed to insert err data: {}", e)))?
        .into_struct_value();

        Ok(result_value.into())
    }

    fn create_option_some(
        &mut self,
        inner_type: BasicTypeEnum<'_>,
        value: BasicValueEnum<'_>,
    ) -> Result<BasicValueEnum<'_>, CursedError> {
        let option_type = self.generate_option_type(inner_type)?;
        let builder = &self.builder.lock().unwrap();
        
        // Create discriminant value (1 for Some)
        let discriminant = self.context.i8_type().const_int(OptionDiscriminant::Some as u64, false);
        
        // Start with undefined struct value
        let mut option_value = option_type.get_undef();
        
        // Insert discriminant at index 0
        option_value = builder.build_insert_value(
            option_value,
            discriminant,
            0,
            "option_some_discriminant"
        ).map_err(|e| CursedError::llvm_error(format!("Failed to insert discriminant: {}", e)))?
        .into_struct_value();

        // Convert and insert the value data
        let converted_data = self.convert_value_to_data_field(value, inner_type, &option_type)?;
        option_value = builder.build_insert_value(
            option_value,
            converted_data,
            1,
            "option_some_data"
        ).map_err(|e| CursedError::llvm_error(format!("Failed to insert some data: {}", e)))?
        .into_struct_value();

        Ok(option_value.into())
    }

    fn create_option_none(
        &mut self,
        inner_type: BasicTypeEnum<'_>,
    ) -> Result<BasicValueEnum<'_>, CursedError> {
        let option_type = self.generate_option_type(inner_type)?;
        let builder = &self.builder.lock().unwrap();
        
        // Create discriminant value (0 for None)
        let discriminant = self.context.i8_type().const_int(OptionDiscriminant::None as u64, false);
        
        // Start with undefined struct value
        let mut option_value = option_type.get_undef();
        
        // Insert discriminant at index 0 (data field will remain undefined for None)
        option_value = builder.build_insert_value(
            option_value,
            discriminant,
            0,
            "option_none_discriminant"
        ).map_err(|e| CursedError::llvm_error(format!("Failed to insert discriminant: {}", e)))?
        .into_struct_value();

        Ok(option_value.into())
    }

    fn is_result_ok(
        &mut self,
        result_value: BasicValueEnum<'_>,
    ) -> Result<IntValue<'_>, CursedError> {
        let builder = &self.builder.lock().unwrap();
        
        // Extract discriminant from index 0
        let discriminant = builder.build_extract_value(
            result_value.into_struct_value(),
            0,
            "result_discriminant"
        ).map_err(|e| CursedError::llvm_error(format!("Failed to extract discriminant: {}", e)))?
        .into_int_value();

        // Compare with Ok discriminant (0)
        let ok_discriminant = self.context.i8_type().const_int(ResultDiscriminant::Ok as u64, false);
        let is_ok = builder.build_int_compare(
            IntPredicate::EQ,
            discriminant,
            ok_discriminant,
            "is_result_ok"
        ).map_err(|e| CursedError::llvm_error(format!("Failed to compare discriminant: {}", e)))?;

        Ok(is_ok)
    }

    fn is_result_err(
        &mut self,
        result_value: BasicValueEnum<'_>,
    ) -> Result<IntValue<'_>, CursedError> {
        let builder = &self.builder.lock().unwrap();
        
        // Extract discriminant from index 0
        let discriminant = builder.build_extract_value(
            result_value.into_struct_value(),
            0,
            "result_discriminant"
        ).map_err(|e| CursedError::llvm_error(format!("Failed to extract discriminant: {}", e)))?
        .into_int_value();

        // Compare with Err discriminant (1)
        let err_discriminant = self.context.i8_type().const_int(ResultDiscriminant::Err as u64, false);
        let is_err = builder.build_int_compare(
            IntPredicate::EQ,
            discriminant,
            err_discriminant,
            "is_result_err"
        ).map_err(|e| CursedError::llvm_error(format!("Failed to compare discriminant: {}", e)))?;

        Ok(is_err)
    }

    fn is_option_some(
        &mut self,
        option_value: BasicValueEnum<'_>,
    ) -> Result<IntValue<'_>, CursedError> {
        let builder = &self.builder.lock().unwrap();
        
        // Extract discriminant from index 0
        let discriminant = builder.build_extract_value(
            option_value.into_struct_value(),
            0,
            "option_discriminant"
        ).map_err(|e| CursedError::llvm_error(format!("Failed to extract discriminant: {}", e)))?
        .into_int_value();

        // Compare with Some discriminant (1)
        let some_discriminant = self.context.i8_type().const_int(OptionDiscriminant::Some as u64, false);
        let is_some = builder.build_int_compare(
            IntPredicate::EQ,
            discriminant,
            some_discriminant,
            "is_option_some"
        ).map_err(|e| CursedError::llvm_error(format!("Failed to compare discriminant: {}", e)))?;

        Ok(is_some)
    }

    fn is_option_none(
        &mut self,
        option_value: BasicValueEnum<'_>,
    ) -> Result<IntValue<'_>, CursedError> {
        let builder = &self.builder.lock().unwrap();
        
        // Extract discriminant from index 0
        let discriminant = builder.build_extract_value(
            option_value.into_struct_value(),
            0,
            "option_discriminant"
        ).map_err(|e| CursedError::llvm_error(format!("Failed to extract discriminant: {}", e)))?
        .into_int_value();

        // Compare with None discriminant (0)
        let none_discriminant = self.context.i8_type().const_int(OptionDiscriminant::None as u64, false);
        let is_none = builder.build_int_compare(
            IntPredicate::EQ,
            discriminant,
            none_discriminant,
            "is_option_none"
        ).map_err(|e| CursedError::llvm_error(format!("Failed to compare discriminant: {}", e)))?;

        Ok(is_none)
    }

    fn extract_result_ok(
        &mut self,
        result_value: BasicValueEnum<'_>,
        ok_type: BasicTypeEnum<'_>,
    ) -> Result<BasicValueEnum<'_>, CursedError> {
        let builder = &self.builder.lock().unwrap();
        
        // Extract data field from index 1
        let data_field = builder.build_extract_value(
            result_value.into_struct_value(),
            1,
            "result_ok_data"
        ).map_err(|e| CursedError::llvm_error(format!("Failed to extract ok data: {}", e)))?;

        // Convert from data field back to the original type
        self.convert_data_field_to_value(data_field, ok_type)
    }

    fn extract_result_err(
        &mut self,
        result_value: BasicValueEnum<'_>,
        err_type: BasicTypeEnum<'_>,
    ) -> Result<BasicValueEnum<'_>, CursedError> {
        let builder = &self.builder.lock().unwrap();
        
        // Extract data field from index 1
        let data_field = builder.build_extract_value(
            result_value.into_struct_value(),
            1,
            "result_err_data"
        ).map_err(|e| CursedError::llvm_error(format!("Failed to extract err data: {}", e)))?;

        // Convert from data field back to the original type
        self.convert_data_field_to_value(data_field, err_type)
    }

    fn extract_option_some(
        &mut self,
        option_value: BasicValueEnum<'_>,
        inner_type: BasicTypeEnum<'_>,
    ) -> Result<BasicValueEnum<'_>, CursedError> {
        let builder = &self.builder.lock().unwrap();
        
        // Extract data field from index 1
        let data_field = builder.build_extract_value(
            option_value.into_struct_value(),
            1,
            "option_some_data"
        ).map_err(|e| CursedError::llvm_error(format!("Failed to extract some data: {}", e)))?;

        // Convert from data field back to the original type
        self.convert_data_field_to_value(data_field, inner_type)
    }

    fn get_result_type_string(&self, ok_type: &str, err_type: &str) -> String {
        format!("Result<{}, {}>", ok_type, err_type)
    }
    
    fn get_option_type_string(&self, inner_type: &str) -> String {
        format!("Option<{}>", inner_type)
    }
}

impl LlvmCodeGenerator {
    /// Get the approximate size of a BasicTypeEnum in bytes
    fn get_type_size(&self, type_enum: BasicTypeEnum<'_>) -> Result<usize, CursedError> {
        match type_enum {
            BasicTypeEnum::IntType(int_type) => {
                Ok((int_type.get_bit_width() / 8) as usize)
            }
            BasicTypeEnum::FloatType(float_type) => {
                match float_type.get_context().f64_type() == float_type {
                    true => Ok(8), // f64
                    false => Ok(4), // f32
                }
            }
            BasicTypeEnum::PointerType(_) => {
                Ok(8) // Assuming 64-bit pointers
            }
            BasicTypeEnum::ArrayType(array_type) => {
                let element_size = self.get_type_size(array_type.get_element_type())?;
                Ok(element_size * array_type.len() as usize)
            }
            BasicTypeEnum::VectorType(vector_type) => {
                let element_size = self.get_type_size(vector_type.get_element_type())?;
                Ok(element_size * vector_type.get_size() as usize)
            }
            BasicTypeEnum::StructType(struct_type) => {
                // Approximate struct size by summing field sizes
                let mut total_size = 0;
                for field_type in struct_type.get_field_types() {
                    total_size += self.get_type_size(field_type)?;
                }
                Ok(total_size)
            }
        }
    }

    /// Convert a value to fit in the data field of a Result/Option struct
    fn convert_value_to_data_field(
        &self,
        value: BasicValueEnum<'_>,
        original_type: BasicTypeEnum<'_>,
        struct_type: &StructType<'_>,
    ) -> Result<BasicValueEnum<'_>, CursedError> {
        let builder = &self.builder.lock().unwrap();
        
        // Get the type of the data field (index 1)
        let data_field_type = struct_type.get_field_type_at_index(1)
            .ok_or_else(|| CursedError::llvm_error("Result/Option struct missing data field".to_string()))?;

        // If the value already matches the target type, use it directly
        if value.get_type() == data_field_type {
            return Ok(value);
        }

        // Handle type conversions based on the data field type
        match data_field_type {
            BasicTypeEnum::IntType(target_int_type) => {
                match value {
                    BasicValueEnum::IntValue(int_val) => {
                        // Extend or truncate integer as needed
                        if int_val.get_type().get_bit_width() < target_int_type.get_bit_width() {
                            let extended = builder.build_int_z_extend(
                                int_val,
                                target_int_type,
                                "extend_to_data_field"
                            ).map_err(|e| CursedError::llvm_error(format!("Failed to extend int: {}", e)))?;
                            Ok(extended.into())
                        } else if int_val.get_type().get_bit_width() > target_int_type.get_bit_width() {
                            let truncated = builder.build_int_truncate(
                                int_val,
                                target_int_type,
                                "truncate_to_data_field"
                            ).map_err(|e| CursedError::llvm_error(format!("Failed to truncate int: {}", e)))?;
                            Ok(truncated.into())
                        } else {
                            Ok(value)
                        }
                    }
                    BasicValueEnum::FloatValue(float_val) => {
                        // Bitcast float to integer of same size
                        let bitcast = builder.build_bitcast(
                            float_val,
                            target_int_type,
                            "float_to_int_data_field"
                        ).map_err(|e| CursedError::llvm_error(format!("Failed to bitcast float: {}", e)))?;
                        Ok(bitcast)
                    }
                    BasicValueEnum::PointerValue(ptr_val) => {
                        // Cast pointer to integer
                        let ptr_to_int = builder.build_ptr_to_int(
                            ptr_val,
                            target_int_type,
                            "ptr_to_int_data_field"
                        ).map_err(|e| CursedError::llvm_error(format!("Failed to cast ptr to int: {}", e)))?;
                        Ok(ptr_to_int.into())
                    }
                    _ => Err(CursedError::llvm_error(format!(
                        "Cannot convert {:?} to integer data field", value.get_type()
                    )))
                }
            }
            BasicTypeEnum::ArrayType(_) => {
                // For array data fields, we need to copy the value into the array
                // This is a complex operation that would require memory operations
                // For now, return an error for unsupported complex conversions
                Err(CursedError::llvm_error(
                    "Array data field conversion not yet implemented".to_string()
                ))
            }
            _ => {
                // For other types, try bitcast
                let bitcast = builder.build_bitcast(
                    value,
                    data_field_type,
                    "convert_to_data_field"
                ).map_err(|e| CursedError::llvm_error(format!("Failed to convert to data field: {}", e)))?;
                Ok(bitcast)
            }
        }
    }

    /// Convert from data field back to the original value type
    fn convert_data_field_to_value(
        &self,
        data_field: BasicValueEnum<'_>,
        target_type: BasicTypeEnum<'_>,
    ) -> Result<BasicValueEnum<'_>, CursedError> {
        let builder = &self.builder.lock().unwrap();

        // If the data field already matches the target type, use it directly
        if data_field.get_type() == target_type {
            return Ok(data_field);
        }

        // Handle type conversions based on the target type
        match target_type {
            BasicTypeEnum::IntType(target_int_type) => {
                match data_field {
                    BasicValueEnum::IntValue(int_val) => {
                        // Extend or truncate as needed
                        if int_val.get_type().get_bit_width() < target_int_type.get_bit_width() {
                            let extended = builder.build_int_z_extend(
                                int_val,
                                target_int_type,
                                "extend_from_data_field"
                            ).map_err(|e| CursedError::llvm_error(format!("Failed to extend int: {}", e)))?;
                            Ok(extended.into())
                        } else if int_val.get_type().get_bit_width() > target_int_type.get_bit_width() {
                            let truncated = builder.build_int_truncate(
                                int_val,
                                target_int_type,
                                "truncate_from_data_field"
                            ).map_err(|e| CursedError::llvm_error(format!("Failed to truncate int: {}", e)))?;
                            Ok(truncated.into())
                        } else {
                            Ok(data_field)
                        }
                    }
                    _ => {
                        let bitcast = builder.build_bitcast(
                            data_field,
                            target_type,
                            "convert_from_data_field"
                        ).map_err(|e| CursedError::llvm_error(format!("Failed to convert from data field: {}", e)))?;
                        Ok(bitcast)
                    }
                }
            }
            BasicTypeEnum::FloatType(target_float_type) => {
                // Bitcast from integer to float
                let bitcast = builder.build_bitcast(
                    data_field,
                    target_float_type,
                    "int_to_float_from_data_field"
                ).map_err(|e| CursedError::llvm_error(format!("Failed to bitcast to float: {}", e)))?;
                Ok(bitcast)
            }
            BasicTypeEnum::PointerType(target_ptr_type) => {
                match data_field {
                    BasicValueEnum::IntValue(int_val) => {
                        // Cast integer to pointer
                        let int_to_ptr = builder.build_int_to_ptr(
                            int_val,
                            target_ptr_type,
                            "int_to_ptr_from_data_field"
                        ).map_err(|e| CursedError::llvm_error(format!("Failed to cast int to ptr: {}", e)))?;
                        Ok(int_to_ptr.into())
                    }
                    _ => {
                        let bitcast = builder.build_bitcast(
                            data_field,
                            target_type,
                            "convert_from_data_field"
                        ).map_err(|e| CursedError::llvm_error(format!("Failed to convert from data field: {}", e)))?;
                        Ok(bitcast)
                    }
                }
            }
            _ => {
                // For other types, try bitcast
                let bitcast = builder.build_bitcast(
                    data_field,
                    target_type,
                    "convert_from_data_field"
                ).map_err(|e| CursedError::llvm_error(format!("Failed to convert from data field: {}", e)))?;
                Ok(bitcast)
            }
        }
    }
}

/// Result and Option type utilities for high-level operations
pub mod result_type_utils {
    use super::*;

    /// Helper to create a Result<i32, String> type for common use cases
    pub fn create_common_result_type<'ctx>(
        context: &'ctx inkwell::context::Context
    ) -> StructType<'ctx> {
        let discriminant_type = context.i8_type();
        let data_type = context.i64_type(); // Large enough for both i32 and pointer
        let field_types = [discriminant_type.into(), data_type.into()];
        context.struct_type(&field_types, false)
    }

    /// Helper to create an Option<i32> type for common use cases
    pub fn create_common_option_type<'ctx>(
        context: &'ctx inkwell::context::Context
    ) -> StructType<'ctx> {
        let discriminant_type = context.i8_type();
        let data_type = context.i32_type();
        let field_types = [discriminant_type.into(), data_type.into()];
        context.struct_type(&field_types, false)
    }

    /// Get a human-readable string representation of a Result type
    pub fn get_result_type_string(ok_type: &str, err_type: &str) -> String {
        format!("Result<{}, {}>", ok_type, err_type)
    }

    /// Get a human-readable string representation of an Option type
    pub fn get_option_type_string(inner_type: &str) -> String {
        format!("Option<{}>", inner_type)
    }

    /// Check if a string represents a Result type
    pub fn is_result_type(type_str: &str) -> bool {
        type_str.starts_with("Result<") && type_str.ends_with('>')
    }

    /// Check if a string represents an Option type
    pub fn is_option_type(type_str: &str) -> bool {
        type_str.starts_with("Option<") && type_str.ends_with('>')
    }

    /// Parse Result type string to extract Ok and Err type names
    pub fn parse_result_type(type_str: &str) -> Option<(String, String)> {
        if !is_result_type(type_str) {
            return None;
        }
        
        let inner = &type_str[7..type_str.len()-1]; // Remove "Result<" and ">"
        let parts: Vec<&str> = inner.splitn(2, ", ").collect();
        if parts.len() == 2 {
            Some((parts[0].to_string(), parts[1].to_string()))
        } else {
            None
        }
    }

    /// Parse Option type string to extract inner type name
    pub fn parse_option_type(type_str: &str) -> Option<String> {
        if !is_option_type(type_str) {
            return None;
        }
        
        let inner = &type_str[7..type_str.len()-1]; // Remove "Option<" and ">"
        Some(inner.to_string())
    }

    /// Calculate memory layout for a Result type
    pub fn calculate_result_layout(ok_size: usize, err_size: usize) -> TypeLayout {
        let max_size = ok_size.max(err_size);
        let discriminant_size = 1; // i8
        let data_size = if max_size <= 8 {
            // Use power-of-2 sizing for small types
            if max_size <= 1 { 1 }
            else if max_size <= 2 { 2 }
            else if max_size <= 4 { 4 }
            else { 8 }
        } else {
            max_size
        };
        
        let total_size = discriminant_size + data_size;
        let alignment = if data_size <= 8 { data_size } else { 8 };
        
        TypeLayout {
            size: total_size,
            alignment,
            discriminant_size,
            data_size,
            needs_padding: total_size % alignment != 0,
        }
    }

    /// Calculate memory layout for an Option type
    pub fn calculate_option_layout(inner_size: usize) -> TypeLayout {
        let discriminant_size = 1; // i8
        let data_size = if inner_size <= 8 {
            // Use power-of-2 sizing for small types
            if inner_size <= 1 { 1 }
            else if inner_size <= 2 { 2 }
            else if inner_size <= 4 { 4 }
            else { 8 }
        } else {
            inner_size
        };
        
        let total_size = discriminant_size + data_size;
        let alignment = if data_size <= 8 { data_size } else { 8 };
        
        TypeLayout {
            size: total_size,
            alignment,
            discriminant_size,
            data_size,
            needs_padding: total_size % alignment != 0,
        }
    }
}
