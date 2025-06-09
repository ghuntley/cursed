//! Basic expression handlers for LLVM code generation
//! 
//! This module implements the compilation of basic expressions like literals, 
//! arithmetic operations, and comparison operations for the LLVM code generator.

use inkwell::values::BasicValueEnum;
use inkwell::IntPredicate;
use inkwell::FloatPredicate;

use crate::ast::traits::{Expression, Node};
use crate::ast::expressions::literals::{IntegerLiteral, FloatLiteral, BooleanLiteral, StringLiteral, NilLiteral};
use crate::ast::expressions::{InfixExpression, PrefixExpression, AssignmentExpression};
use crate::ast::expressions::identifiers::Identifier;
use crate::error::Error;

use super::context::LlvmCodeGenerator;
use super::expression::ExpressionCompilation;
use super::assignment::AssignmentCompilation;
use super::variables::VariableHandling;
use super::string_type::CursedStringType;
use super::bool_conversions::BoolConversions;
use super::nil_operations::{NilOperations, NilOperationsExtension};

/// Trait for handling basic expression operations
pub trait BasicExpressionOperations<'ctx> {
    /// Compile a basic expression (literals, arithmetic operations)
    fn compile_basic_expression(&mut self, expr: &dyn Expression) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Compile an integer literal
    fn compile_integer_literal(&mut self, lit: &IntegerLiteral) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Compile a float literal
    fn compile_float_literal(&mut self, lit: &FloatLiteral) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Compile a boolean literal
    fn compile_boolean_literal(&mut self, lit: &BooleanLiteral) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Compile a string literal
    fn compile_string_literal(&mut self, lit: &StringLiteral) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Compile a nil literal expression
    fn compile_nil_literal_expression(&mut self, lit: &NilLiteral) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Compile an infix expression (a + b, a * b, etc.)
    fn compile_infix_expression(&mut self, expr: &InfixExpression) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Compile a prefix expression (-a, !b, etc.)
    fn compile_prefix_expression(&mut self, expr: &PrefixExpression) -> Result<BasicValueEnum<'ctx>, Error>;
}

