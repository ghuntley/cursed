/// LLVM expression compilation for the CURSED programming language
/// 
/// This module provides comprehensive compilation of all AST expression types
/// to LLVM IR, including proper type handling, error reporting, and 
/// support for Gen Z slang syntax.

use crate::ast::traits::Expression;
use crate::ast::{
    operators::{BinaryExpression, UnaryExpression, AssignmentExpression, IndexExpression},
    expressions::{Literal, LiteralValue, ParenthesizedExpression, FunctionLiteral},
    identifiers::{Identifier, QualifiedName, TypeIdentifier},
    calls::CallExpression,
};
use crate::error::Error;
use crate::debug::SourceLocation;
use crate::type_system::{TypeSystem, TypeInference, TypeEnvironment};
use crate::codegen::llvm::type_system::{TypeCompilationContext, CompiledGenericInstance};
use std::any::Any;
use std::collections::HashMap;

/// LLVM value representation for compiled expressions
#[derive(Debug, Clone)]
pub struct LlvmValue {
    pub value_type: LlvmType,
    pub llvm_name: String,
    pub is_constant: bool,
}

impl LlvmValue {
    /// Create a new LLVM value with a name (stub implementation)
    pub fn new(name: &str) -> Self {
        Self {
            value_type: LlvmType::String,
            llvm_name: name.to_string(),
            is_constant: false,
        }
    }
    
    /// Check if this is a struct value
    pub fn is_struct_value(&self) -> bool {
        true
    }
    
    /// Check if this is a float value
    pub fn is_float_value(&self) -> bool {
        matches!(self.value_type, LlvmType::Float64)
    }
    
    /// Check if this is an int value
    pub fn is_int_value(&self) -> bool {
        matches!(self.value_type, LlvmType::Int32 | LlvmType::Int64)
    }
    
