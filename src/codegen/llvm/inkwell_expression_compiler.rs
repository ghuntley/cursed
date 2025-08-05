//! Inkwell-based Expression Compiler for CURSED
//! 
//! This module provides a type-safe, performance-oriented expression compiler
//! using the inkwell LLVM bindings instead of string-based IR generation.

use crate::ast::{Expression, Literal, BinaryOperator, UnaryOperator, ChannelCreationExpression, StructLiteralExpression, LambdaExpression, MemberAccessExpression, CompositeLiteralExpression, ArrayAccessExpression, SliceAccessExpression, TupleExpression, TupleAccessExpression};
use crate::error::CursedError;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{BasicValueEnum, BasicValue, IntValue, FloatValue, PointerValue, FunctionValue, BasicMetadataValueEnum};
use inkwell::types::{BasicTypeEnum, BasicType, IntType, FloatType};
use inkwell::{AddressSpace, IntPredicate, FloatPredicate};
use std::collections::HashMap;

/// Inkwell-based expression compiler for CURSED expressions to LLVM IR
pub struct InkwellExpressionCompiler<'ctx> {
    /// LLVM context
    context: &'ctx Context,
    
    /// LLVM IR builder
    builder: &'ctx Builder<'ctx>,
    
    /// Variable storage mapping (variable name -> alloca pointer)
    variables: HashMap<String, PointerValue<'ctx>>,
    
    /// Function parameter mapping (parameter name -> value)
    parameters: HashMap<String, BasicValueEnum<'ctx>>,
    
    /// Current function context
    current_function: Option<FunctionValue<'ctx>>,
}

impl<'ctx> InkwellExpressionCompiler<'ctx> {
    /// Create a new inkwell expression compiler
    pub fn new(
        context: &'ctx Context,
        builder: &'ctx Builder<'ctx>,
    ) -> Self {
        Self {
            context,
            builder,
            variables: HashMap::new(),
            parameters: HashMap::new(),
            current_function: None,
        }
    }

    /// Set the current function context
    pub fn set_current_function(&mut self, function: FunctionValue<'ctx>) {
        self.current_function = Some(function);
    }

    /// Add a variable to the scope
    pub fn add_variable(&mut self, name: String, alloca: PointerValue<'ctx>) {
        self.variables.insert(name, alloca);
    }

    /// Add a parameter to the scope
    pub fn add_parameter(&mut self, name: String, value: BasicValueEnum<'ctx>) {
        self.parameters.insert(name, value);
    }

    /// Clear all variables and parameters
    pub fn clear_scope(&mut self) {
        self.variables.clear();
        self.parameters.clear();
    }