impl<'ctx> BasicExpressionOperations<'ctx> for LlvmCodeGenerator<'ctx> {
    fn compile_basic_expression(&mut self, expr: &dyn Expression) -> Result<BasicValueEnum<'ctx>, Error> {
        let any = expr.as_any();
        
        // Handle literals
        if let Some(lit) = any.downcast_ref::<IntegerLiteral>() {
            return self.compile_integer_literal(lit);
        }
        
        if let Some(lit) = any.downcast_ref::<FloatLiteral>() {
            return self.compile_float_literal(lit);
        }
        
        if let Some(lit) = any.downcast_ref::<BooleanLiteral>() {
            return self.compile_boolean_literal(lit);
        }
        
        if let Some(lit) = any.downcast_ref::<StringLiteral>() {
            return self.compile_string_literal(lit);
        }
        
        if let Some(lit) = any.downcast_ref::<NilLiteral>() {
            return self.compile_nil_literal_expression(lit);
        }
        
        // Handle operations
        if let Some(expr) = any.downcast_ref::<InfixExpression>() {
            return self.compile_infix_expression(expr);
        }
        
        if let Some(expr) = any.downcast_ref::<PrefixExpression>() {
            return self.compile_prefix_expression(expr);
        }
        
        // Check for pointer operations
        if let Some(ptr_type) = any.downcast_ref::<crate::ast::PointerType>() {
            return self.compile_pointer_type(ptr_type);
        }
        
        if let Some(ptr_deref) = any.downcast_ref::<crate::ast::PointerDereference>() {
            return self.compile_pointer_dereference(ptr_deref);
        }
        
        // If we reach here, we don't know how to compile this expression
        Err(Error::from_str(&format!("Unsupported expression type: {}", expr.string())))
    }
    
    fn compile_integer_literal(&mut self, lit: &IntegerLiteral) -> Result<BasicValueEnum<'ctx>, Error> {
        // Use the default integer type if set, otherwise use i64
        let int_type = self.get_default_integer_type();
        Ok(int_type.const_int(lit.value as u64, false).into())
    }
    
    fn compile_float_literal(&mut self, lit: &FloatLiteral) -> Result<BasicValueEnum<'ctx>, Error> {
        let f64_type = self.context().f64_type();
        Ok(f64_type.const_float(lit.value).into())
    }
    
    fn compile_boolean_literal(&mut self, lit: &BooleanLiteral) -> Result<BasicValueEnum<'ctx>, Error> {
        // Use the bool conversion trait for consistent bool handling
        Ok(self.create_bool_literal(lit.value))
    }
    
    fn compile_string_literal(&mut self, lit: &StringLiteral) -> Result<BasicValueEnum<'ctx>, Error> {
        // Use the CursedStringType to create a proper string struct
        let string_type = CursedStringType::new(self.context());
        
        // Create a global string literal with the string type
        let global_name = format!("str_literal_{}", self.string_literal_counter);
        self.string_literal_counter += 1;
        
        let string_value = string_type.create_string_literal(
            self.builder(),
            self.module(),
            &lit.value,
            &global_name
        ).map_err(|e| Error::from_str(&format!("Failed to create string literal: {}", e)))?;
        
        Ok(string_value.into())
    }
    
    fn compile_nil_literal_expression(&mut self, lit: &NilLiteral) -> Result<BasicValueEnum<'ctx>, Error> {
        // For nil literals, we need type context to determine the appropriate representation
        // Since we don't have explicit type context here, we'll use a generic null pointer
        // The type system should ensure this gets properly typed during assignment/comparison
        <Self as NilOperations<'ctx>>::compile_nil_literal(self, lit, None)
    }
    
    fn compile_infix_expression(&mut self, expr: &InfixExpression) -> Result<BasicValueEnum<'ctx>, Error> {
        println!("DEBUG: Infix operation: {} {} {}", expr.left.string(), expr.operator, expr.right.string());
        
        // Check for nil comparisons first
        let left_is_nil = expr.left.as_any().downcast_ref::<NilLiteral>().is_some();
        let right_is_nil = expr.right.as_any().downcast_ref::<NilLiteral>().is_some();
        
        if (left_is_nil || right_is_nil) && (expr.operator == "==" || expr.operator == "!=") {
            // Handle nil comparisons with special logic
            let left = self.compile_expression(&*expr.left)?;
            let right = self.compile_expression(&*expr.right)?;
            
            // TODO: We need type information to properly handle nil comparisons
            // For now, this is a placeholder that will be enhanced with type system integration
            return Err(Error::from_str("Nil comparisons require type system integration"));
        }
        
        // Compile the left and right expressions
        let left = self.compile_expression(&*expr.left)?;
        let right = self.compile_expression(&*expr.right)?;
        
        // Special handling for assignment operator
        if expr.operator == "=" {
            // For assignment, we need identifier on the left side
            if let Some(ident) = expr.left.as_any().downcast_ref::<Identifier>() {
                // Get the variable's type
                let var_name = &ident.value;
                tracing::debug!("Assignment to variable: {}", var_name);
                
                // Lookup variable type before evaluating the right side
                let var_type_opt = self.lookup_variable_type(var_name);
                
                // If we found the variable's type, we can attempt type coercion for the assignment
                if let Some(var_type) = var_type_opt {
                    tracing::debug!("Variable has type {:?}, value has type {:?}", var_type, right.get_type());
                    
                    // Check if types match or need coercion
                    let store_value = if right.get_type() != var_type {
                        // Attempt type coercion
                        match (var_type, right) {
                            // Int to Float conversion
                            (t, v) if t.is_float_type() && v.is_int_value() => {
                                tracing::debug!("Coercing int to float for assignment");
                                let int_val = v.into_int_value();
                                let float_val = self.builder()
                                    .build_signed_int_to_float(int_val, t.into_float_type(), "int_to_float")
                                    .map_err(|e| Error::from_str(&format!("Failed to convert int to float: {}", e)))?;
                                float_val.into()
                            },
                            // Types are incompatible and we don't have a conversion rule
                            _ => {
                                return Err(Error::from_str(&format!(
                                    "Type mismatch in assignment: variable '{}' is {:?}, value is {:?}",
                                    var_name, var_type, right.get_type()
                                )));
                            }
                        }
                    } else {
                        // No coercion needed
                        right
                    };
                    
                    // Get the variable's pointer
                    if let Some(var_ptr) = self.lookup_variable(var_name) {
                        // Store the value
                        self.builder().build_store(var_ptr, store_value)
                            .map_err(|e| Error::from_str(&format!("Failed to store value: {}", e)))?;
                        
                        // Return the stored value
                        return Ok(store_value);
                    }
                }
                
                // Fall back to regular variable assignment if lookup failed
                return self.compile_variable_assignment(ident, right);
            } else {
                return Err(Error::from_str("Left side of assignment must be an identifier"));
            }
        }
        
        // Determine if we're working with integers or floats
        let (left_is_int, right_is_int) = (
            left.is_int_value() || left.is_pointer_value(),  // Pointers can be used in integer ops
            right.is_int_value() || right.is_pointer_value()
        );
        let (left_is_float, right_is_float) = (
            left.is_float_value(),
            right.is_float_value()
        );
        
        // Handle integer operations
        if left_is_int && right_is_int {
            let left_int = if left.is_int_value() {
                left.into_int_value()
            } else {
                // Convert pointer to integer
                let int_type = self.context().i64_type();
                self.builder().build_ptr_to_int(
                    left.into_pointer_value(),
                    int_type,
                    "ptr_to_int"
                ).map_err(|e| Error::from_str(&format!("Failed to convert pointer to int: {}", e)))?
            };
            
            let right_int = if right.is_int_value() {
                right.into_int_value()
            } else {
                // Convert pointer to integer
                let int_type = self.context().i64_type();
                self.builder().build_ptr_to_int(
                    right.into_pointer_value(),
                    int_type,
                    "ptr_to_int"
                ).map_err(|e| Error::from_str(&format!("Failed to convert pointer to int: {}", e)))?
            };
            
            match expr.operator.as_str() {
                "+" => {
                    let result = self.builder().build_int_add(left_int, right_int, "add");
                    match result {
                        Ok(value) => Ok(value.into()),
                        Err(e) => Err(Error::from_str(&format!("Failed to build int add: {}", e)))
                    }
                },
                "-" => {
                    let result = self.builder().build_int_sub(left_int, right_int, "sub");
                    match result {
                        Ok(value) => Ok(value.into()),
                        Err(e) => Err(Error::from_str(&format!("Failed to build int sub: {}", e)))
                    }
                },
                "*" => {
                    let result = self.builder().build_int_mul(left_int, right_int, "mul");
                    match result {
                        Ok(value) => Ok(value.into()),
                        Err(e) => Err(Error::from_str(&format!("Failed to build int mul: {}", e)))
                    }
                },
                "/" => {
                    // Check for division by zero
                    let zero = self.context().i64_type().const_int(0, false);
                    let is_div_by_zero = self.builder().build_int_compare(
                        IntPredicate::EQ,
                        right_int,
                        zero,
                        "is_div_by_zero"
                    ).map_err(|e| Error::from_str(&format!("Failed to check division by zero: {}", e)))?;
                    
                    // Create basic blocks for the division and error paths
                    let current_function = self.current_function()
                        .ok_or_else(|| Error::from_str("No current function"))?;
                    
                    let div_block = self.context().append_basic_block(current_function, "div");
                    let div_by_zero_block = self.context().append_basic_block(current_function, "div_by_zero");
                    let cont_block = self.context().append_basic_block(current_function, "cont");
                    
                    // Branch based on the division by zero check
                    self.builder().build_conditional_branch(is_div_by_zero, div_by_zero_block, div_block)
                        .map_err(|e| Error::from_str(&format!("Failed to build conditional branch: {}", e)))?;
                    
                    // Division block
                    self.builder().position_at_end(div_block);
                    let div_result = self.builder().build_int_signed_div(left_int, right_int, "div")
                        .map_err(|e| Error::from_str(&format!("Failed to build int div: {}", e)))?;
                    self.builder().build_unconditional_branch(cont_block)
                        .map_err(|e| Error::from_str(&format!("Failed to build branch: {}", e)))?;
                    
                    // Division by zero block - print error message and return 0
                    self.builder().position_at_end(div_by_zero_block);
                    let error_msg = self.builder().build_global_string_ptr("Runtime error: division by zero", "div_zero_msg")
                        .map_err(|e| Error::from_str(&format!("Failed to build error message: {}", e)))?;
                    
                    // Call printf to report the error
                    let printf_fn = self.module().get_function("printf")
                        .ok_or_else(|| Error::from_str("printf function not found"))?;
                    
                    self.builder().build_call(printf_fn, &[error_msg.as_pointer_value().into()], "printf_call")
                        .map_err(|e| Error::from_str(&format!("Failed to call printf: {}", e)))?;
                    
                    let zero_result = self.context().i64_type().const_int(0, false);
                    self.builder().build_unconditional_branch(cont_block)
                        .map_err(|e| Error::from_str(&format!("Failed to build branch: {}", e)))?;
                    
                    // Continuation block - phi node to select the result
                    self.builder().position_at_end(cont_block);
                    let phi = self.builder().build_phi(self.context().i64_type(), "div_result")
                        .map_err(|e| Error::from_str(&format!("Failed to build phi node: {}", e)))?;
                    
                    phi.add_incoming(&[
                        (&div_result, div_block),
                        (&zero_result, div_by_zero_block)
                    ]);
                    
                    Ok(phi.as_basic_value())
                },
                "%" => {
                    let result = self.builder().build_int_signed_rem(left_int, right_int, "rem");
                    match result {
                        Ok(value) => Ok(value.into()),
                        Err(e) => Err(Error::from_str(&format!("Failed to build int rem: {}", e)))
                    }
                },
                "==" => {
                    let result = self.builder().build_int_compare(
                        IntPredicate::EQ,
                        left_int,
                        right_int,
                        "eq"
                    );
                    match result {
                        Ok(value) => Ok(value.into()),
                        Err(e) => Err(Error::from_str(&format!("Failed to build int eq: {}", e)))
                    }
                },
                "!=" => {
                    let result = self.builder().build_int_compare(
                        IntPredicate::NE,
                        left_int,
                        right_int,
                        "ne"
                    );
                    match result {
                        Ok(value) => Ok(value.into()),
                        Err(e) => Err(Error::from_str(&format!("Failed to build int ne: {}", e)))
                    }
                },
                "<" => {
                    let result = self.builder().build_int_compare(
                        IntPredicate::SLT,
                        left_int,
                        right_int,
                        "lt"
                    );
                    match result {
                        Ok(value) => Ok(value.into()),
                        Err(e) => Err(Error::from_str(&format!("Failed to build int lt: {}", e)))
                    }
                },
                "<=" => {
                    let result = self.builder().build_int_compare(
                        IntPredicate::SLE,
                        left_int,
                        right_int,
                        "le"
                    );
                    match result {
                        Ok(value) => Ok(value.into()),
                        Err(e) => Err(Error::from_str(&format!("Failed to build int le: {}", e)))
                    }
                },
                ">" => {
                    let result = self.builder().build_int_compare(
                        IntPredicate::SGT,
                        left_int,
                        right_int,
                        "gt"
                    );
                    match result {
                        Ok(value) => Ok(value.into()),
                        Err(e) => Err(Error::from_str(&format!("Failed to build int gt: {}", e)))
                    }
                },
                ">=" => {
                    let result = self.builder().build_int_compare(
                        IntPredicate::SGE,
                        left_int,
                        right_int,
                        "ge"
                    );
                    match result {
                        Ok(value) => Ok(value.into()),
                        Err(e) => Err(Error::from_str(&format!("Failed to build int ge: {}", e)))
                    }
                },
                "&&" => {
                    // Convert both operands to bool and perform logical AND
                    let left_bool = self.convert_value_to_bool(left.into())?;
                    let right_bool = self.convert_value_to_bool(right.into())?;
                    self.bool_logical_and(left_bool, right_bool)
                },
                "||" => {
                    // Convert both operands to bool and perform logical OR
                    let left_bool = self.convert_value_to_bool(left.into())?;
                    let right_bool = self.convert_value_to_bool(right.into())?;
                    self.bool_logical_or(left_bool, right_bool)
                },
                _ => Err(Error::from_str(&format!("Unsupported integer operator: {}", expr.operator)))
            }
        }
        // Handle float operations
        else if left_is_float && right_is_float {
            let left_float = left.into_float_value();
            let right_float = right.into_float_value();
            
            match expr.operator.as_str() {
                "+" => {
                    let result = self.builder().build_float_add(left_float, right_float, "fadd");
                    match result {
                        Ok(value) => Ok(value.into()),
                        Err(e) => Err(Error::from_str(&format!("Failed to build float add: {}", e)))
                    }
                },
                "-" => {
                    let result = self.builder().build_float_sub(left_float, right_float, "fsub");
                    match result {
                        Ok(value) => Ok(value.into()),
                        Err(e) => Err(Error::from_str(&format!("Failed to build float sub: {}", e)))
                    }
                },
                "*" => {
                    let result = self.builder().build_float_mul(left_float, right_float, "fmul");
                    match result {
                        Ok(value) => Ok(value.into()),
                        Err(e) => Err(Error::from_str(&format!("Failed to build float mul: {}", e)))
                    }
                },
                "/" => {
                    let result = self.builder().build_float_div(left_float, right_float, "fdiv");
                    match result {
                        Ok(value) => Ok(value.into()),
                        Err(e) => Err(Error::from_str(&format!("Failed to build float div: {}", e)))
                    }
                },
                "==" => {
                    let result = self.builder().build_float_compare(
                        FloatPredicate::OEQ,
                        left_float,
                        right_float,
                        "feq"
                    );
                    match result {
                        Ok(value) => Ok(value.into()),
                        Err(e) => Err(Error::from_str(&format!("Failed to build float eq: {}", e)))
                    }
                },
                "!=" => {
                    let result = self.builder().build_float_compare(
                        FloatPredicate::ONE,
                        left_float,
                        right_float,
                        "fne"
                    );
                    match result {
                        Ok(value) => Ok(value.into()),
                        Err(e) => Err(Error::from_str(&format!("Failed to build float ne: {}", e)))
                    }
                },
                "<" => {
                    let result = self.builder().build_float_compare(
                        FloatPredicate::OLT,
                        left_float,
                        right_float,
                        "flt"
                    );
                    match result {
                        Ok(value) => Ok(value.into()),
                        Err(e) => Err(Error::from_str(&format!("Failed to build float lt: {}", e)))
                    }
                },
                "<=" => {
                    let result = self.builder().build_float_compare(
                        FloatPredicate::OLE,
                        left_float,
                        right_float,
                        "fle"
                    );
                    match result {
                        Ok(value) => Ok(value.into()),
                        Err(e) => Err(Error::from_str(&format!("Failed to build float le: {}", e)))
                    }
                },
                ">" => {
                    let result = self.builder().build_float_compare(
                        FloatPredicate::OGT,
                        left_float,
                        right_float,
                        "fgt"
                    );
                    match result {
                        Ok(value) => Ok(value.into()),
                        Err(e) => Err(Error::from_str(&format!("Failed to build float gt: {}", e)))
                    }
                },
                ">=" => {
                    let result = self.builder().build_float_compare(
                        FloatPredicate::OGE,
                        left_float,
                        right_float,
                        "fge"
                    );
                    match result {
                        Ok(value) => Ok(value.into()),
                        Err(e) => Err(Error::from_str(&format!("Failed to build float ge: {}", e)))
                    }
                },
                "&&" => {
                    // Convert both operands to bool and perform logical AND
                    let left_bool = self.convert_value_to_bool(left.into())?;
                    let right_bool = self.convert_value_to_bool(right.into())?;
                    self.bool_logical_and(left_bool, right_bool)
                },
                "||" => {
                    // Convert both operands to bool and perform logical OR
                    let left_bool = self.convert_value_to_bool(left.into())?;
                    let right_bool = self.convert_value_to_bool(right.into())?;
                    self.bool_logical_or(left_bool, right_bool)
                },
                _ => Err(Error::from_str(&format!("Unsupported float operator: {}", expr.operator)))
            }
        }
        // Type mismatch or unsupported types
        else {
            Err(Error::from_str(&format!(
                "Type mismatch in infix expression: left is {}, right is {}",
                if left_is_int { "integer" } else if left_is_float { "float" } else { "unknown" },
                if right_is_int { "integer" } else if right_is_float { "float" } else { "unknown" }
            )))
        }
    }
    
    fn compile_prefix_expression(&mut self, expr: &PrefixExpression) -> Result<BasicValueEnum<'ctx>, Error> {
        let right = self.compile_expression(&*expr.right)?;
        
        match expr.operator.as_str() {
            "-" => {
                if right.is_int_value() {
                    let right_int = right.into_int_value();
                    let zero = self.context().i64_type().const_int(0, false);
                    let result = self.builder().build_int_sub(zero, right_int, "neg");
                    match result {
                        Ok(value) => Ok(value.into()),
                        Err(e) => Err(Error::from_str(&format!("Failed to build int negation: {}", e)))
                    }
                } else if right.is_float_value() {
                    let right_float = right.into_float_value();
                    let result = self.builder().build_float_neg(right_float, "fneg");
                    match result {
                        Ok(value) => Ok(value.into()),
                        Err(e) => Err(Error::from_str(&format!("Failed to build float negation: {}", e)))
                    }
                } else {
                    Err(Error::from_str("Cannot negate non-numeric value"))
                }
            },
            "!" => {
                // Use bool conversion for logical NOT operation
                self.bool_logical_not(right)
            },
            _ => Err(Error::from_str(&format!("Unsupported prefix operator: {}", expr.operator)))
        }
    }
}

// Add the implementation to LlvmCodeGenerator that delegates to the trait
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Legacy compile_basic_expression that forwards to the trait implementation
    pub fn compile_basic_expression(&mut self, expr: &dyn Expression) -> Result<BasicValueEnum<'ctx>, Error> {
        <Self as BasicExpressionOperations<'ctx>>::compile_basic_expression(self, expr)
    }
}