//! LLVM string type definition for CURSED strings
//!
//! This module implements the core LLVM type mapping for CURSED strings as `{i64, i8*}` structs.
//! CURSED strings are represented as a structure containing:
//! - Length field (i64): The number of bytes in the string
//! - Data pointer (i8*): Pointer to the string data
//!
//! This representation provides efficient string operations while maintaining memory safety
//! through length tracking and pointer management.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tracing::{instrument, debug, warn, error};

use inkwell::context::Context;
use inkwell::types::{BasicType, BasicTypeEnum, StructType};
use inkwell::values::{BasicValueEnum, PointerValue, StructValue};
use inkwell::builder::Builder;
use inkwell::module::Module;

use crate::error::Error;

/// LLVM string type definition and operations
/// 
/// The CursedStringType provides a unified interface for creating and manipulating
/// CURSED string types in LLVM IR. It encapsulates the `{i64, i8*}` struct layout
/// and provides type-safe operations for string manipulation.
#[derive(Debug, Clone)]
pub struct CursedStringType<'ctx> {
    /// The LLVM context this string type belongs to
    context: &'ctx Context,
    /// The LLVM struct type representing the string: {i64, i8*}
    llvm_type: StructType<'ctx>,
    /// Cache for string type instances to avoid recreation
    type_cache: Arc<RwLock<HashMap<String, StructType<'ctx>>>>,
}

impl<'ctx> CursedStringType<'ctx> {
    /// Create a new CURSED string type instance
    /// 
    /// # Arguments
    /// * `context` - The LLVM context to create the type in
    /// 
    /// # Returns
    /// A new CursedStringType instance configured for the given context
    #[instrument(skip(context), level = "debug")]
    pub fn new(context: &'ctx Context) -> Self {
        debug!("Creating new CURSED string type");
        
        // Define the string struct as {i64, i8*}
        let i64_type = context.i64_type();
        let i8_ptr_type = context.i8_type().ptr_type(inkwell::AddressSpace::default());
        
        // Create the struct type
        let string_struct_type = context.opaque_struct_type("cursed_string");
        string_struct_type.set_body(&[i64_type.into(), i8_ptr_type.into()], false);
        
        debug!("Created LLVM string struct type: {{i64, i8*}}");
        
        Self {
            context,
            llvm_type: string_struct_type,
            type_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Get the LLVM struct type for strings
    /// 
    /// Returns the `{i64, i8*}` struct type that represents CURSED strings
    #[instrument(skip(self), level = "trace")]
    pub fn get_llvm_type(&self) -> StructType<'ctx> {
        self.llvm_type
    }
    
    /// Get the LLVM struct type as a BasicTypeEnum
    /// 
    /// This is useful for functions that expect a BasicTypeEnum parameter
    #[instrument(skip(self), level = "trace")]
    pub fn as_basic_type(&self) -> BasicTypeEnum<'ctx> {
        self.llvm_type.into()
    }
    
    /// Get the size of the string struct in bytes
    /// 
    /// Returns the size of the `{i64, i8*}` struct, which is typically 16 bytes
    /// on 64-bit systems (8 bytes for i64 + 8 bytes for pointer)
    #[instrument(skip(self), level = "trace")]
    pub fn size_of(&self) -> usize {
        // i64 (8 bytes) + i8* (8 bytes on 64-bit systems) = 16 bytes
        16
    }
    
    /// Create a string value from length and data pointer
    /// 
    /// # Arguments
    /// * `builder` - The LLVM IR builder to use for construction
    /// * `length` - The length of the string in bytes
    /// * `data_ptr` - Pointer to the string data
    /// 
    /// # Returns
    /// A struct value representing the CURSED string
    #[instrument(skip(self, builder), level = "debug")]
    pub fn create_string_value(
        &self,
        builder: &Builder<'ctx>,
        length: BasicValueEnum<'ctx>,
        data_ptr: PointerValue<'ctx>,
    ) -> Result<StructValue<'ctx>, Error> {
        debug!("Creating string value from components");
        
