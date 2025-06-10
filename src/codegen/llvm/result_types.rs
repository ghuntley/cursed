//! LLVM IR generation for Result and Option types
//!
//! This module provides LLVM code generation for Result<T, E> and Option<T> types,
//! including pattern matching, error propagation, and memory layout management.

use crate::codegen::llvm::LlvmCodeGenerator;
use crate::types::result::{ResultTypeExpression, OptionTypeExpression};
use crate::ast::traits::Expression;
use crate::error::{CursedError, SourceLocation};
use crate::value::Value;

use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicType, BasicTypeEnum, StructType, IntType, PointerType};
use inkwell::values::{BasicValue, BasicValueEnum, IntValue, PointerValue, StructValue, FunctionValue};
use inkwell::{FloatPredicate, IntPredicate, AddressSpace};
use std::collections::HashMap;

/// Result type layout in LLVM:
/// struct Result<T, E> {
///     i8 tag;        // 0 = Ok, 1 = Err
///     union {
///         T ok_value;
///         E err_value;
///     } data;
/// }
pub struct ResultTypeLayout<'ctx> {
    pub result_type: StructType<'ctx>,
    pub tag_type: IntType<'ctx>,
    pub ok_type: BasicTypeEnum<'ctx>,
    pub err_type: BasicTypeEnum<'ctx>,
    pub union_type: StructType<'ctx>,
}

/// Option type layout in LLVM:
/// struct Option<T> {
///     i8 tag;        // 0 = None, 1 = Some
///     T value;       // Only valid when tag == 1
/// }
pub struct OptionTypeLayout<'ctx> {
    pub option_type: StructType<'ctx>,
    pub tag_type: IntType<'ctx>,
    pub inner_type: BasicTypeEnum<'ctx>,
}

/// Trait for compiling Result and Option types
pub trait ResultTypeCompiler {
    /// Generate LLVM type for Result<T, E>
    fn generate_result_type(
        &mut self,
        ok_type: BasicTypeEnum,
        err_type: BasicTypeEnum,
    ) -> Result<ResultTypeLayout, CursedError>;

    /// Generate LLVM type for Option<T>
    fn generate_option_type(
        &mut self,
        inner_type: BasicTypeEnum,
    ) -> Result<OptionTypeLayout, CursedError>;

    /// Create Result::Ok value
    fn create_result_ok(
        &mut self,
        layout: &ResultTypeLayout,
        value: BasicValueEnum,
    ) -> Result<StructValue, CursedError>;

    /// Create Result::Err value
    fn create_result_err(
        &mut self,
        layout: &ResultTypeLayout,
        error: BasicValueEnum,
    ) -> Result<StructValue, CursedError>;

    /// Create Option::Some value
    fn create_option_some(
        &mut self,
        layout: &OptionTypeLayout,
        value: BasicValueEnum,
    ) -> Result<StructValue, CursedError>;

    /// Create Option::None value
    fn create_option_none(
        &mut self,
        layout: &OptionTypeLayout,
    ) -> Result<StructValue, CursedError>;

    /// Check if Result is Ok
    fn is_result_ok(
        &mut self,
        layout: &ResultTypeLayout,
        result_value: StructValue,
    ) -> Result<IntValue, CursedError>;

    /// Check if Result is Err
    fn is_result_err(
        &mut self,
        layout: &ResultTypeLayout,
        result_value: StructValue,
    ) -> Result<IntValue, CursedError>;

    /// Check if Option is Some
    fn is_option_some(
        &mut self,
        layout: &OptionTypeLayout,
        option_value: StructValue,
    ) -> Result<IntValue, CursedError>;

    /// Check if Option is None
    fn is_option_none(
        &mut self,
        layout: &OptionTypeLayout,
        option_value: StructValue,
    ) -> Result<IntValue, CursedError>;

    /// Extract Ok value from Result
    fn extract_result_ok(
        &mut self,
        layout: &ResultTypeLayout,
        result_value: StructValue,
    ) -> Result<BasicValueEnum, CursedError>;

    /// Extract Err value from Result
    fn extract_result_err(
        &mut self,
        layout: &ResultTypeLayout,
        result_value: StructValue,
    ) -> Result<BasicValueEnum, CursedError>;

    /// Extract Some value from Option
    fn extract_option_some(
        &mut self,
        layout: &OptionTypeLayout,
        option_value: StructValue,
    ) -> Result<BasicValueEnum, CursedError>;

