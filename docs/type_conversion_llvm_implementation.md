# CURSED Type Conversion LLVM Implementation Guide

## 1. Overview

This document provides detailed LLVM implementation strategies for each type conversion in the CURSED language. It covers code generation patterns, optimization techniques, and performance considerations for all conversion categories.

## 2. LLVM Implementation Architecture

### 2.1 Core Module Structure

```rust
// src/codegen/llvm/type_conversion.rs
pub struct LlvmTypeConversionBackend<'ctx> {
    pub context: &'ctx Context,
    pub builder: &'ctx Builder,
    pub module: &'ctx Module,
    pub conversion_cache: HashMap<(Type, Type), FunctionValue<'ctx>>,
    pub runtime_support: RuntimeConversionSupport<'ctx>,
}

impl<'ctx> LlvmTypeConversionBackend<'ctx> {
    pub fn convert(&mut self, 
                   value: BasicValueEnum<'ctx>, 
                   source_type: &Type, 
                   target_type: &Type) -> Result<BasicValueEnum<'ctx>, String> {
        match self.get_conversion_strategy(source_type, target_type) {
            ConversionStrategy::ZeroCost => self.zero_cost_conversion(value, target_type),
            ConversionStrategy::Truncation => self.truncation_conversion(value, source_type, target_type),
            ConversionStrategy::Extension => self.extension_conversion(value, source_type, target_type),
            ConversionStrategy::FloatToInt => self.float_to_int_conversion(value, source_type, target_type),
            ConversionStrategy::IntToFloat => self.int_to_float_conversion(value, source_type, target_type),
            ConversionStrategy::Complex => self.complex_conversion(value, source_type, target_type),
            ConversionStrategy::Runtime => self.runtime_conversion(value, source_type, target_type),
        }
    }
}
```

### 2.2 Conversion Strategy Classification

```rust
#[derive(Debug, Clone, Copy)]
pub enum ConversionStrategy {
    ZeroCost,       // No-op or bitcast
    Truncation,     // Losing precision/bits
    Extension,      // Preserving precision/bits
    FloatToInt,     // Float to integer with rounding
    IntToFloat,     // Integer to float with precision
    Complex,        // Multi-step conversions
    Runtime,        // Requires runtime support
}

impl ConversionStrategy {
    pub fn for_types(source: &Type, target: &Type) -> Self {
        use Type::*;
        match (source, target) {
            // Zero-cost conversions
            (Sip, Rune) | (Rune, Sip) => Self::ZeroCost,
            (Byte, Smol) | (Smol, Byte) => Self::ZeroCost,
            
            // Truncation conversions
            (Thicc, Normie) | (Thicc, Mid) | (Thicc, Smol) => Self::Truncation,
            (Normie, Mid) | (Normie, Smol) => Self::Truncation,
            (Mid, Smol) => Self::Truncation,
            (Meal, Snack) => Self::Truncation,
            
            // Extension conversions
            (Smol, Mid) | (Smol, Normie) | (Smol, Thicc) => Self::Extension,
            (Mid, Normie) | (Mid, Thicc) => Self::Extension,
            (Normie, Thicc) => Self::Extension,
            (Snack, Meal) => Self::Extension,
            
            // Float-to-int conversions
            (Snack, Smol) | (Snack, Mid) | (Snack, Normie) | (Snack, Thicc) => Self::FloatToInt,
            (Meal, Smol) | (Meal, Mid) | (Meal, Normie) | (Meal, Thicc) => Self::FloatToInt,
            
            // Int-to-float conversions
            (Smol, Snack) | (Smol, Meal) => Self::IntToFloat,
            (Mid, Snack) | (Mid, Meal) => Self::IntToFloat,
            (Normie, Snack) | (Normie, Meal) => Self::IntToFloat,
            (Thicc, Snack) | (Thicc, Meal) => Self::IntToFloat,
            
            // Complex conversions
            (Array(box Byte, _), Tea) | (Tea, Array(box Byte, _)) => Self::Complex,
            (Slice(box Byte), Tea) | (Tea, Slice(box Byte)) => Self::Complex,
            
            // Runtime conversions
            _ => Self::Runtime,
        }
    }
}
```

## 3. Primitive Type Conversion Implementations

### 3.1 Zero-Cost Conversions

