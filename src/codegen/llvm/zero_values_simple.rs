//! Simple zero value initialization for CURSED types
//!
//! This module provides basic zero value initialization for common types.

use crate::core::type_checker::Type;
use crate::error::Error;
use inkwell::types::{BasicType, BasicTypeEnum};
use inkwell::values::{BasicValue, BasicValueEnum};
use inkwell::AddressSpace;
use super::LlvmCodeGenerator;
use tracing::{debug, instrument};

/// Simple trait for generating zero values in LLVM
pub trait SimpleZeroValueGeneration<'ctx> {
    /// Create a zero value for a given CURSED type
    fn create_simple_zero_value(&self, cursed_type: &Type) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Create a zero value for a given LLVM type
    fn create_simple_zero_value_for_llvm_type(&self, llvm_type: BasicTypeEnum<'ctx>) -> BasicValueEnum<'ctx>;
}

impl<'ctx> SimpleZeroValueGeneration<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self), level = "debug")]
    fn create_simple_zero_value(&self, cursed_type: &Type) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Creating simple zero value for type: {:?}", cursed_type);
        
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
            
            // String type - simple null pointer for now
            Type::Tea => {
                let ptr_type = self.context().i8_type().ptr_type(AddressSpace::default());
                Ok(ptr_type.const_null().into())
            },
            
            // Complex numbers - simple struct with zero values
            Type::Extra => {
                let complex_struct = self.context().struct_type(&[
                    self.context().f64_type().into(),
                    self.context().f64_type().into(),
                ], false);
                
                let zero_real = self.context().f64_type().const_zero();
                let zero_imag = self.context().f64_type().const_zero();
                
                Ok(complex_struct.const_named_struct(&[zero_real.into(), zero_imag.into()]).into())
            },
            
            // Composite types - null pointers for simplicity
            Type::Array(_, _) | Type::Slice(_) | Type::Map(_, _) | Type::Pointer(_) |
            Type::Channel(_) | Type::Function(_, _) | Type::Interface(_, _) |
            Type::Struct(_, _) | Type::Generic(_, _) | Type::Named(_) | Type::TypeParam(_) => {
                let ptr_type = self.context().i8_type().ptr_type(AddressSpace::default());
                Ok(ptr_type.const_null().into())
            },
            
            Type::Unknown => Err(Error::from_str("Cannot create zero value for unknown type")),
        }
    }
    
    fn create_simple_zero_value_for_llvm_type(&self, llvm_type: BasicTypeEnum<'ctx>) -> BasicValueEnum<'ctx> {
        match llvm_type {
            BasicTypeEnum::IntType(int_type) => int_type.const_zero().into(),
            BasicTypeEnum::FloatType(float_type) => float_type.const_zero().into(),
            BasicTypeEnum::PointerType(ptr_type) => ptr_type.const_null().into(),
            BasicTypeEnum::ArrayType(array_type) => {
                // For arrays, just return null pointer for simplicity
                let ptr_type = self.context().i8_type().ptr_type(AddressSpace::default());
                ptr_type.const_null().into()
            },
            BasicTypeEnum::StructType(struct_type) => {
                // For structs, create a simple zero struct
                let field_types = struct_type.get_field_types();
                let zero_fields: Vec<BasicValueEnum> = field_types
                    .iter()
                    .map(|field_type| self.create_simple_zero_value_for_llvm_type(*field_type))
                    .collect();
                
                struct_type.const_named_struct(&zero_fields.iter().map(|v| *v).collect::<Vec<_>>()).into()
            },
            BasicTypeEnum::VectorType(_) => {
                // For vectors, just return null pointer for simplicity
                let ptr_type = self.context().i8_type().ptr_type(AddressSpace::default());
                ptr_type.const_null().into()
            },
        }
    }
}