    /// Convert to int value (stub)
    pub fn into_int_value(&self) -> Self {
        self.clone()
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
pub struct ExpressionContext {
    pub variable_map: HashMap<String, LlvmValue>,
    pub function_map: HashMap<String, LlvmValue>,
    pub type_map: HashMap<String, LlvmType>,
    pub temp_counter: u32,
    pub current_location: Option<SourceLocation>,
    /// Type compilation context for constraint resolution
    pub type_context: Option<TypeCompilationContext>,
    /// Generic method instantiation cache
    pub generic_methods: HashMap<String, CompiledGenericInstance>,
}

impl ExpressionContext {
    pub fn new() -> Self {
        Self {
            variable_map: HashMap::new(),
            function_map: HashMap::new(),
            type_map: HashMap::new(),
            temp_counter: 0,
            current_location: None,
            type_context: None,
            generic_methods: HashMap::new(),
        }
    }

    /// Create context with type compilation support
    pub fn with_type_context(type_context: TypeCompilationContext) -> Self {
        Self {
            variable_map: HashMap::new(),
            function_map: HashMap::new(),
            type_map: HashMap::new(),
            temp_counter: 0,
            current_location: None,
            type_context: Some(type_context),
            generic_methods: HashMap::new(),
        }
    }
    
    pub fn next_temp(&mut self) -> String {
        self.temp_counter += 1;
        format!("%temp_{}", self.temp_counter)
    }
    
    pub fn declare_variable(&mut self, name: String, value: LlvmValue) {
        self.variable_map.insert(name, value);
    }
    
    pub fn get_variable(&self, name: &str) -> Option<&LlvmValue> {
        self.variable_map.get(name)
    }
}

/// Main expression compiler
pub struct LlvmExpressionCompiler {
    context: ExpressionContext,
    ir_output: Vec<String>,
}

impl LlvmExpressionCompiler {
    pub fn new() -> Self {
        Self {
            context: ExpressionContext::new(),
            ir_output: Vec::new(),
        }
    }
    
    /// Compile any expression to LLVM IR
    pub fn compile_expression(&mut self, expr: &dyn Expression) -> Result<LlvmValue, Error> {
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
        } else {
            Err(Error::Compile(format!(
                "Unsupported expression type for compilation: {}",
                expr.string()
            )))
        }
    }
    
    /// Compile literal expressions
    fn compile_literal(&mut self, literal: &Literal) -> Result<LlvmValue, Error> {
        match &literal.value {
            LiteralValue::Integer(value) => {
                let temp_name = self.context.next_temp();
                self.ir_output.push(format!("  {} = add i64 0, {}", temp_name, value));
                Ok(LlvmValue {
                    value_type: LlvmType::Int64,
                    llvm_name: temp_name,
                    is_constant: true,
                })
            }
            LiteralValue::Float(value) => {
                let temp_name = self.context.next_temp();
                self.ir_output.push(format!("  {} = fadd double 0.0, {}", temp_name, value));
                Ok(LlvmValue {
                    value_type: LlvmType::Float64,
                    llvm_name: temp_name,
                    is_constant: true,
                })
            }
            LiteralValue::String(value) => {
                let temp_name = self.context.next_temp();
                let string_literal = format!("@.str_{}", self.context.temp_counter);
                
                // Add global string constant
                self.ir_output.push(format!(
                    "{} = private unnamed_addr constant [{} x i8] c\"{}\\00\", align 1",
                    string_literal,
                    value.len() + 1,
                    value.replace("\"", "\\22").replace("\n", "\\0A")
                ));
                
                // Get pointer to string
                self.ir_output.push(format!(
                    "  {} = getelementptr inbounds [{} x i8], [{} x i8]* {}, i64 0, i64 0",
                    temp_name, value.len() + 1, value.len() + 1, string_literal
                ));
                
                Ok(LlvmValue {
                    value_type: LlvmType::String,
                    llvm_name: temp_name,
                    is_constant: true,
                })
            }
            LiteralValue::Boolean(value) => {
                let temp_name = self.context.next_temp();
                let bool_val = if *value { 1 } else { 0 };
                self.ir_output.push(format!("  {} = add i1 0, {}", temp_name, bool_val));
                Ok(LlvmValue {
                    value_type: LlvmType::Boolean,
                    llvm_name: temp_name,
                    is_constant: true,
                })
            }
            LiteralValue::Nil => {
                let temp_name = self.context.next_temp();
                self.ir_output.push(format!("  {} = inttoptr i64 0 to i8*", temp_name));
                Ok(LlvmValue {
                    value_type: LlvmType::Pointer(Box::new(LlvmType::Void)),
                    llvm_name: temp_name,
                    is_constant: true,
                })
            }
            LiteralValue::Character(value) => {
                let temp_name = self.context.next_temp();
                self.ir_output.push(format!("  {} = add i8 0, {}", temp_name, *value as u8));
                Ok(LlvmValue {
                    value_type: LlvmType::Int32, // Characters are promoted to int32
                    llvm_name: temp_name,
                    is_constant: true,
                })
            }
        }
    }
    
    /// Compile binary expressions (arithmetic, logical, comparison)
    fn compile_binary_expression(&mut self, binary: &BinaryExpression) -> Result<LlvmValue, Error> {
        let left_val = self.compile_expression(binary.left.as_ref())?;
        let right_val = self.compile_expression(binary.right.as_ref())?;
        
        // Type checking and coercion
        let result_type = self.resolve_binary_type(&left_val.value_type, &right_val.value_type, &binary.operator)?;
        let temp_name = self.context.next_temp();
        
        // Generate appropriate LLVM instruction based on operator and types
        let instruction = match binary.operator.as_str() {
            // Arithmetic operators
            "+" => match result_type {
                LlvmType::Int64 => format!("add i64 {}, {}", left_val.llvm_name, right_val.llvm_name),
                LlvmType::Float64 => format!("fadd double {}, {}", left_val.llvm_name, right_val.llvm_name),
                _ => return Err(Error::Compile("Invalid type for addition".to_string())),
            },
            "-" => match result_type {
                LlvmType::Int64 => format!("sub i64 {}, {}", left_val.llvm_name, right_val.llvm_name),
                LlvmType::Float64 => format!("fsub double {}, {}", left_val.llvm_name, right_val.llvm_name),
                _ => return Err(Error::Compile("Invalid type for subtraction".to_string())),
            },
            "*" => match result_type {
                LlvmType::Int64 => format!("mul i64 {}, {}", left_val.llvm_name, right_val.llvm_name),
                LlvmType::Float64 => format!("fmul double {}, {}", left_val.llvm_name, right_val.llvm_name),
                _ => return Err(Error::Compile("Invalid type for multiplication".to_string())),
            },
            "/" => match result_type {
                LlvmType::Int64 => format!("sdiv i64 {}, {}", left_val.llvm_name, right_val.llvm_name),
                LlvmType::Float64 => format!("fdiv double {}, {}", left_val.llvm_name, right_val.llvm_name),
                _ => return Err(Error::Compile("Invalid type for division".to_string())),
            },
            "%" => match result_type {
                LlvmType::Int64 => format!("srem i64 {}, {}", left_val.llvm_name, right_val.llvm_name),
                _ => return Err(Error::Compile("Modulo only supported for integers".to_string())),
            },
            
            // Comparison operators
            "==" => match left_val.value_type {
                LlvmType::Int64 => format!("icmp eq i64 {}, {}", left_val.llvm_name, right_val.llvm_name),
                LlvmType::Float64 => format!("fcmp oeq double {}, {}", left_val.llvm_name, right_val.llvm_name),
                LlvmType::Boolean => format!("icmp eq i1 {}, {}", left_val.llvm_name, right_val.llvm_name),
                _ => return Err(Error::Compile("Invalid type for equality comparison".to_string())),
            },
            "!=" => match left_val.value_type {
                LlvmType::Int64 => format!("icmp ne i64 {}, {}", left_val.llvm_name, right_val.llvm_name),
                LlvmType::Float64 => format!("fcmp one double {}, {}", left_val.llvm_name, right_val.llvm_name),
                LlvmType::Boolean => format!("icmp ne i1 {}, {}", left_val.llvm_name, right_val.llvm_name),
                _ => return Err(Error::Compile("Invalid type for inequality comparison".to_string())),
            },
            "<" => match left_val.value_type {
                LlvmType::Int64 => format!("icmp slt i64 {}, {}", left_val.llvm_name, right_val.llvm_name),
                LlvmType::Float64 => format!("fcmp olt double {}, {}", left_val.llvm_name, right_val.llvm_name),
                _ => return Err(Error::Compile("Invalid type for less-than comparison".to_string())),
            },
            ">" => match left_val.value_type {
                LlvmType::Int64 => format!("icmp sgt i64 {}, {}", left_val.llvm_name, right_val.llvm_name),
                LlvmType::Float64 => format!("fcmp ogt double {}, {}", left_val.llvm_name, right_val.llvm_name),
                _ => return Err(Error::Compile("Invalid type for greater-than comparison".to_string())),
            },
            "<=" => match left_val.value_type {
                LlvmType::Int64 => format!("icmp sle i64 {}, {}", left_val.llvm_name, right_val.llvm_name),
                LlvmType::Float64 => format!("fcmp ole double {}, {}", left_val.llvm_name, right_val.llvm_name),
                _ => return Err(Error::Compile("Invalid type for less-than-or-equal comparison".to_string())),
            },
            ">=" => match left_val.value_type {
                LlvmType::Int64 => format!("icmp sge i64 {}, {}", left_val.llvm_name, right_val.llvm_name),
                LlvmType::Float64 => format!("fcmp oge double {}, {}", left_val.llvm_name, right_val.llvm_name),
                _ => return Err(Error::Compile("Invalid type for greater-than-or-equal comparison".to_string())),
            },
            
            // Logical operators
            "&&" | "and" => format!("and i1 {}, {}", left_val.llvm_name, right_val.llvm_name),
            "||" | "or" => format!("or i1 {}, {}", left_val.llvm_name, right_val.llvm_name),
            
            // Bitwise operators
            "&" => format!("and i64 {}, {}", left_val.llvm_name, right_val.llvm_name),
            "|" => format!("or i64 {}, {}", left_val.llvm_name, right_val.llvm_name),
            "^" => format!("xor i64 {}, {}", left_val.llvm_name, right_val.llvm_name),
            "<<" => format!("shl i64 {}, {}", left_val.llvm_name, right_val.llvm_name),
            ">>" => format!("ashr i64 {}, {}", left_val.llvm_name, right_val.llvm_name),
            
            _ => return Err(Error::Compile(format!("Unsupported binary operator: {}", binary.operator))),
        };
        
        self.ir_output.push(format!("  {} = {}", temp_name, instruction));
        
        Ok(LlvmValue {
            value_type: result_type,
            llvm_name: temp_name,
            is_constant: false,
        })
    }
    
    /// Compile unary expressions
    fn compile_unary_expression(&mut self, unary: &UnaryExpression) -> Result<LlvmValue, Error> {
        let operand_val = self.compile_expression(unary.operand.as_ref())?;
        let temp_name = self.context.next_temp();
        
        let instruction = match unary.operator.as_str() {
            "-" => match operand_val.value_type {
                LlvmType::Int64 => format!("sub i64 0, {}", operand_val.llvm_name),
                LlvmType::Float64 => format!("fsub double 0.0, {}", operand_val.llvm_name),
                _ => return Err(Error::Compile("Invalid type for negation".to_string())),
            },
            "!" | "not" => match operand_val.value_type {
                LlvmType::Boolean => format!("xor i1 {}, true", operand_val.llvm_name),
                _ => return Err(Error::Compile("Logical not requires boolean operand".to_string())),
            },
            "~" => match operand_val.value_type {
                LlvmType::Int64 => format!("xor i64 {}, -1", operand_val.llvm_name),
                _ => return Err(Error::Compile("Bitwise not requires integer operand".to_string())),
            },
            _ => return Err(Error::Compile(format!("Unsupported unary operator: {}", unary.operator))),
        };
        
        self.ir_output.push(format!("  {} = {}", temp_name, instruction));
        
        Ok(LlvmValue {
            value_type: operand_val.value_type,
            llvm_name: temp_name,
            is_constant: false,
        })
    }
    
    /// Compile identifier (variable access)
    fn compile_identifier(&mut self, identifier: &Identifier) -> Result<LlvmValue, Error> {
        // Clone the variable to avoid borrow checker issues
        if let Some(variable) = self.context.get_variable(&identifier.value).cloned() {
            // Load the variable value
            let temp_name = self.context.next_temp();
            self.ir_output.push(format!(
                "  {} = load {}, {}* @{}",
                temp_name,
                variable.value_type.to_llvm_string(),
                variable.value_type.to_llvm_string(),
                identifier.value
            ));
            
            Ok(LlvmValue {
                value_type: variable.value_type,
                llvm_name: temp_name,
                is_constant: false,
            })
        } else {
            Err(Error::Compile(format!(
                "Undefined variable: {}",
                identifier.value
            )))
        }
    }
    
    /// Compile function call expressions
    fn compile_call_expression(&mut self, call: &CallExpression) -> Result<LlvmValue, Error> {
        // Compile function expression
        let function_val = self.compile_expression(call.function.as_ref())?;
        
        // Compile arguments
        let mut arg_values = Vec::new();
        let mut arg_types = Vec::new();
        
        for arg in &call.arguments {
            let arg_val = self.compile_expression(arg.as_ref())?;
            arg_types.push(arg_val.value_type.to_llvm_string());
            arg_values.push(arg_val.llvm_name);
        }
        
        // Determine return type (simplified - would need more sophisticated type analysis)
        let return_type = match function_val.value_type {
            LlvmType::Function { return_type, .. } => *return_type,
            _ => LlvmType::Int64, // Default assumption
        };
        
        let temp_name = self.context.next_temp();
        let args_str = arg_values.iter()
            .zip(arg_types.iter())
            .map(|(val, typ)| format!("{} {}", typ, val))
            .collect::<Vec<_>>()
            .join(", ");
        
        self.ir_output.push(format!(
            "  {} = call {} {}({})",
            temp_name,
            return_type.to_llvm_string(),
            function_val.llvm_name,
            args_str
        ));
        
        Ok(LlvmValue {
            value_type: return_type,
            llvm_name: temp_name,
            is_constant: false,
        })
    }
    
    /// Compile assignment expressions
    fn compile_assignment_expression(&mut self, assignment: &AssignmentExpression) -> Result<LlvmValue, Error> {
        let value_result = self.compile_expression(assignment.value.as_ref())?;
        
        // Get the target variable name
        if let Some(identifier) = assignment.name.as_any().downcast_ref::<Identifier>() {
            // Store the value
            self.ir_output.push(format!(
                "  store {} {}, {}* @{}",
                value_result.value_type.to_llvm_string(),
                value_result.llvm_name,
                value_result.value_type.to_llvm_string(),
                identifier.value
            ));
            
            // Update context
            self.context.declare_variable(identifier.value.clone(), value_result.clone());
            
            Ok(value_result)
        } else {
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
    fn compile_parenthesized_expression(&mut self, paren: &ParenthesizedExpression) -> Result<LlvmValue, Error> {
        // Parentheses don't change the expression, just compile the inner expression
        self.compile_expression(paren.expression.as_ref())
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
    
    /// Get the generated LLVM IR
    pub fn get_ir(&self) -> String {
        self.ir_output.join("\n")
    }
    
    /// Clear the IR output
    pub fn clear_ir(&mut self) {
        self.ir_output.clear();
    }
    
    /// Get compilation context
    pub fn get_context(&self) -> &ExpressionContext {
        &self.context
    }
    
    /// Set current source location for debug info
    pub fn set_location(&mut self, location: SourceLocation) {
        self.context.current_location = Some(location);
    }
}

impl Default for LlvmExpressionCompiler {
    fn default() -> Self {
        Self::new()
    }
}

impl LlvmExpressionCompiler {
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
        let string_literal = Literal::string("hello");
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
}