```rust
impl<'ctx> LlvmTypeConversionBackend<'ctx> {
    pub fn zero_cost_conversion(&self, 
                               value: BasicValueEnum<'ctx>, 
                               target_type: &Type) -> Result<BasicValueEnum<'ctx>, String> {
        let target_llvm_type = convert_type(self.context, target_type)?;
        
        // Check if types are already the same
        if value.get_type() == target_llvm_type {
            return Ok(value);
        }
        
        // Use bitcast for compatible types
        match value {
            BasicValueEnum::IntValue(int_val) => {
                if let BasicTypeEnum::IntType(target_int_type) = target_llvm_type {
                    if int_val.get_type().get_bit_width() == target_int_type.get_bit_width() {
                        Ok(self.builder.build_bitcast(int_val, target_int_type, "zero_cost_cast")
                            .map_err(|e| format!("Bitcast failed: {}", e))?.into())
                    } else {
                        Err("Cannot perform zero-cost conversion between different bit widths".to_string())
                    }
                } else {
                    Err("Target type is not an integer for zero-cost conversion".to_string())
                }
            }
            _ => Err("Unsupported value type for zero-cost conversion".to_string()),
        }
    }
}
```

### 3.2 Integer Truncation Conversions

```rust
impl<'ctx> LlvmTypeConversionBackend<'ctx> {
    pub fn truncation_conversion(&self, 
                                value: BasicValueEnum<'ctx>, 
                                source_type: &Type, 
                                target_type: &Type) -> Result<BasicValueEnum<'ctx>, String> {
        let int_value = value.into_int_value();
        let target_llvm_type = convert_type(self.context, target_type)?;
        
        if let BasicTypeEnum::IntType(target_int_type) = target_llvm_type {
            // Add overflow checking if enabled
            if self.should_check_overflow() {
                self.add_overflow_check(int_value, source_type, target_type)?;
            }
            
            let truncated = self.builder.build_int_truncate(
                int_value, 
                target_int_type, 
                &format!("trunc_{}_{}", self.type_name(source_type), self.type_name(target_type))
            ).map_err(|e| format!("Integer truncation failed: {}", e))?;
            
            Ok(truncated.into())
        } else {
            Err("Target type is not an integer for truncation conversion".to_string())
        }
    }

    fn add_overflow_check(&self, 
                         value: IntValue<'ctx>, 
                         source_type: &Type, 
                         target_type: &Type) -> Result<(), String> {
        // Generate overflow checking code
        let source_bits = self.get_type_bit_width(source_type);
        let target_bits = self.get_type_bit_width(target_type);
        
        if source_bits <= target_bits {
            return Ok(()); // No overflow possible
        }
        
        // Check if value fits in target range
        let max_val = self.context.const_int(
            self.context.custom_width_int_type(source_bits as u32),
            (1u64 << (target_bits - 1)) - 1,
            false
        );
        let min_val = self.context.const_int(
            self.context.custom_width_int_type(source_bits as u32),
            -(1i64 << (target_bits - 1)) as u64,
            true
        );
        
        let in_range = self.builder.build_and(
            self.builder.build_int_compare(inkwell::IntPredicate::SLE, value, max_val, "max_check")
                .map_err(|e| format!("Max value check failed: {}", e))?,
            self.builder.build_int_compare(inkwell::IntPredicate::SGE, value, min_val, "min_check")
                .map_err(|e| format!("Min value check failed: {}", e))?,
            "range_check"
        ).map_err(|e| format!("Range check failed: {}", e))?;
        
        // Call runtime overflow handler if out of range
        let overflow_fn = self.get_or_create_overflow_handler();
        let current_bb = self.builder.get_insert_block().unwrap();
        let overflow_bb = self.context.append_basic_block(current_bb.get_parent().unwrap(), "overflow");
        let continue_bb = self.context.append_basic_block(current_bb.get_parent().unwrap(), "continue");
        
        self.builder.build_conditional_branch(in_range, continue_bb, overflow_bb)
            .map_err(|e| format!("Conditional branch failed: {}", e))?;
        
        // Overflow block
        self.builder.position_at_end(overflow_bb);
        self.builder.build_call(overflow_fn, &[], "overflow_call")
            .map_err(|e| format!("Overflow call failed: {}", e))?;
        self.builder.build_unreachable()
            .map_err(|e| format!("Unreachable instruction failed: {}", e))?;
        
        // Continue block
        self.builder.position_at_end(continue_bb);
        
        Ok(())
    }
}
```