    /// Generate pattern matching code for Result
    fn generate_result_match(
        &mut self,
        layout: &ResultTypeLayout,
        result_value: StructValue,
        ok_block: BasicBlock,
        err_block: BasicBlock,
    ) -> Result<(), CursedError>;

    /// Generate pattern matching code for Option
    fn generate_option_match(
        &mut self,
        layout: &OptionTypeLayout,
        option_value: StructValue,
        some_block: BasicBlock,
        none_block: BasicBlock,
    ) -> Result<(), CursedError>;

    /// Generate question mark operator for Result
    fn generate_result_question_mark(
        &mut self,
        layout: &ResultTypeLayout,
        result_value: StructValue,
        continue_block: BasicBlock,
        return_block: BasicBlock,
    ) -> Result<BasicValueEnum, CursedError>;

    /// Generate question mark operator for Option
    fn generate_option_question_mark(
        &mut self,
        layout: &OptionTypeLayout,
        option_value: StructValue,
        continue_block: BasicBlock,
        return_block: BasicBlock,
    ) -> Result<BasicValueEnum, CursedError>;

    /// Convert Option to Result
    fn convert_option_to_result(
        &mut self,
        option_layout: &OptionTypeLayout,
        result_layout: &ResultTypeLayout,
        option_value: StructValue,
        default_error: BasicValueEnum,
    ) -> Result<StructValue, CursedError>;

    /// Convert Result to Option (discarding error)
    fn convert_result_to_option(
        &mut self,
        result_layout: &ResultTypeLayout,
        option_layout: &OptionTypeLayout,
        result_value: StructValue,
    ) -> Result<StructValue, CursedError>;
}

impl<'ctx> ResultTypeCompiler for LlvmCodeGenerator<'ctx> {
    fn generate_result_type(
        &mut self,
        ok_type: BasicTypeEnum,
        err_type: BasicTypeEnum,
    ) -> Result<ResultTypeLayout<'ctx>, CursedError> {
        let tag_type = self.context.i8_type();
        
        // Create union type for ok/err values
        let union_size = std::cmp::max(
            ok_type.size_of().unwrap().get_zero_extended_constant().unwrap(),
            err_type.size_of().unwrap().get_zero_extended_constant().unwrap(),
        );
        
        // Use array of bytes for union storage
        let union_storage = self.context.i8_type().array_type(union_size as u32);
        let union_type = self.context.struct_type(&[union_storage.into()], false);
        
        // Create the Result struct: { tag, union }
        let result_type = self.context.struct_type(&[
            tag_type.into(),
            union_type.into(),
        ], false);

