//! LLVM code generation for character (sip) type operations
//! Implements Unicode-compliant character methods in LLVM IR

use inkwell::values::{BasicValueEnum, IntValue, FunctionValue};
use inkwell::types::BasicType;
use inkwell::IntPredicate;
use tracing::{instrument, debug, info};

use crate::codegen::llvm::context::LlvmCodeGenerator;
use crate::error::Error;

/// Trait for generating LLVM code for character operations
pub trait CharOperations<'ctx> {
    /// Generate code for is_uppercase() method
    fn compile_char_is_uppercase(&mut self, char_value: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Generate code for is_lowercase() method  
    fn compile_char_is_lowercase(&mut self, char_value: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Generate code for is_alphabetic() method
    fn compile_char_is_alphabetic(&mut self, char_value: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Generate code for is_numeric() method
    fn compile_char_is_numeric(&mut self, char_value: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Generate code for is_whitespace() method
    fn compile_char_is_whitespace(&mut self, char_value: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Generate code for to_uppercase() method
    fn compile_char_to_uppercase(&mut self, char_value: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Generate code for to_lowercase() method
    fn compile_char_to_lowercase(&mut self, char_value: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Generate code for to_string() method
    fn compile_char_to_string(&mut self, char_value: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error>;
}

impl<'ctx> CharOperations<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self))]
    fn compile_char_is_uppercase(&mut self, char_value: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Compiling character is_uppercase operation");
        
        let char_int = match char_value {
            BasicValueEnum::IntValue(int_val) => int_val,
            _ => return Err(Error::codegen(
                "Expected integer value for character"
            )),
        };

        // Declare external function for Unicode uppercase check
        let is_uppercase_fn = self.declare_unicode_is_uppercase_function()?;
        
        // Call the external function
        let result = self.builder()
            .build_call(is_uppercase_fn, &[char_int.into()], "is_uppercase_call")
            .unwrap()
            .try_as_basic_value()
            .left()
            .unwrap();

        debug!("Generated LLVM code for character is_uppercase");
        Ok(result)
    }

    #[instrument(skip(self))]
    fn compile_char_is_lowercase(&mut self, char_value: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Compiling character is_lowercase operation");
        
        let char_int = match char_value {
            BasicValueEnum::IntValue(int_val) => int_val,
            _ => return Err(Error::codegen(
                "Expected integer value for character"
            )),
        };

        // Declare external function for Unicode lowercase check
        let is_lowercase_fn = self.declare_unicode_is_lowercase_function()?;
        
        // Call the external function  
        let result = self.builder()
            .build_call(is_lowercase_fn, &[char_int.into()], "is_lowercase_call")
            .unwrap()
            .try_as_basic_value()
            .left()
            .unwrap();

        debug!("Generated LLVM code for character is_lowercase");
        Ok(result)
    }

    #[instrument(skip(self))]
    fn compile_char_is_alphabetic(&mut self, char_value: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Compiling character is_alphabetic operation");
        
        let char_int = match char_value {
            BasicValueEnum::IntValue(int_val) => int_val,
            _ => return Err(Error::codegen(
                "Expected integer value for character"
            )),
        };

        // Declare external function for Unicode alphabetic check
        let is_alphabetic_fn = self.declare_unicode_is_alphabetic_function()?;
        
        // Call the external function
        let result = self.builder()
            .build_call(is_alphabetic_fn, &[char_int.into()], "is_alphabetic_call")
            .unwrap()
            .try_as_basic_value()
            .left()
            .unwrap();

        debug!("Generated LLVM code for character is_alphabetic");
        Ok(result)
    }

    #[instrument(skip(self))]
    fn compile_char_is_numeric(&mut self, char_value: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Compiling character is_numeric operation");
        
        let char_int = match char_value {
            BasicValueEnum::IntValue(int_val) => int_val,
            _ => return Err(Error::codegen(
                "Expected integer value for character"
            )),
        };

        // Declare external function for Unicode numeric check
        let is_numeric_fn = self.declare_unicode_is_numeric_function()?;
        
        // Call the external function
        let result = self.builder()
            .build_call(is_numeric_fn, &[char_int.into()], "is_numeric_call")
            .unwrap()
            .try_as_basic_value()
            .left()
            .unwrap();

        debug!("Generated LLVM code for character is_numeric");
        Ok(result)
    }

    #[instrument(skip(self))]
    fn compile_char_is_whitespace(&mut self, char_value: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Compiling character is_whitespace operation");
        
        let char_int = match char_value {
            BasicValueEnum::IntValue(int_val) => int_val,
            _ => return Err(Error::codegen(
                "Expected integer value for character"
            )),
        };

        // Declare external function for Unicode whitespace check
        let is_whitespace_fn = self.declare_unicode_is_whitespace_function()?;
        
        // Call the external function
        let result = self.builder()
            .build_call(is_whitespace_fn, &[char_int.into()], "is_whitespace_call")
            .unwrap()
            .try_as_basic_value()
            .left()
            .unwrap();

        debug!("Generated LLVM code for character is_whitespace");
        Ok(result)
    }

    #[instrument(skip(self))]
    fn compile_char_to_uppercase(&mut self, char_value: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Compiling character to_uppercase operation");
        
        let char_int = match char_value {
            BasicValueEnum::IntValue(int_val) => int_val,
            _ => return Err(Error::codegen(
                "Expected integer value for character"
            )),
        };

        // Declare external function for Unicode uppercase conversion
        let to_uppercase_fn = self.declare_unicode_to_uppercase_function()?;
        
        // Call the external function
        let result = self.builder()
            .build_call(to_uppercase_fn, &[char_int.into()], "to_uppercase_call")
            .unwrap()
            .try_as_basic_value()
            .left()
            .unwrap();

        debug!("Generated LLVM code for character to_uppercase");
        Ok(result)
    }

    #[instrument(skip(self))]
    fn compile_char_to_lowercase(&mut self, char_value: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Compiling character to_lowercase operation");
        
        let char_int = match char_value {
            BasicValueEnum::IntValue(int_val) => int_val,
            _ => return Err(Error::codegen(
                "Expected integer value for character"
            )),
        };

        // Declare external function for Unicode lowercase conversion
        let to_lowercase_fn = self.declare_unicode_to_lowercase_function()?;
        
        // Call the external function
        let result = self.builder()
            .build_call(to_lowercase_fn, &[char_int.into()], "to_lowercase_call")
            .unwrap()
            .try_as_basic_value()
            .left()
            .unwrap();

        debug!("Generated LLVM code for character to_lowercase");
        Ok(result)
    }

    #[instrument(skip(self))]
    fn compile_char_to_string(&mut self, char_value: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Compiling character to_string operation");
        
        let char_int = match char_value {
            BasicValueEnum::IntValue(int_val) => int_val,
            _ => return Err(Error::codegen(
                "Expected integer value for character"
            )),
        };

        // Declare external function for character to string conversion
        let to_string_fn = self.declare_unicode_to_string_function()?;
        
        // Call the external function
        let result = self.builder()
            .build_call(to_string_fn, &[char_int.into()], "to_string_call")
            .unwrap()
            .try_as_basic_value()
            .left()
            .unwrap();

        debug!("Generated LLVM code for character to_string");
        Ok(result)
    }
}

/// Implementation for declaring external Unicode function prototypes
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Declare external function for Unicode uppercase checking
    #[instrument(skip(self))]
    fn declare_unicode_is_uppercase_function(&mut self) -> Result<FunctionValue<'ctx>, Error> {
        let fn_name = "cursed_unicode_is_uppercase";
        
        // Check if function already exists
        if let Some(existing_fn) = self.module().get_function(fn_name) {
            return Ok(existing_fn);
        }

        // i32 -> i1 (character code point -> boolean)
        let i32_type = self.context().i32_type();
        let i1_type = self.context().bool_type();
        let fn_type = i1_type.fn_type(&[i32_type.into()], false);
        
        let function = self.module().add_function(fn_name, fn_type, None);
        debug!("Declared external function: {}", fn_name);
        Ok(function)
    }

    /// Declare external function for Unicode lowercase checking
    #[instrument(skip(self))]
    fn declare_unicode_is_lowercase_function(&mut self) -> Result<FunctionValue<'ctx>, Error> {
        let fn_name = "cursed_unicode_is_lowercase";
        
        if let Some(existing_fn) = self.module().get_function(fn_name) {
            return Ok(existing_fn);
        }

        let i32_type = self.context().i32_type();
        let i1_type = self.context().bool_type();
        let fn_type = i1_type.fn_type(&[i32_type.into()], false);
        
        let function = self.module().add_function(fn_name, fn_type, None);
        debug!("Declared external function: {}", fn_name);
        Ok(function)
    }

    /// Declare external function for Unicode alphabetic checking
    #[instrument(skip(self))]
    fn declare_unicode_is_alphabetic_function(&mut self) -> Result<FunctionValue<'ctx>, Error> {
        let fn_name = "cursed_unicode_is_alphabetic";
        
        if let Some(existing_fn) = self.module().get_function(fn_name) {
            return Ok(existing_fn);
        }

        let i32_type = self.context().i32_type();
        let i1_type = self.context().bool_type();
        let fn_type = i1_type.fn_type(&[i32_type.into()], false);
        
        let function = self.module().add_function(fn_name, fn_type, None);
        debug!("Declared external function: {}", fn_name);
        Ok(function)
    }

    /// Declare external function for Unicode numeric checking
    #[instrument(skip(self))]
    fn declare_unicode_is_numeric_function(&mut self) -> Result<FunctionValue<'ctx>, Error> {
        let fn_name = "cursed_unicode_is_numeric";
        
        if let Some(existing_fn) = self.module().get_function(fn_name) {
            return Ok(existing_fn);
        }

        let i32_type = self.context().i32_type();
        let i1_type = self.context().bool_type();
        let fn_type = i1_type.fn_type(&[i32_type.into()], false);
        
        let function = self.module().add_function(fn_name, fn_type, None);
        debug!("Declared external function: {}", fn_name);
        Ok(function)
    }

    /// Declare external function for Unicode whitespace checking
    #[instrument(skip(self))]
    fn declare_unicode_is_whitespace_function(&mut self) -> Result<FunctionValue<'ctx>, Error> {
        let fn_name = "cursed_unicode_is_whitespace";
        
        if let Some(existing_fn) = self.module().get_function(fn_name) {
            return Ok(existing_fn);
        }

        let i32_type = self.context().i32_type();
        let i1_type = self.context().bool_type();
        let fn_type = i1_type.fn_type(&[i32_type.into()], false);
        
        let function = self.module().add_function(fn_name, fn_type, None);
        debug!("Declared external function: {}", fn_name);
        Ok(function)
    }

    /// Declare external function for Unicode uppercase conversion
    #[instrument(skip(self))]
    fn declare_unicode_to_uppercase_function(&mut self) -> Result<FunctionValue<'ctx>, Error> {
        let fn_name = "cursed_unicode_to_uppercase";
        
        if let Some(existing_fn) = self.module().get_function(fn_name) {
            return Ok(existing_fn);
        }

        // i32 -> i32 (character code point -> character code point)
        let i32_type = self.context().i32_type();
        let fn_type = i32_type.fn_type(&[i32_type.into()], false);
        
        let function = self.module().add_function(fn_name, fn_type, None);
        debug!("Declared external function: {}", fn_name);
        Ok(function)
    }

    /// Declare external function for Unicode lowercase conversion
    #[instrument(skip(self))]
    fn declare_unicode_to_lowercase_function(&mut self) -> Result<FunctionValue<'ctx>, Error> {
        let fn_name = "cursed_unicode_to_lowercase";
        
        if let Some(existing_fn) = self.module().get_function(fn_name) {
            return Ok(existing_fn);
        }

        let i32_type = self.context().i32_type();
        let fn_type = i32_type.fn_type(&[i32_type.into()], false);
        
        let function = self.module().add_function(fn_name, fn_type, None);
        debug!("Declared external function: {}", fn_name);
        Ok(function)
    }

    /// Declare external function for character to string conversion
    #[instrument(skip(self))]
    fn declare_unicode_to_string_function(&mut self) -> Result<FunctionValue<'ctx>, Error> {
        let fn_name = "cursed_unicode_to_string";
        
        if let Some(existing_fn) = self.module().get_function(fn_name) {
            return Ok(existing_fn);
        }

        // i32 -> i8* (character code point -> string pointer)
        let i32_type = self.context().i32_type();
        let i8_ptr_type = self.context().i8_type().ptr_type(inkwell::AddressSpace::default());
        let fn_type = i8_ptr_type.fn_type(&[i32_type.into()], false);
        
        let function = self.module().add_function(fn_name, fn_type, None);
        debug!("Declared external function: {}", fn_name);
        Ok(function)
    }
}