### 3.3 Integer Extension Conversions

```rust
impl<'ctx> LlvmTypeConversionBackend<'ctx> {
    pub fn extension_conversion(&self, 
                               value: BasicValueEnum<'ctx>, 
                               source_type: &Type, 
                               target_type: &Type) -> Result<BasicValueEnum<'ctx>, String> {
        let int_value = value.into_int_value();
        let target_llvm_type = convert_type(self.context, target_type)?;
        
        if let BasicTypeEnum::IntType(target_int_type) = target_llvm_type {
            let is_signed = self.is_signed_type(source_type);
            
            let extended = if is_signed {
                self.builder.build_int_s_extend(
                    int_value, 
                    target_int_type, 
                    &format!("sext_{}_{}", self.type_name(source_type), self.type_name(target_type))
                ).map_err(|e| format!("Signed extension failed: {}", e))?
            } else {
                self.builder.build_int_z_extend(
                    int_value, 
                    target_int_type, 
                    &format!("zext_{}_{}", self.type_name(source_type), self.type_name(target_type))
                ).map_err(|e| format!("Zero extension failed: {}", e))?
            };
            
            Ok(extended.into())
        } else {
            Err("Target type is not an integer for extension conversion".to_string())
        }
    }

    fn is_signed_type(&self, ty: &Type) -> bool {
        matches!(ty, Type::Smol | Type::Mid | Type::Normie | Type::Thicc | Type::Sip | Type::Rune)
    }
}
```

### 3.4 Float-to-Integer Conversions

```rust
impl<'ctx> LlvmTypeConversionBackend<'ctx> {
    pub fn float_to_int_conversion(&self, 
                                  value: BasicValueEnum<'ctx>, 
                                  source_type: &Type, 
                                  target_type: &Type) -> Result<BasicValueEnum<'ctx>, String> {
        let float_value = value.into_float_value();
        let target_llvm_type = convert_type(self.context, target_type)?;
        
        if let BasicTypeEnum::IntType(target_int_type) = target_llvm_type {
            let is_signed = self.is_signed_type(target_type);
            
            // Add range checking for float-to-int conversion
            if self.should_check_float_conversion() {
                self.add_float_to_int_range_check(float_value, source_type, target_type)?;
            }
            
            let converted = if is_signed {
                self.builder.build_float_to_signed_int(
                    float_value, 
                    target_int_type, 
                    &format!("fptosi_{}_{}", self.type_name(source_type), self.type_name(target_type))
                ).map_err(|e| format!("Float to signed int conversion failed: {}", e))?
            } else {
                self.builder.build_float_to_unsigned_int(
                    float_value, 
                    target_int_type, 
                    &format!("fptoui_{}_{}", self.type_name(source_type), self.type_name(target_type))
                ).map_err(|e| format!("Float to unsigned int conversion failed: {}", e))?
            };
            
            Ok(converted.into())
        } else {
            Err("Target type is not an integer for float-to-int conversion".to_string())
        }
    }

    fn add_float_to_int_range_check(&self, 
                                   value: FloatValue<'ctx>, 
                                   source_type: &Type, 
                                   target_type: &Type) -> Result<(), String> {
        let target_bits = self.get_type_bit_width(target_type);
        let is_signed = self.is_signed_type(target_type);
        
        let (min_val, max_val) = if is_signed {
            let min = -(1i64 << (target_bits - 1)) as f64;
            let max = ((1u64 << (target_bits - 1)) - 1) as f64;
            (min, max)
        } else {
            let min = 0.0f64;
            let max = ((1u64 << target_bits) - 1) as f64;
            (min, max)
        };
        
        let float_type = value.get_type();
        let min_const = float_type.const_float(min);
        let max_const = float_type.const_float(max);
        
        let in_range = self.builder.build_and(
            self.builder.build_float_compare(
                inkwell::FloatPredicate::OGE, value, min_const, "min_check"
            ).map_err(|e| format!("Float min check failed: {}", e))?,
            self.builder.build_float_compare(
                inkwell::FloatPredicate::OLE, value, max_const, "max_check"
            ).map_err(|e| format!("Float max check failed: {}", e))?,
            "range_check"
        ).map_err(|e| format!("Float range check failed: {}", e))?;
        
        // Check for NaN
        let not_nan = self.builder.build_float_compare(
            inkwell::FloatPredicate::ORD, value, value, "nan_check"
        ).map_err(|e| format!("NaN check failed: {}", e))?;
        
        let valid = self.builder.build_and(in_range, not_nan, "valid_check")
            .map_err(|e| format!("Valid check failed: {}", e))?;
        
        // Branch to error handler if invalid
        let current_bb = self.builder.get_insert_block().unwrap();
        let error_bb = self.context.append_basic_block(current_bb.get_parent().unwrap(), "float_error");
        let continue_bb = self.context.append_basic_block(current_bb.get_parent().unwrap(), "continue");
        
        self.builder.build_conditional_branch(valid, continue_bb, error_bb)
            .map_err(|e| format!("Conditional branch failed: {}", e))?;
        
        // Error block
        self.builder.position_at_end(error_bb);
        let error_fn = self.get_or_create_float_conversion_error_handler();
        self.builder.build_call(error_fn, &[], "float_error_call")
            .map_err(|e| format!("Float error call failed: {}", e))?;
        self.builder.build_unreachable()
            .map_err(|e| format!("Unreachable instruction failed: {}", e))?;
        
        // Continue block
        self.builder.position_at_end(continue_bb);
        
        Ok(())
    }
}
```

