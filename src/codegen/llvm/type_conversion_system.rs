//! Comprehensive Type Conversion System for CURSED Language
//!
//! This module provides a unified interface for all type conversions in the CURSED language,
//! including explicit casts, implicit conversions, type assertions, and runtime type checking.
//! It integrates with the LLVM code generator to provide efficient, safe type conversions.

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicTypeEnum, BasicType, IntType, FloatType};
use inkwell::values::{BasicValueEnum, IntValue, FloatValue, PointerValue, StructValue};
use inkwell::IntPredicate;
use inkwell::FloatPredicate;
use inkwell::AddressSpace;
use tracing::{instrument, debug, warn, error, info};

use crate::ast::expressions::{TypeConversionExpression, TypeAssertion, TypeAssertionQuestion};
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::expression::ExpressionCompilation;
use crate::codegen::llvm::types::convert_type;
use crate::codegen::llvm::string_type::{CursedStringType, StringTypeUtils};
use crate::core::type_checker::Type;
use crate::error::Error;
use crate::error::type_assertion_error::TypeAssertionError;
use crate::error::SourceLocation;

/// Configuration for type conversion behavior
#[derive(Debug, Clone)]
pub struct ConversionConfig {
    /// Enable implicit conversions (e.g., automatic int->float)
    pub allow_implicit_conversions: bool,
    /// Enable lossy conversions (e.g., float->int truncation)
    pub allow_lossy_conversions: bool,
    /// Enable runtime type checking for interface assertions
    pub enable_runtime_type_checking: bool,
    /// Maximum recursion depth for type conversion chains
    pub max_conversion_depth: usize,
}

impl Default for ConversionConfig {
    fn default() -> Self {
        Self {
            allow_implicit_conversions: true,
            allow_lossy_conversions: false,
            enable_runtime_type_checking: true,
            max_conversion_depth: 10,
        }
    }
}

/// Type conversion capabilities and metadata
#[derive(Debug, Clone, PartialEq)]
pub enum ConversionType {
    /// No conversion needed (same type)
    Identity,
    /// Safe conversion without data loss
    Widening,
    /// Conversion with potential data loss
    Narrowing,
    /// Conversion between fundamentally different types
    Transmutation,
    /// Runtime type assertion for interfaces
    Assertion,
    /// Conversion not possible
    Invalid,
}

/// Unified type conversion system for CURSED language
pub trait TypeConversionSystem<'ctx> {
    /// Compile an explicit type conversion expression
    fn compile_explicit_conversion(
        &mut self,
        type_conv: &TypeConversionExpression,
        config: &ConversionConfig,
    ) -> Result<BasicValueEnum<'ctx>, Error>;

    /// Compile an implicit type conversion
    fn compile_implicit_conversion(
        &mut self,
        value: BasicValueEnum<'ctx>,
        from_type: &Type,
        to_type: &Type,
        config: &ConversionConfig,
    ) -> Result<BasicValueEnum<'ctx>, Error>;

    /// Compile a type assertion for interfaces
    fn compile_type_assertion(
        &mut self,
        type_assertion: &TypeAssertion,
        config: &ConversionConfig,
    ) -> Result<BasicValueEnum<'ctx>, Error>;

    /// Check if a conversion is possible
    fn check_conversion_compatibility(
        &self,
        from_type: &Type,
        to_type: &Type,
    ) -> ConversionType;

    /// Get the cost of a conversion (for overload resolution)
    fn get_conversion_cost(
        &self,
        from_type: &Type,
        to_type: &Type,
    ) -> Option<u32>;

    /// Apply a chain of conversions
    fn apply_conversion_chain(
        &mut self,
        value: BasicValueEnum<'ctx>,
        conversion_chain: &[(Type, Type)],
        config: &ConversionConfig,
    ) -> Result<BasicValueEnum<'ctx>, Error>;
}