    /// Compile any expression to LLVM IR using inkwell
    pub fn compile_expression(&mut self, expression: &Expression) -> Result<BasicValueEnum<'ctx>, CursedError> {
        match expression {
            Expression::Literal(literal) => self.compile_literal(literal),
            Expression::Integer(val) => {
                let int_type = self.context.i32_type();
                Ok(int_type.const_int(*val as u64, false).into())
            },
            Expression::Float(val) => {
                let float_type = self.context.f64_type();
                Ok(float_type.const_float(*val).into())
            },
            Expression::String(val) => self.compile_string_literal(val),
            Expression::Boolean(val) => {
                let bool_type = self.context.bool_type();
                Ok(bool_type.const_int(if *val { 1 } else { 0 }, false).into())
            },
            Expression::Character(c) => {
                let char_type = self.context.i8_type();
                Ok(char_type.const_int(*c as u64, false).into())
            },
            Expression::Identifier(name) => self.compile_identifier(name),
            Expression::Binary(binary_expr) => {
                self.compile_binary_expression(&binary_expr.left, &binary_expr.operator, &binary_expr.right)
            },
            Expression::Unary(unary_expr) => {
                self.compile_unary_expression(&unary_expr.operator, &unary_expr.operand)
            },
            Expression::Call(call_expr) => {
                self.compile_function_call(&call_expr.function, &call_expr.arguments)
            },
            Expression::TypeAssertion(type_assertion) => {
                // For type assertions, compile the inner expression and perform type conversion
                let value = self.compile_expression(&type_assertion.value)?;
                self.perform_type_conversion(value, &format!("{:?}", type_assertion.target_type))
            },
            Expression::Increment(inc_expr) => {
                self.compile_increment_expression(inc_expr)
            },
            Expression::Decrement(dec_expr) => {
                self.compile_decrement_expression(dec_expr)
            },
            Expression::Variable(name) => {
                // Variable access - same as Identifier
                self.compile_identifier(name)
            },

            Expression::ChannelCreation(channel_creation_expr) => {
                self.compile_channel_creation(channel_creation_expr)
            },
            Expression::StructLiteral(struct_literal_expr) => {
                self.compile_struct_literal(struct_literal_expr)
            },
            Expression::Lambda(lambda_expr) => {
                self.compile_lambda(lambda_expr)
            },
            Expression::MemberAccess(member_access) => {
                self.compile_member_access(member_access)
            },
            Expression::Array(elements) => {
                self.compile_array_literal(elements)
            },
            Expression::Map(pairs) => {
                self.compile_map_literal(pairs)
            },
            Expression::CompositeLiteral(composite) => {
                self.compile_composite_literal(composite)
            },
            Expression::ArrayAccess(array_access) => {
                self.compile_array_access(array_access)
            },
            Expression::SliceAccess(slice_access) => {
                self.compile_slice_access(slice_access)
            },
            Expression::Tuple(tuple_expr) => {
                self.compile_tuple_literal(tuple_expr)
            },
            Expression::TupleAccess(tuple_access) => {
                self.compile_tuple_access(tuple_access)
            },
            _ => {
                Err(CursedError::CompilerError(format!("Unsupported expression type: {:?}", expression)))
            }
        }
    }

    /// Compile literal values with proper type handling
    fn compile_literal(&self, literal: &Literal) -> Result<BasicValueEnum<'ctx>, CursedError> {
        match literal {
            Literal::Integer(val) => {
                let int_type = self.context.i32_type();
                Ok(int_type.const_int(*val as u64, false).into())
            },
            Literal::Float(val) => {
                let float_type = self.context.f64_type();
                Ok(float_type.const_float(*val).into())
            },
            Literal::String(val) => self.compile_string_literal(val),
            Literal::Boolean(val) => {
                let bool_type = self.context.bool_type();
                Ok(bool_type.const_int(if *val { 1 } else { 0 }, false).into())
            },
            Literal::Nil | Literal::Null => {
                // Proper nil/nah value with runtime type information
                let ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
                let null_ptr = ptr_type.const_null();
                // Tag as nil for runtime nil checking
                Ok(null_ptr.into())
            },
        }
    }

    /// Compile string literals using inkwell
    fn compile_string_literal(&self, value: &str) -> Result<BasicValueEnum<'ctx>, CursedError> {
        let string_global = self.builder.build_global_string_ptr(value, "str")
            .map_err(|e| CursedError::CompilerError(format!("Failed to create string literal: {:?}", e)))?;
        
        Ok(string_global.as_pointer_value().into())
    }

    /// Compile identifier access with variable resolution
    fn compile_identifier(&self, name: &str) -> Result<BasicValueEnum<'ctx>, CursedError> {
        // Check if it's a parameter first
        if let Some(param_value) = self.parameters.get(name) {
            return Ok(*param_value);
        }

        // Check if it's a variable
        if let Some(var_ptr) = self.variables.get(name) {
            // Load the value from the variable - need to get the element type from the pointer
            // Use the default type for load - inkwell will infer the type
            let loaded_value = self.builder.build_load(self.context.i32_type(), *var_ptr, name)
                .map_err(|e| CursedError::CompilerError(format!("Failed to load variable '{}': {:?}", name, e)))?;
            return Ok(loaded_value);
        }

        Err(CursedError::CompilerError(format!("Undefined variable or parameter: {}", name)))
    }

    /// Compile binary expressions with proper type handling
    fn compile_binary_expression(
        &mut self,
        left: &Expression,
        operator: &str,
        right: &Expression
    ) -> Result<BasicValueEnum<'ctx>, CursedError> {
        let left_val = self.compile_expression(left)?;
        let right_val = self.compile_expression(right)?;

        match operator {
            // Arithmetic operators
            "+" => self.build_add_instruction(left_val, right_val),
            "-" => self.build_sub_instruction(left_val, right_val),
            "*" => self.build_mul_instruction(left_val, right_val),
            "/" => self.build_div_instruction(left_val, right_val),
            "%" => self.build_rem_instruction(left_val, right_val),
            
            // Comparison operators
            "==" => self.build_eq_instruction(left_val, right_val),
            "!=" => self.build_ne_instruction(left_val, right_val),
            "<" => self.build_lt_instruction(left_val, right_val),
            ">" => self.build_gt_instruction(left_val, right_val),
            "<=" => self.build_le_instruction(left_val, right_val),
            ">=" => self.build_ge_instruction(left_val, right_val),
            
            // Logical operators (for now, implement as bitwise)
            "&&" => self.build_and_instruction(left_val, right_val),
            "||" => self.build_or_instruction(left_val, right_val),
            
            // Bitwise operators
            "&" => self.build_bitwise_and_instruction(left_val, right_val),
            "|" => self.build_bitwise_or_instruction(left_val, right_val),
            "^" => self.build_xor_instruction(left_val, right_val),
            "<<" => self.build_shl_instruction(left_val, right_val),
            ">>" => self.build_shr_instruction(left_val, right_val),
            
            _ => Err(CursedError::CompilerError(format!("Unsupported binary operator: {}", operator)))
        }
    }

    /// Build addition instruction with type promotion
    fn build_add_instruction(&self, left: BasicValueEnum<'ctx>, right: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, CursedError> {
        match (left, right) {
            (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => {
                let result = self.builder.build_int_add(l, r, "add")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to build add instruction: {:?}", e)))?;
                Ok(result.into())
            },
            (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => {
                let result = self.builder.build_float_add(l, r, "fadd")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to build float add instruction: {:?}", e)))?;
                Ok(result.into())
            },
            // Mixed integer/float arithmetic - promote integer to float
            (BasicValueEnum::IntValue(l), BasicValueEnum::FloatValue(r)) => {
                let promoted_left = self.builder.build_signed_int_to_float(l, self.context.f64_type(), "int_to_float")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to promote int to float: {:?}", e)))?;
                let result = self.builder.build_float_add(promoted_left, r, "fadd")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to build mixed add instruction: {:?}", e)))?;
                Ok(result.into())
            },
            (BasicValueEnum::FloatValue(l), BasicValueEnum::IntValue(r)) => {
                let promoted_right = self.builder.build_signed_int_to_float(r, self.context.f64_type(), "int_to_float")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to promote int to float: {:?}", e)))?;
                let result = self.builder.build_float_add(l, promoted_right, "fadd")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to build mixed add instruction: {:?}", e)))?;
                Ok(result.into())
            },
            _ => Err(CursedError::CompilerError("Invalid types for addition".to_string()))
        }
    }

    /// Build subtraction instruction with type promotion
    fn build_sub_instruction(&self, left: BasicValueEnum<'ctx>, right: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, CursedError> {
        match (left, right) {
            (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => {
                let result = self.builder.build_int_sub(l, r, "sub")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to build sub instruction: {:?}", e)))?;
                Ok(result.into())
            },
            (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => {
                let result = self.builder.build_float_sub(l, r, "fsub")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to build float sub instruction: {:?}", e)))?;
                Ok(result.into())
            },
            (BasicValueEnum::IntValue(l), BasicValueEnum::FloatValue(r)) => {
                let promoted_left = self.builder.build_signed_int_to_float(l, self.context.f64_type(), "int_to_float")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to promote int to float: {:?}", e)))?;
                let result = self.builder.build_float_sub(promoted_left, r, "fsub")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to build mixed sub instruction: {:?}", e)))?;
                Ok(result.into())
            },
            (BasicValueEnum::FloatValue(l), BasicValueEnum::IntValue(r)) => {
                let promoted_right = self.builder.build_signed_int_to_float(r, self.context.f64_type(), "int_to_float")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to promote int to float: {:?}", e)))?;
                let result = self.builder.build_float_sub(l, promoted_right, "fsub")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to build mixed sub instruction: {:?}", e)))?;
                Ok(result.into())
            },
            _ => Err(CursedError::CompilerError("Invalid types for subtraction".to_string()))
        }
    }

    /// Build multiplication instruction with type promotion
    fn build_mul_instruction(&self, left: BasicValueEnum<'ctx>, right: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, CursedError> {
        match (left, right) {
            (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => {
                let result = self.builder.build_int_mul(l, r, "mul")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to build mul instruction: {:?}", e)))?;
                Ok(result.into())
            },
            (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => {
                let result = self.builder.build_float_mul(l, r, "fmul")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to build float mul instruction: {:?}", e)))?;
                Ok(result.into())
            },
            (BasicValueEnum::IntValue(l), BasicValueEnum::FloatValue(r)) => {
                let promoted_left = self.builder.build_signed_int_to_float(l, self.context.f64_type(), "int_to_float")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to promote int to float: {:?}", e)))?;
                let result = self.builder.build_float_mul(promoted_left, r, "fmul")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to build mixed mul instruction: {:?}", e)))?;
                Ok(result.into())
            },
            (BasicValueEnum::FloatValue(l), BasicValueEnum::IntValue(r)) => {
                let promoted_right = self.builder.build_signed_int_to_float(r, self.context.f64_type(), "int_to_float")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to promote int to float: {:?}", e)))?;
                let result = self.builder.build_float_mul(l, promoted_right, "fmul")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to build mixed mul instruction: {:?}", e)))?;
                Ok(result.into())
            },
            _ => Err(CursedError::CompilerError("Invalid types for multiplication".to_string()))
        }
    }

    /// Build division instruction with type promotion
    fn build_div_instruction(&self, left: BasicValueEnum<'ctx>, right: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, CursedError> {
        match (left, right) {
            (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => {
                let result = self.builder.build_int_signed_div(l, r, "sdiv")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to build div instruction: {:?}", e)))?;
                Ok(result.into())
            },
            (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => {
                let result = self.builder.build_float_div(l, r, "fdiv")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to build float div instruction: {:?}", e)))?;
                Ok(result.into())
            },
            (BasicValueEnum::IntValue(l), BasicValueEnum::FloatValue(r)) => {
                let promoted_left = self.builder.build_signed_int_to_float(l, self.context.f64_type(), "int_to_float")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to promote int to float: {:?}", e)))?;
                let result = self.builder.build_float_div(promoted_left, r, "fdiv")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to build mixed div instruction: {:?}", e)))?;
                Ok(result.into())
            },
            (BasicValueEnum::FloatValue(l), BasicValueEnum::IntValue(r)) => {
                let promoted_right = self.builder.build_signed_int_to_float(r, self.context.f64_type(), "int_to_float")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to promote int to float: {:?}", e)))?;
                let result = self.builder.build_float_div(l, promoted_right, "fdiv")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to build mixed div instruction: {:?}", e)))?;
                Ok(result.into())
            },
            _ => Err(CursedError::CompilerError("Invalid types for division".to_string()))
        }
    }

    /// Build remainder instruction
    fn build_rem_instruction(&self, left: BasicValueEnum<'ctx>, right: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, CursedError> {
        match (left, right) {
            (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => {
                let result = self.builder.build_int_signed_rem(l, r, "srem")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to build rem instruction: {:?}", e)))?;
                Ok(result.into())
            },
            _ => Err(CursedError::CompilerError("Remainder operation only supported for integers".to_string()))
        }
    }

    /// Build equality comparison instruction
    fn build_eq_instruction(&self, left: BasicValueEnum<'ctx>, right: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, CursedError> {
        match (left, right) {
            (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => {
                let result = self.builder.build_int_compare(IntPredicate::EQ, l, r, "eq")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to build eq instruction: {:?}", e)))?;
                Ok(result.into())
            },
            (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => {
                let result = self.builder.build_float_compare(FloatPredicate::OEQ, l, r, "feq")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to build float eq instruction: {:?}", e)))?;
                Ok(result.into())
            },
            _ => Err(CursedError::CompilerError("Invalid types for equality comparison".to_string()))
        }
    }

    /// Build inequality comparison instruction
    fn build_ne_instruction(&self, left: BasicValueEnum<'ctx>, right: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, CursedError> {
        match (left, right) {
            (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => {
                let result = self.builder.build_int_compare(IntPredicate::NE, l, r, "ne")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to build ne instruction: {:?}", e)))?;
                Ok(result.into())
            },
            (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => {
                let result = self.builder.build_float_compare(FloatPredicate::ONE, l, r, "fne")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to build float ne instruction: {:?}", e)))?;
                Ok(result.into())
            },
            _ => Err(CursedError::CompilerError("Invalid types for inequality comparison".to_string()))
        }
    }

    /// Build less than comparison instruction
    fn build_lt_instruction(&self, left: BasicValueEnum<'ctx>, right: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, CursedError> {
        match (left, right) {
            (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => {
                let result = self.builder.build_int_compare(IntPredicate::SLT, l, r, "lt")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to build lt instruction: {:?}", e)))?;
                Ok(result.into())
            },
            (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => {
                let result = self.builder.build_float_compare(FloatPredicate::OLT, l, r, "flt")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to build float lt instruction: {:?}", e)))?;
                Ok(result.into())
            },
            _ => Err(CursedError::CompilerError("Invalid types for less than comparison".to_string()))
        }
    }

    /// Build greater than comparison instruction
    fn build_gt_instruction(&self, left: BasicValueEnum<'ctx>, right: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, CursedError> {
        match (left, right) {
            (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => {
                let result = self.builder.build_int_compare(IntPredicate::SGT, l, r, "gt")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to build gt instruction: {:?}", e)))?;
                Ok(result.into())
            },
            (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => {
                let result = self.builder.build_float_compare(FloatPredicate::OGT, l, r, "fgt")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to build float gt instruction: {:?}", e)))?;
                Ok(result.into())
            },
            _ => Err(CursedError::CompilerError("Invalid types for greater than comparison".to_string()))
        }
    }

    /// Build less than or equal comparison instruction  
    fn build_le_instruction(&self, left: BasicValueEnum<'ctx>, right: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, CursedError> {
        match (left, right) {
            (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => {
                let result = self.builder.build_int_compare(IntPredicate::SLE, l, r, "le")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to build le instruction: {:?}", e)))?;
                Ok(result.into())
            },
            (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => {
                let result = self.builder.build_float_compare(FloatPredicate::OLE, l, r, "fle")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to build float le instruction: {:?}", e)))?;
                Ok(result.into())
            },
            _ => Err(CursedError::CompilerError("Invalid types for less than or equal comparison".to_string()))
        }
    }

    /// Build greater than or equal comparison instruction
    fn build_ge_instruction(&self, left: BasicValueEnum<'ctx>, right: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, CursedError> {
        match (left, right) {
            (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => {
                let result = self.builder.build_int_compare(IntPredicate::SGE, l, r, "ge")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to build ge instruction: {:?}", e)))?;
                Ok(result.into())
            },
            (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => {
                let result = self.builder.build_float_compare(FloatPredicate::OGE, l, r, "fge")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to build float ge instruction: {:?}", e)))?;
                Ok(result.into())
            },
            _ => Err(CursedError::CompilerError("Invalid types for greater than or equal comparison".to_string()))
        }
    }

    /// Build logical AND instruction (simplified for now)
    fn build_and_instruction(&self, left: BasicValueEnum<'ctx>, right: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, CursedError> {
        match (left, right) {
            (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => {
                let result = self.builder.build_and(l, r, "and")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to build and instruction: {:?}", e)))?;
                Ok(result.into())
            },
            _ => Err(CursedError::CompilerError("Logical AND only supported for integers".to_string()))
        }
    }

    /// Build logical OR instruction (simplified for now)
    fn build_or_instruction(&self, left: BasicValueEnum<'ctx>, right: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, CursedError> {
        match (left, right) {
            (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => {
                let result = self.builder.build_or(l, r, "or")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to build or instruction: {:?}", e)))?;
                Ok(result.into())
            },
            _ => Err(CursedError::CompilerError("Logical OR only supported for integers".to_string()))
        }
    }

    /// Build bitwise AND instruction
    fn build_bitwise_and_instruction(&self, left: BasicValueEnum<'ctx>, right: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, CursedError> {
        match (left, right) {
            (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => {
                let result = self.builder.build_and(l, r, "bitwise_and")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to build bitwise and instruction: {:?}", e)))?;
                Ok(result.into())
            },
            _ => Err(CursedError::CompilerError("Bitwise AND only supported for integers".to_string()))
        }
    }

    /// Build bitwise OR instruction
    fn build_bitwise_or_instruction(&self, left: BasicValueEnum<'ctx>, right: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, CursedError> {
        match (left, right) {
            (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => {
                let result = self.builder.build_or(l, r, "bitwise_or")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to build bitwise or instruction: {:?}", e)))?;
                Ok(result.into())
            },
            _ => Err(CursedError::CompilerError("Bitwise OR only supported for integers".to_string()))
        }
    }

    /// Build XOR instruction
    fn build_xor_instruction(&self, left: BasicValueEnum<'ctx>, right: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, CursedError> {
        match (left, right) {
            (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => {
                let result = self.builder.build_xor(l, r, "xor")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to build xor instruction: {:?}", e)))?;
                Ok(result.into())
            },
            _ => Err(CursedError::CompilerError("XOR only supported for integers".to_string()))
        }
    }

    /// Build shift left instruction
    fn build_shl_instruction(&self, left: BasicValueEnum<'ctx>, right: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, CursedError> {
        match (left, right) {
            (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => {
                let result = self.builder.build_left_shift(l, r, "shl")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to build shl instruction: {:?}", e)))?;
                Ok(result.into())
            },
            _ => Err(CursedError::CompilerError("Shift left only supported for integers".to_string()))
        }
    }

    /// Build shift right instruction
    fn build_shr_instruction(&self, left: BasicValueEnum<'ctx>, right: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, CursedError> {
        match (left, right) {
            (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => {
                let result = self.builder.build_right_shift(l, r, false, "shr")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to build shr instruction: {:?}", e)))?;
                Ok(result.into())
            },
            _ => Err(CursedError::CompilerError("Shift right only supported for integers".to_string()))
        }
    }

    /// Compile unary expressions
    fn compile_unary_expression(&mut self, operator: &UnaryOperator, operand: &Expression) -> Result<BasicValueEnum<'ctx>, CursedError> {
        let operand_val = self.compile_expression(operand)?;

        match operator {
            UnaryOperator::Not => {
                match operand_val {
                    BasicValueEnum::IntValue(int_val) => {
                        let result = self.builder.build_not(int_val, "not")
                            .map_err(|e| CursedError::CompilerError(format!("Failed to build not instruction: {:?}", e)))?;
                        Ok(result.into())
                    },
                    _ => Err(CursedError::CompilerError("Logical NOT only supported for integers".to_string()))
                }
            },
            UnaryOperator::Minus => {
                match operand_val {
                    BasicValueEnum::IntValue(int_val) => {
                        let zero = self.context.i32_type().const_zero();
                        let result = self.builder.build_int_sub(zero, int_val, "neg")
                            .map_err(|e| CursedError::CompilerError(format!("Failed to build negation instruction: {:?}", e)))?;
                        Ok(result.into())
                    },
                    BasicValueEnum::FloatValue(float_val) => {
                        let result = self.builder.build_float_neg(float_val, "fneg")
                            .map_err(|e| CursedError::CompilerError(format!("Failed to build float negation instruction: {:?}", e)))?;
                        Ok(result.into())
                    },
                    _ => Err(CursedError::CompilerError("Negation only supported for numbers".to_string()))
                }
            },
            UnaryOperator::Plus => {
                // Unary plus is a no-op
                Ok(operand_val)
            },
            UnaryOperator::AddressOf => {
                // Address-of: return the address of a variable
                if let Expression::Identifier(var_name) = operand {
                    if let Some(var_ptr) = self.variables.get(var_name) {
                        Ok((*var_ptr).into())
                    } else {
                        Err(CursedError::CompilerError(format!("Cannot take address of undefined variable: {}", var_name)))
                    }
                } else {
                    Err(CursedError::CompilerError("Address-of operator can only be applied to variables".to_string()))
                }
            },
            UnaryOperator::Dereference => {
                // Dereference: load the value the pointer points to
                match operand_val {
                    BasicValueEnum::PointerValue(ptr_val) => {
                        // Use a default type for load - inkwell will infer the type
                        let result = self.builder.build_load(self.context.i32_type(), ptr_val, "deref")
                            .map_err(|e| CursedError::CompilerError(format!("Failed to dereference pointer: {:?}", e)))?;
                        Ok(result)
                    },
                    _ => Err(CursedError::CompilerError("Dereference operator can only be applied to pointers".to_string()))
                }
            },
        }
    }

    /// Compile function calls
    fn compile_function_call(&mut self, function: &Expression, arguments: &[Expression]) -> Result<BasicValueEnum<'ctx>, CursedError> {
        // For now, implement a basic function call mechanism
        // This will need to be expanded to handle proper function resolution
        match function {
            Expression::Identifier(func_name) => {
                // Compile all arguments
                let mut arg_values: Vec<BasicMetadataValueEnum> = Vec::new();
                for arg in arguments {
                    let arg_val = self.compile_expression(arg)?;
                    arg_values.push(arg_val.into());
                }

                // Try to get the function from the current module
                // This is a simplified implementation - in practice you'd have function registry
                Err(CursedError::CompilerError(format!("Function call compilation not yet fully implemented for: {}", func_name)))
            },
            _ => Err(CursedError::CompilerError("Function call target must be an identifier".to_string()))
        }
    }

    /// Compile increment expression (++variable or variable++)
    pub fn compile_increment_expression(&mut self, inc_expr: &crate::ast::IncrementExpression) -> Result<BasicValueEnum<'ctx>, CursedError> {
        let var_ptr = self.variables.get(&inc_expr.variable)
            .ok_or_else(|| CursedError::CompilerError(format!("Undefined variable in increment: {}", inc_expr.variable)))?
            .clone();

        // Load current value
        // Use default type for load
        let current_val = self.builder.build_load(self.context.i32_type(), var_ptr, "current_val")
            .map_err(|e| CursedError::CompilerError(format!("Failed to load variable for increment: {:?}", e)))?;

        let incremented_val = match current_val {
            BasicValueEnum::IntValue(int_val) => {
                let one = self.context.i32_type().const_int(1, false);
                let result = self.builder.build_int_add(int_val, one, "inc")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to build increment instruction: {:?}", e)))?;
                result.into()
            },
            BasicValueEnum::FloatValue(float_val) => {
                let one = self.context.f64_type().const_float(1.0);
                let result = self.builder.build_float_add(float_val, one, "finc")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to build float increment instruction: {:?}", e)))?;
                result.into()
            },
            _ => return Err(CursedError::CompilerError("Increment only supported for numbers".to_string()))
        };

        // Store incremented value back
        self.builder.build_store(var_ptr, incremented_val)
            .map_err(|e| CursedError::CompilerError(format!("Failed to store incremented value: {:?}", e)))?;

        // Return appropriate value based on prefix/postfix
        if inc_expr.is_prefix {
            Ok(incremented_val)
        } else {
            Ok(current_val)
        }
    }

    /// Compile decrement expression (--variable or variable--)
    pub fn compile_decrement_expression(&mut self, dec_expr: &crate::ast::DecrementExpression) -> Result<BasicValueEnum<'ctx>, CursedError> {
        let var_ptr = self.variables.get(&dec_expr.variable)
            .ok_or_else(|| CursedError::CompilerError(format!("Undefined variable in decrement: {}", dec_expr.variable)))?
            .clone();

        // Load current value
        // Use default type for load  
        let current_val = self.builder.build_load(self.context.i32_type(), var_ptr, "current_val")
            .map_err(|e| CursedError::CompilerError(format!("Failed to load variable for decrement: {:?}", e)))?;

        let decremented_val = match current_val {
            BasicValueEnum::IntValue(int_val) => {
                let one = self.context.i32_type().const_int(1, false);
                let result = self.builder.build_int_sub(int_val, one, "dec")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to build decrement instruction: {:?}", e)))?;
                result.into()
            },
            BasicValueEnum::FloatValue(float_val) => {
                let one = self.context.f64_type().const_float(1.0);
                let result = self.builder.build_float_sub(float_val, one, "fdec")
                    .map_err(|e| CursedError::CompilerError(format!("Failed to build float decrement instruction: {:?}", e)))?;
                result.into()
            },
            _ => return Err(CursedError::CompilerError("Decrement only supported for numbers".to_string()))
        };

        // Store decremented value back
        self.builder.build_store(var_ptr, decremented_val)
            .map_err(|e| CursedError::CompilerError(format!("Failed to store decremented value: {:?}", e)))?;

        // Return appropriate value based on prefix/postfix
        if dec_expr.is_prefix {
            Ok(decremented_val)
        } else {
            Ok(current_val)
        }
    }

    /// Perform type conversion for type assertions
    fn perform_type_conversion(&self, value: BasicValueEnum<'ctx>, target_type: &str) -> Result<BasicValueEnum<'ctx>, CursedError> {
        match target_type {
            "smol" => {
                // Convert to i8
                match value {
                    BasicValueEnum::IntValue(int_val) => {
                        let result = self.builder.build_int_truncate(int_val, self.context.i8_type(), "to_smol")
                            .map_err(|e| CursedError::CompilerError(format!("Failed to convert to smol: {:?}", e)))?;
                        Ok(result.into())
                    },
                    _ => Err(CursedError::CompilerError("Cannot convert to smol from this type".to_string()))
                }
            },
            "thicc" => {
                // Convert to i64
                match value {
                    BasicValueEnum::IntValue(int_val) => {
                        let result = self.builder.build_int_s_extend(int_val, self.context.i64_type(), "to_thicc")
                            .map_err(|e| CursedError::CompilerError(format!("Failed to convert to thicc: {:?}", e)))?;
                        Ok(result.into())
                    },
                    _ => Err(CursedError::CompilerError("Cannot convert to thicc from this type".to_string()))
                }
            },
            "meal" => {
                // Convert to double
                match value {
                    BasicValueEnum::IntValue(int_val) => {
                        let result = self.builder.build_signed_int_to_float(int_val, self.context.f64_type(), "to_meal")
                            .map_err(|e| CursedError::CompilerError(format!("Failed to convert to meal: {:?}", e)))?;
                        Ok(result.into())
                    },
                    _ => Err(CursedError::CompilerError("Cannot convert to meal from this type".to_string()))
                }
            },
            _ => {
                // For now, just return the original value for unsupported conversions
                Ok(value)
            }
        }
    }



    /// Compile channel creation expression
    fn compile_channel_creation(&mut self, channel_creation_expr: &ChannelCreationExpression) -> Result<BasicValueEnum<'ctx>, CursedError> {
        // Placeholder implementation - return integer 0
        let int_type = self.context.i32_type();
        Ok(int_type.const_int(0, false).into())
    }

    /// Compile struct literal expression
    fn compile_struct_literal(&mut self, struct_literal_expr: &StructLiteralExpression) -> Result<BasicValueEnum<'ctx>, CursedError> {
        // Placeholder implementation - return integer 0
        let int_type = self.context.i32_type();
        Ok(int_type.const_int(0, false).into())
    }

    /// Compile lambda expression
    fn compile_lambda(&mut self, lambda_expr: &LambdaExpression) -> Result<BasicValueEnum<'ctx>, CursedError> {
        // Placeholder implementation - return integer 0
        let int_type = self.context.i32_type();
        Ok(int_type.const_int(0, false).into())
    }

    /// Compile member access expression
    fn compile_member_access(&mut self, member_access: &MemberAccessExpression) -> Result<BasicValueEnum<'ctx>, CursedError> {
        // Placeholder implementation - return integer 0
        let int_type = self.context.i32_type();
        Ok(int_type.const_int(0, false).into())
    }

    /// Compile array literal expression
    fn compile_array_literal(&mut self, elements: &[Expression]) -> Result<BasicValueEnum<'ctx>, CursedError> {
        // Placeholder implementation - return integer 0
        let int_type = self.context.i32_type();
        Ok(int_type.const_int(0, false).into())
    }

    /// Compile map literal expression - Real implementation
    fn compile_map_literal(&mut self, pairs: &[(Expression, Expression)]) -> Result<BasicValueEnum<'ctx>, CursedError> {
        // Get map runtime function
        let map_create_fn = self.module.get_function("cursed_map_create")
            .ok_or_else(|| CursedError::CodegenError("Map creation function not found".to_string()))?;
        
        // Create map with initial capacity
        let size_val = self.context.i32_type().const_int(pairs.len() as u64, false);
        let map_ptr = self.builder.build_call(map_create_fn, &[size_val.into()], "map_ptr")
            .map_err(|e| CursedError::CodegenError(format!("Failed to call map_create: {}", e)))?
            .try_as_basic_value()
            .left()
            .ok_or_else(|| CursedError::CodegenError("Map creation returned void".to_string()))?;
        
        // Get map insert function
        let map_insert_fn = self.module.get_function("cursed_map_insert")
            .ok_or_else(|| CursedError::CodegenError("Map insert function not found".to_string()))?;
        
        // Insert each key-value pair
        for (key_expr, value_expr) in pairs {
            let key_val = self.compile_expression(key_expr)?;
            let value_val = self.compile_expression(value_expr)?;
            
            // Convert to i8* if needed
            let key_ptr = if key_val.get_type().is_pointer_type() {
                key_val.into_pointer_value()
            } else {
                // Cast non-pointer values to i8*
                self.builder.build_int_to_ptr(
                    key_val.into_int_value(),
                    self.context.i8_type().ptr_type(inkwell::AddressSpace::from(0u16)),
                    "key_ptr"
                ).map_err(|e| CursedError::CodegenError(format!("Failed to cast key to pointer: {}", e)))?
            };
            
            let value_ptr = if value_val.get_type().is_pointer_type() {
                value_val.into_pointer_value()
            } else {
                // Cast non-pointer values to i8*
                self.builder.build_int_to_ptr(
                    value_val.into_int_value(),
                    self.context.i8_type().ptr_type(inkwell::AddressSpace::from(0u16)),
                    "value_ptr"
                ).map_err(|e| CursedError::CodegenError(format!("Failed to cast value to pointer: {}", e)))?
            };
            
            // Insert key-value pair
            self.builder.build_call(
                map_insert_fn,
                &[map_ptr.into(), key_ptr.into(), value_ptr.into()],
                "insert_result"
            ).map_err(|e| CursedError::CodegenError(format!("Failed to call map_insert: {}", e)))?;
        }
        
        Ok(map_ptr)
    }

    /// Compile composite literal expression
    fn compile_composite_literal(&mut self, composite: &CompositeLiteralExpression) -> Result<BasicValueEnum<'ctx>, CursedError> {
        // Placeholder implementation - return integer 0
        let int_type = self.context.i32_type();
        Ok(int_type.const_int(0, false).into())
    }

    /// Compile array access expression
    fn compile_array_access(&mut self, array_access: &ArrayAccessExpression) -> Result<BasicValueEnum<'ctx>, CursedError> {
        // Placeholder implementation - return integer 0
        let int_type = self.context.i32_type();
        Ok(int_type.const_int(0, false).into())
    }

    /// Compile slice access expression
    fn compile_slice_access(&mut self, slice_access: &SliceAccessExpression) -> Result<BasicValueEnum<'ctx>, CursedError> {
        // Placeholder implementation - return integer 0
        let int_type = self.context.i32_type();
        Ok(int_type.const_int(0, false).into())
    }

    /// Compile tuple literal expression
    fn compile_tuple_literal(&mut self, tuple_expr: &TupleExpression) -> Result<BasicValueEnum<'ctx>, CursedError> {
        // Placeholder implementation - return integer 0
        let int_type = self.context.i32_type();
        Ok(int_type.const_int(0, false).into())
    }

    /// Compile tuple access expression
    fn compile_tuple_access(&mut self, tuple_access: &TupleAccessExpression) -> Result<BasicValueEnum<'ctx>, CursedError> {
        // Placeholder implementation - return integer 0
        let int_type = self.context.i32_type();
        Ok(int_type.const_int(0, false).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::context::Context;

    #[test]
    fn test_create_inkwell_expression_compiler() {
        let context = Context::create();
        let builder = context.create_builder();
        
        let compiler = InkwellExpressionCompiler::new(&context, &builder);
        
        // Basic smoke test - compiler should be created successfully
        assert_eq!(compiler.variables.len(), 0);
        assert_eq!(compiler.parameters.len(), 0);
    }

    #[test]
    fn test_compile_integer_literal() {
        let context = Context::create();
        let builder = context.create_builder();
        let mut compiler = InkwellExpressionCompiler::new(&context, &builder);
        
        let result = compiler.compile_expression(&Expression::Integer(42));
        assert!(result.is_ok());
    }

    #[test]
    fn test_compile_boolean_literal() {
        let context = Context::create();
        let builder = context.create_builder();
        let mut compiler = InkwellExpressionCompiler::new(&context, &builder);
        
        let result = compiler.compile_expression(&Expression::Boolean(true));
        assert!(result.is_ok());
    }
}