### 3.5 Integer-to-Float Conversions

```rust
impl<'ctx> LlvmTypeConversionBackend<'ctx> {
    pub fn int_to_float_conversion(&self, 
                                  value: BasicValueEnum<'ctx>, 
                                  source_type: &Type, 
                                  target_type: &Type) -> Result<BasicValueEnum<'ctx>, String> {
        let int_value = value.into_int_value();
        let target_llvm_type = convert_type(self.context, target_type)?;
        
        if let BasicTypeEnum::FloatType(target_float_type) = target_llvm_type {
            let is_signed = self.is_signed_type(source_type);
            
            // Check for precision loss warnings
            if self.should_warn_precision_loss() {
                self.check_precision_loss(int_value, source_type, target_type)?;
            }
            
            let converted = if is_signed {
                self.builder.build_signed_int_to_float(
                    int_value, 
                    target_float_type, 
                    &format!("sitofp_{}_{}", self.type_name(source_type), self.type_name(target_type))
                ).map_err(|e| format!("Signed int to float conversion failed: {}", e))?
            } else {
                self.builder.build_unsigned_int_to_float(
                    int_value, 
                    target_float_type, 
                    &format!("uitofp_{}_{}", self.type_name(source_type), self.type_name(target_type))
                ).map_err(|e| format!("Unsigned int to float conversion failed: {}", e))?
            };
            
            Ok(converted.into())
        } else {
            Err("Target type is not a float for int-to-float conversion".to_string())
        }
    }

    fn check_precision_loss(&self, 
                           value: IntValue<'ctx>, 
                           source_type: &Type, 
                           target_type: &Type) -> Result<(), String> {
        // For i64 -> f32, warn about potential precision loss
        if matches!((source_type, target_type), (Type::Thicc, Type::Snack)) {
            // Check if the integer value is too large to represent exactly in f32
            let mantissa_bits = 23; // f32 mantissa
            let max_exact = 1u64 << mantissa_bits;
            
            let abs_value = self.builder.build_select(
                self.builder.build_int_compare(
                    inkwell::IntPredicate::SGE, 
                    value, 
                    value.get_type().const_zero(), 
                    "is_positive"
                ).map_err(|e| format!("Sign check failed: {}", e))?,
                value,
                self.builder.build_int_neg(value, "negate")
                    .map_err(|e| format!("Negation failed: {}", e))?,
                "abs_value"
            ).map_err(|e| format!("Select failed: {}", e))?;
            
            let max_exact_const = value.get_type().const_int(max_exact, false);
            let may_lose_precision = self.builder.build_int_compare(
                inkwell::IntPredicate::UGT, 
                abs_value.into_int_value(), 
                max_exact_const, 
                "precision_check"
            ).map_err(|e| format!("Precision check failed: {}", e))?;
            
            // Emit warning if precision may be lost
            let current_bb = self.builder.get_insert_block().unwrap();
            let warn_bb = self.context.append_basic_block(current_bb.get_parent().unwrap(), "precision_warn");
            let continue_bb = self.context.append_basic_block(current_bb.get_parent().unwrap(), "continue");
            
            self.builder.build_conditional_branch(may_lose_precision, warn_bb, continue_bb)
                .map_err(|e| format!("Conditional branch failed: {}", e))?;
            
            // Warning block
            self.builder.position_at_end(warn_bb);
            let warn_fn = self.get_or_create_precision_warning_handler();
            self.builder.build_call(warn_fn, &[], "precision_warn_call")
                .map_err(|e| format!("Precision warning call failed: {}", e))?;
            self.builder.build_unconditional_branch(continue_bb)
                .map_err(|e| format!("Unconditional branch failed: {}", e))?;
            
            // Continue block
            self.builder.position_at_end(continue_bb);
        }
        
        Ok(())
    }
}
```

