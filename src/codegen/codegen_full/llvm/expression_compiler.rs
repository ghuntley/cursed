/// LLVM expression compilation for the CURSED programming language
/// 
/// This module provides comprehensive compilation of all AST expression types
/// to LLVM IR, including proper type handling, error reporting, and 
/// support for Gen Z slang syntax.

use crate::ast::traits::Expression;
use crate::ast::{
// };
use crate::error::CursedError;

// use crate::debug::SourceLocation;
use crate::type_system::{TypeSystem, TypeInference, TypeEnvironment};
use crate::codegen::llvm::type_system::{TypeCompilationContext, CompiledGenericInstance};
use crate::codegen::llvm::variable_management::VariableManager;
use inkwell::{
// };

use std::any::Any;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use tracing::{debug, info, warn, error, instrument};

/// LLVM value representation for compiled expressions
#[derive(Debug, Clone)]
pub struct LlvmValue {
    pub llvm_name: String, // Add llvm_name field for backward compatibility
impl LlvmValue {
    /// Create a new LLVM value with actual LLVM value
    pub fn new(value_type: LlvmType, llvm_value: BasicValueEnum<'static>, is_constant: bool) -> Self {
        // Generate a name based on the value type and properties
            if is_constant { "const" } else { "var" });
        
        Self {
        }
    }
    
    /// Create a new LLVM value with explicit name
    pub fn new_with_name(value_type: LlvmType, llvm_value: BasicValueEnum<'static>, is_constant: bool, llvm_name: String) -> Self {
        Self {
        }
    }
    
    /// Check if this is a struct value
    pub fn is_struct_value(&self) -> bool {
        self.llvm_value.is_struct_value()
    /// Check if this is a float value
    pub fn is_float_value(&self) -> bool {
        matches!(self.value_type, LlvmType::Float64) && self.llvm_value.is_float_value()
    /// Check if this is an int value
    pub fn is_int_value(&self) -> bool {
        matches!(self.value_type, LlvmType::Int32 | LlvmType::Int64) && self.llvm_value.is_int_value()
    /// Convert to int value
    pub fn into_int_value(self) -> IntValue<'static> {
        self.llvm_value.into_int_value()
    /// Convert to float value
    pub fn into_float_value(self) -> FloatValue<'static> {
        self.llvm_value.into_float_value()
    /// Convert to pointer value
    pub fn into_pointer_value(self) -> PointerValue<'static> {
        self.llvm_value.into_pointer_value()
    /// Get the underlying LLVM value
    pub fn as_basic_value(&self) -> BasicValueEnum<'static> {
        self.llvm_value
    }
}

/// LLVM type system mapping
#[derive(Debug, Clone, PartialEq)]
pub enum LlvmType {
    Function {
impl LlvmType {
    pub fn to_llvm_string(&self) -> String {
        match self {
            LlvmType::Array => "i8*".to_string(), // Arrays are represented as opaque pointers
            LlvmType::Object => "i8*".to_string(), // Objects are represented as opaque pointers
            LlvmType::Function { return_type, param_types } => {
                let params: Vec<String> = param_types.iter()
                    .map(|t| t.to_llvm_string())
                    .collect();
                format!("{} ({})", return_type.to_llvm_string(), params.join(", "))
            }
        }
    }
}

/// Expression compilation context
#[derive(Debug)]
pub struct ExpressionContext<'ctx> {
    /// Type compilation context for constraint resolution
    /// Generic method instantiation cache
    /// Variable manager for handling variable operations
impl<'ctx> ExpressionContext<'ctx> {
    pub fn new() -> Self {
        Self {
        }
    }

    /// Create context with type compilation support
    pub fn with_type_context(type_context: TypeCompilationContext) -> Self {
        Self {
        }
    }

    /// Set variable manager
    pub fn set_variable_manager(&mut self, variable_manager: Rc<RefCell<VariableManager<'ctx>>>) {
        self.variable_manager = Some(variable_manager);
    pub fn next_temp(&mut self) -> u32 {
        self.temp_counter += 1;
        self.temp_counter
    }
}

/// Main expression compiler
pub struct LlvmExpressionCompiler<'ctx> {
    /// LLVM context
    /// LLVM module
    /// LLVM builder
    /// Expression compilation context
    /// Variable manager
impl<'ctx> LlvmExpressionCompiler<'ctx> {
    pub fn new(
    ) -> Self {
        let mut context = ExpressionContext::new();
        context.set_variable_manager(variable_manager.clone());
        
        Self {
        }
    }
    
    /// Add external function declarations for error propagation
    #[instrument(skip(self))]
    pub fn add_error_propagation_declarations(&mut self) {
        debug!("Adding error propagation function declarations");
        
        // Declare error propagation runtime functions
        let void_type = self.llvm_context.void_type();
        let i8_ptr_type = self.llvm_context.i8_type().ptr_type(AddressSpace::default());
        let i32_type = self.llvm_context.i32_type();
        
        // cursed_error_propagation_init()
        let init_fn_type = void_type.fn_type(&[], false);
        self.module.add_function("cursed_error_propagation_init", init_fn_type, None);
        
        // cursed_error_propagation_cleanup()
        let cleanup_fn_type = void_type.fn_type(&[], false);
        self.module.add_function("cursed_error_propagation_cleanup", cleanup_fn_type, None);
        
        // cursed_error_propagation(i8*, i32, i32) -> i8*
        let propagate_fn_type = i8_ptr_type.fn_type(&[
        ], false);
        self.module.add_function("cursed_error_propagation", propagate_fn_type, None);
        
        // cursed_error_propagation_panic(i8*)
        let panic_fn_type = void_type.fn_type(&[i8_ptr_type.into()], false);
        self.module.add_function("cursed_error_propagation_panic", panic_fn_type, None);
        
        debug!("CursedError propagation declarations added successfully");
    /// Compile any expression to LLVM IR
    #[instrument(skip(self, expr))]
    pub fn compile_expression(&mut self, expr: &dyn Expression) -> crate::error::Result<()> {
        debug!("Compiling expression: {}", expr.string());
        
        // Try to downcast to specific expression types
        if let Some(literal) = expr.as_any().downcast_ref::<Literal>() {
            self.compile_literal(literal)
        } else if let Some(binary) = expr.as_any().downcast_ref::<BinaryExpression>() {
            self.compile_binary_expression(binary)
        } else if let Some(unary) = expr.as_any().downcast_ref::<UnaryExpression>() {
            self.compile_unary_expression(unary)
        } else if let Some(identifier) = expr.as_any().downcast_ref::<Identifier>() {
            self.compile_identifier(identifier)
        } else if let Some(call) = expr.as_any().downcast_ref::<CallExpression>() {
            self.compile_call_expression(call)
        } else if let Some(assignment) = expr.as_any().downcast_ref::<AssignmentExpression>() {
            self.compile_assignment_expression(assignment)
        } else if let Some(index) = expr.as_any().downcast_ref::<IndexExpression>() {
            self.compile_index_expression(index)
        } else if let Some(paren) = expr.as_any().downcast_ref::<ParenthesizedExpression>() {
            self.compile_parenthesized_expression(paren)
        } else if let Some(question_mark) = expr.as_any().downcast_ref::<QuestionMarkExpression>() {
            self.compile_question_mark_expression(question_mark)
        } else {
            error!("Unsupported expression type: {}", expr.string());
            Err(CursedError::Compile(format!(
                expr.string()
            )))
        }
    }
    
    /// Compile literal expressions
    #[instrument(skip(self, literal))]
    fn compile_literal(&mut self, literal: &Literal) -> crate::error::Result<()> {
        debug!("Compiling literal: {:?}", literal.value);
        
        match &literal.value {
            LiteralValue::Integer(value) => {
                debug!("Compiling integer literal: {}", value);
                let i64_type = self.llvm_context.i64_type();
                let const_value = i64_type.const_int(*value as u64, false);
                
                Ok(LlvmValue::new(
                ))
            }
            LiteralValue::Float(value) => {
                debug!("Compiling float literal: {}", value);
                let f64_type = self.llvm_context.f64_type();
                let const_value = f64_type.const_float(*value);
                
                Ok(LlvmValue::new(
                ))
            }
            LiteralValue::String(value) => {
                debug!("Compiling string literal: \"{}\"", value);
                
                // Create global string constant
                let string_bytes = value.as_bytes();
                let string_type = self.llvm_context.i8_type().array_type(string_bytes.len() as u32 + 1);
                let string_const = self.llvm_context.const_string(string_bytes, true);
                
                // Create unique global name
                let global_name = format!("str_{}", self.context.next_temp());
                let global = self.module.add_global(string_type, Some(AddressSpace::default()), &global_name);
                global.set_initializer(&string_const);
                global.set_constant(true);
                global.set_linkage(inkwell::module::Linkage::Private);
                
                // Get pointer to first element
                let zero = self.llvm_context.i32_type().const_zero();
                let gep = unsafe {
                    self.builder.build_gep(
                        "str_ptr"
                    ).map_err(|e| CursedError::Compile(format!("Failed to build GEP for string: {:?}", e)))?
                
                Ok(LlvmValue::new(
                ))
            }
            LiteralValue::Boolean(value) => {
                debug!("Compiling boolean literal: {}", value);
                let i1_type = self.llvm_context.bool_type();
                let const_value = i1_type.const_int(*value as u64, false);
                
                Ok(LlvmValue::new(
                ))
            }
            LiteralValue::Nil => {
                debug!("Compiling nil literal");
                let i8_ptr_type = self.llvm_context.i8_type().ptr_type(AddressSpace::default());
                let null_ptr = i8_ptr_type.const_null();
                
                Ok(LlvmValue::new(
                ))
            }
        }
    /// Compile binary expressions (arithmetic, logical, comparison)
    #[instrument(skip(self, binary))]
    fn compile_binary_expression(&mut self, binary: &BinaryExpression) -> crate::error::Result<()> {
               binary.left.string(), binary.operator, binary.right.string());
        
        let left_val = self.compile_expression(binary.left.as_ref())?;
        let right_val = self.compile_expression(binary.right.as_ref())?;
        
        // Type checking and coercion
        let result_type = self.resolve_binary_type(&left_val.value_type, &right_val.value_type, &binary.operator)?;
        
        // Generate appropriate LLVM instruction based on operator and types
        let result_value = match binary.operator.as_str() {
            // Arithmetic operators
            "+" => match (&left_val.value_type, &right_val.value_type) {
                (LlvmType::Int64, LlvmType::Int64) => {
                    let left_int = left_val.llvm_value.into_int_value();
                    let right_int = right_val.llvm_value.into_int_value();
                    self.builder.build_int_add(left_int, right_int, "add_tmp")
                        .map_err(|e| CursedError::Compile(format!("Failed to build int add: {:?}", e)))?
                        .into()
                (LlvmType::Float64, LlvmType::Float64) => {
                    let left_float = left_val.llvm_value.into_float_value();
                    let right_float = right_val.llvm_value.into_float_value();
                    self.builder.build_float_add(left_float, right_float, "fadd_tmp")
                        .map_err(|e| CursedError::Compile(format!("Failed to build float add: {:?}", e)))?
                        .into()
            "-" => match (&left_val.value_type, &right_val.value_type) {
                (LlvmType::Int64, LlvmType::Int64) => {
                    let left_int = left_val.llvm_value.into_int_value();
                    let right_int = right_val.llvm_value.into_int_value();
                    self.builder.build_int_sub(left_int, right_int, "sub_tmp")
                        .map_err(|e| CursedError::Compile(format!("Failed to build int sub: {:?}", e)))?
                        .into()
                (LlvmType::Float64, LlvmType::Float64) => {
                    let left_float = left_val.llvm_value.into_float_value();
                    let right_float = right_val.llvm_value.into_float_value();
                    self.builder.build_float_sub(left_float, right_float, "fsub_tmp")
                        .map_err(|e| CursedError::Compile(format!("Failed to build float sub: {:?}", e)))?
                        .into()
            "*" => match (&left_val.value_type, &right_val.value_type) {
                (LlvmType::Int64, LlvmType::Int64) => {
                    let left_int = left_val.llvm_value.into_int_value();
                    let right_int = right_val.llvm_value.into_int_value();
                    self.builder.build_int_mul(left_int, right_int, "mul_tmp")
                        .map_err(|e| CursedError::Compile(format!("Failed to build int mul: {:?}", e)))?
                        .into()
                (LlvmType::Float64, LlvmType::Float64) => {
                    let left_float = left_val.llvm_value.into_float_value();
                    let right_float = right_val.llvm_value.into_float_value();
                    self.builder.build_float_mul(left_float, right_float, "fmul_tmp")
                        .map_err(|e| CursedError::Compile(format!("Failed to build float mul: {:?}", e)))?
                        .into()
            "/" => match (&left_val.value_type, &right_val.value_type) {
                (LlvmType::Int64, LlvmType::Int64) => {
                    let left_int = left_val.llvm_value.into_int_value();
                    let right_int = right_val.llvm_value.into_int_value();
                    self.builder.build_int_signed_div(left_int, right_int, "div_tmp")
                        .map_err(|e| CursedError::Compile(format!("Failed to build int div: {:?}", e)))?
                        .into()
                (LlvmType::Float64, LlvmType::Float64) => {
                    let left_float = left_val.llvm_value.into_float_value();
                    let right_float = right_val.llvm_value.into_float_value();
                    self.builder.build_float_div(left_float, right_float, "fdiv_tmp")
                        .map_err(|e| CursedError::Compile(format!("Failed to build float div: {:?}", e)))?
                        .into()
            "%" => match (&left_val.value_type, &right_val.value_type) {
                (LlvmType::Int64, LlvmType::Int64) => {
                    let left_int = left_val.llvm_value.into_int_value();
                    let right_int = right_val.llvm_value.into_int_value();
                    self.builder.build_int_signed_rem(left_int, right_int, "rem_tmp")
                        .map_err(|e| CursedError::Compile(format!("Failed to build int rem: {:?}", e)))?
                        .into()
            
            // Comparison operators
            "==" => match (&left_val.value_type, &right_val.value_type) {
                (LlvmType::Int64, LlvmType::Int64) => {
                    let left_int = left_val.llvm_value.into_int_value();
                    let right_int = right_val.llvm_value.into_int_value();
                    self.builder.build_int_compare(IntPredicate::EQ, left_int, right_int, "eq_tmp")
                        .map_err(|e| CursedError::Compile(format!("Failed to build int compare: {:?}", e)))?
                        .into()
                (LlvmType::Float64, LlvmType::Float64) => {
                    let left_float = left_val.llvm_value.into_float_value();
                    let right_float = right_val.llvm_value.into_float_value();
                    self.builder.build_float_compare(FloatPredicate::OEQ, left_float, right_float, "feq_tmp")
                        .map_err(|e| CursedError::Compile(format!("Failed to build float compare: {:?}", e)))?
                        .into()
                (LlvmType::Boolean, LlvmType::Boolean) => {
                    let left_bool = left_val.llvm_value.into_int_value();
                    let right_bool = right_val.llvm_value.into_int_value();
                    self.builder.build_int_compare(IntPredicate::EQ, left_bool, right_bool, "beq_tmp")
                        .map_err(|e| CursedError::Compile(format!("Failed to build bool compare: {:?}", e)))?
                        .into()
            "!=" => match (&left_val.value_type, &right_val.value_type) {
                (LlvmType::Int64, LlvmType::Int64) => {
                    let left_int = left_val.llvm_value.into_int_value();
                    let right_int = right_val.llvm_value.into_int_value();
                    self.builder.build_int_compare(IntPredicate::NE, left_int, right_int, "ne_tmp")
                        .map_err(|e| CursedError::Compile(format!("Failed to build int compare: {:?}", e)))?
                        .into()
                (LlvmType::Float64, LlvmType::Float64) => {
                    let left_float = left_val.llvm_value.into_float_value();
                    let right_float = right_val.llvm_value.into_float_value();
                    self.builder.build_float_compare(FloatPredicate::ONE, left_float, right_float, "fne_tmp")
                        .map_err(|e| CursedError::Compile(format!("Failed to build float compare: {:?}", e)))?
                        .into()
                (LlvmType::Boolean, LlvmType::Boolean) => {
                    let left_bool = left_val.llvm_value.into_int_value();
                    let right_bool = right_val.llvm_value.into_int_value();
                    self.builder.build_int_compare(IntPredicate::NE, left_bool, right_bool, "bne_tmp")
                        .map_err(|e| CursedError::Compile(format!("Failed to build bool compare: {:?}", e)))?
                        .into()
            "<" => match (&left_val.value_type, &right_val.value_type) {
                (LlvmType::Int64, LlvmType::Int64) => {
                    let left_int = left_val.llvm_value.into_int_value();
                    let right_int = right_val.llvm_value.into_int_value();
                    self.builder.build_int_compare(IntPredicate::SLT, left_int, right_int, "lt_tmp")
                        .map_err(|e| CursedError::Compile(format!("Failed to build int compare: {:?}", e)))?
                        .into()
                (LlvmType::Float64, LlvmType::Float64) => {
                    let left_float = left_val.llvm_value.into_float_value();
                    let right_float = right_val.llvm_value.into_float_value();
                    self.builder.build_float_compare(FloatPredicate::OLT, left_float, right_float, "flt_tmp")
                        .map_err(|e| CursedError::Compile(format!("Failed to build float compare: {:?}", e)))?
                        .into()
            ">" => match (&left_val.value_type, &right_val.value_type) {
                (LlvmType::Int64, LlvmType::Int64) => {
                    let left_int = left_val.llvm_value.into_int_value();
                    let right_int = right_val.llvm_value.into_int_value();
                    self.builder.build_int_compare(IntPredicate::SGT, left_int, right_int, "gt_tmp")
                        .map_err(|e| CursedError::Compile(format!("Failed to build int compare: {:?}", e)))?
                        .into()
                (LlvmType::Float64, LlvmType::Float64) => {
                    let left_float = left_val.llvm_value.into_float_value();
                    let right_float = right_val.llvm_value.into_float_value();
                    self.builder.build_float_compare(FloatPredicate::OGT, left_float, right_float, "fgt_tmp")
                        .map_err(|e| CursedError::Compile(format!("Failed to build float compare: {:?}", e)))?
                        .into()
            "<=" => match (&left_val.value_type, &right_val.value_type) {
                (LlvmType::Int64, LlvmType::Int64) => {
                    let left_int = left_val.llvm_value.into_int_value();
                    let right_int = right_val.llvm_value.into_int_value();
                    self.builder.build_int_compare(IntPredicate::SLE, left_int, right_int, "le_tmp")
                        .map_err(|e| CursedError::Compile(format!("Failed to build int compare: {:?}", e)))?
                        .into()
                (LlvmType::Float64, LlvmType::Float64) => {
                    let left_float = left_val.llvm_value.into_float_value();
                    let right_float = right_val.llvm_value.into_float_value();
                    self.builder.build_float_compare(FloatPredicate::OLE, left_float, right_float, "fle_tmp")
                        .map_err(|e| CursedError::Compile(format!("Failed to build float compare: {:?}", e)))?
                        .into()
            ">=" => match (&left_val.value_type, &right_val.value_type) {
                (LlvmType::Int64, LlvmType::Int64) => {
                    let left_int = left_val.llvm_value.into_int_value();
                    let right_int = right_val.llvm_value.into_int_value();
                    self.builder.build_int_compare(IntPredicate::SGE, left_int, right_int, "ge_tmp")
                        .map_err(|e| CursedError::Compile(format!("Failed to build int compare: {:?}", e)))?
                        .into()
                (LlvmType::Float64, LlvmType::Float64) => {
                    let left_float = left_val.llvm_value.into_float_value();
                    let right_float = right_val.llvm_value.into_float_value();
                    self.builder.build_float_compare(FloatPredicate::OGE, left_float, right_float, "fge_tmp")
                        .map_err(|e| CursedError::Compile(format!("Failed to build float compare: {:?}", e)))?
                        .into()
            
            // Logical operators
            "&&" | "and" => {
                if left_val.value_type == LlvmType::Boolean && right_val.value_type == LlvmType::Boolean {
                    let left_bool = left_val.llvm_value.into_int_value();
                    let right_bool = right_val.llvm_value.into_int_value();
                    self.builder.build_and(left_bool, right_bool, "and_tmp")
                        .map_err(|e| CursedError::Compile(format!("Failed to build and: {:?}", e)))?
                        .into()
                } else {
                    return Err(CursedError::Compile("Logical and requires boolean operands".to_string()));
                }
            "||" | "or" => {
                if left_val.value_type == LlvmType::Boolean && right_val.value_type == LlvmType::Boolean {
                    let left_bool = left_val.llvm_value.into_int_value();
                    let right_bool = right_val.llvm_value.into_int_value();
                    self.builder.build_or(left_bool, right_bool, "or_tmp")
                        .map_err(|e| CursedError::Compile(format!("Failed to build or: {:?}", e)))?
                        .into()
                } else {
                    return Err(CursedError::Compile("Logical or requires boolean operands".to_string()));
                }
            
            // Bitwise operators
            "&" => {
                if left_val.value_type == LlvmType::Int64 && right_val.value_type == LlvmType::Int64 {
                    let left_int = left_val.llvm_value.into_int_value();
                    let right_int = right_val.llvm_value.into_int_value();
                    self.builder.build_and(left_int, right_int, "bitand_tmp")
                        .map_err(|e| CursedError::Compile(format!("Failed to build bitwise and: {:?}", e)))?
                        .into()
                } else {
                    return Err(CursedError::Compile("Bitwise and requires integer operands".to_string()));
                }
            "|" => {
                if left_val.value_type == LlvmType::Int64 && right_val.value_type == LlvmType::Int64 {
                    let left_int = left_val.llvm_value.into_int_value();
                    let right_int = right_val.llvm_value.into_int_value();
                    self.builder.build_or(left_int, right_int, "bitor_tmp")
                        .map_err(|e| CursedError::Compile(format!("Failed to build bitwise or: {:?}", e)))?
                        .into()
                } else {
                    return Err(CursedError::Compile("Bitwise or requires integer operands".to_string()));
                }
            "^" => {
                if left_val.value_type == LlvmType::Int64 && right_val.value_type == LlvmType::Int64 {
                    let left_int = left_val.llvm_value.into_int_value();
                    let right_int = right_val.llvm_value.into_int_value();
                    self.builder.build_xor(left_int, right_int, "xor_tmp")
                        .map_err(|e| CursedError::Compile(format!("Failed to build xor: {:?}", e)))?
                        .into()
                } else {
                    return Err(CursedError::Compile("Bitwise xor requires integer operands".to_string()));
                }
            "<<" => {
                if left_val.value_type == LlvmType::Int64 && right_val.value_type == LlvmType::Int64 {
                    let left_int = left_val.llvm_value.into_int_value();
                    let right_int = right_val.llvm_value.into_int_value();
                    self.builder.build_left_shift(left_int, right_int, "shl_tmp")
                        .map_err(|e| CursedError::Compile(format!("Failed to build left shift: {:?}", e)))?
                        .into()
                } else {
                    return Err(CursedError::Compile("Left shift requires integer operands".to_string()));
                }
            ">>" => {
                if left_val.value_type == LlvmType::Int64 && right_val.value_type == LlvmType::Int64 {
                    let left_int = left_val.llvm_value.into_int_value();
                    let right_int = right_val.llvm_value.into_int_value();
                    self.builder.build_right_shift(left_int, right_int, true, "shr_tmp")
                        .map_err(|e| CursedError::Compile(format!("Failed to build right shift: {:?}", e)))?
                        .into()
                } else {
                    return Err(CursedError::Compile("Right shift requires integer operands".to_string()));
                }
            
        
        debug!("Binary expression compiled successfully");
        Ok(LlvmValue::new(result_type, result_value, false))
    /// Compile unary expressions
    #[instrument(skip(self, unary))]
    fn compile_unary_expression(&mut self, unary: &UnaryExpression) -> crate::error::Result<()> {
        debug!("Compiling unary expression: {} {}", unary.operator, unary.operand.string());
        
        let operand_val = self.compile_expression(unary.operand.as_ref())?;
        
        let result_value = match unary.operator.as_str() {
            "-" => match operand_val.value_type {
                LlvmType::Int64 => {
                    let operand_int = operand_val.llvm_value.into_int_value();
                    let zero = self.llvm_context.i64_type().const_zero();
                    self.builder.build_int_sub(zero, operand_int, "neg_tmp")
                        .map_err(|e| CursedError::Compile(format!("Failed to build int negation: {:?}", e)))?
                        .into()
                LlvmType::Float64 => {
                    let operand_float = operand_val.llvm_value.into_float_value();
                    let zero = self.llvm_context.f64_type().const_zero();
                    self.builder.build_float_sub(zero, operand_float, "fneg_tmp")
                        .map_err(|e| CursedError::Compile(format!("Failed to build float negation: {:?}", e)))?
                        .into()
            "!" | "not" => match operand_val.value_type {
                LlvmType::Boolean => {
                    let operand_bool = operand_val.llvm_value.into_int_value();
                    let true_val = self.llvm_context.bool_type().const_int(1, false);
                    self.builder.build_xor(operand_bool, true_val, "not_tmp")
                        .map_err(|e| CursedError::Compile(format!("Failed to build logical not: {:?}", e)))?
                        .into()
            "~" => match operand_val.value_type {
                LlvmType::Int64 => {
                    let operand_int = operand_val.llvm_value.into_int_value();
                    let all_ones = self.llvm_context.i64_type().const_int(u64::MAX, false);
                    self.builder.build_xor(operand_int, all_ones, "bitnot_tmp")
                        .map_err(|e| CursedError::Compile(format!("Failed to build bitwise not: {:?}", e)))?
                        .into()
        
        debug!("Unary expression compiled successfully");
        Ok(LlvmValue::new(operand_val.value_type, result_value, false))
    /// Compile identifier (variable access)
    #[instrument(skip(self, identifier))]
    fn compile_identifier(&mut self, identifier: &Identifier) -> crate::error::Result<()> {
        debug!("Compiling identifier: {}", identifier.value);
        
        // Use variable manager to load the variable
        if let Some(ref var_manager) = self.variable_manager {
            let var_manager_ref = var_manager.borrow();
            let value = var_manager_ref.load_variable(&identifier.value)
                .map_err(|e| CursedError::Compile(format!("Failed to load variable '{}': {}", identifier.value, e)))?;
            
            // Get variable type information
            if let Some((_, var_type)) = var_manager_ref.get_variable(&identifier.value) {
                let llvm_type = match var_type {
                    _ => LlvmType::Int64, // Default fallback
                
                debug!("Variable '{}' loaded successfully with type {:?}", identifier.value, llvm_type);
                Ok(LlvmValue::new(llvm_type, value, false))
            } else {
                error!("Variable '{}' not found in variable manager", identifier.value);
                Err(CursedError::Compile(format!("Undefined variable: {}", identifier.value)))
            }
        } else {
            error!("No variable manager available for identifier compilation");
            Err(CursedError::Compile("No variable manager available".to_string()))
        }
    }
    
    /// Compile function call expressions
    #[instrument(skip(self, call))]
    fn compile_call_expression(&mut self, call: &CallExpression) -> crate::error::Result<()> {
        debug!("Compiling function call: {}", call.function.string());
        
        let func_name = call.function.string();
        
        // Compile arguments and determine their types
        let mut arg_values = Vec::new();
        let mut arg_types = Vec::new();
        let mut arg_llvm_names = Vec::new();
        
        for arg in &call.arguments {
            let arg_val = self.compile_expression(arg.as_ref())?;
            arg_types.push(arg_val.value_type.clone());
            arg_llvm_names.push(arg_val.llvm_name.clone());
            arg_values.push(arg_val);
        // For now, we'll use a simplified approach since we don't have access to the function registry
        // In a full implementation, this would integrate with the function registry from the main generator
        
        // Check for built-in functions
        let (return_type, llvm_return_type) = match func_name.as_str() {
            _ => {
                info!("Unknown function '{}', assuming i32 return type", func_name);
                (LlvmType::Int32, self.llvm_context.i32_type().into())
            }
        
        // Generate function call IR
        let call_temp = self.context.next_temp();
        let call_temp_name = format!("%call_{}", call_temp);
        
        // Get or declare the function
        let func_type = self.get_or_create_function_type(&func_name, &arg_types, &return_type)?;
        let function = self.module.get_function(&func_name)
            .unwrap_or_else(|| {
                // Declare external function if not found
                self.module.add_function(&func_name, func_type, None)
            });
        
        // Build arguments for the call
        let call_args: Vec<BasicValueEnum> = arg_values.iter()
            .map(|arg| arg.llvm_value)
            .collect();
        
        // Build the function call
        let call_result = self.builder.build_call(function, &call_args, &call_temp_name)
            .map_err(|e| CursedError::Compile(format!("Failed to build function call: {:?}", e)))?;
        
        debug!("Function call '{}' compiled successfully", func_name);
        
        // Return the call result
        if return_type == LlvmType::Void {
            // For void functions, return a dummy value
            let dummy_value = self.llvm_context.i32_type().const_zero();
            Ok(LlvmValue::new_with_name(LlvmType::Void, dummy_value.into(), false, call_temp_name))
        } else {
            let result_value = call_result.try_as_basic_value().left()
                .ok_or_else(|| CursedError::Compile("Function call did not return a value".to_string()))?;
            Ok(LlvmValue::new_with_name(return_type, result_value, false, call_temp_name))
        }
    }
    
    /// Get or create function type
    fn get_or_create_function_type(&self, func_name: &str, arg_types: &[LlvmType], return_type: &LlvmType) -> crate::error::Result<()> {
        // Convert LLVM types to inkwell types
        let param_types: Result<Vec<_>, _> = arg_types.iter()
            .map(|arg_type| self.llvm_type_to_inkwell_type(arg_type))
            .collect();
        let param_types = param_types?;
        
        let return_inkwell_type = self.llvm_type_to_inkwell_type(return_type)?;
        
        // Create function type
        match return_inkwell_type {
        }
    }
    
    /// Convert LlvmType to inkwell BasicTypeEnum
    fn llvm_type_to_inkwell_type(&self, llvm_type: &LlvmType) -> crate::error::Result<()> {
        match llvm_type {
            LlvmType::Void => {
                // For void types, we'll return i32 as a placeholder since BasicTypeEnum doesn't include void
                Ok(self.llvm_context.i32_type().into())
            LlvmType::Function { .. } => {
                // Function types are represented as function pointers
                Ok(self.llvm_context.i8_type().ptr_type(AddressSpace::default()).into())
            }
        }
    /// Compile assignment expressions
    #[instrument(skip(self, assignment))]
    fn compile_assignment_expression(&mut self, assignment: &AssignmentExpression) -> crate::error::Result<()> {
        debug!("Compiling assignment expression");
        
        let value_result = self.compile_expression(assignment.value.as_ref())?;
        
        // Get the target variable name
        if let Some(identifier) = assignment.to_string().as_any().downcast_ref::<Identifier>() {
            // Use variable manager to handle the assignment
            if let Some(ref var_manager) = self.variable_manager {
                let mut var_manager_ref = var_manager.borrow_mut();
                
                // Create assignment expression for the variable manager
                let assign_expr = crate::ast::operators::AssignmentExpression::new(
                );
                
                var_manager_ref.compile_assignment(&assign_expr)
                    .map_err(|e| CursedError::Compile(format!("Failed to compile assignment: {}", e)))?;
                
                debug!("Assignment compiled successfully");
                Ok(value_result)
            } else {
                error!("No variable manager available for assignment");
                Err(CursedError::Compile("No variable manager available".to_string()))
            }
        } else {
            error!("Assignment target must be an identifier");
            Err(CursedError::Compile("Assignment target must be an identifier".to_string()))
        }
    }
    
    /// Compile index expressions (array access)
    fn compile_index_expression(&mut self, index: &IndexExpression) -> crate::error::Result<()> {
        let array_val = self.compile_expression(index.left.as_ref())?;
        let index_val = self.compile_expression(index.index.as_ref())?;
        
        // Ensure index is integer type
        if index_val.value_type != LlvmType::Int64 {
            return Err(CursedError::Compile("Array index must be integer".to_string()));
        let temp_name = self.context.next_temp();
        
        // Generate GEP instruction for array access
        match array_val.value_type {
            LlvmType::Pointer(ref inner_type) => {
                self.ir_output.push(format!(
                    index_val.llvm_name
                ));
                
                // Load the value
                let load_temp = self.context.next_temp();
                self.ir_output.push(format!(
                    temp_name
                ));
                
                Ok(LlvmValue {
                })
            }
            _ => Err(CursedError::Compile("Index operation requires pointer/array type".to_string())),
        }
    }
    
    /// Compile parenthesized expressions
    #[instrument(skip(self, paren))]
    fn compile_parenthesized_expression(&mut self, paren: &ParenthesizedExpression) -> crate::error::Result<()> {
        debug!("Compiling parenthesized expression");
        // Parentheses don't change the expression, just compile the inner expression
        self.compile_expression(paren.expression.as_ref())
    /// Compile question mark operator expressions
    pub fn compile_question_mark_expression(&mut self, question_mark: &QuestionMarkExpression) -> crate::error::Result<()> {
        // Compile the inner expression first
        let inner_value = self.compile_expression(question_mark.expression.as_ref())?;
        
        // Generate temporary names for the IR
        let is_ok_temp = self.context.next_temp();
        let value_temp = self.context.next_temp();
        let error_temp = self.context.next_temp();
        let propagation_temp = self.context.next_temp();
        
        // Create basic blocks for control flow
        let success_block = format!("question_mark_success_{}", self.context.temp_counter);
        let error_block = format!("question_mark_error_{}", self.context.temp_counter);
        let merge_block = format!("question_mark_merge_{}", self.context.temp_counter);
        
        self.context.temp_counter += 1;
        
        // Determine if this is a Result or Option type and generate appropriate checks
        let inner_type_string = self.get_type_string(&inner_value.value_type);
        
        if inner_type_string.starts_with("Result<") {
            // Extract value and error types from Result<T, E>
            let (value_type, error_type) = self.extract_result_types(&inner_type_string)?;
            
            // Extract is_ok flag from Result type
            self.ir_output.push(format!(
                is_ok_temp, inner_type_string, inner_value.llvm_name
            ));
            
            // Branch based on is_ok flag
            self.ir_output.push(format!(
                is_ok_temp, success_block, error_block
            ));
            
            // Success block: extract value and continue
            self.ir_output.push(format!("{}:", success_block));
            self.ir_output.push(format!(
                value_temp, inner_type_string, inner_value.llvm_name
            ));
            self.ir_output.push(format!("  br label %{}", merge_block));
            
            // CursedError block: extract error and propagate (early return)
            self.ir_output.push(format!("{}:", error_block));
            self.ir_output.push(format!(
                error_temp, inner_type_string, inner_value.llvm_name
            ));
            
            // Generate error propagation runtime call
            self.ir_output.push(format!(
                propagation_temp, error_temp
            ));
            
            // Create propagated Result with error
            let result_temp = self.context.next_temp();
            self.ir_output.push(format!(
                result_temp, inner_type_string
            ));
            self.ir_output.push(format!(
                result_temp, inner_type_string, result_temp, error_type.to_llvm_string(), error_temp
            ));
            
            // Early return with propagated error
            self.ir_output.push(format!("  ret {} {}", inner_type_string, result_temp));
            
            // Merge block - only reached on success
            self.ir_output.push(format!("{}:", merge_block));
            
            // Return the extracted value
            Ok(LlvmValue {
            })
        } else if inner_type_string.starts_with("Option<") {
            // Extract value type from Option<T>
            let value_type = self.extract_option_type(&inner_type_string)?;
            
            // Extract is_some flag from Option type
            self.ir_output.push(format!(
                is_ok_temp, inner_type_string, inner_value.llvm_name
            ));
            
            // Branch based on is_some flag
            self.ir_output.push(format!(
                is_ok_temp, success_block, error_block
            ));
            
            // Success block: extract value and continue
            self.ir_output.push(format!("{}:", success_block));
            self.ir_output.push(format!(
                value_temp, inner_type_string, inner_value.llvm_name
            ));
            self.ir_output.push(format!("  br label %{}", merge_block));
            
            // CursedError block: propagate None
            self.ir_output.push(format!("{}:", error_block));
            
            // Generate None propagation (optional runtime call for debugging)
            self.ir_output.push(format!(
                "  call void @cursed_error_propagation(i8* null, i32 0, i32 0)  ; Log None propagation"
            ));
            
            // Create propagated Option with None
            let none_temp = self.context.next_temp();
            self.ir_output.push(format!(
                none_temp, inner_type_string
            ));
            
            // Early return with None
            self.ir_output.push(format!("  ret {} {}", inner_type_string, none_temp));
            
            // Merge block - only reached on success
            self.ir_output.push(format!("{}:", merge_block));
            
            // Return the extracted value
            Ok(LlvmValue {
            })
        } else {
            Err(CursedError::Compile(format!(
                inner_type_string
            )))
        }
    }
    
    /// Resolve the result type of binary operations
    fn resolve_binary_type(&self, left: &LlvmType, right: &LlvmType, operator: &str) -> crate::error::Result<()> {
        match (left, right, operator) {
            // Arithmetic operations
            (LlvmType::Int64, LlvmType::Int64, "+"|"-"|"*"|"/"|"%") => Ok(LlvmType::Int64),
            (LlvmType::Float64, LlvmType::Float64, "+"|"-"|"*"|"/") => Ok(LlvmType::Float64),
            (LlvmType::Int64, LlvmType::Float64, "+"|"-"|"*"|"/") => Ok(LlvmType::Float64),
            (LlvmType::Float64, LlvmType::Int64, "+"|"-"|"*"|"/") => Ok(LlvmType::Float64),
            
            // Comparison operations always return boolean
            
            // Logical operations
            
            // Bitwise operations
            
            _ => Err(CursedError::Compile(format!(
                operator, left, right
        }
    }
    
    /// Get compilation context
    pub fn get_context(&self) -> &ExpressionContext<'ctx> {
        &self.context
    /// Get mutable compilation context
    pub fn get_context_mut(&mut self) -> &mut ExpressionContext<'ctx> {
        &mut self.context
    /// Set current source location for debug info
    pub fn set_location(&mut self, location: SourceLocation) {
        self.context.current_location = Some(location);
    /// Get the LLVM context
    pub fn llvm_context(&self) -> &'ctx Context {
        self.llvm_context
    /// Get the LLVM module
    pub fn module(&self) -> &'ctx Module<'ctx> {
        self.module
    /// Get the LLVM builder
    pub fn builder(&self) -> &'ctx Builder<'ctx> {
        self.builder
    /// Get variable manager
    pub fn variable_manager(&self) -> &Rc<RefCell<VariableManager<'ctx>>> {
        &self.variable_manager
    /// Get string representation of a type (helper for question mark operator)
    fn get_type_string(&self, llvm_type: &LlvmType) -> String {
        match llvm_type {
            LlvmType::Function { return_type, param_types } => {
                let params = param_types.iter()
                    .map(|t| self.get_type_string(t))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("({}) -> {}", params, self.get_type_string(return_type))
            }
        }
    }
    
    /// Extract value and error types from Result<T, E> string
    fn extract_result_types(&self, result_type_string: &str) -> crate::error::Result<()> {
        if !result_type_string.starts_with("Result<") || !result_type_string.ends_with('>') {
            return Err(CursedError::Compile("Invalid Result type format".to_string()));
        // Extract "T, E" from "Result<T, E>"
        let inner = &result_type_string[7..result_type_string.len()-1];
        let parts: Vec<&str> = inner.splitn(2, ',').collect();
        
        if parts.len() != 2 {
            return Err(CursedError::Compile("Result must have exactly two type parameters".to_string()));
        let value_type = self.parse_type_string(parts[0].trim())?;
        let error_type = self.parse_type_string(parts[1].trim())?;
        
        Ok((value_type, error_type))
    /// Extract value type from Option<T> string
    fn extract_option_type(&self, option_type_string: &str) -> crate::error::Result<()> {
        if !option_type_string.starts_with("Option<") || !option_type_string.ends_with('>') {
            return Err(CursedError::Compile("Invalid Option type format".to_string()));
        // Extract "T" from "Option<T>"
        let inner = &option_type_string[7..option_type_string.len()-1];
        self.parse_type_string(inner.trim())
    /// Parse a type string into LlvmType
    fn parse_type_string(&self, type_str: &str) -> crate::error::Result<()> {
        match type_str {
            s if s.starts_with('*') => {
                let inner_type = self.parse_type_string(&s[1..])?;
                Ok(LlvmType::Pointer(Box::new(inner_type)))
            }
            _ => {
                // Default to i32 for unknown types (this could be improved with proper type inference)
                Ok(LlvmType::Int32)
            }
        }
    }
}

// Note: Default implementation removed as it requires lifetime parameters

impl<'ctx> LlvmExpressionCompiler<'ctx> {
    /// Compile a generic method call with constraint checking
    pub fn compile_generic_call(
        type_context: &mut TypeCompilationContext
    ) -> crate::error::Result<()> {
        let function_name = call.function.string();
        
        // Check if function name contains generic type indicators
        if !function_name.contains('<') && !function_name.contains("_") {
            return Err(CursedError::TypeCompilation("Not a generic call".to_string()));
        // Extract base function name and type arguments
        let (base_name, type_args) = self.parse_generic_call(&function_name)?;
        
        // Check for cached instantiation
        let instance_key = format!("{}_{}", base_name, type_args.join("_"));
        if let Some(cached) = self.context.generic_methods.get(&instance_key).cloned() {
            return self.generate_instantiated_call(&cached, call);
        // Instantiate generic method
        let instance = type_context.instantiate_generic(&base_name, &type_args)?;
        
        // Cache the instantiation
        self.context.generic_methods.insert(instance_key, instance.clone());
        
        // Generate call with instantiated method
        self.generate_instantiated_call(&instance, call)
    /// Parse generic call syntax to extract base name and type arguments
    fn parse_generic_call(&self, function_name: &str) -> crate::error::Result<()> {
        // Handle syntax like "function<T, U>" or "function_T_U"
        if function_name.contains('<') && function_name.contains('>') {
            let parts: Vec<&str> = function_name.splitn(2, '<').collect();
            if parts.len() != 2 {
                return Err(CursedError::TypeCompilation("Invalid generic syntax".to_string()));
            let base_name = parts[0].to_string();
            let type_part = parts[1].trim_end_matches('>');
            let type_args: Vec<String> = type_part.split(',')
                .map(|s| s.trim().to_string())
                .collect();
            
            Ok((base_name, type_args))
        } else if function_name.contains('_') {
            // Handle underscore syntax: "function_Type1_Type2"
            let parts: Vec<&str> = function_name.split('_').collect();
            if parts.len() < 2 {
                return Err(CursedError::TypeCompilation("Invalid generic syntax".to_string()));
            let base_name = parts[0].to_string();
            let type_args: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();
            
            Ok((base_name, type_args))
        } else {
            Err(CursedError::TypeCompilation("No generic syntax found".to_string()))
        }
    }

    /// Generate call IR for instantiated generic method
    fn generate_instantiated_call(
        call: &CallExpression
    ) -> crate::error::Result<()> {
        // Compile arguments
        let mut arg_values = Vec::new();
        let mut arg_types = Vec::new();
        
        for arg in &call.arguments {
            let arg_val = self.compile_expression(arg.as_ref())?;
            arg_types.push(arg_val.value_type.to_llvm_string());
            arg_values.push(arg_val.llvm_name);
        let temp_name = self.context.next_temp();
        let args_str = arg_values.iter()
            .zip(arg_types.iter())
            .map(|(val, typ)| format!("{} {}", typ, val))
            .collect::<Vec<_>>()
            .join(", ");
        
        // Generate call to instantiated function
        self.ir_output.push(format!(
            "i32", // Return type from instance metadata
            args_str
        ));
        
        Ok(LlvmValue {
            value_type: LlvmType::Int32, // Should be inferred from instance
        })
    /// Compile expression with type inference (simplified for now)
    pub fn compile_expression_with_inference(
        _type_context: &TypeCompilationContext
    ) -> crate::error::Result<()> {
        // For now, just fall back to regular compilation
        // TODO: Integrate proper type inference
        self.compile_expression(expr)
    /// Compile literal with type information
    fn compile_typed_literal(
        inferred_type: &str
    ) -> crate::error::Result<()> {
        let literal_value = expr.string();
        let temp_name = self.context.next_temp();
        
        match inferred_type {
            "normie" => {
                self.ir_output.push(format!("  {} = add i64 0, {}", temp_name, literal_value));
                Ok(LlvmValue {
                })
            "facts" => {
                let bool_val = if literal_value == "true" { "1" } else { "0" };
                self.ir_output.push(format!("  {} = add i1 0, {}", temp_name, bool_val));
                Ok(LlvmValue {
                })
            "tea" => {
                let str_name = self.context.next_temp();
                self.ir_output.push(format!(
                    str_name
                ));
                Ok(LlvmValue {
                })
            _ => {
                // Fallback to regular compilation
                self.compile_expression(expr)
            }
        }
    }
}