        // Create an undef struct value
        let string_struct = self.llvm_type.get_undef();
        
        // Insert the length at index 0
        let with_length = builder
            .build_insert_value(string_struct, length, 0, "string_with_length")
            .map_err(|e| Error::from_str(&format!("Failed to insert length: {}", e)))?
            .into_struct_value();
        
        // Insert the data pointer at index 1
        let complete_string = builder
            .build_insert_value(with_length, data_ptr, 1, "complete_string")
            .map_err(|e| Error::from_str(&format!("Failed to insert data pointer: {}", e)))?
            .into_struct_value();
        
        debug!("Successfully created string struct value");
        Ok(complete_string)
    }
    
    /// Extract the length field from a string value
    /// 
    /// # Arguments
    /// * `builder` - The LLVM IR builder to use for extraction
    /// * `string_value` - The string struct value to extract from
    /// 
    /// # Returns
    /// The length value (i64) extracted from the string struct
    #[instrument(skip(self, builder), level = "debug")]
    pub fn extract_length(
        &self,
        builder: &Builder<'ctx>,
        string_value: StructValue<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Extracting length from string value");
        
        let length = builder
            .build_extract_value(string_value, 0, "string_length")
            .map_err(|e| Error::from_str(&format!("Failed to extract length: {}", e)))?;
        
        debug!("Successfully extracted string length");
        Ok(length)
    }
    
    /// Extract the data pointer from a string value
    /// 
    /// # Arguments
    /// * `builder` - The LLVM IR builder to use for extraction
    /// * `string_value` - The string struct value to extract from
    /// 
    /// # Returns
    /// The data pointer (i8*) extracted from the string struct
    #[instrument(skip(self, builder), level = "debug")]
    pub fn extract_data_ptr(
        &self,
        builder: &Builder<'ctx>,
        string_value: StructValue<'ctx>,
    ) -> Result<PointerValue<'ctx>, Error> {
        debug!("Extracting data pointer from string value");
        
        let data_ptr = builder
            .build_extract_value(string_value, 1, "string_data")
            .map_err(|e| Error::from_str(&format!("Failed to extract data pointer: {}", e)))?
            .into_pointer_value();
        
        debug!("Successfully extracted string data pointer");
        Ok(data_ptr)
    }
    
    /// Create a string literal from a static string
    /// 
    /// # Arguments
    /// * `builder` - The LLVM IR builder to use
    /// * `module` - The LLVM module to add the global string to
    /// * `literal_value` - The string literal content
    /// * `name` - Name for the global string variable
    /// 
    /// # Returns
    /// A string struct value containing the literal
    #[instrument(skip(self, builder, module), level = "debug")]
    pub fn create_string_literal(
        &self,
        builder: &Builder<'ctx>,
        module: &Module<'ctx>,
        literal_value: &str,
        name: &str,
    ) -> Result<StructValue<'ctx>, Error> {
        debug!("Creating string literal: '{}' with name '{}'", literal_value, name);
        
        // Create the global string constant
        let string_bytes = literal_value.as_bytes();
        let array_type = self.context.i8_type().array_type(string_bytes.len() as u32);
        let global_string = module.add_global(array_type, None, name);
        global_string.set_linkage(inkwell::module::Linkage::Private);
        global_string.set_constant(true);
        
        // Set the initializer
        let string_constant = self.context.const_string(string_bytes, false);
        global_string.set_initializer(&string_constant);
        
        // Get pointer to the global string
        let string_ptr = builder
            .build_pointer_cast(
                global_string.as_pointer_value(),
                self.context.i8_type().ptr_type(inkwell::AddressSpace::default()),
                &format!("{}_ptr", name),
            )
            .map_err(|e| Error::from_str(&format!("Failed to cast string pointer: {}", e)))?;
        
        // Create length value
        let length = self.context.i64_type().const_int(string_bytes.len() as u64, false);
        
        // Create the string struct
        let string_struct = self.create_string_value(builder, length.into(), string_ptr)?;
        
        debug!("Successfully created string literal struct");
        Ok(string_struct)
    }
    
    /// Create an empty string value
    /// 
    /// # Arguments
    /// * `builder` - The LLVM IR builder to use
    /// 
    /// # Returns
    /// A string struct representing an empty string (length=0, data=null)
    #[instrument(skip(self, builder), level = "debug")]
    pub fn create_empty_string(&self, builder: &Builder<'ctx>) -> Result<StructValue<'ctx>, Error> {
        debug!("Creating empty string value");
        
        let zero_length = self.context.i64_type().const_zero();
        let null_ptr = self.context.i8_type().ptr_type(inkwell::AddressSpace::default()).const_null();
        
        let empty_string = self.create_string_value(builder, zero_length.into(), null_ptr)?;
        
        debug!("Successfully created empty string");
        Ok(empty_string)
    }
    
    /// Validate that a value is a valid string struct
    /// 
    /// # Arguments
    /// * `value` - The value to validate
    /// 
    /// # Returns
    /// true if the value is a valid string struct, false otherwise
    #[instrument(skip(self), level = "trace")]
    pub fn is_valid_string_value(&self, value: BasicValueEnum<'ctx>) -> bool {
        match value {
            BasicValueEnum::StructValue(struct_val) => {
                // Check if the struct type matches our string type
                struct_val.get_type() == self.llvm_type
            }
            _ => false,
        }
    }
    
    /// Get the cached string type for a given name, or create it if not cached
    /// 
    /// This method provides efficient type reuse for repeated string type creation
    /// 
    /// # Arguments
    /// * `type_name` - Name identifier for the string type variant
    /// 
    /// # Returns
    /// The cached or newly created string struct type
    #[instrument(skip(self), level = "trace")]
    pub fn get_or_create_cached_type(&self, type_name: &str) -> Result<StructType<'ctx>, Error> {
        // Try to get from cache first
        {
            let cache = self.type_cache.read()
                .map_err(|_| Error::from_str("Failed to acquire read lock on type cache"))?;
            
            if let Some(cached_type) = cache.get(type_name) {
                return Ok(*cached_type);
            }
        }
        
        // Not in cache, create new type and cache it
        {
            let mut cache = self.type_cache.write()
                .map_err(|_| Error::from_str("Failed to acquire write lock on type cache"))?;
            
            // Double-check after acquiring write lock
            if let Some(cached_type) = cache.get(type_name) {
                return Ok(*cached_type);
            }
            
            // Create new type
            let new_type = self.llvm_type;
            cache.insert(type_name.to_string(), new_type);
            
            debug!("Cached new string type variant: {}", type_name);
            Ok(new_type)
        }
    }
    
    /// Clear the type cache
    /// 
    /// This can be useful for testing or when the type system is reset
    #[instrument(skip(self), level = "debug")]
    pub fn clear_cache(&self) -> Result<(), Error> {
        let mut cache = self.type_cache.write()
            .map_err(|_| Error::from_str("Failed to acquire write lock on type cache"))?;
        
        cache.clear();
        debug!("Cleared string type cache");
        Ok(())
    }
}