## 4. Complex Type Conversion Implementations

### 4.1 String to Byte Array Conversion

```rust
impl<'ctx> LlvmTypeConversionBackend<'ctx> {
    pub fn string_to_byte_array(&self, 
                               value: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, String> {
        let string_struct = value.into_struct_value();
        
        // Extract length and data pointer from string struct {len, data}
        let len = self.builder.build_extract_value(string_struct, 0, "str_len")
            .map_err(|e| format!("String length extraction failed: {}", e))?
            .into_int_value();
        let data_ptr = self.builder.build_extract_value(string_struct, 1, "str_data")
            .map_err(|e| format!("String data extraction failed: {}", e))?
            .into_pointer_value();
        
        // Allocate byte array
        let byte_array_type = self.context.i8_type().array_type(0); // Dynamic size
        let alloca = self.builder.build_array_alloca(
            self.context.i8_type(), 
            len, 
            "byte_array_alloc"
        ).map_err(|e| format!("Byte array allocation failed: {}", e))?;
        
        // Copy string data to byte array
        let copy_fn = self.get_or_create_memcpy_function();
        self.builder.build_call(
            copy_fn, 
            &[
                alloca.into(), 
                data_ptr.into(), 
                len.into()
            ], 
            "memcpy_call"
        ).map_err(|e| format!("Memory copy failed: {}", e))?;
        
        // Create result struct {array_ptr, length}
        let result_type = self.context.struct_type(&[
            self.context.i8_type().ptr_type(inkwell::AddressSpace::default()).into(),
            self.context.i64_type().into(),
        ], false);
        
        let result = result_type.const_zero();
        let result = self.builder.build_insert_value(result, alloca, 0, "insert_ptr")
            .map_err(|e| format!("Insert pointer failed: {}", e))?;
        let result = self.builder.build_insert_value(result, len, 1, "insert_len")
            .map_err(|e| format!("Insert length failed: {}", e))?;
        
        Ok(result.into())
    }
}
```

### 4.2 Byte Array to String Conversion

