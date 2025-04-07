//! LLVM code generation for string operations

use inkwell::values::BasicValueEnum;
use super::context::LlvmCodeGenerator;

impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Initialize string helper functions in the module 
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
    
    /// Compile string concatenation
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
    
    /// Compile string comparison
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