impl<'ctx> TypeConversionSystem<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self, type_conv, config), level = "debug")]
    fn compile_explicit_conversion(
        &mut self,
        type_conv: &TypeConversionExpression,
        config: &ConversionConfig,
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Compiling explicit type conversion: {} as {}", 
               type_conv.expression.string(), type_conv.type_name);

        // Compile the source expression
        let source_value = self.compile_expression(type_conv.expression.as_ref())?;
        
        // Parse target type
        let target_type = self.parse_type_name(&type_conv.type_name)?;
        
        // Infer source type from LLVM value
        let source_type = self.infer_cursed_type_from_llvm_value(source_value)?;
        
        // Check conversion compatibility
        let conversion_type = self.check_conversion_compatibility(&source_type, &target_type);
        match conversion_type {
            ConversionType::Invalid => {
                return Err(Error::from(format!(
                    "Invalid type conversion from {:?} to {:?}",
                    source_type, target_type
                )));
            }
            ConversionType::Narrowing if !config.allow_lossy_conversions => {
                return Err(Error::from(format!(
                    "Lossy conversion from {:?} to {:?} not allowed",
                    source_type, target_type
                )));
            }
            _ => {}
        }

        // Perform the conversion
        self.perform_type_conversion(source_value, &source_type, &target_type, config)
    }

    #[instrument(skip(self, value, config), level = "debug")]
    fn compile_implicit_conversion(
        &mut self,
        value: BasicValueEnum<'ctx>,
        from_type: &Type,
        to_type: &Type,
        config: &ConversionConfig,
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Compiling implicit conversion: {:?} -> {:?}", from_type, to_type);

        if !config.allow_implicit_conversions {
            return Err(Error::from("Implicit conversions are disabled"));
        }

        let conversion_type = self.check_conversion_compatibility(from_type, to_type);
        match conversion_type {
            ConversionType::Identity => Ok(value),
            ConversionType::Widening => {
                self.perform_type_conversion(value, from_type, to_type, config)
            }
            ConversionType::Narrowing if config.allow_lossy_conversions => {
                warn!("Performing implicit lossy conversion: {:?} -> {:?}", from_type, to_type);
                self.perform_type_conversion(value, from_type, to_type, config)
            }
            _ => {
                Err(Error::from(format!(
                    "Implicit conversion from {:?} to {:?} not allowed",
                    from_type, to_type
                )))
            }
        }
    }

    #[instrument(skip(self, type_assertion, config), level = "debug")]
    fn compile_type_assertion(
        &mut self,
        type_assertion: &TypeAssertion,
        config: &ConversionConfig,
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Compiling type assertion: {} as {}", 
               type_assertion.expression.string(), type_assertion.type_name);

        if !config.enable_runtime_type_checking {
            return Err(Error::from("Runtime type checking is disabled"));
        }

        // Compile the interface expression
        let interface_value = self.compile_expression(type_assertion.expression.as_ref())?;
        
        // Get target type name
        let target_type_name = &type_assertion.type_name;
        
        // Perform runtime type assertion
        self.perform_runtime_type_assertion(interface_value, &target_type_name, config)
    }

    fn check_conversion_compatibility(
        &self,
        from_type: &Type,
        to_type: &Type,
    ) -> ConversionType {
        if from_type == to_type {
            return ConversionType::Identity;
        }

        match (from_type, to_type) {
            // Integer widening conversions
            (Type::Smol, Type::Mid | Type::Normie | Type::Thicc) => ConversionType::Widening,
            (Type::Mid, Type::Normie | Type::Thicc) => ConversionType::Widening,
            (Type::Normie, Type::Thicc) => ConversionType::Widening,

            // Integer narrowing conversions
            (Type::Thicc, Type::Normie | Type::Mid | Type::Smol) => ConversionType::Narrowing,
            (Type::Normie, Type::Mid | Type::Smol) => ConversionType::Narrowing,
            (Type::Mid, Type::Smol) => ConversionType::Narrowing,

            // Float widening
            (Type::Snack, Type::Meal) => ConversionType::Widening,

            // Float narrowing
            (Type::Meal, Type::Snack) => ConversionType::Narrowing,

            // Integer to float (potentially lossy for large integers)
            (Type::Smol | Type::Mid, Type::Snack | Type::Meal) => ConversionType::Widening,
            (Type::Normie | Type::Thicc, Type::Snack | Type::Meal) => ConversionType::Narrowing,

            // Float to integer (always lossy)
            (Type::Snack | Type::Meal, Type::Smol | Type::Mid | Type::Normie | Type::Thicc) => {
                ConversionType::Narrowing
            }

            // Boolean conversions
            (Type::Lit, Type::Smol | Type::Mid | Type::Normie | Type::Thicc) => ConversionType::Widening,
            (Type::Smol | Type::Mid | Type::Normie | Type::Thicc, Type::Lit) => ConversionType::Narrowing,

            // Character conversions
            (Type::Sip, Type::Rune) => ConversionType::Widening,
            (Type::Rune, Type::Sip) => ConversionType::Narrowing,
            (Type::Sip | Type::Rune, Type::Smol | Type::Mid | Type::Normie | Type::Thicc) => {
                ConversionType::Widening
            }
            (Type::Smol | Type::Mid | Type::Normie | Type::Thicc, Type::Sip | Type::Rune) => {
                ConversionType::Narrowing
            }

            // String and character conversions
            (Type::Sip | Type::Rune, Type::Tea) => ConversionType::Transmutation,
            (Type::Tea, Type::Sip | Type::Rune) => ConversionType::Transmutation,

            // Interface assertions
            (_, _) if matches!(to_type, Type::Interface(_, _)) => ConversionType::Assertion,
            (Type::Interface(_, _), _) => ConversionType::Assertion,

            // Everything else is invalid
            _ => ConversionType::Invalid,
        }
    }

    fn get_conversion_cost(
        &self,
        from_type: &Type,
        to_type: &Type,
    ) -> Option<u32> {
        match self.check_conversion_compatibility(from_type, to_type) {
            ConversionType::Identity => Some(0),
            ConversionType::Widening => Some(1),
            ConversionType::Narrowing => Some(10),
            ConversionType::Transmutation => Some(100),
            ConversionType::Assertion => Some(1000),
            ConversionType::Invalid => None,
        }
    }

    #[instrument(skip(self, value, config), level = "debug")]
    fn apply_conversion_chain(
        &mut self,
        mut value: BasicValueEnum<'ctx>,
        conversion_chain: &[(Type, Type)],
        config: &ConversionConfig,
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        if conversion_chain.len() > config.max_conversion_depth {
            return Err(Error::from(format!(
                "Conversion chain too deep: {} > {}",
                conversion_chain.len(),
                config.max_conversion_depth
            )));
        }

        for (from_type, to_type) in conversion_chain {
            debug!("Applying conversion step: {:?} -> {:?}", from_type, to_type);
            value = self.perform_type_conversion(value, from_type, to_type, config)?;
        }

        Ok(value)
    }
}

impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Core type conversion implementation
    #[instrument(skip(self, value, config), level = "debug")]
    fn perform_type_conversion(
        &mut self,
        value: BasicValueEnum<'ctx>,
        from_type: &Type,
        to_type: &Type,
        config: &ConversionConfig,
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        if from_type == to_type {
            return Ok(value);
        }

        match (from_type, to_type) {
            // Integer conversions
            (Type::Smol | Type::Mid | Type::Normie | Type::Thicc, 
             Type::Smol | Type::Mid | Type::Normie | Type::Thicc) => {
                self.convert_integer_types(value, from_type, to_type)
            }

            // Float conversions
            (Type::Snack | Type::Meal, Type::Snack | Type::Meal) => {
                self.convert_float_types(value, from_type, to_type)
            }

            // Integer to float
            (Type::Smol | Type::Mid | Type::Normie | Type::Thicc, Type::Snack | Type::Meal) => {
                self.convert_integer_to_float(value, from_type, to_type)
            }

            // Float to integer
            (Type::Snack | Type::Meal, Type::Smol | Type::Mid | Type::Normie | Type::Thicc) => {
                self.convert_float_to_integer(value, from_type, to_type)
            }

            // Boolean conversions
            (Type::Lit, _) => self.convert_boolean_to_type(value, to_type),
            (_, Type::Lit) => self.convert_type_to_boolean(value, from_type),

            // Character conversions
            (Type::Sip, Type::Rune) => self.convert_sip_to_rune(value),
            (Type::Rune, Type::Sip) => self.convert_rune_to_sip(value),

            // String and character conversions
            (Type::Sip | Type::Rune, Type::Tea) => self.convert_char_to_string(value, from_type),
            (Type::Tea, Type::Sip | Type::Rune) => self.convert_string_to_char(value, to_type),

            _ => Err(Error::from(format!(
                "Unsupported type conversion: {:?} -> {:?}",
                from_type, to_type
            ))),
        }
    }

    /// Convert between integer types
    fn convert_integer_types(
        &mut self,
        value: BasicValueEnum<'ctx>,
        from_type: &Type,
        to_type: &Type,
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        let int_value = value.into_int_value();
        let from_bits = self.get_integer_bits(from_type)?;
        let to_bits = self.get_integer_bits(to_type)?;

        let result = if to_bits > from_bits {
            // Widening conversion (sign extend)
            self.builder.build_int_s_extend(
                int_value,
                self.get_integer_type(to_type)?,
                "widen_int",
            )?
        } else if to_bits < from_bits {
            // Narrowing conversion (truncate)
            self.builder.build_int_truncate(
                int_value,
                self.get_integer_type(to_type)?,
                "narrow_int",
            )?
        } else {
            // Same size, just return the value
            int_value
        };

        Ok(result.into())
    }

    /// Convert between float types
    fn convert_float_types(
        &mut self,
        value: BasicValueEnum<'ctx>,
        from_type: &Type,
        to_type: &Type,
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        let float_value = value.into_float_value();

        let result = match (from_type, to_type) {
            (Type::Snack, Type::Meal) => {
                // f32 to f64 (widening)
                self.builder.build_float_ext(
                    float_value,
                    self.context.f64_type(),
                    "widen_float",
                )?
            }
            (Type::Meal, Type::Snack) => {
                // f64 to f32 (narrowing)
                self.builder.build_float_trunc(
                    float_value,
                    self.context.f32_type(),
                    "narrow_float",
                )?
            }
            _ => return Err(Error::from("Invalid float conversion")),
        };

        Ok(result.into())
    }

    /// Convert integer to float
    fn convert_integer_to_float(
        &mut self,
        value: BasicValueEnum<'ctx>,
        from_type: &Type,
        to_type: &Type,
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        let int_value = value.into_int_value();
        let float_type = self.get_float_type(to_type)?;

        let result = self.builder.build_signed_int_to_float(
            int_value,
            float_type,
            "int_to_float",
        )?;

        Ok(result.into())
    }

    /// Convert float to integer
    fn convert_float_to_integer(
        &mut self,
        value: BasicValueEnum<'ctx>,
        from_type: &Type,
        to_type: &Type,
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        let float_value = value.into_float_value();
        let int_type = self.get_integer_type(to_type)?;

        let result = self.builder.build_float_to_signed_int(
            float_value,
            int_type,
            "float_to_int",
        )?;

        Ok(result.into())
    }

    /// Convert boolean to another type
    fn convert_boolean_to_type(
        &mut self,
        value: BasicValueEnum<'ctx>,
        to_type: &Type,
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        let bool_value = value.into_int_value();

        match to_type {
            Type::Smol | Type::Mid | Type::Normie | Type::Thicc => {
                let target_type = self.get_integer_type(to_type)?;
                let result = self.builder.build_int_z_extend(bool_value, target_type, "bool_to_int")?;
                Ok(result.into())
            }
            _ => Err(Error::from(format!("Cannot convert boolean to {:?}", to_type))),
        }
    }

    /// Convert type to boolean
    fn convert_type_to_boolean(
        &mut self,
        value: BasicValueEnum<'ctx>,
        from_type: &Type,
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        match from_type {
            Type::Smol | Type::Mid | Type::Normie | Type::Thicc => {
                let int_value = value.into_int_value();
                let zero = int_value.get_type().const_zero();
                let result = self.builder.build_int_compare(
                    IntPredicate::NE,
                    int_value,
                    zero,
                    "int_to_bool",
                )?;
                Ok(result.into())
            }
            Type::Snack | Type::Meal => {
                let float_value = value.into_float_value();
                let zero = float_value.get_type().const_zero();
                let result = self.builder.build_float_compare(
                    FloatPredicate::ONE,
                    float_value,
                    zero,
                    "float_to_bool",
                )?;
                Ok(result.into())
            }
            _ => Err(Error::from(format!("Cannot convert {:?} to boolean", from_type))),
        }
    }

    /// Convert sip (byte) to rune (int32)
    fn convert_sip_to_rune(&mut self, value: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error> {
        let byte_value = value.into_int_value();
        let result = self.builder.build_int_z_extend(
            byte_value,
            self.context.i32_type(),
            "sip_to_rune",
        )?;
        Ok(result.into())
    }

    /// Convert rune (int32) to sip (byte)
    fn convert_rune_to_sip(&mut self, value: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error> {
        let rune_value = value.into_int_value();
        let result = self.builder.build_int_truncate(
            rune_value,
            self.context.i8_type(),
            "rune_to_sip",
        )?;
        Ok(result.into())
    }

    /// Convert character to string
    fn convert_char_to_string(
        &mut self,
        value: BasicValueEnum<'ctx>,
        from_type: &Type,
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        match from_type {
            Type::Sip => {
                let byte_value = value.into_int_value();
                self.create_string_from_byte(byte_value)
            }
            Type::Rune => {
                let rune_value = value.into_int_value();
                self.create_string_from_rune(rune_value)
            }
            _ => Err(Error::from("Invalid character type for string conversion")),
        }
    }

    /// Convert string to character
    fn convert_string_to_char(
        &mut self,
        value: BasicValueEnum<'ctx>,
        to_type: &Type,
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        match to_type {
            Type::Sip => self.get_first_byte_from_string(value),
            Type::Rune => self.get_first_rune_from_string(value),
            _ => Err(Error::from("Invalid target type for character conversion")),
        }
    }

    /// Parse a type name string into a Type enum
    fn parse_type_name(&self, type_name: &str) -> Result<Type, Error> {
        match type_name {
            "smol" => Ok(Type::Smol),
            "mid" => Ok(Type::Mid),
            "normie" => Ok(Type::Normie),
            "thicc" => Ok(Type::Thicc),
            "snack" => Ok(Type::Snack),
            "meal" => Ok(Type::Meal),
            "lit" => Ok(Type::Lit),
            "sip" => Ok(Type::Sip),
            "rune" => Ok(Type::Rune),
            "tea" => Ok(Type::Tea),
            _ => Err(Error::from(format!("Unknown type name: {}", type_name))),
        }
    }

    /// Infer CURSED type from LLVM value
    fn infer_cursed_type_from_llvm_value(&self, value: BasicValueEnum<'ctx>) -> Result<Type, Error> {
        match value {
            BasicValueEnum::IntValue(int_val) => {
                match int_val.get_type().get_bit_width() {
                    1 => Ok(Type::Lit),
                    8 => Ok(Type::Smol),
                    16 => Ok(Type::Mid),
                    32 => Ok(Type::Normie),
                    64 => Ok(Type::Thicc),
                    _ => Err(Error::from("Unknown integer type")),
                }
            }
            BasicValueEnum::FloatValue(float_val) => {
                if float_val.get_type() == self.context.f32_type() {
                    Ok(Type::Snack)
                } else if float_val.get_type() == self.context.f64_type() {
                    Ok(Type::Meal)
                } else {
                    Err(Error::from("Unknown float type"))
                }
            }
            BasicValueEnum::PointerValue(_) => Ok(Type::Tea), // Assume string for now
            BasicValueEnum::StructValue(_) => Ok(Type::Tea), // Assume string struct
            _ => Err(Error::from("Cannot infer type from LLVM value")),
        }
    }

    /// Helper methods for type information
    fn get_integer_bits(&self, typ: &Type) -> Result<u32, Error> {
        match typ {
            Type::Smol => Ok(8),
            Type::Mid => Ok(16),
            Type::Normie => Ok(32),
            Type::Thicc => Ok(64),
            Type::Lit => Ok(1),
            _ => Err(Error::from("Not an integer type")),
        }
    }

    fn get_integer_type(&self, typ: &Type) -> Result<IntType<'ctx>, Error> {
        match typ {
            Type::Smol => Ok(self.context.i8_type()),
            Type::Mid => Ok(self.context.i16_type()),
            Type::Normie => Ok(self.context.i32_type()),
            Type::Thicc => Ok(self.context.i64_type()),
            Type::Lit => Ok(self.context.bool_type()),
            _ => Err(Error::from("Not an integer type")),
        }
    }

    fn get_float_type(&self, typ: &Type) -> Result<FloatType<'ctx>, Error> {
        match typ {
            Type::Snack => Ok(self.context.f32_type()),
            Type::Meal => Ok(self.context.f64_type()),
            _ => Err(Error::from("Not a float type")),
        }
    }

    /// Runtime type assertion implementation
    fn perform_runtime_type_assertion(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
        target_type_name: &str,
        config: &ConversionConfig,
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // This would integrate with the existing type assertion system
        // For now, delegate to the existing implementation
        use crate::codegen::llvm::type_assertion::InterfaceTypeAssertion;
        
        let source_location = None; // Could be passed through from AST
        self.check_instance_of(interface_value, target_type_name, source_location)
    }

    /// String conversion helpers (simplified implementations)
    fn create_string_from_byte(&mut self, byte_value: IntValue<'ctx>) -> Result<BasicValueEnum<'ctx>, Error> {
        // Create a single-byte string
        // For now, return an empty string placeholder - this would need proper string runtime integration
        let string_type = self.context.i8_type().ptr_type(inkwell::AddressSpace::default());
        let null_ptr = string_type.const_null();
        Ok(null_ptr.into())
    }

    fn create_string_from_rune(&mut self, rune_value: IntValue<'ctx>) -> Result<BasicValueEnum<'ctx>, Error> {
        // Create a UTF-8 string from Unicode code point
        // For now, return an empty string placeholder - this would need proper string runtime integration
        let string_type = self.context.i8_type().ptr_type(inkwell::AddressSpace::default());
        let null_ptr = string_type.const_null();
        Ok(null_ptr.into())
    }

    fn get_first_byte_from_string(&mut self, string_value: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error> {
        // Extract the first byte from the string
        // For now, return a zero byte - this would need proper string runtime integration
        let byte_value = self.context.i8_type().const_zero();
        Ok(byte_value.into())
    }

    fn get_first_rune_from_string(&mut self, string_value: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error> {
        // Extract the first Unicode code point from the string
        // For now, return a zero rune - this would need proper string runtime integration
        let rune_value = self.context.i32_type().const_zero();
        Ok(rune_value.into())
    }
}

/// Conversion statistics for performance monitoring
#[derive(Debug, Clone, Default)]
pub struct ConversionStatistics {
    pub explicit_conversions: u64,
    pub implicit_conversions: u64,
    pub type_assertions: u64,
    pub failed_conversions: u64,
    pub total_conversion_time_us: u64,
}

impl ConversionStatistics {
    pub fn record_explicit_conversion(&mut self, duration_us: u64) {
        self.explicit_conversions += 1;
        self.total_conversion_time_us += duration_us;
    }

    pub fn record_implicit_conversion(&mut self, duration_us: u64) {
        self.implicit_conversions += 1;
        self.total_conversion_time_us += duration_us;
    }

    pub fn record_type_assertion(&mut self, duration_us: u64) {
        self.type_assertions += 1;
        self.total_conversion_time_us += duration_us;
    }

    pub fn record_failed_conversion(&mut self) {
        self.failed_conversions += 1;
    }

    pub fn average_conversion_time_us(&self) -> f64 {
        let total_conversions = self.explicit_conversions + self.implicit_conversions + self.type_assertions;
        if total_conversions > 0 {
            self.total_conversion_time_us as f64 / total_conversions as f64
        } else {
            0.0
        }
    }
}