```rust
impl<'ctx> LlvmTypeConversionBackend<'ctx> {
    pub fn byte_array_to_string(&self, 
                               value: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, String> {
        let array_struct = value.into_struct_value();
        
        // Extract array pointer and length
        let array_ptr = self.builder.build_extract_value(array_struct, 0, "array_ptr")
            .map_err(|e| format!("Array pointer extraction failed: {}", e))?
            .into_pointer_value();
        let array_len = self.builder.build_extract_value(array_struct, 1, "array_len")
            .map_err(|e| format!("Array length extraction failed: {}", e))?
            .into_int_value();
        
        // Validate UTF-8 if required
        if self.should_validate_utf8() {
            self.validate_utf8_bytes(array_ptr, array_len)?;
        }
        
        // Allocate string buffer
        let string_alloc_fn = self.get_or_create_string_alloc_function();
        let string_ptr = self.builder.build_call(
            string_alloc_fn, 
            &[array_len.into()], 
            "string_alloc"
        ).map_err(|e| format!("String allocation failed: {}", e))?
            .try_as_basic_value()
            .left()
            .unwrap()
            .into_pointer_value();
        
        // Copy bytes to string buffer
        let copy_fn = self.get_or_create_memcpy_function();
        self.builder.build_call(
            copy_fn, 
            &[
                string_ptr.into(), 
                array_ptr.into(), 
                array_len.into()
            ], 
            "string_copy"
        ).map_err(|e| format!("String copy failed: {}", e))?;
        
        // Create string struct {len, data}
        let string_type = self.context.struct_type(&[
            self.context.i64_type().into(),
            self.context.i8_type().ptr_type(inkwell::AddressSpace::default()).into(),
        ], false);
        
        let result = string_type.const_zero();
        let result = self.builder.build_insert_value(result, array_len, 0, "insert_len")
            .map_err(|e| format!("Insert string length failed: {}", e))?;
        let result = self.builder.build_insert_value(result, string_ptr, 1, "insert_data")
            .map_err(|e| format!("Insert string data failed: {}", e))?;
        
        Ok(result.into())
    }

    fn validate_utf8_bytes(&self, 
                          bytes_ptr: PointerValue<'ctx>, 
                          len: IntValue<'ctx>) -> Result<(), String> {
        let utf8_validate_fn = self.get_or_create_utf8_validate_function();
        let is_valid = self.builder.build_call(
            utf8_validate_fn, 
            &[bytes_ptr.into(), len.into()], 
            "utf8_validate"
        ).map_err(|e| format!("UTF-8 validation call failed: {}", e))?
            .try_as_basic_value()
            .left()
            .unwrap()
            .into_int_value();
        
        let current_bb = self.builder.get_insert_block().unwrap();
        let error_bb = self.context.append_basic_block(current_bb.get_parent().unwrap(), "utf8_error");
        let continue_bb = self.context.append_basic_block(current_bb.get_parent().unwrap(), "continue");
        
        let is_valid_bool = self.builder.build_int_compare(
            inkwell::IntPredicate::NE, 
            is_valid, 
            self.context.bool_type().const_zero(), 
            "is_valid_bool"
        ).map_err(|e| format!("Boolean conversion failed: {}", e))?;
        
        self.builder.build_conditional_branch(is_valid_bool, continue_bb, error_bb)
            .map_err(|e| format!("Conditional branch failed: {}", e))?;
        
        // Error block
        self.builder.position_at_end(error_bb);
        let error_fn = self.get_or_create_utf8_error_handler();
        self.builder.build_call(error_fn, &[], "utf8_error_call")
            .map_err(|e| format!("UTF-8 error call failed: {}", e))?;
        self.builder.build_unreachable()
            .map_err(|e| format!("Unreachable instruction failed: {}", e))?;
        
        // Continue block
        self.builder.position_at_end(continue_bb);
        
        Ok(())
    }
}
```

## 5. Runtime Support Functions

### 5.1 Runtime Function Management

```rust
impl<'ctx> LlvmTypeConversionBackend<'ctx> {
    fn get_or_create_overflow_handler(&self) -> FunctionValue<'ctx> {
        if let Some(func) = self.module.get_function("cursed_overflow_handler") {
            func
        } else {
            let fn_type = self.context.void_type().fn_type(&[], false);
            self.module.add_function("cursed_overflow_handler", fn_type, None)
        }
    }

    fn get_or_create_float_conversion_error_handler(&self) -> FunctionValue<'ctx> {
        if let Some(func) = self.module.get_function("cursed_float_conversion_error") {
            func
        } else {
            let fn_type = self.context.void_type().fn_type(&[], false);
            self.module.add_function("cursed_float_conversion_error", fn_type, None)
        }
    }

    fn get_or_create_precision_warning_handler(&self) -> FunctionValue<'ctx> {
        if let Some(func) = self.module.get_function("cursed_precision_warning") {
            func
        } else {
            let fn_type = self.context.void_type().fn_type(&[], false);
            self.module.add_function("cursed_precision_warning", fn_type, None)
        }
    }

    fn get_or_create_memcpy_function(&self) -> FunctionValue<'ctx> {
        if let Some(func) = self.module.get_function("llvm.memcpy.p0i8.p0i8.i64") {
            func
        } else {
            let i8_ptr_type = self.context.i8_type().ptr_type(inkwell::AddressSpace::default());
            let fn_type = self.context.void_type().fn_type(&[
                i8_ptr_type.into(),
                i8_ptr_type.into(),
                self.context.i64_type().into(),
                self.context.bool_type().into(), // is_volatile
            ], false);
            self.module.add_function("llvm.memcpy.p0i8.p0i8.i64", fn_type, None)
        }
    }

    fn get_or_create_string_alloc_function(&self) -> FunctionValue<'ctx> {
        if let Some(func) = self.module.get_function("cursed_string_alloc") {
            func
        } else {
            let i8_ptr_type = self.context.i8_type().ptr_type(inkwell::AddressSpace::default());
            let fn_type = i8_ptr_type.fn_type(&[
                self.context.i64_type().into(), // size
            ], false);
            self.module.add_function("cursed_string_alloc", fn_type, None)
        }
    }

    fn get_or_create_utf8_validate_function(&self) -> FunctionValue<'ctx> {
        if let Some(func) = self.module.get_function("cursed_utf8_validate") {
            func
        } else {
            let i8_ptr_type = self.context.i8_type().ptr_type(inkwell::AddressSpace::default());
            let fn_type = self.context.bool_type().fn_type(&[
                i8_ptr_type.into(), // bytes
                self.context.i64_type().into(), // length
            ], false);
            self.module.add_function("cursed_utf8_validate", fn_type, None)
        }
    }

    fn get_or_create_utf8_error_handler(&self) -> FunctionValue<'ctx> {
        if let Some(func) = self.module.get_function("cursed_utf8_error") {
            func
        } else {
            let fn_type = self.context.void_type().fn_type(&[], false);
            self.module.add_function("cursed_utf8_error", fn_type, None)
        }
    }
}
```

