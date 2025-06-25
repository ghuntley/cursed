// Simplified LLVM IR generation for Result and Option types
//
// This module provides simplified LLVM code generation for Result<T, E> and Option<T> types,
// working with the current dummy type infrastructure.

use crate::codegen::llvm::LlvmCodeGenerator;
use crate::error::CursedError;

/// Simplified Result type layout for dummy implementation
pub struct ResultTypeLayout {
    pub ok_type_name: String,
    pub err_type_name: String,
    pub type_name: String,
}

/// Simplified Option type layout for dummy implementation  
pub struct OptionTypeLayout {
    pub inner_type_name: String,
    pub type_name: String,
}

/// Trait for compiling Result and Option types with dummy implementation
pub trait ResultTypeCompiler {
    /// Generate LLVM type for Result<T, E>
    fn generate_result_type(
        &mut self,
        ok_type: &str,
        err_type: &str,
    ) -> crate::error::Result<()>;

    /// Generate LLVM type for Option<T>
    fn generate_option_type(
        &mut self,
        inner_type: &str,
    ) -> crate::error::Result<()>;

    /// Create Result::Ok value
    fn create_result_ok(
        &mut self,
        layout: &ResultTypeLayout,
        value_name: &str,
    ) -> crate::error::Result<()>;

    /// Create Result::Err value
    fn create_result_err(
        &mut self,
        layout: &ResultTypeLayout,
        error_name: &str,
    ) -> crate::error::Result<()>;

    /// Create Option::Some value
    fn create_option_some(
        &mut self,
        layout: &OptionTypeLayout,
        value_name: &str,
    ) -> crate::error::Result<()>;

    /// Create Option::None value
    fn create_option_none(
        &mut self,
        layout: &OptionTypeLayout,
    ) -> crate::error::Result<()>;

    /// Check if Result is Ok
    fn is_result_ok(
        &mut self,
        layout: &ResultTypeLayout,
        result_value: &str,
    ) -> crate::error::Result<()>;

    /// Check if Result is Err
    fn is_result_err(
        &mut self,
        layout: &ResultTypeLayout,
        result_value: &str,
    ) -> crate::error::Result<()>;

    /// Check if Option is Some
    fn is_option_some(
        &mut self,
        layout: &OptionTypeLayout,
        option_value: &str,
    ) -> crate::error::Result<()>;

    /// Check if Option is None
    fn is_option_none(
        &mut self,
        layout: &OptionTypeLayout,
        option_value: &str,
    ) -> crate::error::Result<()>;

    /// Extract Ok value from Result
    fn extract_result_ok(
        &mut self,
        layout: &ResultTypeLayout,
        result_value: &str,
    ) -> crate::error::Result<()>;

    /// Extract Err value from Result
    fn extract_result_err(
        &mut self,
        layout: &ResultTypeLayout,
        result_value: &str,
    ) -> crate::error::Result<()>;

    /// Extract Some value from Option
    fn extract_option_some(
        &mut self,
        layout: &OptionTypeLayout,
        option_value: &str,
    ) -> crate::error::Result<()>;

    /// Generate question mark operator for Result
    fn generate_result_question_mark(
        &mut self,
        layout: &ResultTypeLayout,
        result_value: &str,
        continue_block: &str,
        return_block: &str,
    ) -> crate::error::Result<()>;

    /// Generate question mark operator for Option
    fn generate_option_question_mark(
        &mut self,
        layout: &OptionTypeLayout,
        option_value: &str,
        continue_block: &str,
        return_block: &str,
    ) -> crate::error::Result<()>;
}

impl ResultTypeCompiler for LlvmCodeGenerator {
    fn generate_result_type(
        &mut self,
        ok_type: &str,
        err_type: &str,
    ) -> crate::error::Result<()> {
        let type_name = format!("Result<{}, {}>", ok_type, err_type);
        
        Ok(ResultTypeLayout {
            ok_type_name: ok_type.to_string(),
            err_type_name: err_type.to_string(),
            type_name,
        })
    }

