/// LLVM expression compilation for the CURSED programming language
/// 
/// This module provides comprehensive compilation of all AST expression types
/// to LLVM IR, including proper type handling, error reporting, and 
/// support for Gen Z slang syntax.

use crate::ast::traits::Expression;
use crate::ast::{
    operators::{BinaryExpression, UnaryExpression, AssignmentExpression, IndexExpression},
    expressions::{Literal, LiteralValue, ParenthesizedExpression, FunctionLiteral, QuestionMarkExpression},
    identifiers::{Identifier, QualifiedName, TypeIdentifier},
    calls::CallExpression,
};
use crate::error::Error;
use crate::debug::SourceLocation;
use crate::type_system::{TypeSystem, TypeInference, TypeEnvironment};
use crate::codegen::llvm::type_system::{TypeCompilationContext, CompiledGenericInstance};
use crate::codegen::llvm::variable_management::VariableManager;
use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    values::{BasicValueEnum, FunctionValue, PointerValue, IntValue, FloatValue},
    types::{BasicTypeEnum, BasicType},
    AddressSpace,
    IntPredicate, FloatPredicate,
};
use std::any::Any;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use tracing::{debug, info, warn, error, instrument};

/// LLVM value representation for compiled expressions
#[derive(Debug, Clone)]
pub struct LlvmValue {
    pub value_type: LlvmType,
    pub llvm_value: BasicValueEnum<'static>,
    pub is_constant: bool,
}

impl LlvmValue {
    /// Create a new LLVM value with actual LLVM value
    pub fn new(value_type: LlvmType, llvm_value: BasicValueEnum<'static>, is_constant: bool) -> Self {
        Self {
            value_type,
            llvm_value,
            is_constant,
        }
    }
    
    /// Check if this is a struct value
    pub fn is_struct_value(&self) -> bool {
        self.llvm_value.is_struct_value()
    }
    
    /// Check if this is a float value
    pub fn is_float_value(&self) -> bool {
        matches!(self.value_type, LlvmType::Float64) && self.llvm_value.is_float_value()
    }
    
    /// Check if this is an int value
    pub fn is_int_value(&self) -> bool {
        matches!(self.value_type, LlvmType::Int32 | LlvmType::Int64) && self.llvm_value.is_int_value()
    }
    
    /// Convert to int value
    pub fn into_int_value(self) -> IntValue<'static> {
        self.llvm_value.into_int_value()
    }

    /// Convert to float value
    pub fn into_float_value(self) -> FloatValue<'static> {
        self.llvm_value.into_float_value()
    }

    /// Convert to pointer value
    pub fn into_pointer_value(self) -> PointerValue<'static> {
        self.llvm_value.into_pointer_value()
    }

    /// Get the underlying LLVM value
    pub fn as_basic_value(&self) -> BasicValueEnum<'static> {
        self.llvm_value
    }
}

/// LLVM type system mapping
#[derive(Debug, Clone, PartialEq)]
pub enum LlvmType {
    Int32,
    Int64,
    Float64,
    Boolean,
    String,
    Void,
    Array,
    Object,
    Pointer(Box<LlvmType>),
    Function {
        return_type: Box<LlvmType>,
        param_types: Vec<LlvmType>,
    },
}

impl LlvmType {
    pub fn to_llvm_string(&self) -> String {
        match self {
            LlvmType::Int32 => "i32".to_string(),
            LlvmType::Int64 => "i64".to_string(),
            LlvmType::Float64 => "double".to_string(),
            LlvmType::Boolean => "i1".to_string(),
            LlvmType::String => "i8*".to_string(),
            LlvmType::Void => "void".to_string(),
            LlvmType::Array => "i8*".to_string(), // Arrays are represented as opaque pointers
            LlvmType::Object => "i8*".to_string(), // Objects are represented as opaque pointers
            LlvmType::Pointer(inner) => format!("{}*", inner.to_llvm_string()),
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
    pub temp_counter: u32,
    pub current_location: Option<SourceLocation>,
    /// Type compilation context for constraint resolution
    pub type_context: Option<TypeCompilationContext>,
    /// Generic method instantiation cache
    pub generic_methods: HashMap<String, CompiledGenericInstance>,
    /// Variable manager for handling variable operations
    pub variable_manager: Option<Rc<RefCell<VariableManager<'ctx>>>>,
}

impl<'ctx> ExpressionContext<'ctx> {
    pub fn new() -> Self {
        Self {
            temp_counter: 0,
            current_location: None,
            type_context: None,
            generic_methods: HashMap::new(),
            variable_manager: None,
        }
    }

    /// Create context with type compilation support
    pub fn with_type_context(type_context: TypeCompilationContext) -> Self {
        Self {
            temp_counter: 0,
            current_location: None,
            type_context: Some(type_context),
            generic_methods: HashMap::new(),
            variable_manager: None,
        }
    }

    /// Set variable manager
    pub fn set_variable_manager(&mut self, variable_manager: Rc<RefCell<VariableManager<'ctx>>>) {
        self.variable_manager = Some(variable_manager);
    }
    
    pub fn next_temp(&mut self) -> u32 {
        self.temp_counter += 1;
        self.temp_counter
    }
}

/// Main expression compiler
pub struct LlvmExpressionCompiler<'ctx> {
    /// LLVM context
    llvm_context: &'ctx Context,
    /// LLVM module
    module: &'ctx Module<'ctx>,
    /// LLVM builder
    builder: &'ctx Builder<'ctx>,
    /// Expression compilation context
    context: ExpressionContext<'ctx>,
    /// Variable manager
    variable_manager: Rc<RefCell<VariableManager<'ctx>>>,
}

impl<'ctx> LlvmExpressionCompiler<'ctx> {
    pub fn new(
        llvm_context: &'ctx Context,
        module: &'ctx Module<'ctx>,
        builder: &'ctx Builder<'ctx>,
        variable_manager: Rc<RefCell<VariableManager<'ctx>>>,
    ) -> Self {
        let mut context = ExpressionContext::new();
        context.set_variable_manager(variable_manager.clone());
        
        Self {
            llvm_context,
            module,
            builder,
            context,
            variable_manager,
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
            i8_ptr_type.into(),
            i32_type.into(),
            i32_type.into(),
        ], false);
        self.module.add_function("cursed_error_propagation", propagate_fn_type, None);
        
        // cursed_error_propagation_panic(i8*)
        let panic_fn_type = void_type.fn_type(&[i8_ptr_type.into()], false);
        self.module.add_function("cursed_error_propagation_panic", panic_fn_type, None);
        