## 6. Optimization Strategies

### 6.1 Conversion Function Caching

```rust
impl<'ctx> LlvmTypeConversionBackend<'ctx> {
    pub fn get_cached_conversion(&mut self, 
                                source_type: &Type, 
                                target_type: &Type) -> Option<FunctionValue<'ctx>> {
        self.conversion_cache.get(&(source_type.clone(), target_type.clone())).copied()
    }

    pub fn cache_conversion(&mut self, 
                           source_type: Type, 
                           target_type: Type, 
                           function: FunctionValue<'ctx>) {
        self.conversion_cache.insert((source_type, target_type), function);
    }

    pub fn create_conversion_function(&mut self, 
                                     source_type: &Type, 
                                     target_type: &Type) -> Result<FunctionValue<'ctx>, String> {
        let source_llvm_type = convert_type(self.context, source_type)?;
        let target_llvm_type = convert_type(self.context, target_type)?;
        
        let fn_name = format!("convert_{}_{}", 
                             self.type_name(source_type), 
                             self.type_name(target_type));
        
        let fn_type = target_llvm_type.fn_type(&[source_llvm_type.into()], false);
        let function = self.module.add_function(&fn_name, fn_type, None);
        
        // Create entry block
        let entry_bb = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry_bb);
        
        // Get function parameter
        let param = function.get_first_param().unwrap();
        
        // Generate conversion code
        let result = self.convert(param, source_type, target_type)?;
        
        // Return result
        self.builder.build_return(Some(&result))
            .map_err(|e| format!("Return instruction failed: {}", e))?;
        
        // Cache and return
        self.cache_conversion(source_type.clone(), target_type.clone(), function);
        Ok(function)
    }
}
```

### 6.2 Constant Conversion Optimization

```rust
impl<'ctx> LlvmTypeConversionBackend<'ctx> {
    pub fn optimize_constant_conversion(&self, 
                                       value: BasicValueEnum<'ctx>, 
                                       source_type: &Type, 
                                       target_type: &Type) -> Option<BasicValueEnum<'ctx>> {
        // Check if value is a constant
        if !self.is_constant_value(value) {
            return None;
        }
        
        match (source_type, target_type) {
            // Constant integer conversions
            (Type::Normie, Type::Smol) => {
                if let Some(const_val) = value.into_int_value().get_sign_extended_constant() {
                    if const_val >= i8::MIN as i64 && const_val <= i8::MAX as i64 {
                        Some(self.context.i8_type().const_int(const_val as u64, true).into())
                    } else {
                        None // Would overflow, let runtime handle it
                    }
                } else {
                    None
                }
            }
            
            // Constant float conversions
            (Type::Meal, Type::Snack) => {
                if let Some(const_val) = value.into_float_value().get_constant() {
                    Some(self.context.f32_type().const_float(const_val).into())
                } else {
                    None
                }
            }
            
            // Add more constant optimization cases as needed
            _ => None,
        }
    }

    fn is_constant_value(&self, value: BasicValueEnum<'ctx>) -> bool {
        match value {
            BasicValueEnum::IntValue(iv) => iv.is_const(),
            BasicValueEnum::FloatValue(fv) => fv.is_const(),
            BasicValueEnum::PointerValue(pv) => pv.is_const(),
            BasicValueEnum::StructValue(sv) => sv.is_const(),
            BasicValueEnum::ArrayValue(av) => av.is_const(),
            BasicValueEnum::VectorValue(vv) => vv.is_const(),
        }
    }
}
```