        Ok(ResultTypeLayout {
            result_type,
            tag_type,
            ok_type,
            err_type,
            union_type,
        })
    }

    fn generate_option_type(
        &mut self,
        inner_type: BasicTypeEnum,
    ) -> Result<OptionTypeLayout<'ctx>, CursedError> {
        let tag_type = self.context.i8_type();
        
        // Create the Option struct: { tag, value }
        let option_type = self.context.struct_type(&[
            tag_type.into(),
            inner_type,
        ], false);

        Ok(OptionTypeLayout {
            option_type,
            tag_type,
            inner_type,
        })
    }

    fn create_result_ok(
        &mut self,
        layout: &ResultTypeLayout<'ctx>,
        value: BasicValueEnum<'ctx>,
    ) -> Result<StructValue<'ctx>, CursedError> {
        let tag_ok = layout.tag_type.const_int(0, false); // Ok = 0

        // Allocate storage for the union and store the ok value
        let union_ptr = self.builder.build_alloca(layout.union_type, "result_union")
            .map_err(|e| CursedError::code_generation_error(
                format!("Failed to allocate union storage: {}", e), None, None
            ))?;

        // Cast union storage to the appropriate type and store value
        let value_ptr = self.builder.build_bitcast(
            union_ptr,
            layout.ok_type.ptr_type(AddressSpace::default()),
            "ok_value_ptr"
        ).map_err(|e| CursedError::code_generation_error(
            format!("Failed to cast union pointer: {}", e), None, None
        ))?;

        self.builder.build_store(value_ptr.into_pointer_value(), value)
            .map_err(|e| CursedError::code_generation_error(
                format!("Failed to store ok value: {}", e), None, None
            ))?;

        let union_value = self.builder.build_load(layout.union_type, union_ptr, "union_value")
            .map_err(|e| CursedError::code_generation_error(
                format!("Failed to load union value: {}", e), None, None
            ))?;

        // Create the Result struct
        let result_value = layout.result_type.const_named_struct(&[
            tag_ok.into(),
            union_value,
        ]);

        Ok(result_value)
    }

    fn create_result_err(
        &mut self,
        layout: &ResultTypeLayout<'ctx>,
        error: BasicValueEnum<'ctx>,
    ) -> Result<StructValue<'ctx>, CursedError> {
        let tag_err = layout.tag_type.const_int(1, false); // Err = 1

        // Allocate storage for the union and store the error value
        let union_ptr = self.builder.build_alloca(layout.union_type, "result_union")
            .map_err(|e| CursedError::code_generation_error(
                format!("Failed to allocate union storage: {}", e), None, None
            ))?;

        // Cast union storage to the appropriate type and store error
        let error_ptr = self.builder.build_bitcast(
            union_ptr,
            layout.err_type.ptr_type(AddressSpace::default()),
            "err_value_ptr"
        ).map_err(|e| CursedError::code_generation_error(
            format!("Failed to cast union pointer: {}", e), None, None
        ))?;

        self.builder.build_store(error_ptr.into_pointer_value(), error)
            .map_err(|e| CursedError::code_generation_error(
                format!("Failed to store error value: {}", e), None, None
            ))?;

        let union_value = self.builder.build_load(layout.union_type, union_ptr, "union_value")
            .map_err(|e| CursedError::code_generation_error(
                format!("Failed to load union value: {}", e), None, None
            ))?;

        // Create the Result struct
        let result_value = layout.result_type.const_named_struct(&[
            tag_err.into(),
            union_value,
        ]);

        Ok(result_value)
    }

    fn create_option_some(
        &mut self,
        layout: &OptionTypeLayout<'ctx>,
        value: BasicValueEnum<'ctx>,
    ) -> Result<StructValue<'ctx>, CursedError> {
        let tag_some = layout.tag_type.const_int(1, false); // Some = 1

        let option_value = layout.option_type.const_named_struct(&[
            tag_some.into(),
            value,
        ]);

        Ok(option_value)
    }

    fn create_option_none(
        &mut self,
        layout: &OptionTypeLayout<'ctx>,
    ) -> Result<StructValue<'ctx>, CursedError> {
        let tag_none = layout.tag_type.const_int(0, false); // None = 0
        let zero_value = layout.inner_type.const_zero();

        let option_value = layout.option_type.const_named_struct(&[
            tag_none.into(),
            zero_value,
        ]);

        Ok(option_value)
    }

    fn is_result_ok(
        &mut self,
        layout: &ResultTypeLayout<'ctx>,
        result_value: StructValue<'ctx>,
    ) -> Result<IntValue<'ctx>, CursedError> {
        let tag_ptr = self.builder.build_struct_gep(layout.result_type, result_value.as_pointer_value(), 0, "tag_ptr")
            .map_err(|e| CursedError::code_generation_error(
                format!("Failed to get tag pointer: {}", e), None, None
            ))?;

        let tag = self.builder.build_load(layout.tag_type, tag_ptr, "tag")
            .map_err(|e| CursedError::code_generation_error(
                format!("Failed to load tag: {}", e), None, None
            ))?;

        let ok_tag = layout.tag_type.const_int(0, false);
        let is_ok = self.builder.build_int_compare(
            IntPredicate::EQ,
            tag.into_int_value(),
            ok_tag,
            "is_ok"
        ).map_err(|e| CursedError::code_generation_error(
            format!("Failed to compare tag: {}", e), None, None
        ))?;

        Ok(is_ok)
    }

    fn is_result_err(
        &mut self,
        layout: &ResultTypeLayout<'ctx>,
        result_value: StructValue<'ctx>,
    ) -> Result<IntValue<'ctx>, CursedError> {
        let is_ok = self.is_result_ok(layout, result_value)?;
        let is_err = self.builder.build_not(is_ok, "is_err")
            .map_err(|e| CursedError::code_generation_error(
                format!("Failed to negate is_ok: {}", e), None, None
            ))?;

        Ok(is_err)
    }

    fn is_option_some(
        &mut self,
        layout: &OptionTypeLayout<'ctx>,
        option_value: StructValue<'ctx>,
    ) -> Result<IntValue<'ctx>, CursedError> {
        let tag_ptr = self.builder.build_struct_gep(layout.option_type, option_value.as_pointer_value(), 0, "tag_ptr")
            .map_err(|e| CursedError::code_generation_error(
                format!("Failed to get tag pointer: {}", e), None, None
            ))?;

        let tag = self.builder.build_load(layout.tag_type, tag_ptr, "tag")
            .map_err(|e| CursedError::code_generation_error(
                format!("Failed to load tag: {}", e), None, None
            ))?;

        let some_tag = layout.tag_type.const_int(1, false);
        let is_some = self.builder.build_int_compare(
            IntPredicate::EQ,
            tag.into_int_value(),
            some_tag,
            "is_some"
        ).map_err(|e| CursedError::code_generation_error(
            format!("Failed to compare tag: {}", e), None, None
        ))?;

        Ok(is_some)
    }

    fn is_option_none(
        &mut self,
        layout: &OptionTypeLayout<'ctx>,
        option_value: StructValue<'ctx>,
    ) -> Result<IntValue<'ctx>, CursedError> {
        let is_some = self.is_option_some(layout, option_value)?;
        let is_none = self.builder.build_not(is_some, "is_none")
            .map_err(|e| CursedError::code_generation_error(
                format!("Failed to negate is_some: {}", e), None, None
            ))?;

        Ok(is_none)
    }

    fn extract_result_ok(
        &mut self,
        layout: &ResultTypeLayout<'ctx>,
        result_value: StructValue<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, CursedError> {
        // Get pointer to union data
        let union_ptr = self.builder.build_struct_gep(layout.result_type, result_value.as_pointer_value(), 1, "union_ptr")
            .map_err(|e| CursedError::code_generation_error(
                format!("Failed to get union pointer: {}", e), None, None
            ))?;

        // Cast to ok type pointer and load
        let ok_ptr = self.builder.build_bitcast(
            union_ptr,
            layout.ok_type.ptr_type(AddressSpace::default()),
            "ok_ptr"
        ).map_err(|e| CursedError::code_generation_error(
            format!("Failed to cast to ok pointer: {}", e), None, None
        ))?;

        let ok_value = self.builder.build_load(layout.ok_type, ok_ptr.into_pointer_value(), "ok_value")
            .map_err(|e| CursedError::code_generation_error(
                format!("Failed to load ok value: {}", e), None, None
            ))?;

        Ok(ok_value)
    }

    fn extract_result_err(
        &mut self,
        layout: &ResultTypeLayout<'ctx>,
        result_value: StructValue<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, CursedError> {
        // Get pointer to union data
        let union_ptr = self.builder.build_struct_gep(layout.result_type, result_value.as_pointer_value(), 1, "union_ptr")
            .map_err(|e| CursedError::code_generation_error(
                format!("Failed to get union pointer: {}", e), None, None
            ))?;

        // Cast to err type pointer and load
        let err_ptr = self.builder.build_bitcast(
            union_ptr,
            layout.err_type.ptr_type(AddressSpace::default()),
            "err_ptr"
        ).map_err(|e| CursedError::code_generation_error(
            format!("Failed to cast to err pointer: {}", e), None, None
        ))?;

        let err_value = self.builder.build_load(layout.err_type, err_ptr.into_pointer_value(), "err_value")
            .map_err(|e| CursedError::code_generation_error(
                format!("Failed to load err value: {}", e), None, None
            ))?;

        Ok(err_value)
    }

    fn extract_option_some(
        &mut self,
        layout: &OptionTypeLayout<'ctx>,
        option_value: StructValue<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, CursedError> {
        let value_ptr = self.builder.build_struct_gep(layout.option_type, option_value.as_pointer_value(), 1, "value_ptr")
            .map_err(|e| CursedError::code_generation_error(
                format!("Failed to get value pointer: {}", e), None, None
            ))?;

        let value = self.builder.build_load(layout.inner_type, value_ptr, "value")
            .map_err(|e| CursedError::code_generation_error(
                format!("Failed to load value: {}", e), None, None
            ))?;

        Ok(value)
    }

    fn generate_result_match(
        &mut self,
        layout: &ResultTypeLayout<'ctx>,
        result_value: StructValue<'ctx>,
        ok_block: BasicBlock<'ctx>,
        err_block: BasicBlock<'ctx>,
    ) -> Result<(), CursedError> {
        let is_ok = self.is_result_ok(layout, result_value)?;

        self.builder.build_conditional_branch(is_ok, ok_block, err_block)
            .map_err(|e| CursedError::code_generation_error(
                format!("Failed to build conditional branch: {}", e), None, None
            ))?;

        Ok(())
    }

    fn generate_option_match(
        &mut self,
        layout: &OptionTypeLayout<'ctx>,
        option_value: StructValue<'ctx>,
        some_block: BasicBlock<'ctx>,
        none_block: BasicBlock<'ctx>,
    ) -> Result<(), CursedError> {
        let is_some = self.is_option_some(layout, option_value)?;

        self.builder.build_conditional_branch(is_some, some_block, none_block)
            .map_err(|e| CursedError::code_generation_error(
                format!("Failed to build conditional branch: {}", e), None, None
            ))?;

        Ok(())
    }

    fn generate_result_question_mark(
        &mut self,
        layout: &ResultTypeLayout<'ctx>,
        result_value: StructValue<'ctx>,
        continue_block: BasicBlock<'ctx>,
        return_block: BasicBlock<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, CursedError> {
        let is_ok = self.is_result_ok(layout, result_value)?;

        // Create blocks for ok and err paths
        let ok_block = self.context.append_basic_block(
            self.builder.get_insert_block().unwrap().get_parent().unwrap(),
            "result_ok"
        );
        let err_block = self.context.append_basic_block(
            self.builder.get_insert_block().unwrap().get_parent().unwrap(),
            "result_err"
        );

        // Branch based on result tag
        self.builder.build_conditional_branch(is_ok, ok_block, err_block)
            .map_err(|e| CursedError::code_generation_error(
                format!("Failed to build conditional branch: {}", e), None, None
            ))?;

        // Ok path: extract value and continue
        self.builder.position_at_end(ok_block);
        let ok_value = self.extract_result_ok(layout, result_value)?;
        self.builder.build_unconditional_branch(continue_block)
            .map_err(|e| CursedError::code_generation_error(
                format!("Failed to build branch to continue: {}", e), None, None
            ))?;

        // Err path: propagate error by returning
        self.builder.position_at_end(err_block);
        let err_value = self.extract_result_err(layout, result_value)?;
        
        // Create a new Result with the error for early return
        let return_result = self.create_result_err(layout, err_value)?;
        self.builder.build_return(Some(&return_result.into()))
            .map_err(|e| CursedError::code_generation_error(
                format!("Failed to build return: {}", e), None, None
            ))?;

        // Position at continue block for subsequent code
        self.builder.position_at_end(continue_block);

        Ok(ok_value)
    }

    fn generate_option_question_mark(
        &mut self,
        layout: &OptionTypeLayout<'ctx>,
        option_value: StructValue<'ctx>,
        continue_block: BasicBlock<'ctx>,
        return_block: BasicBlock<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, CursedError> {
        let is_some = self.is_option_some(layout, option_value)?;

        // Create blocks for some and none paths
        let some_block = self.context.append_basic_block(
            self.builder.get_insert_block().unwrap().get_parent().unwrap(),
            "option_some"
        );
        let none_block = self.context.append_basic_block(
            self.builder.get_insert_block().unwrap().get_parent().unwrap(),
            "option_none"
        );

        // Branch based on option tag
        self.builder.build_conditional_branch(is_some, some_block, none_block)
            .map_err(|e| CursedError::code_generation_error(
                format!("Failed to build conditional branch: {}", e), None, None
            ))?;

        // Some path: extract value and continue
        self.builder.position_at_end(some_block);
        let some_value = self.extract_option_some(layout, option_value)?;
        self.builder.build_unconditional_branch(continue_block)
            .map_err(|e| CursedError::code_generation_error(
                format!("Failed to build branch to continue: {}", e), None, None
            ))?;

        // None path: return None
        self.builder.position_at_end(none_block);
        let return_none = self.create_option_none(layout)?;
        self.builder.build_return(Some(&return_none.into()))
            .map_err(|e| CursedError::code_generation_error(
                format!("Failed to build return: {}", e), None, None
            ))?;

        // Position at continue block for subsequent code
        self.builder.position_at_end(continue_block);

        Ok(some_value)
    }

    fn convert_option_to_result(
        &mut self,
        option_layout: &OptionTypeLayout<'ctx>,
        result_layout: &ResultTypeLayout<'ctx>,
        option_value: StructValue<'ctx>,
        default_error: BasicValueEnum<'ctx>,
    ) -> Result<StructValue<'ctx>, CursedError> {
        let is_some = self.is_option_some(option_layout, option_value)?;

        // Create blocks for conversion
        let some_block = self.context.append_basic_block(
            self.builder.get_insert_block().unwrap().get_parent().unwrap(),
            "option_to_result_some"
        );
        let none_block = self.context.append_basic_block(
            self.builder.get_insert_block().unwrap().get_parent().unwrap(),
            "option_to_result_none"
        );
        let merge_block = self.context.append_basic_block(
            self.builder.get_insert_block().unwrap().get_parent().unwrap(),
            "option_to_result_merge"
        );

        // Branch based on option tag
        self.builder.build_conditional_branch(is_some, some_block, none_block)
            .map_err(|e| CursedError::code_generation_error(
                format!("Failed to build conditional branch: {}", e), None, None
            ))?;

        // Some path: create Result::Ok
        self.builder.position_at_end(some_block);
        let some_value = self.extract_option_some(option_layout, option_value)?;
        let result_ok = self.create_result_ok(result_layout, some_value)?;
        self.builder.build_unconditional_branch(merge_block)
            .map_err(|e| CursedError::code_generation_error(
                format!("Failed to build branch to merge: {}", e), None, None
            ))?;

        // None path: create Result::Err
        self.builder.position_at_end(none_block);
        let result_err = self.create_result_err(result_layout, default_error)?;
        self.builder.build_unconditional_branch(merge_block)
            .map_err(|e| CursedError::code_generation_error(
                format!("Failed to build branch to merge: {}", e), None, None
            ))?;

        // Merge block: use phi node to select result
        self.builder.position_at_end(merge_block);
        let phi = self.builder.build_phi(result_layout.result_type, "option_to_result")
            .map_err(|e| CursedError::code_generation_error(
                format!("Failed to build phi node: {}", e), None, None
            ))?;

        phi.add_incoming(&[(&result_ok.into(), some_block), (&result_err.into(), none_block)]);

        Ok(phi.as_basic_value().into_struct_value())
    }

    fn convert_result_to_option(
        &mut self,
        result_layout: &ResultTypeLayout<'ctx>,
        option_layout: &OptionTypeLayout<'ctx>,
        result_value: StructValue<'ctx>,
    ) -> Result<StructValue<'ctx>, CursedError> {
        let is_ok = self.is_result_ok(result_layout, result_value)?;

        // Create blocks for conversion
        let ok_block = self.context.append_basic_block(
            self.builder.get_insert_block().unwrap().get_parent().unwrap(),
            "result_to_option_ok"
        );
        let err_block = self.context.append_basic_block(
            self.builder.get_insert_block().unwrap().get_parent().unwrap(),
            "result_to_option_err"
        );
        let merge_block = self.context.append_basic_block(
            self.builder.get_insert_block().unwrap().get_parent().unwrap(),
            "result_to_option_merge"
        );

        // Branch based on result tag
        self.builder.build_conditional_branch(is_ok, ok_block, err_block)
            .map_err(|e| CursedError::code_generation_error(
                format!("Failed to build conditional branch: {}", e), None, None
            ))?;

        // Ok path: create Option::Some
        self.builder.position_at_end(ok_block);
        let ok_value = self.extract_result_ok(result_layout, result_value)?;
        let option_some = self.create_option_some(option_layout, ok_value)?;
        self.builder.build_unconditional_branch(merge_block)
            .map_err(|e| CursedError::code_generation_error(
                format!("Failed to build branch to merge: {}", e), None, None
            ))?;

        // Err path: create Option::None
        self.builder.position_at_end(err_block);
        let option_none = self.create_option_none(option_layout)?;
        self.builder.build_unconditional_branch(merge_block)
            .map_err(|e| CursedError::code_generation_error(
                format!("Failed to build branch to merge: {}", e), None, None
            ))?;

        // Merge block: use phi node to select option
        self.builder.position_at_end(merge_block);
        let phi = self.builder.build_phi(option_layout.option_type, "result_to_option")
            .map_err(|e| CursedError::code_generation_error(
                format!("Failed to build phi node: {}", e), None, None
            ))?;

        phi.add_incoming(&[(&option_some.into(), ok_block), (&option_none.into(), err_block)]);

        Ok(phi.as_basic_value().into_struct_value())
    }
}

/// Utility functions for Result/Option type management
pub mod result_type_utils {
    use super::*;

    /// Get the LLVM type for a Result<T, E>
    pub fn get_result_llvm_type<'ctx>(
        context: &'ctx Context,
        ok_type: BasicTypeEnum<'ctx>,
        err_type: BasicTypeEnum<'ctx>,
    ) -> StructType<'ctx> {
        let tag_type = context.i8_type();
        let union_size = std::cmp::max(
            ok_type.size_of().unwrap().get_zero_extended_constant().unwrap(),
            err_type.size_of().unwrap().get_zero_extended_constant().unwrap(),
        );
        let union_storage = context.i8_type().array_type(union_size as u32);
        let union_type = context.struct_type(&[union_storage.into()], false);
        
        context.struct_type(&[tag_type.into(), union_type.into()], false)
    }

    /// Get the LLVM type for an Option<T>
    pub fn get_option_llvm_type<'ctx>(
        context: &'ctx Context,
        inner_type: BasicTypeEnum<'ctx>,
    ) -> StructType<'ctx> {
        let tag_type = context.i8_type();
        context.struct_type(&[tag_type.into(), inner_type], false)
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

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::context::Context;
    use inkwell::targets::{InitializationConfig, Target};

    fn setup_test_context() -> Context {
        Target::initialize_all(&InitializationConfig::default());
        Context::create()
    }

    #[test]
    fn test_result_type_layout() {
        let context = setup_test_context();
        let module = context.create_module("test");
        let builder = context.create_builder();
        
        let mut codegen = LlvmCodeGenerator {
            context: &context,
            module,
            builder,
            functions: HashMap::new(),
            current_function: None,
            current_loop: None,
            variables: HashMap::new(),
            break_blocks: Vec::new(),
            continue_blocks: Vec::new(),
        };

        let ok_type = context.i32_type().into();
        let err_type = context.i8_type().ptr_type(AddressSpace::default()).into();

        let layout = codegen.generate_result_type(ok_type, err_type).unwrap();
        
        assert_eq!(layout.ok_type, ok_type);
        assert_eq!(layout.err_type, err_type);
        assert_eq!(layout.tag_type, context.i8_type());
    }

    #[test]
    fn test_option_type_layout() {
        let context = setup_test_context();
        let module = context.create_module("test");
        let builder = context.create_builder();
        
        let mut codegen = LlvmCodeGenerator {
            context: &context,
            module,
            builder,
            functions: HashMap::new(),
            current_function: None,
            current_loop: None,
            variables: HashMap::new(),
            break_blocks: Vec::new(),
            continue_blocks: Vec::new(),
        };

        let inner_type = context.i32_type().into();
        let layout = codegen.generate_option_type(inner_type).unwrap();
        
        assert_eq!(layout.inner_type, inner_type);
        assert_eq!(layout.tag_type, context.i8_type());
    }

    #[test]
    fn test_result_type_utils() {
        assert!(result_type_utils::is_result_type("Result<i32, String>"));
        assert!(!result_type_utils::is_result_type("Option<i32>"));
        assert!(!result_type_utils::is_result_type("i32"));

        assert!(result_type_utils::is_option_type("Option<i32>"));
        assert!(!result_type_utils::is_option_type("Result<i32, String>"));
        assert!(!result_type_utils::is_option_type("i32"));

        let (ok_type, err_type) = result_type_utils::parse_result_types("Result<i32, String>").unwrap();
        assert_eq!(ok_type, "i32");
        assert_eq!(err_type, "String");

        let inner_type = result_type_utils::parse_option_type("Option<i32>").unwrap();
        assert_eq!(inner_type, "i32");

        assert_eq!(result_type_utils::result_type_name("i32", "String"), "Result<i32, String>");
        assert_eq!(result_type_utils::option_type_name("i32"), "Option<i32>");
    }

    #[test]
    fn test_type_name_parsing() {
        let result_types = result_type_utils::parse_result_types("Result<normie, based>");
        assert_eq!(result_types, Some(("normie".to_string(), "based".to_string())));

        let option_type = result_type_utils::parse_option_type("Option<normie>");
        assert_eq!(option_type, Some("normie".to_string()));

        // Test invalid formats
        assert_eq!(result_type_utils::parse_result_types("Result<i32>"), None);
        assert_eq!(result_type_utils::parse_option_type("Option<>"), Some("".to_string()));
    }
}