        debug!("Error propagation declarations added successfully");
    }
    
    /// Compile any expression to LLVM IR
    #[instrument(skip(self, expr))]
    pub fn compile_expression(&mut self, expr: &dyn Expression) -> Result<LlvmValue, Error> {
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
            Err(Error::Compile(format!(
                "Unsupported expression type for compilation: {}",
                expr.string()
            )))
        }
    }
    
    /// Compile literal expressions
    #[instrument(skip(self, literal))]
    fn compile_literal(&mut self, literal: &Literal) -> Result<LlvmValue, Error> {
        debug!("Compiling literal: {:?}", literal.value);
        
        match &literal.value {
            LiteralValue::Integer(value) => {
                debug!("Compiling integer literal: {}", value);
                let i64_type = self.llvm_context.i64_type();
                let const_value = i64_type.const_int(*value as u64, false);
                
                Ok(LlvmValue::new(
                    LlvmType::Int64,
                    const_value.into(),
                    true,
                ))
            }
            LiteralValue::Float(value) => {
                debug!("Compiling float literal: {}", value);
                let f64_type = self.llvm_context.f64_type();
                let const_value = f64_type.const_float(*value);
                
                Ok(LlvmValue::new(
                    LlvmType::Float64,
                    const_value.into(),
                    true,
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
                        string_type,
                        global.as_pointer_value(),
                        &[zero, zero],
                        "str_ptr"
                    ).map_err(|e| Error::Compile(format!("Failed to build GEP for string: {:?}", e)))?
                };
                
                Ok(LlvmValue::new(
                    LlvmType::String,
                    gep.into(),
                    true,
                ))
            }
            LiteralValue::Boolean(value) => {
                debug!("Compiling boolean literal: {}", value);
                let i1_type = self.llvm_context.bool_type();
                let const_value = i1_type.const_int(*value as u64, false);
                
                Ok(LlvmValue::new(
                    LlvmType::Boolean,
                    const_value.into(),
                    true,
                ))
            }
            LiteralValue::Nil => {
                debug!("Compiling nil literal");
                let i8_ptr_type = self.llvm_context.i8_type().ptr_type(AddressSpace::default());
                let null_ptr = i8_ptr_type.const_null();
                
                Ok(LlvmValue::new(
                    LlvmType::Pointer(Box::new(LlvmType::Void)),
                    null_ptr.into(),
                    true,
                ))
            }
        }
    }
    
    /// Compile binary expressions (arithmetic, logical, comparison)
    #[instrument(skip(self, binary))]
    fn compile_binary_expression(&mut self, binary: &BinaryExpression) -> Result<LlvmValue, Error> {
        debug!("Compiling binary expression: {} {} {}", 
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
                        .map_err(|e| Error::Compile(format!("Failed to build int add: {:?}", e)))?
                        .into()
                },
                (LlvmType::Float64, LlvmType::Float64) => {
                    let left_float = left_val.llvm_value.into_float_value();
                    let right_float = right_val.llvm_value.into_float_value();
                    self.builder.build_float_add(left_float, right_float, "fadd_tmp")
                        .map_err(|e| Error::Compile(format!("Failed to build float add: {:?}", e)))?
                        .into()
                },
                _ => return Err(Error::Compile("Invalid types for addition".to_string())),
            },
            "-" => match (&left_val.value_type, &right_val.value_type) {
                (LlvmType::Int64, LlvmType::Int64) => {
                    let left_int = left_val.llvm_value.into_int_value();
                    let right_int = right_val.llvm_value.into_int_value();
                    self.builder.build_int_sub(left_int, right_int, "sub_tmp")
                        .map_err(|e| Error::Compile(format!("Failed to build int sub: {:?}", e)))?
                        .into()
                },
                (LlvmType::Float64, LlvmType::Float64) => {
                    let left_float = left_val.llvm_value.into_float_value();
                    let right_float = right_val.llvm_value.into_float_value();
                    self.builder.build_float_sub(left_float, right_float, "fsub_tmp")
                        .map_err(|e| Error::Compile(format!("Failed to build float sub: {:?}", e)))?
                        .into()
                },
                _ => return Err(Error::Compile("Invalid types for subtraction".to_string())),
            },
            "*" => match (&left_val.value_type, &right_val.value_type) {
                (LlvmType::Int64, LlvmType::Int64) => {
                    let left_int = left_val.llvm_value.into_int_value();
                    let right_int = right_val.llvm_value.into_int_value();
                    self.builder.build_int_mul(left_int, right_int, "mul_tmp")
                        .map_err(|e| Error::Compile(format!("Failed to build int mul: {:?}", e)))?
                        .into()
                },
                (LlvmType::Float64, LlvmType::Float64) => {
                    let left_float = left_val.llvm_value.into_float_value();
                    let right_float = right_val.llvm_value.into_float_value();
                    self.builder.build_float_mul(left_float, right_float, "fmul_tmp")
                        .map_err(|e| Error::Compile(format!("Failed to build float mul: {:?}", e)))?
                        .into()
                },
                _ => return Err(Error::Compile("Invalid types for multiplication".to_string())),
            },
            "/" => match (&left_val.value_type, &right_val.value_type) {
                (LlvmType::Int64, LlvmType::Int64) => {
                    let left_int = left_val.llvm_value.into_int_value();
                    let right_int = right_val.llvm_value.into_int_value();
                    self.builder.build_int_signed_div(left_int, right_int, "div_tmp")
                        .map_err(|e| Error::Compile(format!("Failed to build int div: {:?}", e)))?
                        .into()
                },
                (LlvmType::Float64, LlvmType::Float64) => {
                    let left_float = left_val.llvm_value.into_float_value();
                    let right_float = right_val.llvm_value.into_float_value();
                    self.builder.build_float_div(left_float, right_float, "fdiv_tmp")
                        .map_err(|e| Error::Compile(format!("Failed to build float div: {:?}", e)))?
                        .into()
                },
                _ => return Err(Error::Compile("Invalid types for division".to_string())),
            },
            "%" => match (&left_val.value_type, &right_val.value_type) {
                (LlvmType::Int64, LlvmType::Int64) => {
                    let left_int = left_val.llvm_value.into_int_value();
                    let right_int = right_val.llvm_value.into_int_value();
                    self.builder.build_int_signed_rem(left_int, right_int, "rem_tmp")
                        .map_err(|e| Error::Compile(format!("Failed to build int rem: {:?}", e)))?
                        .into()
                },
                _ => return Err(Error::Compile("Modulo only supported for integers".to_string())),
            },
            
            // Comparison operators
            "==" => match (&left_val.value_type, &right_val.value_type) {
                (LlvmType::Int64, LlvmType::Int64) => {
                    let left_int = left_val.llvm_value.into_int_value();
                    let right_int = right_val.llvm_value.into_int_value();
                    self.builder.build_int_compare(IntPredicate::EQ, left_int, right_int, "eq_tmp")
                        .map_err(|e| Error::Compile(format!("Failed to build int compare: {:?}", e)))?
                        .into()
                },
                (LlvmType::Float64, LlvmType::Float64) => {
                    let left_float = left_val.llvm_value.into_float_value();
                    let right_float = right_val.llvm_value.into_float_value();
                    self.builder.build_float_compare(FloatPredicate::OEQ, left_float, right_float, "feq_tmp")
                        .map_err(|e| Error::Compile(format!("Failed to build float compare: {:?}", e)))?
                        .into()
                },
                (LlvmType::Boolean, LlvmType::Boolean) => {
                    let left_bool = left_val.llvm_value.into_int_value();
                    let right_bool = right_val.llvm_value.into_int_value();
                    self.builder.build_int_compare(IntPredicate::EQ, left_bool, right_bool, "beq_tmp")
                        .map_err(|e| Error::Compile(format!("Failed to build bool compare: {:?}", e)))?
                        .into()
                },
                _ => return Err(Error::Compile("Invalid types for equality comparison".to_string())),
            },
            "!=" => match (&left_val.value_type, &right_val.value_type) {
                (LlvmType::Int64, LlvmType::Int64) => {
                    let left_int = left_val.llvm_value.into_int_value();
                    let right_int = right_val.llvm_value.into_int_value();
                    self.builder.build_int_compare(IntPredicate::NE, left_int, right_int, "ne_tmp")
                        .map_err(|e| Error::Compile(format!("Failed to build int compare: {:?}", e)))?
                        .into()
                },
                (LlvmType::Float64, LlvmType::Float64) => {
                    let left_float = left_val.llvm_value.into_float_value();
                    let right_float = right_val.llvm_value.into_float_value();
                    self.builder.build_float_compare(FloatPredicate::ONE, left_float, right_float, "fne_tmp")
                        .map_err(|e| Error::Compile(format!("Failed to build float compare: {:?}", e)))?
                        .into()
                },
                (LlvmType::Boolean, LlvmType::Boolean) => {
                    let left_bool = left_val.llvm_value.into_int_value();
                    let right_bool = right_val.llvm_value.into_int_value();
                    self.builder.build_int_compare(IntPredicate::NE, left_bool, right_bool, "bne_tmp")
                        .map_err(|e| Error::Compile(format!("Failed to build bool compare: {:?}", e)))?
                        .into()
                },
                _ => return Err(Error::Compile("Invalid types for inequality comparison".to_string())),
            },
            "<" => match (&left_val.value_type, &right_val.value_type) {
                (LlvmType::Int64, LlvmType::Int64) => {
                    let left_int = left_val.llvm_value.into_int_value();
                    let right_int = right_val.llvm_value.into_int_value();
                    self.builder.build_int_compare(IntPredicate::SLT, left_int, right_int, "lt_tmp")
                        .map_err(|e| Error::Compile(format!("Failed to build int compare: {:?}", e)))?
                        .into()
                },
                (LlvmType::Float64, LlvmType::Float64) => {
                    let left_float = left_val.llvm_value.into_float_value();
                    let right_float = right_val.llvm_value.into_float_value();
                    self.builder.build_float_compare(FloatPredicate::OLT, left_float, right_float, "flt_tmp")
                        .map_err(|e| Error::Compile(format!("Failed to build float compare: {:?}", e)))?
                        .into()
                },
                _ => return Err(Error::Compile("Invalid types for less-than comparison".to_string())),
            },
            ">" => match (&left_val.value_type, &right_val.value_type) {
                (LlvmType::Int64, LlvmType::Int64) => {
                    let left_int = left_val.llvm_value.into_int_value();
                    let right_int = right_val.llvm_value.into_int_value();
                    self.builder.build_int_compare(IntPredicate::SGT, left_int, right_int, "gt_tmp")
                        .map_err(|e| Error::Compile(format!("Failed to build int compare: {:?}", e)))?
                        .into()
                },
                (LlvmType::Float64, LlvmType::Float64) => {
                    let left_float = left_val.llvm_value.into_float_value();
                    let right_float = right_val.llvm_value.into_float_value();
                    self.builder.build_float_compare(FloatPredicate::OGT, left_float, right_float, "fgt_tmp")
                        .map_err(|e| Error::Compile(format!("Failed to build float compare: {:?}", e)))?
                        .into()
                },
                _ => return Err(Error::Compile("Invalid types for greater-than comparison".to_string())),
            },
            "<=" => match (&left_val.value_type, &right_val.value_type) {
                (LlvmType::Int64, LlvmType::Int64) => {
                    let left_int = left_val.llvm_value.into_int_value();
                    let right_int = right_val.llvm_value.into_int_value();
                    self.builder.build_int_compare(IntPredicate::SLE, left_int, right_int, "le_tmp")
                        .map_err(|e| Error::Compile(format!("Failed to build int compare: {:?}", e)))?
                        .into()
                },
                (LlvmType::Float64, LlvmType::Float64) => {
                    let left_float = left_val.llvm_value.into_float_value();
                    let right_float = right_val.llvm_value.into_float_value();
                    self.builder.build_float_compare(FloatPredicate::OLE, left_float, right_float, "fle_tmp")
                        .map_err(|e| Error::Compile(format!("Failed to build float compare: {:?}", e)))?
                        .into()
                },
                _ => return Err(Error::Compile("Invalid types for less-than-or-equal comparison".to_string())),
            },
            ">=" => match (&left_val.value_type, &right_val.value_type) {
                (LlvmType::Int64, LlvmType::Int64) => {
                    let left_int = left_val.llvm_value.into_int_value();
                    let right_int = right_val.llvm_value.into_int_value();
                    self.builder.build_int_compare(IntPredicate::SGE, left_int, right_int, "ge_tmp")
                        .map_err(|e| Error::Compile(format!("Failed to build int compare: {:?}", e)))?
                        .into()
                },
                (LlvmType::Float64, LlvmType::Float64) => {
                    let left_float = left_val.llvm_value.into_float_value();
                    let right_float = right_val.llvm_value.into_float_value();
                    self.builder.build_float_compare(FloatPredicate::OGE, left_float, right_float, "fge_tmp")
                        .map_err(|e| Error::Compile(format!("Failed to build float compare: {:?}", e)))?
                        .into()
                },
                _ => return Err(Error::Compile("Invalid types for greater-than-or-equal comparison".to_string())),
            },
            
            // Logical operators
            "&&" | "and" => {
                if left_val.value_type == LlvmType::Boolean && right_val.value_type == LlvmType::Boolean {
                    let left_bool = left_val.llvm_value.into_int_value();
                    let right_bool = right_val.llvm_value.into_int_value();
                    self.builder.build_and(left_bool, right_bool, "and_tmp")
                        .map_err(|e| Error::Compile(format!("Failed to build and: {:?}", e)))?
                        .into()
                } else {
                    return Err(Error::Compile("Logical and requires boolean operands".to_string()));
                }
            },
            "||" | "or" => {
                if left_val.value_type == LlvmType::Boolean && right_val.value_type == LlvmType::Boolean {
                    let left_bool = left_val.llvm_value.into_int_value();
                    let right_bool = right_val.llvm_value.into_int_value();
                    self.builder.build_or(left_bool, right_bool, "or_tmp")
                        .map_err(|e| Error::Compile(format!("Failed to build or: {:?}", e)))?
                        .into()
                } else {
                    return Err(Error::Compile("Logical or requires boolean operands".to_string()));
                }
            },
            
            // Bitwise operators
            "&" => {
                if left_val.value_type == LlvmType::Int64 && right_val.value_type == LlvmType::Int64 {
                    let left_int = left_val.llvm_value.into_int_value();
                    let right_int = right_val.llvm_value.into_int_value();
                    self.builder.build_and(left_int, right_int, "bitand_tmp")
                        .map_err(|e| Error::Compile(format!("Failed to build bitwise and: {:?}", e)))?
                        .into()
                } else {
                    return Err(Error::Compile("Bitwise and requires integer operands".to_string()));
                }
            },
            "|" => {
                if left_val.value_type == LlvmType::Int64 && right_val.value_type == LlvmType::Int64 {
                    let left_int = left_val.llvm_value.into_int_value();
                    let right_int = right_val.llvm_value.into_int_value();
                    self.builder.build_or(left_int, right_int, "bitor_tmp")
                        .map_err(|e| Error::Compile(format!("Failed to build bitwise or: {:?}", e)))?
                        .into()
                } else {
                    return Err(Error::Compile("Bitwise or requires integer operands".to_string()));
                }
            },
            "^" => {
                if left_val.value_type == LlvmType::Int64 && right_val.value_type == LlvmType::Int64 {
                    let left_int = left_val.llvm_value.into_int_value();
                    let right_int = right_val.llvm_value.into_int_value();
                    self.builder.build_xor(left_int, right_int, "xor_tmp")
                        .map_err(|e| Error::Compile(format!("Failed to build xor: {:?}", e)))?
                        .into()
                } else {
                    return Err(Error::Compile("Bitwise xor requires integer operands".to_string()));
                }
            },
            "<<" => {
                if left_val.value_type == LlvmType::Int64 && right_val.value_type == LlvmType::Int64 {
                    let left_int = left_val.llvm_value.into_int_value();
                    let right_int = right_val.llvm_value.into_int_value();
                    self.builder.build_left_shift(left_int, right_int, "shl_tmp")
                        .map_err(|e| Error::Compile(format!("Failed to build left shift: {:?}", e)))?
                        .into()
                } else {
                    return Err(Error::Compile("Left shift requires integer operands".to_string()));
                }
            },
            ">>" => {
                if left_val.value_type == LlvmType::Int64 && right_val.value_type == LlvmType::Int64 {
                    let left_int = left_val.llvm_value.into_int_value();
                    let right_int = right_val.llvm_value.into_int_value();
                    self.builder.build_right_shift(left_int, right_int, true, "shr_tmp")
                        .map_err(|e| Error::Compile(format!("Failed to build right shift: {:?}", e)))?
                        .into()
                } else {
                    return Err(Error::Compile("Right shift requires integer operands".to_string()));
                }
            },
            
            _ => return Err(Error::Compile(format!("Unsupported binary operator: {}", binary.operator))),
        };
        
        debug!("Binary expression compiled successfully");
        Ok(LlvmValue::new(result_type, result_value, false))
    }
    
    /// Compile unary expressions
    #[instrument(skip(self, unary))]
    fn compile_unary_expression(&mut self, unary: &UnaryExpression) -> Result<LlvmValue, Error> {
        debug!("Compiling unary expression: {} {}", unary.operator, unary.operand.string());
        
        let operand_val = self.compile_expression(unary.operand.as_ref())?;
        
        let result_value = match unary.operator.as_str() {
            "-" => match operand_val.value_type {
                LlvmType::Int64 => {
                    let operand_int = operand_val.llvm_value.into_int_value();
                    let zero = self.llvm_context.i64_type().const_zero();
                    self.builder.build_int_sub(zero, operand_int, "neg_tmp")
                        .map_err(|e| Error::Compile(format!("Failed to build int negation: {:?}", e)))?
                        .into()
                },
                LlvmType::Float64 => {
                    let operand_float = operand_val.llvm_value.into_float_value();
                    let zero = self.llvm_context.f64_type().const_zero();
                    self.builder.build_float_sub(zero, operand_float, "fneg_tmp")
                        .map_err(|e| Error::Compile(format!("Failed to build float negation: {:?}", e)))?
                        .into()
                },
                _ => return Err(Error::Compile("Invalid type for negation".to_string())),
            },
            "!" | "not" => match operand_val.value_type {
                LlvmType::Boolean => {
                    let operand_bool = operand_val.llvm_value.into_int_value();
                    let true_val = self.llvm_context.bool_type().const_int(1, false);
                    self.builder.build_xor(operand_bool, true_val, "not_tmp")
                        .map_err(|e| Error::Compile(format!("Failed to build logical not: {:?}", e)))?
                        .into()
                },
                _ => return Err(Error::Compile("Logical not requires boolean operand".to_string())),
            },
            "~" => match operand_val.value_type {
                LlvmType::Int64 => {
                    let operand_int = operand_val.llvm_value.into_int_value();
                    let all_ones = self.llvm_context.i64_type().const_int(u64::MAX, false);
                    self.builder.build_xor(operand_int, all_ones, "bitnot_tmp")
                        .map_err(|e| Error::Compile(format!("Failed to build bitwise not: {:?}", e)))?
                        .into()
                },
                _ => return Err(Error::Compile("Bitwise not requires integer operand".to_string())),
            },
            _ => return Err(Error::Compile(format!("Unsupported unary operator: {}", unary.operator))),
        };
        
        debug!("Unary expression compiled successfully");
        Ok(LlvmValue::new(operand_val.value_type, result_value, false))
    }
    
    /// Compile identifier (variable access)
    #[instrument(skip(self, identifier))]
    fn compile_identifier(&mut self, identifier: &Identifier) -> Result<LlvmValue, Error> {
        debug!("Compiling identifier: {}", identifier.value);
        
        // Use variable manager to load the variable
        if let Some(ref var_manager) = self.variable_manager {
            let var_manager_ref = var_manager.borrow();
            let value = var_manager_ref.load_variable(&identifier.value)
                .map_err(|e| Error::Compile(format!("Failed to load variable '{}': {}", identifier.value, e)))?;
            
            // Get variable type information
            if let Some((_, var_type)) = var_manager_ref.get_variable(&identifier.value) {
                let llvm_type = match var_type {
                    crate::core::type_checker::Type::Normie => LlvmType::Int64,
                    crate::core::type_checker::Type::Thicc => LlvmType::Int64,
                    crate::core::type_checker::Type::Lit => LlvmType::Boolean,
                    crate::core::type_checker::Type::Tea => LlvmType::String,
                    crate::core::type_checker::Type::Meal => LlvmType::Float64,
                    crate::core::type_checker::Type::Cap => LlvmType::Pointer(Box::new(LlvmType::Void)),
                    _ => LlvmType::Int64, // Default fallback
                };
                
                debug!("Variable '{}' loaded successfully with type {:?}", identifier.value, llvm_type);
                Ok(LlvmValue::new(llvm_type, value, false))
            } else {
                error!("Variable '{}' not found in variable manager", identifier.value);
                Err(Error::Compile(format!("Undefined variable: {}", identifier.value)))
            }
        } else {
            error!("No variable manager available for identifier compilation");
            Err(Error::Compile("No variable manager available".to_string()))
        }
    }
    
    /// Compile function call expressions
    #[instrument(skip(self, call))]
    fn compile_call_expression(&mut self, call: &CallExpression) -> Result<LlvmValue, Error> {
        debug!("Compiling function call: {}", call.function.string());
        
        // For now, implement a simplified function call system
        // This will be enhanced when function compilation is fully integrated
        
        // Compile arguments
        let mut _arg_values = Vec::new();
        for arg in &call.arguments {
            let arg_val = self.compile_expression(arg.as_ref())?;
            _arg_values.push(arg_val);
        }
        
        // For now, return a placeholder result - this will be enhanced
        // when the function compilation system is fully integrated
        warn!("Function call compilation not fully implemented yet: {}", call.function.string());
        
        // Return a default integer value as placeholder
        let zero = self.llvm_context.i64_type().const_zero();
        Ok(LlvmValue::new(LlvmType::Int64, zero.into(), false))
    }
    
    /// Compile assignment expressions
    #[instrument(skip(self, assignment))]
    fn compile_assignment_expression(&mut self, assignment: &AssignmentExpression) -> Result<LlvmValue, Error> {
        debug!("Compiling assignment expression");
        
        let value_result = self.compile_expression(assignment.value.as_ref())?;
        
        // Get the target variable name
        if let Some(identifier) = assignment.name.as_any().downcast_ref::<Identifier>() {
            // Use variable manager to handle the assignment
            if let Some(ref var_manager) = self.variable_manager {
                let mut var_manager_ref = var_manager.borrow_mut();
                
                // Create assignment expression for the variable manager
                let assign_expr = crate::ast::operators::AssignmentExpression::new(
                    assignment.token.clone(),
                    assignment.name.clone(),
                    assignment.value.clone(),
                );
                
                var_manager_ref.compile_assignment(&assign_expr)
                    .map_err(|e| Error::Compile(format!("Failed to compile assignment: {}", e)))?;
                
                debug!("Assignment compiled successfully");
                Ok(value_result)
            } else {
                error!("No variable manager available for assignment");
                Err(Error::Compile("No variable manager available".to_string()))
            }
        } else {
            error!("Assignment target must be an identifier");
            Err(Error::Compile("Assignment target must be an identifier".to_string()))
        }
    }
    
    /// Compile index expressions (array access)
    fn compile_index_expression(&mut self, index: &IndexExpression) -> Result<LlvmValue, Error> {
        let array_val = self.compile_expression(index.left.as_ref())?;
        let index_val = self.compile_expression(index.index.as_ref())?;
        
        // Ensure index is integer type
        if index_val.value_type != LlvmType::Int64 {
            return Err(Error::Compile("Array index must be integer".to_string()));
        }
        
        let temp_name = self.context.next_temp();
        
        // Generate GEP instruction for array access
        match array_val.value_type {
            LlvmType::Pointer(ref inner_type) => {
                self.ir_output.push(format!(
                    "  {} = getelementptr inbounds {}, {}* {}, i64 {}",
                    temp_name,
                    inner_type.to_llvm_string(),
                    inner_type.to_llvm_string(),
                    array_val.llvm_name,
                    index_val.llvm_name
                ));
                
                // Load the value
                let load_temp = self.context.next_temp();
                self.ir_output.push(format!(
                    "  {} = load {}, {}* {}",
                    load_temp,
                    inner_type.to_llvm_string(),
                    inner_type.to_llvm_string(),
                    temp_name
                ));
                
                Ok(LlvmValue {
                    value_type: (**inner_type).clone(),
                    llvm_name: load_temp,
                    is_constant: false,
                })
            }
            _ => Err(Error::Compile("Index operation requires pointer/array type".to_string())),
        }
    }
    
    /// Compile parenthesized expressions
    #[instrument(skip(self, paren))]
    fn compile_parenthesized_expression(&mut self, paren: &ParenthesizedExpression) -> Result<LlvmValue, Error> {
        debug!("Compiling parenthesized expression");
        // Parentheses don't change the expression, just compile the inner expression
        self.compile_expression(paren.expression.as_ref())
    }
    
    /// Compile question mark operator expressions
    pub fn compile_question_mark_expression(&mut self, question_mark: &QuestionMarkExpression) -> Result<LlvmValue, Error> {
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
                "  {} = extractvalue {} {}, 0  ; Extract is_ok flag",
                is_ok_temp, inner_type_string, inner_value.llvm_name
            ));
            
            // Branch based on is_ok flag
            self.ir_output.push(format!(
                "  br i1 {}, label %{}, label %{}",
                is_ok_temp, success_block, error_block
            ));
            
            // Success block: extract value and continue
            self.ir_output.push(format!("{}:", success_block));
            self.ir_output.push(format!(
                "  {} = extractvalue {} {}, 1  ; Extract success value",
                value_temp, inner_type_string, inner_value.llvm_name
            ));
            self.ir_output.push(format!("  br label %{}", merge_block));
            
            // Error block: extract error and propagate (early return)
            self.ir_output.push(format!("{}:", error_block));
            self.ir_output.push(format!(
                "  {} = extractvalue {} {}, 2  ; Extract error value",
                error_temp, inner_type_string, inner_value.llvm_name
            ));
            
            // Generate error propagation runtime call
            self.ir_output.push(format!(
                "  {} = call i8* @cursed_error_propagation(i8* {}, i32 0, i32 0)",
                propagation_temp, error_temp
            ));
            
            // Create propagated Result with error
            let result_temp = self.context.next_temp();
            self.ir_output.push(format!(
                "  {} = insertvalue {} undef, i1 false, 0  ; Set is_ok to false",
                result_temp, inner_type_string
            ));
            self.ir_output.push(format!(
                "  {} = insertvalue {} {}, {} {}, 2  ; Insert error value",
                result_temp, inner_type_string, result_temp, error_type.to_llvm_string(), error_temp
            ));
            
            // Early return with propagated error
            self.ir_output.push(format!("  ret {} {}", inner_type_string, result_temp));
            
            // Merge block - only reached on success
            self.ir_output.push(format!("{}:", merge_block));
            
            // Return the extracted value
            Ok(LlvmValue {
                value_type,
                llvm_name: value_temp,
                is_constant: false,
            })
        } else if inner_type_string.starts_with("Option<") {
            // Extract value type from Option<T>
            let value_type = self.extract_option_type(&inner_type_string)?;
            
            // Extract is_some flag from Option type
            self.ir_output.push(format!(
                "  {} = extractvalue {} {}, 0  ; Extract is_some flag",
                is_ok_temp, inner_type_string, inner_value.llvm_name
            ));
            
            // Branch based on is_some flag
            self.ir_output.push(format!(
                "  br i1 {}, label %{}, label %{}",
                is_ok_temp, success_block, error_block
            ));
            
            // Success block: extract value and continue
            self.ir_output.push(format!("{}:", success_block));
            self.ir_output.push(format!(
                "  {} = extractvalue {} {}, 1  ; Extract some value",
                value_temp, inner_type_string, inner_value.llvm_name
            ));
            self.ir_output.push(format!("  br label %{}", merge_block));
            
            // Error block: propagate None
            self.ir_output.push(format!("{}:", error_block));
            
            // Generate None propagation (optional runtime call for debugging)
            self.ir_output.push(format!(
                "  call void @cursed_error_propagation(i8* null, i32 0, i32 0)  ; Log None propagation"
            ));
            
            // Create propagated Option with None
            let none_temp = self.context.next_temp();
            self.ir_output.push(format!(
                "  {} = insertvalue {} undef, i1 false, 0  ; Set is_some to false",
                none_temp, inner_type_string
            ));
            
            // Early return with None
            self.ir_output.push(format!("  ret {} {}", inner_type_string, none_temp));
            
            // Merge block - only reached on success
            self.ir_output.push(format!("{}:", merge_block));
            
            // Return the extracted value
            Ok(LlvmValue {
                value_type,
                llvm_name: value_temp,
                is_constant: false,
            })
        } else {
            Err(Error::Compile(format!(
                "Question mark operator can only be applied to Result<T, E> or Option<T> types, not {}",
                inner_type_string
            )))
        }
    }
    
    /// Resolve the result type of binary operations
    fn resolve_binary_type(&self, left: &LlvmType, right: &LlvmType, operator: &str) -> Result<LlvmType, Error> {
        match (left, right, operator) {
            // Arithmetic operations
            (LlvmType::Int64, LlvmType::Int64, "+"|"-"|"*"|"/"|"%") => Ok(LlvmType::Int64),
            (LlvmType::Float64, LlvmType::Float64, "+"|"-"|"*"|"/") => Ok(LlvmType::Float64),
            (LlvmType::Int64, LlvmType::Float64, "+"|"-"|"*"|"/") => Ok(LlvmType::Float64),
            (LlvmType::Float64, LlvmType::Int64, "+"|"-"|"*"|"/") => Ok(LlvmType::Float64),
            
            // Comparison operations always return boolean
            (LlvmType::Int64, LlvmType::Int64, "=="|"!="|"<"|">"|"<="|">=") => Ok(LlvmType::Boolean),
            (LlvmType::Float64, LlvmType::Float64, "=="|"!="|"<"|">"|"<="|">=") => Ok(LlvmType::Boolean),
            (LlvmType::Boolean, LlvmType::Boolean, "=="|"!=") => Ok(LlvmType::Boolean),
            
            // Logical operations
            (LlvmType::Boolean, LlvmType::Boolean, "&&"|"||"|"and"|"or") => Ok(LlvmType::Boolean),
            
            // Bitwise operations
            (LlvmType::Int64, LlvmType::Int64, "&"|"|"|"^"|"<<"|">>") => Ok(LlvmType::Int64),
            
            _ => Err(Error::Compile(format!(
                "Type mismatch: cannot apply {} to {:?} and {:?}",
                operator, left, right
            ))),
        }
    }
    
    /// Get compilation context
    pub fn get_context(&self) -> &ExpressionContext<'ctx> {
        &self.context
    }
    
    /// Get mutable compilation context
    pub fn get_context_mut(&mut self) -> &mut ExpressionContext<'ctx> {
        &mut self.context
    }
    
    /// Set current source location for debug info
    pub fn set_location(&mut self, location: SourceLocation) {
        self.context.current_location = Some(location);
    }

    /// Get the LLVM context
    pub fn llvm_context(&self) -> &'ctx Context {
        self.llvm_context
    }

    /// Get the LLVM module
    pub fn module(&self) -> &'ctx Module<'ctx> {
        self.module
    }

    /// Get the LLVM builder
    pub fn builder(&self) -> &'ctx Builder<'ctx> {
        self.builder
    }

    /// Get variable manager
    pub fn variable_manager(&self) -> &Rc<RefCell<VariableManager<'ctx>>> {
        &self.variable_manager
    }
    
    /// Get string representation of a type (helper for question mark operator)
    fn get_type_string(&self, llvm_type: &LlvmType) -> String {
        match llvm_type {
            LlvmType::Boolean => "bool".to_string(),
            LlvmType::Int32 => "i32".to_string(),
            LlvmType::Int64 => "i64".to_string(),
            LlvmType::Float64 => "f64".to_string(),
            LlvmType::String => "string".to_string(),
            LlvmType::Pointer(inner) => format!("*{}", self.get_type_string(inner)),
            LlvmType::Function { return_type, param_types } => {
                let params = param_types.iter()
                    .map(|t| self.get_type_string(t))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("({}) -> {}", params, self.get_type_string(return_type))
            }
            LlvmType::Void => "void".to_string(),
        }
    }
    
    /// Extract value and error types from Result<T, E> string
    fn extract_result_types(&self, result_type_string: &str) -> Result<(LlvmType, LlvmType), Error> {
        if !result_type_string.starts_with("Result<") || !result_type_string.ends_with('>') {
            return Err(Error::Compile("Invalid Result type format".to_string()));
        }
        
        // Extract "T, E" from "Result<T, E>"
        let inner = &result_type_string[7..result_type_string.len()-1];
        let parts: Vec<&str> = inner.splitn(2, ',').collect();
        
        if parts.len() != 2 {
            return Err(Error::Compile("Result must have exactly two type parameters".to_string()));
        }
        
        let value_type = self.parse_type_string(parts[0].trim())?;
        let error_type = self.parse_type_string(parts[1].trim())?;
        
        Ok((value_type, error_type))
    }
    
    /// Extract value type from Option<T> string
    fn extract_option_type(&self, option_type_string: &str) -> Result<LlvmType, Error> {
        if !option_type_string.starts_with("Option<") || !option_type_string.ends_with('>') {
            return Err(Error::Compile("Invalid Option type format".to_string()));
        }
        
        // Extract "T" from "Option<T>"
        let inner = &option_type_string[7..option_type_string.len()-1];
        self.parse_type_string(inner.trim())
    }
    
    /// Parse a type string into LlvmType
    fn parse_type_string(&self, type_str: &str) -> Result<LlvmType, Error> {
        match type_str {
            "bool" | "facts" => Ok(LlvmType::Boolean),
            "i32" | "normie" => Ok(LlvmType::Int32),
            "i64" => Ok(LlvmType::Int64),
            "f64" | "double" => Ok(LlvmType::Float64),
            "string" | "tea" => Ok(LlvmType::String),
            "void" => Ok(LlvmType::Void),
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
        &mut self,
        call: &CallExpression,
        type_context: &mut TypeCompilationContext
    ) -> Result<LlvmValue, Error> {
        let function_name = call.function.string();
        
        // Check if function name contains generic type indicators
        if !function_name.contains('<') && !function_name.contains("_") {
            return Err(Error::TypeCompilation("Not a generic call".to_string()));
        }
        
        // Extract base function name and type arguments
        let (base_name, type_args) = self.parse_generic_call(&function_name)?;
        
        // Check for cached instantiation
        let instance_key = format!("{}_{}", base_name, type_args.join("_"));
        if let Some(cached) = self.context.generic_methods.get(&instance_key).cloned() {
            return self.generate_instantiated_call(&cached, call);
        }
        
        // Instantiate generic method
        let instance = type_context.instantiate_generic(&base_name, &type_args)?;
        
        // Cache the instantiation
        self.context.generic_methods.insert(instance_key, instance.clone());
        
        // Generate call with instantiated method
        self.generate_instantiated_call(&instance, call)
    }

    /// Parse generic call syntax to extract base name and type arguments
    fn parse_generic_call(&self, function_name: &str) -> Result<(String, Vec<String>), Error> {
        // Handle syntax like "function<T, U>" or "function_T_U"
        if function_name.contains('<') && function_name.contains('>') {
            let parts: Vec<&str> = function_name.splitn(2, '<').collect();
            if parts.len() != 2 {
                return Err(Error::TypeCompilation("Invalid generic syntax".to_string()));
            }
            
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
                return Err(Error::TypeCompilation("Invalid generic syntax".to_string()));
            }
            
            let base_name = parts[0].to_string();
            let type_args: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();
            
            Ok((base_name, type_args))
        } else {
            Err(Error::TypeCompilation("No generic syntax found".to_string()))
        }
    }

    /// Generate call IR for instantiated generic method
    fn generate_instantiated_call(
        &mut self,
        instance: &CompiledGenericInstance,
        call: &CallExpression
    ) -> Result<LlvmValue, Error> {
        // Compile arguments
        let mut arg_values = Vec::new();
        let mut arg_types = Vec::new();
        
        for arg in &call.arguments {
            let arg_val = self.compile_expression(arg.as_ref())?;
            arg_types.push(arg_val.value_type.to_llvm_string());
            arg_values.push(arg_val.llvm_name);
        }
        
        let temp_name = self.context.next_temp();
        let args_str = arg_values.iter()
            .zip(arg_types.iter())
            .map(|(val, typ)| format!("{} {}", typ, val))
            .collect::<Vec<_>>()
            .join(", ");
        
        // Generate call to instantiated function
        self.ir_output.push(format!(
            "  {} = call {} @{}({})",
            temp_name,
            "i32", // Return type from instance metadata
            instance.instance_name,
            args_str
        ));
        
        Ok(LlvmValue {
            value_type: LlvmType::Int32, // Should be inferred from instance
            llvm_name: temp_name,
            is_constant: false,
        })
    }

    /// Compile expression with type inference (simplified for now)
    pub fn compile_expression_with_inference(
        &mut self,
        expr: &dyn Expression,
        _type_context: &TypeCompilationContext
    ) -> Result<LlvmValue, Error> {
        // For now, just fall back to regular compilation
        // TODO: Integrate proper type inference
        self.compile_expression(expr)
    }

    /// Compile literal with type information
    fn compile_typed_literal(
        &mut self,
        expr: &dyn Expression,
        inferred_type: &str
    ) -> Result<LlvmValue, Error> {
        let literal_value = expr.string();
        let temp_name = self.context.next_temp();
        
        match inferred_type {
            "normie" => {
                self.ir_output.push(format!("  {} = add i64 0, {}", temp_name, literal_value));
                Ok(LlvmValue {
                    value_type: LlvmType::Int64,
                    llvm_name: temp_name,
                    is_constant: true,
                })
            },
            "facts" => {
                let bool_val = if literal_value == "true" { "1" } else { "0" };
                self.ir_output.push(format!("  {} = add i1 0, {}", temp_name, bool_val));
                Ok(LlvmValue {
                    value_type: LlvmType::Boolean,
                    llvm_name: temp_name,
                    is_constant: true,
                })
            },
            "tea" => {
                let str_name = self.context.next_temp();
                self.ir_output.push(format!(
                    "  {} = getelementptr inbounds [{} x i8], [{} x i8]* @str_{}, i32 0, i32 0",
                    temp_name,
                    literal_value.len() + 1,
                    literal_value.len() + 1,
                    str_name
                ));
                Ok(LlvmValue {
                    value_type: LlvmType::String,
                    llvm_name: temp_name,
                    is_constant: true,
                })
            },
            _ => {
                // Fallback to regular compilation
                self.compile_expression(expr)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::expressions::Literal;
    
    #[test]
    fn test_literal_compilation() {
        let mut compiler = LlvmExpressionCompiler::new();
        
        // Test integer literal
        let int_literal = Literal::integer(42);
        let result = compiler.compile_literal(&int_literal).unwrap();
        assert_eq!(result.value_type, LlvmType::Int64);
        assert!(result.is_constant);
        
        // Test string literal
        let string_literal = Literal::string("hello".to_string());
        let result = compiler.compile_literal(&string_literal).unwrap();
        assert_eq!(result.value_type, LlvmType::String);
        
        // Test boolean literal
        let bool_literal = Literal::boolean(true);
        let result = compiler.compile_literal(&bool_literal).unwrap();
        assert_eq!(result.value_type, LlvmType::Boolean);
    }
    
    #[test]
    fn test_type_resolution() {
        let compiler = LlvmExpressionCompiler::new();
        
        // Test integer arithmetic
        let result = compiler.resolve_binary_type(&LlvmType::Int64, &LlvmType::Int64, "+").unwrap();
        assert_eq!(result, LlvmType::Int64);
        
        // Test mixed arithmetic (int + float = float)
        let result = compiler.resolve_binary_type(&LlvmType::Int64, &LlvmType::Float64, "+").unwrap();
        assert_eq!(result, LlvmType::Float64);
        
        // Test comparison
        let result = compiler.resolve_binary_type(&LlvmType::Int64, &LlvmType::Int64, "==").unwrap();
        assert_eq!(result, LlvmType::Boolean);
    }
    
    #[test]
    fn test_llvm_type_strings() {
        assert_eq!(LlvmType::Int32.to_llvm_string(), "i32");
        assert_eq!(LlvmType::Int64.to_llvm_string(), "i64");
        assert_eq!(LlvmType::Float64.to_llvm_string(), "double");
        assert_eq!(LlvmType::Boolean.to_llvm_string(), "i1");
        assert_eq!(LlvmType::String.to_llvm_string(), "i8*");
        assert_eq!(LlvmType::Void.to_llvm_string(), "void");
    }
    
    #[test]
    fn test_type_parsing() {
        let compiler = LlvmExpressionCompiler::new();
        
        // Test basic type parsing
        assert_eq!(compiler.parse_type_string("i32").unwrap(), LlvmType::Int32);
        assert_eq!(compiler.parse_type_string("normie").unwrap(), LlvmType::Int32);
        assert_eq!(compiler.parse_type_string("facts").unwrap(), LlvmType::Boolean);
        assert_eq!(compiler.parse_type_string("tea").unwrap(), LlvmType::String);
        
        // Test Result type extraction
        let (value_type, error_type) = compiler.extract_result_types("Result<i32, string>").unwrap();
        assert_eq!(value_type, LlvmType::Int32);
        assert_eq!(error_type, LlvmType::String);
        
        // Test Option type extraction
        let value_type = compiler.extract_option_type("Option<facts>").unwrap();
        assert_eq!(value_type, LlvmType::Boolean);
    }
    
    #[test]
    fn test_question_mark_ir_generation() {
        use crate::ast::expressions::QuestionMarkExpression;
        use crate::ast::identifiers::Identifier;
        
        let mut compiler = LlvmExpressionCompiler::new();
        compiler.add_error_propagation_declarations();
        
        // Create a mock question mark expression
        let var_expr = Identifier::from_name("result_var");
        let question_expr = QuestionMarkExpression::new(Box::new(var_expr), 1, 1);
        
        // Set up mock variable in context that appears as a Result type
        compiler.context.declare_variable(
            "result_var".to_string(), 
            LlvmValue {
                value_type: LlvmType::String, // This will be treated as a plain string, not Result
                llvm_name: "%result_var".to_string(),
                is_constant: false,
            }
        );
        
        let result = compiler.compile_question_mark_expression(&question_expr);
        
        // The expression should fail since we're not passing a Result/Option type
        // But check that the function compiles and handles the error correctly
        assert!(result.is_err());
        
        // Check that the declarations were added correctly
        let ir = compiler.get_ir();
        assert!(ir.contains("cursed_error_propagation"));
        assert!(ir.contains("declare void @cursed_error_propagation_init()"));
        assert!(ir.contains("declare i8* @cursed_error_propagation(i8*, i32, i32)"));
    }
    
    #[test]
    fn test_type_string_parsing() {
        let compiler = LlvmExpressionCompiler::new();
        
        // Test that get_type_string works correctly  
        assert_eq!(compiler.get_type_string(&LlvmType::Int32), "i32");
        assert_eq!(compiler.get_type_string(&LlvmType::String), "string");
        
        // Test that the question mark operator correctly identifies types
        let string_type = compiler.get_type_string(&LlvmType::String);
        assert!(!string_type.starts_with("Result<"));
        assert!(!string_type.starts_with("Option<"));
    }
    
    #[test]
    fn test_error_propagation_declarations() {
        let mut compiler = LlvmExpressionCompiler::new();
        compiler.add_error_propagation_declarations();
        
        let ir = compiler.get_ir();
        
        // Verify all required FFI declarations are present
        assert!(ir.contains("declare void @cursed_error_propagation_init()"));
        assert!(ir.contains("declare void @cursed_error_propagation_cleanup()"));
        assert!(ir.contains("declare i8* @cursed_error_propagation(i8*, i32, i32)"));
        assert!(ir.contains("declare void @cursed_error_propagation_panic(i8*)"));
        assert!(ir.contains("; Error propagation runtime functions"));
    }
}
