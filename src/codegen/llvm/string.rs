//! LLVM code generation for string operations in the CURSED language.
//!
//! This module provides functionality for translating CURSED string operations
//! into LLVM IR. It handles string manipulation operations such as concatenation,
//! comparison, length calculation, and substring extraction.
//!
//! Since LLVM doesn't have native string types, the implementation relies on external
//! runtime functions that handle string operations. These functions are declared in
//! the LLVM module but would be provided by a runtime library linked with the
//! compiled program.
//!
//! String operations supported include:
//! - Concatenation (string + string)
//! - Equality and comparison (string == string, string < string, etc.)
//! - Length calculation
//! - Substring extraction

use inkwell::values::BasicValueEnum;
use super::context::LlvmCodeGenerator;

impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Initializes the string helper functions in the LLVM module.
    ///
    /// This method declares the external functions that implement string operations.
    /// These functions would typically be provided by a runtime library that's linked
    /// with the compiled CURSED program. The declared functions include:
    ///
    /// - `string_concat`: Concatenates two strings
    /// - `string_equals`: Checks if two strings are equal
    /// - `string_compare`: Compares two strings lexicographically
    /// - `string_length`: Gets the length of a string
    /// - `string_substring`: Extracts a substring
    ///
    /// The method is idempotent - it only initializes the functions if they haven't
    /// been initialized already.
    pub fn init_string_helpers(&mut self) {
        // Skip initialization if we've already done it
        if self.module.get_function("string_concat").is_some() {
            return;
        }
        
        // Set up common types
        let i8_ptr_type = self.context.i8_type().ptr_type(inkwell::AddressSpace::default());
        let i32_type = self.context.i32_type();
        let i64_type = self.context.i64_type();
        
        // String concatenation (string + string)
        let concat_type = i8_ptr_type.fn_type(&[i8_ptr_type.into(), i8_ptr_type.into()], false);
        self.module.add_function("string_concat", concat_type, Some(inkwell::module::Linkage::External));
        
        // String comparison functions (for ==, !=, <, >, etc)
        let compare_type = i32_type.fn_type(&[i8_ptr_type.into(), i8_ptr_type.into()], false);
        self.module.add_function("string_equals", compare_type, Some(inkwell::module::Linkage::External));
        self.module.add_function("string_compare", compare_type, Some(inkwell::module::Linkage::External));
        
        // Get string length
        let len_type = i64_type.fn_type(&[i8_ptr_type.into()], false);
        self.module.add_function("string_length", len_type, Some(inkwell::module::Linkage::External));
        
        // Create a substring
        let substring_type = i8_ptr_type.fn_type(
            &[i8_ptr_type.into(), i64_type.into(), i64_type.into()], 
            false
        );
        self.module.add_function("string_substring", substring_type, Some(inkwell::module::Linkage::External));
    }
    
    /// Compiles a string concatenation operation to LLVM IR.
    ///
    /// This method translates a CURSED string concatenation operation (e.g., `str1 + str2`)
    /// into a call to the runtime's string_concat function. String concatenation creates
    /// a new string by joining the contents of two existing strings.
    ///
    /// # Arguments
    ///
    /// * `left` - The LLVM value representing the left operand (first string)
    /// * `right` - The LLVM value representing the right operand (second string)
    ///
    /// # Returns
    ///
    /// * `Result<BasicValueEnum, String>` - The result of the concatenation as a string pointer, or an error message
    pub fn compile_string_concat(
        &mut self, 
        left: BasicValueEnum<'ctx>, 
        right: BasicValueEnum<'ctx>
    ) -> Result<BasicValueEnum<'ctx>, String> {
        // Make sure string helpers are initialized
        self.init_string_helpers();
        
        // Get the concat function
        let concat_fn = self.module.get_function("string_concat").ok_or_else(|| 
            "string_concat function not found".to_string()
        )?;
        
        // Call the function with left and right strings
        let result = self.builder.build_call(
            concat_fn,
            &[left.into(), right.into()],
            "concat_result"
        ).unwrap();
        
        // Return the result string
        Ok(result.try_as_basic_value().left().unwrap())
    }
    
    /// Compiles a string comparison operation to LLVM IR.
    ///
    /// This method translates a CURSED string comparison operation (e.g., `str1 == str2`
    /// or `str1 < str2`) into a call to the appropriate runtime comparison function.
    /// It can handle both equality checks and lexicographical comparisons.
    ///
    /// # Arguments
    ///
    /// * `left` - The LLVM value representing the left operand (first string)
    /// * `right` - The LLVM value representing the right operand (second string)
    /// * `is_equals` - If true, performs equality comparison (==, !=); if false, performs lexicographical comparison (<, >, etc.)
    ///
    /// # Returns
    ///
    /// * `Result<BasicValueEnum, String>` - The result of the comparison as an integer value, or an error message
    pub fn compile_string_comparison(
        &mut self, 
        left: BasicValueEnum<'ctx>, 
        right: BasicValueEnum<'ctx>,
        is_equals: bool
    ) -> Result<BasicValueEnum<'ctx>, String> {
        // Make sure string helpers are initialized
        self.init_string_helpers();
        
        // Get the appropriate comparison function
        let func_name = if is_equals { "string_equals" } else { "string_compare" };
        let compare_fn = self.module.get_function(func_name).ok_or_else(|| 
            format!("{} function not found", func_name)
        )?;
        
        // Call the function with left and right strings
        let result = self.builder.build_call(
            compare_fn,
            &[left.into(), right.into()],
            "compare_result"
        ).unwrap();
        
        // Return the result code
        Ok(result.try_as_basic_value().left().unwrap())
    }
}