    fn generate_option_type(
        &mut self,
        inner_type: &str,
    ) -> crate::error::Result<()> {
        let type_name = format!("Option<{}>", inner_type);
        
        Ok(OptionTypeLayout {
            inner_type_name: inner_type.to_string(),
            type_name,
        })
    }

    fn create_result_ok(
        &mut self,
        layout: &ResultTypeLayout,
        value_name: &str,
    ) -> crate::error::Result<()> {
        let temp_id = self.next_temp_id();
        Ok(format!("%result_ok_{} = insertvalue {} undef, i8 0, 0\n  %result_ok_val_{} = insertvalue {} %result_ok_{}, {} {}, 1", 
                  temp_id, layout.type_name, temp_id, layout.type_name, temp_id, layout.ok_type_name, value_name))
    }

    fn create_result_err(
        &mut self,
        layout: &ResultTypeLayout,
        error_name: &str,
    ) -> crate::error::Result<()> {
        let temp_id = self.next_temp_id();
        Ok(format!("%result_err_{} = insertvalue {} undef, i8 1, 0\n  %result_err_val_{} = insertvalue {} %result_err_{}, {} {}, 1", 
                  temp_id, layout.type_name, temp_id, layout.type_name, temp_id, layout.err_type_name, error_name))
    }

    fn create_option_some(
        &mut self,
        layout: &OptionTypeLayout,
        value_name: &str,
    ) -> crate::error::Result<()> {
        let temp_id = self.next_temp_id();
        Ok(format!("%option_some_{} = insertvalue {} undef, i8 1, 0\n  %option_some_val_{} = insertvalue {} %option_some_{}, {} {}, 1", 
                  temp_id, layout.type_name, temp_id, layout.type_name, temp_id, layout.inner_type_name, value_name))
    }

    fn create_option_none(
        &mut self,
        layout: &OptionTypeLayout,
    ) -> crate::error::Result<()> {
        let temp_id = self.next_temp_id();
        Ok(format!("%option_none_{} = insertvalue {} undef, i8 0, 0", 
                  temp_id, layout.type_name))
    }

    fn is_result_ok(
        &mut self,
        layout: &ResultTypeLayout,
        result_value: &str,
    ) -> crate::error::Result<()> {
        let temp_id = self.next_temp_id();
        Ok(format!("%tag_{} = extractvalue {} {}, 0\n  %is_ok_{} = icmp eq i8 %tag_{}, 0", 
                  temp_id, layout.type_name, result_value, temp_id, temp_id))
    }

    fn is_result_err(
        &mut self,
        layout: &ResultTypeLayout,
        result_value: &str,
    ) -> crate::error::Result<()> {
        let temp_id = self.next_temp_id();
        Ok(format!("%tag_{} = extractvalue {} {}, 0\n  %is_err_{} = icmp eq i8 %tag_{}, 1", 
                  temp_id, layout.type_name, result_value, temp_id, temp_id))
    }

    fn is_option_some(
        &mut self,
        layout: &OptionTypeLayout,
        option_value: &str,
    ) -> crate::error::Result<()> {
        let temp_id = self.next_temp_id();
        Ok(format!("%tag_{} = extractvalue {} {}, 0\n  %is_some_{} = icmp eq i8 %tag_{}, 1", 
                  temp_id, layout.type_name, option_value, temp_id, temp_id))
    }

    fn is_option_none(
        &mut self,
        layout: &OptionTypeLayout,
        option_value: &str,
    ) -> crate::error::Result<()> {
        let temp_id = self.next_temp_id();
        Ok(format!("%tag_{} = extractvalue {} {}, 0\n  %is_none_{} = icmp eq i8 %tag_{}, 0", 
                  temp_id, layout.type_name, option_value, temp_id, temp_id))
    }

    fn extract_result_ok(
        &mut self,
        layout: &ResultTypeLayout,
        result_value: &str,
    ) -> crate::error::Result<()> {
        let temp_id = self.next_temp_id();
        Ok(format!("%ok_value_{} = extractvalue {} {}, 1", 
                  temp_id, layout.type_name, result_value))
    }

    fn extract_result_err(
        &mut self,
        layout: &ResultTypeLayout,
        result_value: &str,
    ) -> crate::error::Result<()> {
        let temp_id = self.next_temp_id();
        Ok(format!("%err_value_{} = extractvalue {} {}, 1", 
                  temp_id, layout.type_name, result_value))
    }

