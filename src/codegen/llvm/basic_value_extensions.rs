//! Extension traits for inkwell BasicValueEnum and related types
//!
//! This module provides extension methods for LLVM value types that are commonly needed
//! but not provided by inkwell directly.

use inkwell::values::{BasicValueEnum, IntValue, PointerValue};
use inkwell::types::{BasicTypeEnum, PointerType, StructType, IntType};
use crate::error::Error;

/// Extension trait for BasicValueEnum providing commonly needed conversion methods
pub trait BasicValueExt<'ctx> {
    /// Tries to convert the value to an IntValue
    fn try_into_int_value(self) -> Result<IntValue<'ctx>, Error>;

    /// Converts the value to an IntValue, panicking if conversion fails
    fn into_int_value(self) -> IntValue<'ctx>;

    /// Checks if a pointer value is null
    fn is_null(&self) -> bool;
}

impl<'ctx> BasicValueExt<'ctx> for BasicValueEnum<'ctx> {
    fn try_into_int_value(self) -> Result<IntValue<'ctx>, Error> {
        match self {
            BasicValueEnum::IntValue(int_val) => Ok(int_val),
            _ => Err(Error::Compilation(format!(
                "Cannot convert {:?} to IntValue", 
                self.get_type()
            ))),
        }
    }

    fn into_int_value(self) -> IntValue<'ctx> {
        self.try_into_int_value().expect("Failed to convert to IntValue")
    }

    fn is_null(&self) -> bool {
        match self {
            BasicValueEnum::PointerValue(ptr) => ptr.is_null(),
            _ => false,
        }
    }
}

/// Extension trait for PointerType providing const_null method
pub trait PointerTypeExt<'ctx> {
    /// Creates a null pointer of this type
    fn const_null(&self) -> PointerValue<'ctx>;
}

impl<'ctx> PointerTypeExt<'ctx> for PointerType<'ctx> {
    fn const_null(&self) -> PointerValue<'ctx> {
        self.const_null()
    }
}

/// Extension trait for StructType providing const_null method
pub trait StructTypeExt<'ctx> {
    /// Creates a null struct value of this type
    fn const_null(&self) -> PointerValue<'ctx>;
}

impl<'ctx> StructTypeExt<'ctx> for StructType<'ctx> {
    fn const_null(&self) -> PointerValue<'ctx> {
        self.ptr_type(inkwell::AddressSpace::default()).const_null()
    }
}

/// Extension trait for boolean values used in LLVM context
pub trait BoolValueExt<'ctx> {
    /// Converts a boolean to an IntValue in the given context
    fn into_int_value(&self, context: &'ctx inkwell::context::Context) -> IntValue<'ctx>;
}

impl<'ctx> BoolValueExt<'ctx> for bool {
    fn into_int_value(&self, context: &'ctx inkwell::context::Context) -> IntValue<'ctx> {
        context.bool_type().const_int(*self as u64, false)
    }
}

/// Extension trait for numeric values used in LLVM context
pub trait NumericValueExt<'ctx> {
    /// Converts a numeric value to an IntValue in the given context
    fn into_int_value(&self, context: &'ctx inkwell::context::Context) -> IntValue<'ctx>;
}

impl<'ctx> NumericValueExt<'ctx> for u64 {
    fn into_int_value(&self, context: &'ctx inkwell::context::Context) -> IntValue<'ctx> {
        context.i64_type().const_int(*self, false)
    }
}

impl<'ctx> NumericValueExt<'ctx> for i64 {
    fn into_int_value(&self, context: &'ctx inkwell::context::Context) -> IntValue<'ctx> {
        context.i64_type().const_int(*self as u64, true)
    }
}

impl<'ctx> NumericValueExt<'ctx> for u32 {
    fn into_int_value(&self, context: &'ctx inkwell::context::Context) -> IntValue<'ctx> {
        context.i32_type().const_int(*self as u64, false)
    }
}

impl<'ctx> NumericValueExt<'ctx> for i32 {
    fn into_int_value(&self, context: &'ctx inkwell::context::Context) -> IntValue<'ctx> {
        context.i32_type().const_int(*self as u64, true)
    }
}
