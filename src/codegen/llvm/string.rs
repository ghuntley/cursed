//! LLVM code generation for string operations in the CURSED language.
//!
//! This module provides functionality for translating CURSED string operations
//! into LLVM IR. It handles string manipulation operations such as concatenation,
//! comparison, length calculation, and substring extraction.
//!
//! CURSED strings are represented as `{i64, i8*}` structs containing:
//! - Length field (i64): The number of bytes in the string
//! - Data pointer (i8*): Pointer to the string data
//!
//! This representation provides efficient string operations while maintaining memory safety
//! through length tracking and proper runtime function integration.
//!
//! String operations supported include:
//! - Concatenation (string + string)
//! - Equality and comparison (string == string, string < string, etc.)
//! - Length calculation
//! - Substring extraction

use inkwell::values::{BasicValueEnum, StructValue};
use super::context::LlvmCodeGenerator;
use super::string_type::{CursedStringType, StringTypeUtils};
use tracing::{instrument, debug};

impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Initializes the string helper functions in the LLVM module.
    ///
    /// This method declares the runtime functions that implement string operations
    /// for CURSED `{i64, i8*}` string structs. These functions are provided by
    /// the CURSED runtime library. The declared functions include:
    ///
    /// - `cursed_string_concat`: Concatenates two string structs
    /// - `cursed_string_equals`: Checks if two string structs are equal
    /// - `cursed_string_compare`: Compares two string structs lexicographically
    /// - `cursed_string_length`: Gets the length from a string struct
    /// - `cursed_string_substring`: Extracts a substring from a string struct
    /// - `cursed_string_from_literal`: Creates a string struct from a C string literal
    ///
    /// The method is idempotent - it only initializes the functions if they haven't
    /// been initialized already.
    #[instrument(skip(self), level = "debug")]
    pub fn init_string_helpers(&mut self) {
        debug!("Initializing CURSED string helper functions");
        
        // Skip initialization if we've already done it
        if self.module.get_function("cursed_string_concat").is_some() {
            debug!("String helpers already initialized, skipping");
            return;
        }
        
        // Set up common types
        let string_type = CursedStringType::new(self.context);
        let string_struct_type = string_type.get_llvm_type();
        let i8_ptr_type = self.context.i8_type().ptr_type(inkwell::AddressSpace::default());
        let i32_type = self.context.i32_type();
        let i64_type = self.context.i64_type();
        
        debug!("Creating runtime string function declarations");
        
        // String concatenation: cursed_string cursed_string_concat(cursed_string, cursed_string)
        let concat_type = string_struct_type.fn_type(
            &[string_struct_type.into(), string_struct_type.into()], 
            false
        );
        self.module.add_function(
            "cursed_string_concat", 
            concat_type, 
            Some(inkwell::module::Linkage::External)
        );
        
        // String equality: i32 cursed_string_equals(cursed_string, cursed_string)
        let equals_type = i32_type.fn_type(
            &[string_struct_type.into(), string_struct_type.into()], 
            false
        );
        self.module.add_function(
            "cursed_string_equals", 
            equals_type, 
            Some(inkwell::module::Linkage::External)
        );
        
        // String comparison: i32 cursed_string_compare(cursed_string, cursed_string)
        let compare_type = i32_type.fn_type(
            &[string_struct_type.into(), string_struct_type.into()], 
            false
        );
        self.module.add_function(
            "cursed_string_compare", 
            compare_type, 
            Some(inkwell::module::Linkage::External)
        );
        
        // String length: i64 cursed_string_length(cursed_string)
        let length_type = i64_type.fn_type(&[string_struct_type.into()], false);
        self.module.add_function(
            "cursed_string_length", 
            length_type, 
            Some(inkwell::module::Linkage::External)
        );
        
        // String substring: cursed_string cursed_string_substring(cursed_string, i64, i64)
        let substring_type = string_struct_type.fn_type(
            &[string_struct_type.into(), i64_type.into(), i64_type.into()], 
            false
        );
        self.module.add_function(
            "cursed_string_substring", 
            substring_type, 
            Some(inkwell::module::Linkage::External)
        );
        
        // String from literal: cursed_string cursed_string_from_literal(i8*, i64)
        let from_literal_type = string_struct_type.fn_type(
            &[i8_ptr_type.into(), i64_type.into()], 
            false
        );
        self.module.add_function(
            "cursed_string_from_literal", 
            from_literal_type, 
            Some(inkwell::module::Linkage::External)
        );
        
        debug!("Successfully initialized CURSED string helper functions");
    }
    
    /// Compiles a string concatenation operation to LLVM IR.
    ///
    /// This method translates a CURSED string concatenation operation (e.g., `str1 + str2`)
    /// into a call to the runtime's cursed_string_concat function. String concatenation creates
    /// a new string struct by joining the contents of two existing string structs.
    ///
    /// # Arguments
    ///
    /// * `left` - The LLVM value representing the left operand (first string struct)
    /// * `right` - The LLVM value representing the right operand (second string struct)
    ///
    /// # Returns
    ///
    /// * `Result<BasicValueEnum, String>` - The result of the concatenation as a string struct, or an error message
    #[instrument(skip(self, left, right), level = "debug")]
    pub fn compile_string_concat(
        &mut self, 
        left: BasicValueEnum<'ctx>, 
        right: BasicValueEnum<'ctx>
    ) -> Result<BasicValueEnum<'ctx>, String> {
        debug!("Compiling string concatenation operation");
        
        // Validate that both operands are string structs
        let string_type = CursedStringType::new(self.context);
        if !string_type.is_valid_string_value(left) {
            return Err("Left operand is not a valid string struct".to_string());
        }
        if !string_type.is_valid_string_value(right) {
            return Err("Right operand is not a valid string struct".to_string());
        }
        
        // Make sure string helpers are initialized
        self.init_string_helpers();
        
        // Get the concat function
        let concat_fn = self.module.get_function("cursed_string_concat").ok_or_else(|| 
            "cursed_string_concat function not found".to_string()
        )?;
        
        // Call the function with left and right string structs
        let result = self.builder.build_call(
            concat_fn,
            &[left.into(), right.into()],
            "concat_result"
        ).map_err(|e| format!("Failed to build concat call: {}", e))?;
        
        // Return the result string struct
        let result_value = result.try_as_basic_value().left()
            .ok_or_else(|| "Concat function returned void instead of string struct".to_string())?;
        
        debug!("Successfully compiled string concatenation");
        Ok(result_value)
    }
    
    /// Compiles a string comparison operation to LLVM IR.
    ///
    /// This method translates a CURSED string comparison operation (e.g., `str1 == str2`
    /// or `str1 < str2`) into a call to the appropriate runtime comparison function.
    /// It can handle both equality checks and lexicographical comparisons.
    ///
    /// # Arguments
    ///
    /// * `left` - The LLVM value representing the left operand (first string struct)
    /// * `right` - The LLVM value representing the right operand (second string struct)
    /// * `is_equals` - If true, performs equality comparison (==, !=); if false, performs lexicographical comparison (<, >, etc.)
    ///
    /// # Returns
    ///
    /// * `Result<BasicValueEnum, String>` - The result of the comparison as an integer value, or an error message
    #[instrument(skip(self, left, right), level = "debug")]
    pub fn compile_string_comparison(
        &mut self, 
        left: BasicValueEnum<'ctx>, 
        right: BasicValueEnum<'ctx>,
        is_equals: bool
    ) -> Result<BasicValueEnum<'ctx>, String> {
        debug!("Compiling string comparison operation (equals: {})", is_equals);
        
        // Validate that both operands are string structs
        let string_type = CursedStringType::new(self.context);
        if !string_type.is_valid_string_value(left) {
            return Err("Left operand is not a valid string struct".to_string());
        }
        if !string_type.is_valid_string_value(right) {
            return Err("Right operand is not a valid string struct".to_string());
        }
        
        // Make sure string helpers are initialized
        self.init_string_helpers();
        
        // Get the appropriate comparison function
        let func_name = if is_equals { "cursed_string_equals" } else { "cursed_string_compare" };
        let compare_fn = self.module.get_function(func_name).ok_or_else(|| 
            format!("{} function not found", func_name)
        )?;
        
        // Call the function with left and right string structs
        let result = self.builder.build_call(
            compare_fn,
            &[left.into(), right.into()],
            "compare_result"
        ).map_err(|e| format!("Failed to build comparison call: {}", e))?;
        
        // Return the result code
        let result_value = result.try_as_basic_value().left()
            .ok_or_else(|| "Comparison function returned void instead of integer".to_string())?;
        
        debug!("Successfully compiled string comparison");
        Ok(result_value)
    }
    
    /// Create a string literal from a static string value
    ///
    /// This method creates a CURSED string struct from a string literal by either
    /// creating a global constant or calling the runtime string creation function.
    ///
    /// # Arguments
    ///
    /// * `literal_value` - The string literal content
    /// * `name` - Name for the global string variable
    ///
    /// # Returns
    ///
    /// * `Result<BasicValueEnum, String>` - The string struct representing the literal
    #[instrument(skip(self), level = "debug")]
    pub fn create_string_literal(
        &mut self,
        literal_value: &str,
        name: &str,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        debug!("Creating string literal: '{}' with name '{}'", literal_value, name);
        
        let string_type = CursedStringType::new(self.context);
        
        // Create the string literal using our string type utility
        let string_struct = string_type.create_string_literal(
            &self.builder,
            &self.module,
            literal_value,
            name,
        ).map_err(|e| format!("Failed to create string literal: {}", e))?;
        
        debug!("Successfully created string literal struct");
        Ok(string_struct.into())
    }
    
    /// Extract the length from a string struct
    ///
    /// # Arguments
    ///
    /// * `string_value` - The string struct to extract length from
    ///
    /// # Returns
    ///
    /// * `Result<BasicValueEnum, String>` - The length value (i64)
    #[instrument(skip(self, string_value), level = "debug")]
    pub fn extract_string_length(
        &mut self,
        string_value: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        debug!("Extracting string length");
        
        let string_type = CursedStringType::new(self.context);
        
        // Validate the string value
        if !string_type.is_valid_string_value(string_value) {
            return Err("Value is not a valid string struct".to_string());
        }
        
        let string_struct = string_value.into_struct_value();
        let length = string_type.extract_length(&self.builder, string_struct)
            .map_err(|e| format!("Failed to extract string length: {}", e))?;
        
        debug!("Successfully extracted string length");
        Ok(length)
    }
    
    /// Extract the data pointer from a string struct
    ///
    /// # Arguments
    ///
    /// * `string_value` - The string struct to extract data pointer from
    ///
    /// # Returns
    ///
    /// * `Result<BasicValueEnum, String>` - The data pointer (i8*)
    #[instrument(skip(self, string_value), level = "debug")]
    pub fn extract_string_data_ptr(
        &mut self,
        string_value: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        debug!("Extracting string data pointer");
        
        let string_type = CursedStringType::new(self.context);
        
        // Validate the string value
        if !string_type.is_valid_string_value(string_value) {
            return Err("Value is not a valid string struct".to_string());
        }
        
        let string_struct = string_value.into_struct_value();
        let data_ptr = string_type.extract_data_ptr(&self.builder, string_struct)
            .map_err(|e| format!("Failed to extract string data pointer: {}", e))?;
        
        debug!("Successfully extracted string data pointer");
        Ok(data_ptr.into())
    }
}