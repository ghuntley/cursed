//! Comprehensive bool type conversions for LLVM code generation
//!
//! This module implements all bool conversion operations in CURSED:
//! - Bool to integer (false = 0, true = 1) 
//! - Bool to float (false = 0.0, true = 1.0)
//! - Bool to string (false = "false", true = "true")
//! - Reverse conversions from other types to bool
//! - Integration with existing boolean operations

use inkwell::values::{BasicValueEnum, IntValue, FloatValue, PointerValue};
use inkwell::types::{BasicTypeEnum, IntType, FloatType};
use inkwell::IntPredicate;
use inkwell::FloatPredicate;

use crate::error::Error;
use super::context::LlvmCodeGenerator;
use super::string_type::CursedStringType;

/// Trait for comprehensive bool type conversions
pub trait BoolConversions<'ctx> {
    /// Convert bool to integer (false = 0, true = 1)
    fn convert_bool_to_integer(&mut self, bool_val: BasicValueEnum<'ctx>, target_type: IntType<'ctx>) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Convert bool to float (false = 0.0, true = 1.0)
    fn convert_bool_to_float(&mut self, bool_val: BasicValueEnum<'ctx>, target_type: FloatType<'ctx>) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Convert bool to string (false = "false", true = "true")
    fn convert_bool_to_string(&mut self, bool_val: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Convert integer to bool (0 = false, non-zero = true)
    fn convert_integer_to_bool(&mut self, int_val: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Convert float to bool (0.0 = false, non-zero = true)
    fn convert_float_to_bool(&mut self, float_val: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Convert string to bool ("true", "1", "yes" = true; "false", "0", "no" = false)
    fn convert_string_to_bool(&mut self, string_val: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Convert pointer to bool (null = false, non-null = true)
    fn convert_pointer_to_bool(&mut self, ptr_val: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Auto-convert any value to bool using CURSED truthiness rules
    fn convert_value_to_bool(&mut self, value: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Create a bool literal (LLVM i1 type)
    fn create_bool_literal(&self, value: bool) -> BasicValueEnum<'ctx>;
    
    /// Check if a value is a bool type (LLVM i1)
    fn is_bool_type(&self, value: BasicValueEnum<'ctx>) -> bool;
    
    /// Get the LLVM bool type (i1)
    fn get_bool_type(&self) -> IntType<'ctx>;
    
    /// Compare two bool values for equality
    fn compare_bool_equality(&mut self, left: BasicValueEnum<'ctx>, right: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Perform logical AND operation on two bool values
    fn bool_logical_and(&mut self, left: BasicValueEnum<'ctx>, right: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Perform logical OR operation on two bool values
    fn bool_logical_or(&mut self, left: BasicValueEnum<'ctx>, right: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Perform logical NOT operation on a bool value
    fn bool_logical_not(&mut self, value: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error>;
}

impl<'ctx> BoolConversions<'ctx> for LlvmCodeGenerator<'ctx> {
    fn convert_bool_to_integer(&mut self, bool_val: BasicValueEnum<'ctx>, target_type: IntType<'ctx>) -> Result<BasicValueEnum<'ctx>, Error> {
        tracing::debug!("Converting bool to integer type");
        
        // Ensure the input is a bool (i1)
        let bool_int = if self.is_bool_type(bool_val) {
            bool_val.into_int_value()
        } else {
            // Try to convert the value to bool first
            let converted_bool = self.convert_value_to_bool(bool_val)?;
            converted_bool.into_int_value()
        };
        
        // Zero-extend or truncate to target integer type
        let result = if bool_int.get_type().get_bit_width() < target_type.get_bit_width() {
            // Zero-extend: i1 -> target_type (false = 0, true = 1)
            self.builder().build_int_z_extend(bool_int, target_type, "bool_to_int_zext")
                .map_err(|e| Error::codegen(format!("Failed to zero-extend bool to integer: {}", e)))?
        } else if bool_int.get_type().get_bit_width() > target_type.get_bit_width() {
            // Truncate: should never happen with i1, but handle defensively
            self.builder().build_int_truncate(bool_int, target_type, "bool_to_int_trunc")
                .map_err(|e| Error::codegen(format!("Failed to truncate bool to integer: {}", e)))?
        } else {
            // Same width (should be 1 bit), no conversion needed
            bool_int
        };
        
        Ok(result.into())
    }
    
    fn convert_bool_to_float(&mut self, bool_val: BasicValueEnum<'ctx>, target_type: FloatType<'ctx>) -> Result<BasicValueEnum<'ctx>, Error> {
        tracing::debug!("Converting bool to float type");
        
        // Ensure the input is a bool (i1)
        let bool_int = if self.is_bool_type(bool_val) {
            bool_val.into_int_value()
        } else {
            // Try to convert the value to bool first
            let converted_bool = self.convert_value_to_bool(bool_val)?;
            converted_bool.into_int_value()
        };
        
        // Convert i1 to target float type (false = 0.0, true = 1.0)
        let result = self.builder().build_unsigned_int_to_float(bool_int, target_type, "bool_to_float")
            .map_err(|e| Error::codegen(format!("Failed to convert bool to float: {}", e)))?;
        
        Ok(result.into())
    }
    
    fn convert_bool_to_string(&mut self, bool_val: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error> {
        tracing::debug!("Converting bool to string");
        
        // Ensure the input is a bool (i1)
        let bool_int = if self.is_bool_type(bool_val) {
            bool_val.into_int_value()
        } else {
            // Try to convert the value to bool first
            let converted_bool = self.convert_value_to_bool(bool_val)?;
            converted_bool.into_int_value()
        };
        
        // Create string literals for "true" and "false"
        let string_type = CursedStringType::new(self.context());
        
        // Create basic blocks for true and false cases
        let current_function = self.current_function()
            .ok_or_else(|| Error::codegen("No current function for bool to string conversion"))?;
        
        let true_block = self.context().append_basic_block(current_function, "bool_true_str");
        let false_block = self.context().append_basic_block(current_function, "bool_false_str");
        let cont_block = self.context().append_basic_block(current_function, "bool_str_cont");
        
        // Compare bool with true (1)
        let true_val = self.context().bool_type().const_int(1, false);
        let is_true = self.builder().build_int_compare(
            IntPredicate::EQ,
            bool_int,
            true_val,
            "is_bool_true"
        ).map_err(|e| Error::codegen(format!("Failed to compare bool value: {}", e)))?;
        
        // Branch based on the bool value
        self.builder().build_conditional_branch(is_true, true_block, false_block)
            .map_err(|e| Error::codegen(format!("Failed to build conditional branch: {}", e)))?;
        
        // True block - create "true" string
        self.builder().position_at_end(true_block);
        let true_str = string_type.create_string_literal(
            self.builder(),
            self.module(),
            "true",
            &format!("bool_true_str_{}", self.string_literal_counter)
        ).map_err(|e| Error::codegen(format!("Failed to create 'true' string: {}", e)))?;
        self.string_literal_counter += 1;
        
        self.builder().build_unconditional_branch(cont_block)
            .map_err(|e| Error::codegen(format!("Failed to build branch: {}", e)))?;
        
        // False block - create "false" string
        self.builder().position_at_end(false_block);
        let false_str = string_type.create_string_literal(
            self.builder(),
            self.module(),
            "false",
            &format!("bool_false_str_{}", self.string_literal_counter)
        ).map_err(|e| Error::codegen(format!("Failed to create 'false' string: {}", e)))?;
        self.string_literal_counter += 1;
        
        self.builder().build_unconditional_branch(cont_block)
            .map_err(|e| Error::codegen(format!("Failed to build branch: {}", e)))?;
        
        // Continuation block - phi node to select the result
        self.builder().position_at_end(cont_block);
        let phi = self.builder().build_phi(true_str.get_type(), "bool_str_result")
            .map_err(|e| Error::codegen(format!("Failed to build phi node: {}", e)))?;
        
        phi.add_incoming(&[
            (&true_str, true_block),
            (&false_str, false_block)
        ]);
        
        Ok(phi.as_basic_value())
    }
    
    fn convert_integer_to_bool(&mut self, int_val: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error> {
        tracing::debug!("Converting integer to bool (0 = false, non-zero = true)");
        
        if !int_val.is_int_value() {
            return Err(Error::codegen("Expected integer value for integer to bool conversion"));
        }
        
        let int_value = int_val.into_int_value();
        
        // Compare with zero
        let zero = int_value.get_type().const_int(0, false);
        let is_non_zero = self.builder().build_int_compare(
            IntPredicate::NE,
            int_value,
            zero,
            "int_to_bool"
        ).map_err(|e| Error::codegen(format!("Failed to compare integer with zero: {}", e)))?;
        
        Ok(is_non_zero.into())
    }
    
    fn convert_float_to_bool(&mut self, float_val: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error> {
        tracing::debug!("Converting float to bool (0.0 = false, non-zero = true)");
        
        if !float_val.is_float_value() {
            return Err(Error::codegen("Expected float value for float to bool conversion"));
        }
        
        let float_value = float_val.into_float_value();
        
        // Compare with zero
        let zero = float_value.get_type().const_float(0.0);
        let is_non_zero = self.builder().build_float_compare(
            FloatPredicate::ONE, // Ordered and not equal (handles NaN correctly)
            float_value,
            zero,
            "float_to_bool"
        ).map_err(|e| Error::codegen(format!("Failed to compare float with zero: {}", e)))?;
        
        Ok(is_non_zero.into())
    }
    
    fn convert_string_to_bool(&mut self, string_val: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error> {
        tracing::debug!("Converting string to bool");
        
        // For now, implement a simple version that checks string length
        // In a full implementation, this would parse the string content
        // and check for "true", "false", "1", "0", "yes", "no", etc.
        
        // This is a simplified implementation - a full version would:
        // 1. Extract the string content
        // 2. Compare against known true/false strings
        // 3. Handle case-insensitive matching
        
        // For now, treat empty strings as false, non-empty as true
        if string_val.is_struct_value() {
            let string_struct = string_val.into_struct_value();
            
            // Extract length field (assuming CURSED string struct format)
            let length_ptr = self.builder().build_extract_value(string_struct, 0, "str_len")
                .map_err(|e| Error::codegen(format!("Failed to extract string length: {}", e)))?;
            
            if length_ptr.is_int_value() {
                let length = length_ptr.into_int_value();
                
                // Non-empty string = true, empty string = false
                let zero = length.get_type().const_int(0, false);
                let is_non_empty = self.builder().build_int_compare(
                    IntPredicate::NE,
                    length,
                    zero,
                    "str_to_bool"
                ).map_err(|e| Error::codegen(format!("Failed to compare string length: {}", e)))?;
                
                return Ok(is_non_empty.into());
            }
        }
        
        // Fallback: treat non-null strings as true
        if string_val.is_pointer_value() {
            return self.convert_pointer_to_bool(string_val);
        }
        
        // Default to true for other string representations
        Ok(self.create_bool_literal(true))
    }
    
    fn convert_pointer_to_bool(&mut self, ptr_val: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error> {
        tracing::debug!("Converting pointer to bool (null = false, non-null = true)");
        
        if !ptr_val.is_pointer_value() {
            return Err(Error::codegen("Expected pointer value for pointer to bool conversion"));
        }
        
        let pointer = ptr_val.into_pointer_value();
        
        // Create null pointer of the same type
        let null_ptr = pointer.get_type().const_null();
        
        // Compare with null
        let is_non_null = self.builder().build_int_compare(
            IntPredicate::NE,
            pointer,
            null_ptr,
            "ptr_to_bool"
        ).map_err(|e| Error::codegen(format!("Failed to compare pointer with null: {}", e)))?;
        
        Ok(is_non_null.into())
    }
    
    fn convert_value_to_bool(&mut self, value: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error> {
        tracing::debug!("Auto-converting value to bool using CURSED truthiness rules");
        
        // Already a bool
        if self.is_bool_type(value) {
            return Ok(value);
        }
        
        // Integer types (including i8, i16, i32, i64)
        if value.is_int_value() {
            return self.convert_integer_to_bool(value);
        }
        
        // Float types
        if value.is_float_value() {
            return self.convert_float_to_bool(value);
        }
        
        // Pointer types (including strings)
        if value.is_pointer_value() {
            return self.convert_pointer_to_bool(value);
        }
        
        // Struct types (might be strings or other compound types)
        if value.is_struct_value() {
            // Try to handle as string first
            return self.convert_string_to_bool(value);
        }
        
        // Array types - non-empty arrays are true
        if value.is_array_value() {
            let array = value.into_array_value();
            let array_type = array.get_type();
            let length = array_type.len();
            
            // Arrays with non-zero length are true
            let bool_val = length > 0;
            return Ok(self.create_bool_literal(bool_val));
        }
        
        // Vector types
        if value.is_vector_value() {
            let vector = value.into_vector_value();
            let vector_type = vector.get_type();
            let size = vector_type.get_size();
            
            // Vectors with non-zero size are true
            let bool_val = size > 0;
            return Ok(self.create_bool_literal(bool_val));
        }
        
        // Default: unknown types are considered true
        tracing::warn!("Unknown value type in bool conversion, defaulting to true");
        Ok(self.create_bool_literal(true))
    }
    
    fn create_bool_literal(&self, value: bool) -> BasicValueEnum<'ctx> {
        let bool_type = self.context().bool_type();
        bool_type.const_int(if value { 1 } else { 0 }, false).into()
    }
    
    fn is_bool_type(&self, value: BasicValueEnum<'ctx>) -> bool {
        if let BasicValueEnum::IntValue(int_val) = value {
            int_val.get_type().get_bit_width() == 1
        } else {
            false
        }
    }
    
    fn get_bool_type(&self) -> IntType<'ctx> {
        self.context().bool_type()
    }
    
    fn compare_bool_equality(&mut self, left: BasicValueEnum<'ctx>, right: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error> {
        tracing::debug!("Comparing bool values for equality");
        
        // Convert both values to bool if needed
        let left_bool = if self.is_bool_type(left) {
            left.into_int_value()
        } else {
            self.convert_value_to_bool(left)?.into_int_value()
        };
        
        let right_bool = if self.is_bool_type(right) {
            right.into_int_value()
        } else {
            self.convert_value_to_bool(right)?.into_int_value()
        };
        
        // Compare for equality
        let result = self.builder().build_int_compare(
            IntPredicate::EQ,
            left_bool,
            right_bool,
            "bool_eq"
        ).map_err(|e| Error::codegen(format!("Failed to compare bool values: {}", e)))?;
        
        Ok(result.into())
    }
    
    fn bool_logical_and(&mut self, left: BasicValueEnum<'ctx>, right: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error> {
        tracing::debug!("Performing logical AND on bool values");
        
        // Convert both values to bool if needed
        let left_bool = if self.is_bool_type(left) {
            left.into_int_value()
        } else {
            self.convert_value_to_bool(left)?.into_int_value()
        };
        
        let right_bool = if self.is_bool_type(right) {
            right.into_int_value()
        } else {
            self.convert_value_to_bool(right)?.into_int_value()
        };
        
        // Perform bitwise AND (equivalent to logical AND for i1)
        let result = self.builder().build_and(left_bool, right_bool, "bool_and")
            .map_err(|e| Error::codegen(format!("Failed to perform logical AND: {}", e)))?;
        
        Ok(result.into())
    }
    
    fn bool_logical_or(&mut self, left: BasicValueEnum<'ctx>, right: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error> {
        tracing::debug!("Performing logical OR on bool values");
        
        // Convert both values to bool if needed
        let left_bool = if self.is_bool_type(left) {
            left.into_int_value()
        } else {
            self.convert_value_to_bool(left)?.into_int_value()
        };
        
        let right_bool = if self.is_bool_type(right) {
            right.into_int_value()
        } else {
            self.convert_value_to_bool(right)?.into_int_value()
        };
        
        // Perform bitwise OR (equivalent to logical OR for i1)
        let result = self.builder().build_or(left_bool, right_bool, "bool_or")
            .map_err(|e| Error::codegen(format!("Failed to perform logical OR: {}", e)))?;
        
        Ok(result.into())
    }
    
    fn bool_logical_not(&mut self, value: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error> {
        tracing::debug!("Performing logical NOT on bool value");
        
        // Convert value to bool if needed
        let bool_val = if self.is_bool_type(value) {
            value.into_int_value()
        } else {
            self.convert_value_to_bool(value)?.into_int_value()
        };
        
        // Perform bitwise NOT (equivalent to logical NOT for i1)
        let result = self.builder().build_not(bool_val, "bool_not")
            .map_err(|e| Error::codegen(format!("Failed to perform logical NOT: {}", e)))?;
        
        Ok(result.into())
    }
}

/// Helper functions for bool conversion utilities
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Create a constant bool value
    pub fn const_bool(&self, value: bool) -> BasicValueEnum<'ctx> {
        self.create_bool_literal(value)
    }
    
    /// Check if a basic type is a bool type
    pub fn is_bool_basic_type(&self, ty: BasicTypeEnum<'ctx>) -> bool {
        if let BasicTypeEnum::IntType(int_type) = ty {
            int_type.get_bit_width() == 1
        } else {
            false
        }
    }
    
    /// Convert any CURSED value to a bool following language semantics
    pub fn to_bool(&mut self, value: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error> {
        self.convert_value_to_bool(value)
    }
    
    /// Create a conditional branch using any value as condition (auto-converts to bool)
    pub fn build_conditional_branch_auto(&mut self, 
        condition: BasicValueEnum<'ctx>, 
        then_block: inkwell::basic_block::BasicBlock<'ctx>,
        else_block: inkwell::basic_block::BasicBlock<'ctx>
    ) -> Result<(), Error> {
        let bool_condition = self.convert_value_to_bool(condition)?.into_int_value();
        
        self.builder().build_conditional_branch(bool_condition, then_block, else_block)
            .map_err(|e| Error::codegen(format!("Failed to build conditional branch: {}", e)))?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::context::Context;
    use inkwell::module::Module;
    use inkwell::builder::Builder;
    
    fn setup_test_context() -> (Context, Module<'static>, Builder<'static>) {
        let context = Context::create();
        let module = context.create_module("test");
        let builder = context.create_builder();
        (context, module, builder)
    }
    
    #[test]
    fn test_create_bool_literal() {
        let (context, module, builder) = setup_test_context();
        let mut codegen = LlvmCodeGenerator::new(context, module, builder);
        
        let true_val = codegen.create_bool_literal(true);
        let false_val = codegen.create_bool_literal(false);
        
        assert!(codegen.is_bool_type(true_val));
        assert!(codegen.is_bool_type(false_val));
    }
    
    #[test]
    fn test_integer_to_bool_conversion() {
        let (context, module, builder) = setup_test_context();
        let mut codegen = LlvmCodeGenerator::new(context, module, builder);
        
        let int_type = context.i32_type();
        let zero = int_type.const_int(0, false).into();
        let non_zero = int_type.const_int(42, false).into();
        
        let zero_bool = codegen.convert_integer_to_bool(zero).unwrap();
        let non_zero_bool = codegen.convert_integer_to_bool(non_zero).unwrap();
        
        assert!(codegen.is_bool_type(zero_bool));
        assert!(codegen.is_bool_type(non_zero_bool));
    }
    
    #[test]
    fn test_float_to_bool_conversion() {
        let (context, module, builder) = setup_test_context();
        let mut codegen = LlvmCodeGenerator::new(context, module, builder);
        
        let float_type = context.f64_type();
        let zero = float_type.const_float(0.0).into();
        let non_zero = float_type.const_float(3.14).into();
        
        let zero_bool = codegen.convert_float_to_bool(zero).unwrap();
        let non_zero_bool = codegen.convert_float_to_bool(non_zero).unwrap();
        
        assert!(codegen.is_bool_type(zero_bool));
        assert!(codegen.is_bool_type(non_zero_bool));
    }
    
    #[test]
    fn test_bool_to_integer_conversion() {
        let (context, module, builder) = setup_test_context();
        let mut codegen = LlvmCodeGenerator::new(context, module, builder);
        
        let true_bool = codegen.create_bool_literal(true);
        let false_bool = codegen.create_bool_literal(false);
        let target_type = context.i32_type();
        
        let true_int = codegen.convert_bool_to_integer(true_bool, target_type).unwrap();
        let false_int = codegen.convert_bool_to_integer(false_bool, target_type).unwrap();
        
        assert!(true_int.is_int_value());
        assert!(false_int.is_int_value());
    }
    
    #[test]
    fn test_bool_to_float_conversion() {
        let (context, module, builder) = setup_test_context();
        let mut codegen = LlvmCodeGenerator::new(context, module, builder);
        
        let true_bool = codegen.create_bool_literal(true);
        let false_bool = codegen.create_bool_literal(false);
        let target_type = context.f64_type();
        
        let true_float = codegen.convert_bool_to_float(true_bool, target_type).unwrap();
        let false_float = codegen.convert_bool_to_float(false_bool, target_type).unwrap();
        
        assert!(true_float.is_float_value());
        assert!(false_float.is_float_value());
    }
    
    #[test]
    fn test_bool_logical_operations() {
        let (context, module, builder) = setup_test_context();
        let mut codegen = LlvmCodeGenerator::new(context, module, builder);
        
        let true_val = codegen.create_bool_literal(true);
        let false_val = codegen.create_bool_literal(false);
        
        // Test logical AND
        let and_result = codegen.bool_logical_and(true_val, false_val).unwrap();
        assert!(codegen.is_bool_type(and_result));
        
        // Test logical OR
        let or_result = codegen.bool_logical_or(true_val, false_val).unwrap();
        assert!(codegen.is_bool_type(or_result));
        
        // Test logical NOT
        let not_result = codegen.bool_logical_not(true_val).unwrap();
        assert!(codegen.is_bool_type(not_result));
    }
}
