//! Enhanced error handling for interface type assertions
//!
//! This module provides specialized error handling and reporting for
//! interface type assertions, including runtime type information and
//! detailed error messages to help diagnose assertion failures.

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::values::{BasicValueEnum, IntValue, PointerValue};
use inkwell::AddressSpace;
use inkwell::module::Module;

use std::rc::Rc;

use crate::error::Error;
use crate::error_enhanced::CursedError;

/// Type for handling interface type assertion errors
pub struct TypeAssertionErrorHandler<'ctx> {
    context: &'ctx Context,
    builder: &'ctx Builder<'ctx>,
    module: &'ctx Module<'ctx>,
}

impl<'ctx> TypeAssertionErrorHandler<'ctx> {
    /// Create a new type assertion error handler
    pub fn new(
        context: &'ctx Context,
        builder: &'ctx Builder<'ctx>,
        module: &'ctx Module<'ctx>,
    ) -> Self {
        Self {
            context,
            builder,
            module,
        }
    }
    
    /// Generate code to log a type assertion error at runtime
    pub fn generate_error_logging(
        &self,
        expected_type: &str,
        actual_type_id: BasicValueEnum<'ctx>,
        value_ptr: PointerValue<'ctx>,
        source_location: Option<(&str, u32)>,
    ) -> Result<(), Error> {
        // Create a global string constant for the error message template
        let error_msg = format!(
            "Type assertion failed: expected {}, but got %s",
            expected_type
        );
        
        let error_msg_global = self.builder
            .build_global_string_ptr(&error_msg, "type_assertion_error_msg")
            .map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Create string constant for expected type
        let expected_type_global = self.builder
            .build_global_string_ptr(expected_type, "expected_type")
            .map_err(|e| Error::Compilation(e.to_string()))?;
            
        // Find or create external function to log type assertion errors
        let log_func_type = self.context.void_type().fn_type(
            &[
                self.context.i8_type().ptr_type(AddressSpace::default()).into(), // error_msg
                self.context.i8_type().ptr_type(AddressSpace::default()).into(), // expected_type
                self.context.i64_type().into(),                                  // actual_type_id
                self.context.i8_type().ptr_type(AddressSpace::default()).into(), // value_ptr
                self.context.i8_type().ptr_type(AddressSpace::default()).into(), // source_file
                self.context.i32_type().into(),                                  // source_line
            ],
            false,
        );
        
        let log_func = self.module.add_function(
            "__cursed_log_type_assertion_error",
            log_func_type,
            None,
        );
        
        // Create source location constants
        let (source_file, source_line) = match source_location {
            Some((file, line)) => {
                let file_global = self.builder
                    .build_global_string_ptr(file, "source_file")
                    .map_err(|e| Error::Compilation(e.to_string()))?;
                let line_const = self.context.i32_type().const_int(line as u64, false);
                (file_global, line_const)
            },
            None => {
                let file_global = self.builder
                    .build_global_string_ptr("<unknown>", "source_file")
                    .map_err(|e| Error::Compilation(e.to_string()))?;
                let line_const = self.context.i32_type().const_int(0, false);
                (file_global, line_const)
            },
        };
        
        // Call the logging function
        self.builder.build_call(
            log_func,
            &[
                error_msg_global.as_pointer_value().into(),
                expected_type_global.as_pointer_value().into(),
                actual_type_id,
                value_ptr.into(),
                source_file.as_pointer_value().into(),
                source_line.into(),
            ],
            "log_assertion_error",
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        Ok(())
    }
    
    /// Generate code to create and return a runtime error for type assertion failure
    pub fn generate_error_return(
        &self,
        expected_type: &str,
        actual_type_id: BasicValueEnum<'ctx>,
        source_location: Option<(&str, u32)>,
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Create error message with type information
        let error_msg = format!("Type assertion error: expected {}", expected_type);
        
        // Create global string constant for the error message
        let error_global = self.builder
            .build_global_string_ptr(&error_msg, "type_error_msg")
            .map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Find or create function to create a type assertion error
        let error_func_type = self.context.i8_type().ptr_type(AddressSpace::default()).fn_type(
            &[
                self.context.i8_type().ptr_type(AddressSpace::default()).into(), // error_msg
                self.context.i64_type().into(),                                  // actual_type_id
                self.context.i8_type().ptr_type(AddressSpace::default()).into(), // source_file
                self.context.i32_type().into(),                                  // source_line
            ],
            false,
        );
        
        let error_func = self.module.add_function(
            "__cursed_create_type_assertion_error",
            error_func_type,
            None,
        );
        
        // Create source location constants
        let (source_file, source_line) = match source_location {
            Some((file, line)) => {
                let file_global = self.builder
                    .build_global_string_ptr(file, "source_file")
                    .map_err(|e| Error::Compilation(e.to_string()))?;
                let line_const = self.context.i32_type().const_int(line as u64, false);
                (file_global, line_const)
            },
            None => {
                let file_global = self.builder
                    .build_global_string_ptr("<unknown>", "source_file")
                    .map_err(|e| Error::Compilation(e.to_string()))?;
                let line_const = self.context.i32_type().const_int(0, false);
                (file_global, line_const)
            },
        };
        
        // Call the error creation function
        let error_result = self.builder.build_call(
            error_func,
            &[
                error_global.as_pointer_value().into(),
                actual_type_id,
                source_file.as_pointer_value().into(),
                source_line.into(),
            ],
            "type_assertion_error",
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Return the error pointer
        Ok(error_result.try_as_basic_value().left().unwrap())
    }
}

/// Runtime handler for interface type assertion errors
#[no_mangle]
pub extern "C" fn __cursed_log_type_assertion_error(
    error_msg: *const i8,
    expected_type: *const i8,
    actual_type_id: u64,
    value_ptr: *const u8,
    source_file: *const i8,
    source_line: u32,
) {
    use std::ffi::CStr;
    
    // Convert C strings to Rust strings
    let error_msg = unsafe {
        CStr::from_ptr(error_msg).to_string_lossy().into_owned()
    };
    let expected_type = unsafe {
        CStr::from_ptr(expected_type).to_string_lossy().into_owned()
    };
    let source_file = unsafe {
        CStr::from_ptr(source_file).to_string_lossy().into_owned()
    };
    
    // Build the error message with context
    let message = format!(
        "{}. Actual type ID: {:x}, Expected: {}",
        error_msg, actual_type_id, expected_type
    );
    
    // Log the error with source location
    tracing::error!(
        error = %message,
        source_file = %source_file,
        source_line = source_line,
        value_address = %format!("{:p}", value_ptr),
        "Type assertion failed"
    );
}

/// Create a runtime error for type assertion failure
#[no_mangle]
pub extern "C" fn __cursed_create_type_assertion_error(
    error_msg: *const i8,
    actual_type_id: u64,
    source_file: *const i8,
    source_line: u32,
) -> *mut u8 {
    use std::ffi::CStr;
    
    // Convert C strings to Rust strings
    let error_msg = unsafe {
        CStr::from_ptr(error_msg).to_string_lossy().into_owned()
    };
    let source_file = unsafe {
        CStr::from_ptr(source_file).to_string_lossy().into_owned()
    };
    
    // Create a error with context information
    let error = CursedError::new(
        "TypeAssertionError",
        &format!("{} (actual type ID: {:x})", error_msg, actual_type_id),
    )
    .with_context("source_file", source_file)
    .with_context("source_line", source_line.to_string())
    .with_context("actual_type_id", format!("{:x}", actual_type_id));
    
    // Box the error and leak the box so it can be returned to C
    // This memory will need to be freed by the runtime
    let error_box = Box::new(Error::TypeAssertion(error));
    Box::into_raw(error_box) as *mut u8
}