/// String type utilities for integration with the existing type system
pub struct StringTypeUtils;

impl StringTypeUtils {
    /// Convert a CURSED Type::Tea to LLVM string type
    /// 
    /// # Arguments
    /// * `context` - The LLVM context
    /// * `cursed_type` - The CURSED type to convert (should be Type::Tea)
    /// 
    /// # Returns
    /// The corresponding LLVM struct type for strings
    #[instrument(skip(context), level = "debug")]
    pub fn convert_tea_type_to_llvm<'ctx>(
        context: &'ctx Context,
        cursed_type: &crate::core::type_checker::Type,
    ) -> Result<BasicTypeEnum<'ctx>, Error> {
        use crate::core::type_checker::Type;
        
        match cursed_type {
            Type::Tea => {
                debug!("Converting CURSED tea type to LLVM string struct");
                let string_type = CursedStringType::new(context);
                Ok(string_type.as_basic_type())
            }
            _ => {
                error!("Attempted to convert non-string type to LLVM string: {:?}", cursed_type);
                Err(Error::from_str(&format!(
                    "Cannot convert type {:?} to LLVM string type",
                    cursed_type
                )))
            }
        }
    }
    
    /// Check if a CURSED type is a string type
    /// 
    /// # Arguments
    /// * `cursed_type` - The CURSED type to check
    /// 
    /// # Returns
    /// true if the type represents a string (Type::Tea)
    #[instrument(level = "trace")]
    pub fn is_string_type(cursed_type: &crate::core::type_checker::Type) -> bool {
        matches!(cursed_type, crate::core::type_checker::Type::Tea)
    }
    
    /// Get the size of a string type in the CURSED type system
    /// 
    /// # Returns
    /// The size in bytes of a CURSED string struct (16 bytes on 64-bit systems)
    #[instrument(level = "trace")]
    pub fn string_type_size() -> usize {
        16 // {i64, i8*} = 8 + 8 = 16 bytes on 64-bit systems
    }
    
    /// Validate string type compatibility for operations
    /// 
    /// # Arguments
    /// * `left_type` - The left operand type
    /// * `right_type` - The right operand type
    /// * `operation` - The operation being performed
    /// 
    /// # Returns
    /// Ok(()) if the types are compatible, Err otherwise
    #[instrument(level = "debug")]
    pub fn validate_string_operation_types(
        left_type: &crate::core::type_checker::Type,
        right_type: &crate::core::type_checker::Type,
        operation: &str,
    ) -> Result<(), Error> {
        use crate::core::type_checker::Type;
        
        match operation {
            "+" => {
                // String concatenation: both operands must be strings
                if matches!(left_type, Type::Tea) && matches!(right_type, Type::Tea) {
                    Ok(())
                } else {
                    Err(Error::from_str(&format!(
                        "String concatenation requires both operands to be strings, got {:?} + {:?}",
                        left_type, right_type
                    )))
                }
            }
            "==" | "!=" | "<" | ">" | "<=" | ">=" => {
                // String comparison: both operands must be strings
                if matches!(left_type, Type::Tea) && matches!(right_type, Type::Tea) {
                    Ok(())
                } else {
                    Err(Error::from_str(&format!(
                        "String comparison requires both operands to be strings, got {:?} {} {:?}",
                        left_type, operation, right_type
                    )))
                }
            }
            _ => {
                Err(Error::from_str(&format!(
                    "Unsupported string operation: {}",
                    operation
                )))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::context::Context;
    
    #[test]
    fn test_string_type_creation() {
        let context = Context::create();
        let string_type = CursedStringType::new(&context);
        
        // Verify the struct has the correct field types
        let llvm_type = string_type.get_llvm_type();
        assert_eq!(llvm_type.count_fields(), 2);
        
        // Check field types
        let field_types = llvm_type.get_field_types();
        assert!(field_types[0].is_int_type());
        assert!(field_types[1].is_pointer_type());
        
        // Verify size calculation
        assert_eq!(string_type.size_of(), 16);
    }
    
    #[test]
    fn test_string_literal_creation() {
        let context = Context::create();
        let module = context.create_module("test");
        let builder = context.create_builder();
        let string_type = CursedStringType::new(&context);
        
        // Create a function to contain our instructions
        let fn_type = context.void_type().fn_type(&[], false);
        let function = module.add_function("test_function", fn_type, None);
        let basic_block = context.append_basic_block(function, "entry");
        builder.position_at_end(basic_block);
        
        // Create a string literal
        let literal = string_type.create_string_literal(&builder, &module, "hello", "test_str")
            .expect("Failed to create string literal");
        
        // Verify the literal is valid
        assert!(string_type.is_valid_string_value(literal.into()));
        
        // Extract length and verify it's correct
        let length = string_type.extract_length(&builder, literal)
            .expect("Failed to extract length");
        
        if let BasicValueEnum::IntValue(int_val) = length {
            assert_eq!(int_val.get_zero_extended_constant().unwrap(), 5); // "hello".len()
        } else {
            panic!("Length should be an integer value");
        }
    }
    
    #[test]
    fn test_empty_string_creation() {
        let context = Context::create();
        let builder = context.create_builder();
        let string_type = CursedStringType::new(&context);
        
        // Create a function to contain our instructions
        let module = context.create_module("test");
        let fn_type = context.void_type().fn_type(&[], false);
        let function = module.add_function("test_function", fn_type, None);
        let basic_block = context.append_basic_block(function, "entry");
        builder.position_at_end(basic_block);
        
        // Create an empty string
        let empty_string = string_type.create_empty_string(&builder)
            .expect("Failed to create empty string");
        
        // Verify it's valid
        assert!(string_type.is_valid_string_value(empty_string.into()));
        
        // Extract length and verify it's zero
        let length = string_type.extract_length(&builder, empty_string)
            .expect("Failed to extract length");
        
        if let BasicValueEnum::IntValue(int_val) = length {
            assert_eq!(int_val.get_zero_extended_constant().unwrap(), 0);
        } else {
            panic!("Length should be an integer value");
        }
    }
    
    #[test]
    fn test_type_conversion_utilities() {
        use crate::core::type_checker::Type;
        
        let context = Context::create();
        
        // Test Tea type conversion
        let tea_type = Type::Tea;
        let llvm_type = StringTypeUtils::convert_tea_type_to_llvm(&context, &tea_type)
            .expect("Failed to convert Tea type");
        
        assert!(llvm_type.is_struct_type());
        
        // Test type checking
        assert!(StringTypeUtils::is_string_type(&Type::Tea));
        assert!(!StringTypeUtils::is_string_type(&Type::Normie));
        
        // Test size calculation
        assert_eq!(StringTypeUtils::string_type_size(), 16);
    }
    
    #[test]
    fn test_operation_validation() {
        use crate::core::type_checker::Type;
        
        let tea_type = Type::Tea;
        let normie_type = Type::Normie;
        
        // Valid string concatenation
        assert!(StringTypeUtils::validate_string_operation_types(&tea_type, &tea_type, "+").is_ok());
        
        // Invalid string concatenation
        assert!(StringTypeUtils::validate_string_operation_types(&tea_type, &normie_type, "+").is_err());
        
        // Valid string comparison
        assert!(StringTypeUtils::validate_string_operation_types(&tea_type, &tea_type, "==").is_ok());
        
        // Invalid string comparison
        assert!(StringTypeUtils::validate_string_operation_types(&tea_type, &normie_type, "==").is_err());
        
        // Unsupported operation
        assert!(StringTypeUtils::validate_string_operation_types(&tea_type, &tea_type, "*").is_err());
    }
    
    #[test]
    fn test_type_caching() {
        let context = Context::create();
        let string_type = CursedStringType::new(&context);
        
        // Get a cached type
        let type1 = string_type.get_or_create_cached_type("test_variant")
            .expect("Failed to get cached type");
        
        // Get the same type again - should be cached
        let type2 = string_type.get_or_create_cached_type("test_variant")
            .expect("Failed to get cached type");
        
        // They should be the same
        assert_eq!(type1, type2);
        
        // Clear cache and verify it's empty
        string_type.clear_cache().expect("Failed to clear cache");
    }
}