    fn extract_option_some(
        &mut self,
        layout: &OptionTypeLayout,
        option_value: &str,
    ) -> crate::error::Result<()> {
        let temp_id = self.next_temp_id();
        Ok(format!("%some_value_{} = extractvalue {} {}, 1", 
                  temp_id, layout.type_name, option_value))
    }

    fn generate_result_question_mark(
        &mut self,
        layout: &ResultTypeLayout,
        result_value: &str,
        continue_block: &str,
        return_block: &str,
    ) -> crate::error::Result<()> {
        let temp_id = self.next_temp_id();
        let check_ir = self.is_result_ok(layout, result_value)?;
        let extract_ir = self.extract_result_ok(layout, result_value)?;
        
        Ok(format!("{}\n  br i1 %is_ok_{}, label %{}, label %{}\n\n{}:\n  {}\n  br label %{}\n\n{}:\n  ret {} {}", 
                  check_ir, temp_id, continue_block, return_block, 
                  continue_block, extract_ir, continue_block,
                  return_block, layout.type_name, result_value))
    }

    fn generate_option_question_mark(
        &mut self,
        layout: &OptionTypeLayout,
        option_value: &str,
        continue_block: &str,
        return_block: &str,
    ) -> crate::error::Result<()> {
        let temp_id = self.next_temp_id();
        let check_ir = self.is_option_some(layout, option_value)?;
        let extract_ir = self.extract_option_some(layout, option_value)?;
        let none_ir = self.create_option_none(layout)?;
        
        Ok(format!("{}\n  br i1 %is_some_{}, label %{}, label %{}\n\n{}:\n  {}\n  br label %{}\n\n{}:\n  {}\n  ret {} %option_none_{}", 
                  check_ir, temp_id, continue_block, return_block,
                  continue_block, extract_ir, continue_block,
                  return_block, none_ir, layout.type_name, temp_id))
    }
}

/// Utility functions for Result/Option type management
pub mod result_type_utils {
    /// Get the LLVM type name for a Result<T, E>
    pub fn get_result_llvm_type_name(
        ok_type: &str,
        err_type: &str,
    ) -> String {
        format!("{{ i8, {} }}", get_union_type_name(ok_type, err_type))
    }

    /// Get the LLVM type name for an Option<T>
    pub fn get_option_llvm_type_name(
        inner_type: &str,
    ) -> String {
        format!("{{ i8, {} }}", inner_type)
    }

    /// Get union type name for Result storage
    fn get_union_type_name(ok_type: &str, err_type: &str) -> String {
        format!("{{ {}, {} }}", ok_type, err_type)
    }

    /// Create type name for Result<T, E>
    pub fn result_type_name(ok_type: &str, err_type: &str) -> String {
        format!("Result<{}, {}>", ok_type, err_type)
    }

    /// Create type name for Option<T>
    pub fn option_type_name(inner_type: &str) -> String {
        format!("Option<{}>", inner_type)
    }

    /// Check if a type name represents a Result type
    pub fn is_result_type(type_name: &str) -> bool {
        type_name.starts_with("Result<") && type_name.ends_with('>')
    }

    /// Check if a type name represents an Option type
    pub fn is_option_type(type_name: &str) -> bool {
        type_name.starts_with("Option<") && type_name.ends_with('>')
    }

    /// Parse Result type parameters from type name
    pub fn parse_result_types(type_name: &str) -> Option<(String, String)> {
        if !is_result_type(type_name) {
            return None;
        }

        let inner = &type_name[7..type_name.len()-1]; // Remove "Result<" and ">"
        let parts: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();
        
        if parts.len() == 2 {
            Some((parts[0].to_string(), parts[1].to_string()))
        } else {
            None
        }
    }

    /// Parse Option type parameter from type name
    pub fn parse_option_type(type_name: &str) -> Option<String> {
        if !is_option_type(type_name) {
            return None;
        }

        let inner = &type_name[7..type_name.len()-1]; // Remove "Option<" and ">"
        Some(inner.to_string())
    }
}