## 7. Performance Considerations

### 7.1 SIMD Optimizations

```rust
impl<'ctx> LlvmTypeConversionBackend<'ctx> {
    pub fn vectorized_conversion(&self, 
                                values: &[BasicValueEnum<'ctx>], 
                                source_type: &Type, 
                                target_type: &Type) -> Result<Vec<BasicValueEnum<'ctx>>, String> {
        // Check if vectorization is beneficial
        if values.len() < 4 || !self.supports_vectorization(source_type, target_type) {
            return self.scalar_batch_conversion(values, source_type, target_type);
        }
        
        let vector_size = self.get_optimal_vector_size(source_type, target_type);
        let mut results = Vec::new();
        
        for chunk in values.chunks(vector_size) {
            if chunk.len() == vector_size {
                // Full vector conversion
                let vector_result = self.convert_vector(chunk, source_type, target_type)?;
                results.extend(vector_result);
            } else {
                // Scalar fallback for remainder
                let scalar_results = self.scalar_batch_conversion(chunk, source_type, target_type)?;
                results.extend(scalar_results);
            }
        }
        
        Ok(results)
    }

    fn convert_vector(&self, 
                     values: &[BasicValueEnum<'ctx>], 
                     source_type: &Type, 
                     target_type: &Type) -> Result<Vec<BasicValueEnum<'ctx>>, String> {
        let source_llvm_type = convert_type(self.context, source_type)?;
        let target_llvm_type = convert_type(self.context, target_type)?;
        
        // Create vector types
        let vector_size = values.len() as u32;
        let source_vector_type = source_llvm_type.vec_type(vector_size);
        let target_vector_type = target_llvm_type.vec_type(vector_size);
        
        // Build vector from individual values
        let mut vector = source_vector_type.get_undef();
        for (i, value) in values.iter().enumerate() {
            vector = self.builder.build_insert_element(
                vector, 
                *value, 
                self.context.i32_type().const_int(i as u64, false), 
                &format!("insert_{}", i)
            ).map_err(|e| format!("Vector insert failed: {}", e))?;
        }
        
        // Perform vector conversion
        let converted_vector = match ConversionStrategy::for_types(source_type, target_type) {
            ConversionStrategy::Truncation => {
                self.builder.build_int_truncate(
                    vector.into_int_value(), 
                    target_vector_type.into_int_type(), 
                    "vec_trunc"
                ).map_err(|e| format!("Vector truncation failed: {}", e))?
            }
            ConversionStrategy::Extension => {
                if self.is_signed_type(source_type) {
                    self.builder.build_int_s_extend(
                        vector.into_int_value(), 
                        target_vector_type.into_int_type(), 
                        "vec_sext"
                    ).map_err(|e| format!("Vector sign extension failed: {}", e))?
                } else {
                    self.builder.build_int_z_extend(
                        vector.into_int_value(), 
                        target_vector_type.into_int_type(), 
                        "vec_zext"
                    ).map_err(|e| format!("Vector zero extension failed: {}", e))?
                }
            }
            _ => return self.scalar_batch_conversion(values, source_type, target_type),
        };
        
        // Extract individual results
        let mut results = Vec::new();
        for i in 0..vector_size {
            let element = self.builder.build_extract_element(
                converted_vector.into(), 
                self.context.i32_type().const_int(i as u64, false), 
                &format!("extract_{}", i)
            ).map_err(|e| format!("Vector extract failed: {}", e))?;
            results.push(element);
        }
        
        Ok(results)
    }
}
```

This implementation guide provides comprehensive LLVM code generation strategies for all type conversions in the CURSED language, with focus on performance, safety, and maintainability.
