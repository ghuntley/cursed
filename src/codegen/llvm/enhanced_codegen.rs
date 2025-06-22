//! Enhanced LLVM Code Generator with Debug Integration
//! 
//! This module provides a comprehensive LLVM code generator that integrates
//! debug metadata generation throughout the compilation pipeline. It ensures
//! that all AST nodes are properly mapped to source locations and debug
//! information is generated for functions, variables, and expressions.

use crate::ast::{AstNodeType, Expression, Statement, FunctionStatement, VariableStatement, Parameter};
use crate::codegen::llvm::debug_metadata::{LlvmDebugMetadata, DebugStats, LlvmDebugIntegration};
use crate::debug::{DebugConfig, SourceLocation};
use crate::error::Error as CursedError;

use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::{FunctionValue, BasicValueEnum, PointerValue, InstructionValue};
use inkwell::types::{BasicTypeEnum, FunctionType, IntType, FloatType};
use inkwell::{AddressSpace, IntPredicate, FloatPredicate, OptimizationLevel};
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::targets::{Target, TargetMachine, RelocMode, CodeModel, FileType};

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::fmt;

use tracing::{debug, error, info, instrument, warn, span, Level};

/// Enhanced LLVM code generator with comprehensive debug support
pub struct EnhancedLlvmCodegen<'ctx> {
    /// LLVM context
    context: &'ctx Context,
    
    /// LLVM module
    module: Module<'ctx>,
    
    /// LLVM IR builder
    builder: Builder<'ctx>,
    
    /// Debug metadata generator
    debug_metadata: Option<LlvmDebugMetadata<'ctx>>,
    
    /// Compiled functions cache
    functions: HashMap<String, FunctionValue<'ctx>>,
    
    /// Variable storage mapping
    variables: HashMap<String, PointerValue<'ctx>>,
    
    /// Current function context
    current_function: Option<FunctionValue<'ctx>>,
    
    /// Configuration
    config: CodegenConfig,
    
    /// Statistics
    stats: CodegenStats,
    
    /// Error tracking
    errors: Vec<CursedError>,
}

/// Configuration for the enhanced code generator
#[derive(Debug, Clone)]
pub struct CodegenConfig {
    /// Debug configuration
    pub debug_config: DebugConfig,
    
    /// Optimization level
    pub optimization_level: OptimizationLevel,
    
    /// Target triple
    pub target_triple: Option<String>,
    
    /// Enable verification
    pub verify_module: bool,
    
    /// Enable execution engine
    pub enable_jit: bool,
    
    /// Module name
    pub module_name: String,
}

impl Default for CodegenConfig {
    fn default() -> Self {
        Self {
            debug_config: DebugConfig::default(),
            optimization_level: OptimizationLevel::O0,
            target_triple: None,
            verify_module: true,
            enable_jit: false,
            module_name: "cursed_module".to_string(),
        }
    }
}

/// Code generation statistics
#[derive(Debug, Clone, Default)]
pub struct CodegenStats {
    pub functions_compiled: usize,
    pub variables_allocated: usize,
    pub expressions_compiled: usize,
    pub statements_compiled: usize,
    pub debug_locations_set: usize,
    pub errors_encountered: usize,
}

impl<'ctx> EnhancedLlvmCodegen<'ctx> {
    /// Create a new enhanced LLVM code generator
    #[instrument(skip(context), fields(module = %config.module_name))]
    pub fn new(
        context: &'ctx Context,
        source_file: &Path,
        config: CodegenConfig,
    ) -> Result<Self, CursedError> {
        let span = span!(Level::INFO, "enhanced_codegen_init");
        let _enter = span.enter();
        
        info!("Creating enhanced LLVM code generator with debug support");
        
        // Create LLVM module
        let module = context.create_module(&config.module_name);
        
        // Set target triple if specified
        if let Some(triple) = &config.target_triple {
            module.set_triple(triple);
        }
        
        // Create builder
        let builder = context.create_builder();
        
        // Create debug metadata if enabled
        let debug_metadata = if config.debug_config.generate_debug_info {
            Some(LlvmDebugMetadata::new(
                context,
                &module,
                &builder,
                source_file,
                config.debug_config.clone(),
            )?)
        } else {
            None
        };
        
        let codegen = Self {
            context,
            module,
            builder,
            debug_metadata,
            functions: HashMap::new(),
            variables: HashMap::new(),
            current_function: None,
            config,
            stats: CodegenStats::default(),
            errors: Vec::new(),
        };
        
        info!("Enhanced LLVM code generator created successfully");
        Ok(codegen)
    }
    
    /// Compile an entire AST with debug information
    #[instrument(skip(self, ast), fields(ast_type = ?std::mem::discriminant(ast)))]
    pub fn compile_ast(&mut self, ast: &AST) -> Result<(), CursedError> {
        let span = span!(Level::INFO, "compile_ast");
        let _enter = span.enter();
        
        info!("Compiling AST with debug information");
        
        match &ast.node_type {
            AstNodeType::Program(program) => {
                // Set debug location for program start
                if let Some(debug) = &mut self.debug_metadata {
                    if let Some(location) = &ast.location {
                        debug.set_debug_location_from_source(location)?;
                    }
                }
                
                for statement in &program.statements {
                    self.compile_statement(statement)?;
                }
            }
            AstNodeType::ModuleDeclaration(module_decl) => {
                info!(module = %module_decl.name.value, "Compiling module");
                
                if let Some(debug) = &mut self.debug_metadata {
                    if let Some(location) = &ast.location {
                        debug.set_debug_location_from_source(location)?;
                    }
                }
                
                for item in &module_decl.body {
                    self.compile_statement(item)?;
                }
            }
            AstNodeType::FunctionDeclaration(func_decl) => {
                self.compile_function(func_decl)?;
            }
            _ => {
                warn!("Unsupported AST node type for compilation");
                return Err(CursedError::Compile("Unsupported AST node".to_string()));
            }
        }
        
        info!("AST compilation completed successfully");
        Ok(())
    }
    
    /// Compile a function declaration with debug information
    #[instrument(skip(self, func_decl), fields(name = %func_decl.name))]
    pub fn compile_function(&mut self, func_decl: &FunctionDeclaration) -> Result<FunctionValue<'ctx>, CursedError> {
        let span = span!(Level::DEBUG, "compile_function", function = %func_decl.name);
        let _enter = span.enter();
        
        debug!("Compiling function with debug information");
        
        // Create function type
        let return_type = self.get_llvm_type(&func_decl.return_type)?;
        let param_types: Result<Vec<_>, _> = func_decl.parameters
            .iter()
            .map(|param| self.get_llvm_type(&param.param_type))
            .collect();
        let param_types = param_types?;
        
        let function_type = match return_type {
            Some(ret_type) => ret_type.fn_type(&param_types, false),
            None => self.context.void_type().fn_type(&param_types, false),
        };
        
        // Create LLVM function
        let function = self.module.add_function(&func_decl.name, function_type, None);
        
        // Generate debug information for function
        if let Some(debug) = &mut self.debug_metadata {
            debug.generate_function_debug(function, &func_decl.name, func_decl)?;
        }
        
        // Set function as current
        self.current_function = Some(function);
        
        // Create entry basic block
        let entry_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry_block);
        
        // Allocate space for parameters
        for (i, param) in func_decl.parameters.iter().enumerate() {
            if let Some(llvm_param) = function.get_nth_param(i as u32) {
                let param_type = self.get_llvm_type(&param.param_type)?
                    .ok_or_else(|| CursedError::Compile("Invalid parameter type".to_string()))?;
                
                let alloca = self.builder.build_alloca(param_type, &param.name)
                    .map_err(|e| CursedError::Compile(format!("Parameter allocation failed: {}", e)))?;
                
                self.builder.build_store(alloca, llvm_param)
                    .map_err(|e| CursedError::Compile(format!("Parameter store failed: {}", e)))?;
                
                self.variables.insert(param.name.clone(), alloca);
                self.stats.variables_allocated += 1;
            }
        }
        
        // Compile function body
        for statement in &func_decl.body {
            self.compile_statement(statement)?;
        }
        
        // Add default return if needed
        if !self.has_terminator() {
            match func_decl.return_type.as_str() {
                "void" => {
                    self.builder.build_return(None)
                        .map_err(|e| CursedError::Compile(format!("Return build failed: {}", e)))?;
                }
                _ => {
                    let zero_value = self.get_zero_value(&func_decl.return_type)?;
                    self.builder.build_return(Some(&zero_value))
                        .map_err(|e| CursedError::Compile(format!("Return build failed: {}", e)))?;
                }
            }
        }
        
        // Cache the function
        self.functions.insert(func_decl.name.clone(), function);
        self.stats.functions_compiled += 1;
        
        info!(function = %func_decl.name, "Function compilation completed successfully");
        Ok(function)
    }
    
    /// Compile a statement with debug information
    #[instrument(skip(self, stmt), fields(stmt_type = ?std::mem::discriminant(stmt)))]
    pub fn compile_statement(&mut self, stmt: &dyn Statement) -> Result<(), CursedError> {
        let span = span!(Level::TRACE, "compile_statement");
        let _enter = span.enter();
        
        // Generate debug information for statement
        if let Some(debug) = &mut self.debug_metadata {
            debug.generate_statement_debug(stmt)?;
        }
        
        match stmt {
            Statement::VariableDeclaration { declaration, location } => {
                self.compile_variable_declaration(declaration)?;
            }
            Statement::Expression { expression, location } => {
                if let Some(debug) = &mut self.debug_metadata {
                    debug.set_debug_location_from_source(location)?;
                }
                self.compile_expression(expression)?;
            }
            Statement::Return { value, location } => {
                if let Some(debug) = &mut self.debug_metadata {
                    debug.set_debug_location_from_source(location)?;
                }
                
                match value {
                    Some(expr) => {
                        let return_value = self.compile_expression(expr)?;
                        self.builder.build_return(Some(&return_value))
                            .map_err(|e| CursedError::Compile(format!("Return build failed: {}", e)))?;
                    }
                    None => {
                        self.builder.build_return(None)
                            .map_err(|e| CursedError::Compile(format!("Return build failed: {}", e)))?;
                    }
                }
            }
            Statement::If { condition, then_branch, else_branch, location } => {
                self.compile_if_statement(condition, then_branch, else_branch.as_ref(), location)?;
            }
            Statement::While { condition, body, location } => {
                self.compile_while_statement(condition, body, location)?;
            }
            Statement::Block { statements, location } => {
                self.compile_block_statement(statements, location)?;
            }
            _ => {
                warn!("Unsupported statement type");
                return Err(CursedError::Compile("Unsupported statement".to_string()));
            }
        }
        
        self.stats.statements_compiled += 1;
        Ok(())
    }
    
    /// Compile a variable declaration with debug information
    #[instrument(skip(self, var_decl), fields(name = %var_decl.name))]
    fn compile_variable_declaration(&mut self, var_decl: &VariableDeclaration) -> Result<(), CursedError> {
        debug!("Compiling variable declaration with debug information");
        
        // Get variable type
        let var_type = self.get_llvm_type(&var_decl.var_type)?
            .ok_or_else(|| CursedError::Compile("Invalid variable type".to_string()))?;
        
        // Allocate storage
        let alloca = self.builder.build_alloca(var_type, &var_decl.name)
            .map_err(|e| CursedError::Compile(format!("Variable allocation failed: {}", e)))?;
        
        // Generate debug information for variable
        if let Some(debug) = &mut self.debug_metadata {
            debug.generate_variable_debug(&var_decl.name, alloca, var_decl)?;
        }
        
        // Initialize with value if provided
        if let Some(init_expr) = &var_decl.value {
            let init_value = self.compile_expression(init_expr)?;
            self.builder.build_store(alloca, init_value)
                .map_err(|e| CursedError::Compile(format!("Variable store failed: {}", e)))?;
        }
        
        // Cache the variable
        self.variables.insert(var_decl.name.clone(), alloca);
        self.stats.variables_allocated += 1;
        
        info!(variable = %var_decl.name, "Variable declaration compiled successfully");
        Ok(())
    }
    
    /// Compile an expression with debug information
    #[instrument(skip(self, expr), fields(expr_type = ?std::mem::discriminant(expr)))]
    pub fn compile_expression(&mut self, expr: &dyn Expression) -> Result<BasicValueEnum<'ctx>, CursedError> {
        let span = span!(Level::TRACE, "compile_expression");
        let _enter = span.enter();
        
        // Set debug location for expression
        if let Some(debug) = &mut self.debug_metadata {
            debug.generate_expression_debug(expr, None)?;
        }
        
        let result = match expr {
            Expression::Literal { value, location } => {
                self.compile_literal(value, location)
            }
            Expression::Variable { name, location } => {
                self.compile_variable_reference(name, location)
            }
            Expression::BinaryOp { left, operator, right, location } => {
                self.compile_binary_operation(left, operator, right, location)
            }
            Expression::UnaryOp { operator, operand, location } => {
                self.compile_unary_operation(operator, operand, location)
            }
            Expression::FunctionCall { name, arguments, location } => {
                self.compile_function_call(name, arguments, location)
            }
            Expression::Assignment { target, value, location } => {
                self.compile_assignment(target, value, location)
            }
            _ => {
                warn!("Unsupported expression type");
                Err(CursedError::Compile("Unsupported expression".to_string()))
            }
        };
        
        if result.is_ok() {
            self.stats.expressions_compiled += 1;
        } else {
            self.stats.errors_encountered += 1;
        }
        
        result
    }
    
    /// Compile a literal value
    fn compile_literal(&self, literal: &crate::ast::Literal, location: &SourceLocation) -> Result<BasicValueEnum<'ctx>, CursedError> {
        match literal {
            crate::ast::Literal::Integer(val) => {
                Ok(self.context.i32_type().const_int(*val as u64, false).into())
            }
            crate::ast::Literal::Float(val) => {
                Ok(self.context.f64_type().const_float(*val).into())
            }
            crate::ast::Literal::String(val) => {
                let string_value = self.builder.build_global_string_ptr(val, "str")
                    .map_err(|e| CursedError::Compile(format!("String literal failed: {}", e)))?;
                Ok(string_value.as_pointer_value().into())
            }
            crate::ast::Literal::Boolean(val) => {
                Ok(self.context.bool_type().const_int(if *val { 1 } else { 0 }, false).into())
            }
            _ => Err(CursedError::Compile("Unsupported literal type".to_string())),
        }
    }
    
    /// Compile a variable reference
    fn compile_variable_reference(&self, name: &str, location: &SourceLocation) -> Result<BasicValueEnum<'ctx>, CursedError> {
        let var_ptr = self.variables.get(name)
            .ok_or_else(|| CursedError::Compile(format!("Undefined variable: {}", name)))?;
        
        self.builder.build_load(*var_ptr, name)
            .map_err(|e| CursedError::Compile(format!("Variable load failed: {}", e)))
    }
    
    /// Compile a binary operation
    fn compile_binary_operation(
        &mut self,
        left: &dyn Expression,
        operator: &crate::ast::BinaryOperator,
        right: &dyn Expression,
        location: &SourceLocation,
    ) -> Result<BasicValueEnum<'ctx>, CursedError> {
        let left_val = self.compile_expression(left)?;
        let right_val = self.compile_expression(right)?;
        
        match operator {
            crate::ast::BinaryOperator::Add => {
                if let (Ok(left_int), Ok(right_int)) = (left_val.into_int_value(), right_val.into_int_value()) {
                    Ok(self.builder.build_int_add(left_int, right_int, "add")
                        .map_err(|e| CursedError::Compile(format!("Add failed: {}", e)))?.into())
                } else if let (Ok(left_float), Ok(right_float)) = (left_val.into_float_value(), right_val.into_float_value()) {
                    Ok(self.builder.build_float_add(left_float, right_float, "fadd")
                        .map_err(|e| CursedError::Compile(format!("Float add failed: {}", e)))?.into())
                } else {
                    Err(CursedError::Compile("Invalid types for addition".to_string()))
                }
            }
            crate::ast::BinaryOperator::Subtract => {
                if let (Ok(left_int), Ok(right_int)) = (left_val.into_int_value(), right_val.into_int_value()) {
                    Ok(self.builder.build_int_sub(left_int, right_int, "sub")
                        .map_err(|e| CursedError::Compile(format!("Subtract failed: {}", e)))?.into())
                } else if let (Ok(left_float), Ok(right_float)) = (left_val.into_float_value(), right_val.into_float_value()) {
                    Ok(self.builder.build_float_sub(left_float, right_float, "fsub")
                        .map_err(|e| CursedError::Compile(format!("Float subtract failed: {}", e)))?.into())
                } else {
                    Err(CursedError::Compile("Invalid types for subtraction".to_string()))
                }
            }
            crate::ast::BinaryOperator::Multiply => {
                if let (Ok(left_int), Ok(right_int)) = (left_val.into_int_value(), right_val.into_int_value()) {
                    Ok(self.builder.build_int_mul(left_int, right_int, "mul")
                        .map_err(|e| CursedError::Compile(format!("Multiply failed: {}", e)))?.into())
                } else if let (Ok(left_float), Ok(right_float)) = (left_val.into_float_value(), right_val.into_float_value()) {
                    Ok(self.builder.build_float_mul(left_float, right_float, "fmul")
                        .map_err(|e| CursedError::Compile(format!("Float multiply failed: {}", e)))?.into())
                } else {
                    Err(CursedError::Compile("Invalid types for multiplication".to_string()))
                }
            }
            crate::ast::BinaryOperator::Divide => {
                if let (Ok(left_int), Ok(right_int)) = (left_val.into_int_value(), right_val.into_int_value()) {
                    Ok(self.builder.build_int_signed_div(left_int, right_int, "div")
                        .map_err(|e| CursedError::Compile(format!("Divide failed: {}", e)))?.into())
                } else if let (Ok(left_float), Ok(right_float)) = (left_val.into_float_value(), right_val.into_float_value()) {
                    Ok(self.builder.build_float_div(left_float, right_float, "fdiv")
                        .map_err(|e| CursedError::Compile(format!("Float divide failed: {}", e)))?.into())
                } else {
                    Err(CursedError::Compile("Invalid types for division".to_string()))
                }
            }
            _ => Err(CursedError::Compile("Unsupported binary operator".to_string())),
        }
    }
    
    /// Compile a unary operation
    fn compile_unary_operation(
        &mut self,
        operator: &crate::ast::UnaryOperator,
        operand: &dyn Expression,
        location: &SourceLocation,
    ) -> Result<BasicValueEnum<'ctx>, CursedError> {
        let operand_val = self.compile_expression(operand)?;
        
        match operator {
            crate::ast::UnaryOperator::Minus => {
                if let Ok(int_val) = operand_val.into_int_value() {
                    Ok(self.builder.build_int_neg(int_val, "neg")
                        .map_err(|e| CursedError::Compile(format!("Negation failed: {}", e)))?.into())
                } else if let Ok(float_val) = operand_val.into_float_value() {
                    Ok(self.builder.build_float_neg(float_val, "fneg")
                        .map_err(|e| CursedError::Compile(format!("Float negation failed: {}", e)))?.into())
                } else {
                    Err(CursedError::Compile("Invalid type for negation".to_string()))
                }
            }
            crate::ast::UnaryOperator::Not => {
                if let Ok(int_val) = operand_val.into_int_value() {
                    Ok(self.builder.build_not(int_val, "not")
                        .map_err(|e| CursedError::Compile(format!("Not failed: {}", e)))?.into())
                } else {
                    Err(CursedError::Compile("Invalid type for logical not".to_string()))
                }
            }
            _ => Err(CursedError::Compile("Unsupported unary operator".to_string())),
        }
    }
    
    /// Compile a function call
    fn compile_function_call(
        &mut self,
        name: &str,
        arguments: &[dyn Expression],
        location: &SourceLocation,
    ) -> Result<BasicValueEnum<'ctx>, CursedError> {
        let function = self.functions.get(name)
            .ok_or_else(|| CursedError::Compile(format!("Undefined function: {}", name)))?;
        
        let mut args = Vec::new();
        for arg in arguments {
            let arg_val = self.compile_expression(arg)?;
            args.push(arg_val.into());
        }
        
        self.builder.build_call(*function, &args, "call")
            .map_err(|e| CursedError::Compile(format!("Function call failed: {}", e)))?
            .try_as_basic_value()
            .left()
            .ok_or_else(|| CursedError::Compile("Function call returned void".to_string()))
    }
    
    /// Compile an assignment
    fn compile_assignment(
        &mut self,
        target: &dyn Expression,
        value: &dyn Expression,
        location: &SourceLocation,
    ) -> Result<BasicValueEnum<'ctx>, CursedError> {
        let value_result = self.compile_expression(value)?;
        
        match target {
            Expression::Variable { name, .. } => {
                let var_ptr = self.variables.get(name)
                    .ok_or_else(|| CursedError::Compile(format!("Undefined variable: {}", name)))?;
                
                self.builder.build_store(*var_ptr, value_result)
                    .map_err(|e| CursedError::Compile(format!("Assignment failed: {}", e)))?;
                
                Ok(value_result)
            }
            _ => Err(CursedError::Compile("Invalid assignment target".to_string())),
        }
    }
    
    /// Compile if statement
    fn compile_if_statement(
        &mut self,
        condition: &dyn Expression,
        then_branch: &dyn Statement,
        else_branch: Option<&dyn Statement>,
        location: &SourceLocation,
    ) -> Result<(), CursedError> {
        let condition_val = self.compile_expression(condition)?;
        let condition_int = condition_val.into_int_value()
            .map_err(|_| CursedError::Compile("Condition must be integer".to_string()))?;
        
        let function = self.current_function
            .ok_or_else(|| CursedError::Compile("No current function".to_string()))?;
        
        let then_block = self.context.append_basic_block(function, "then");
        let else_block = self.context.append_basic_block(function, "else");
        let merge_block = self.context.append_basic_block(function, "merge");
        
        // Build conditional branch
        let zero = self.context.i32_type().const_zero();
        let cmp = self.builder.build_int_compare(IntPredicate::NE, condition_int, zero, "cmp")
            .map_err(|e| CursedError::Compile(format!("Comparison failed: {}", e)))?;
        
        self.builder.build_conditional_branch(cmp, then_block, else_block)
            .map_err(|e| CursedError::Compile(format!("Conditional branch failed: {}", e)))?;
        
        // Compile then branch
        self.builder.position_at_end(then_block);
        self.compile_statement(then_branch)?;
        if !self.has_terminator() {
            self.builder.build_unconditional_branch(merge_block)
                .map_err(|e| CursedError::Compile(format!("Branch failed: {}", e)))?;
        }
        
        // Compile else branch
        self.builder.position_at_end(else_block);
        if let Some(else_stmt) = else_branch {
            self.compile_statement(else_stmt)?;
        }
        if !self.has_terminator() {
            self.builder.build_unconditional_branch(merge_block)
                .map_err(|e| CursedError::Compile(format!("Branch failed: {}", e)))?;
        }
        
        // Continue at merge block
        self.builder.position_at_end(merge_block);
        
        Ok(())
    }
    
    /// Compile while statement
    fn compile_while_statement(
        &mut self,
        condition: &dyn Expression,
        body: &dyn Statement,
        location: &SourceLocation,
    ) -> Result<(), CursedError> {
        let function = self.current_function
            .ok_or_else(|| CursedError::Compile("No current function".to_string()))?;
        
        let loop_block = self.context.append_basic_block(function, "loop");
        let body_block = self.context.append_basic_block(function, "body");
        let exit_block = self.context.append_basic_block(function, "exit");
        
        // Jump to loop condition
        self.builder.build_unconditional_branch(loop_block)
            .map_err(|e| CursedError::Compile(format!("Branch failed: {}", e)))?;
        
        // Build loop condition
        self.builder.position_at_end(loop_block);
        let condition_val = self.compile_expression(condition)?;
        let condition_int = condition_val.into_int_value()
            .map_err(|_| CursedError::Compile("Condition must be integer".to_string()))?;
        
        let zero = self.context.i32_type().const_zero();
        let cmp = self.builder.build_int_compare(IntPredicate::NE, condition_int, zero, "cmp")
            .map_err(|e| CursedError::Compile(format!("Comparison failed: {}", e)))?;
        
        self.builder.build_conditional_branch(cmp, body_block, exit_block)
            .map_err(|e| CursedError::Compile(format!("Conditional branch failed: {}", e)))?;
        
        // Compile loop body
        self.builder.position_at_end(body_block);
        self.compile_statement(body)?;
        if !self.has_terminator() {
            self.builder.build_unconditional_branch(loop_block)
                .map_err(|e| CursedError::Compile(format!("Branch failed: {}", e)))?;
        }
        
        // Continue at exit block
        self.builder.position_at_end(exit_block);
        
        Ok(())
    }
    
    /// Compile block statement
    fn compile_block_statement(
        &mut self,
        statements: &[dyn Statement],
        location: &SourceLocation,
    ) -> Result<(), CursedError> {
        // Enter lexical scope if debug is enabled
        if let Some(debug) = &mut self.debug_metadata {
            let file = debug.get_or_create_file(&location.file);
            debug.enter_lexical_scope(file, location.line, location.column)?;
        }
        
        // Compile all statements in block
        for statement in statements {
            self.compile_statement(statement)?;
        }
        
        // Exit lexical scope
        if let Some(debug) = &mut self.debug_metadata {
            debug.exit_lexical_scope();
        }
        
        Ok(())
    }
    
    /// Get LLVM type for CURSED type
    fn get_llvm_type(&self, type_name: &str) -> Result<Option<BasicTypeEnum<'ctx>>, CursedError> {
        match type_name {
            "sus" => Ok(Some(self.context.i32_type().into())),
            "facts" => Ok(Some(self.context.bool_type().into())),
            "vibes" => Ok(Some(self.context.f64_type().into())),
            "tea" => Ok(Some(self.context.i8_type().ptr_type(AddressSpace::default()).into())),
            "void" => Ok(None),
            _ => Err(CursedError::Compile(format!("Unknown type: {}", type_name))),
        }
    }
    
    /// Get zero value for type
    fn get_zero_value(&self, type_name: &str) -> Result<BasicValueEnum<'ctx>, CursedError> {
        match type_name {
            "sus" => Ok(self.context.i32_type().const_zero().into()),
            "facts" => Ok(self.context.bool_type().const_zero().into()),
            "vibes" => Ok(self.context.f64_type().const_zero().into()),
            "tea" => Ok(self.context.i8_type().ptr_type(AddressSpace::default()).const_null().into()),
            _ => Err(CursedError::Compile(format!("No zero value for type: {}", type_name))),
        }
    }
    
    /// Check if current block has terminator
    fn has_terminator(&self) -> bool {
        if let Some(block) = self.builder.get_insert_block() {
            block.get_terminator().is_some()
        } else {
            false
        }
    }
    
    /// Finalize code generation
    #[instrument(skip(self))]
    pub fn finalize(mut self) -> Result<CodegenResult<'ctx>, CursedError> {
        let span = span!(Level::INFO, "finalize_codegen");
        let _enter = span.enter();
        
        info!("Finalizing code generation");
        
        // Verify module if enabled
        if self.config.verify_module {
            if let Err(error) = self.module.verify() {
                return Err(CursedError::Compile(format!("Module verification failed: {}", error)));
            }
        }
        
        // Finalize debug information
        let debug_stats = if let Some(debug) = self.debug_metadata.take() {
            Some(debug.finalize()?)
        } else {
            None
        };
        
        info!(
            functions = self.stats.functions_compiled,
            variables = self.stats.variables_allocated,
            expressions = self.stats.expressions_compiled,
            statements = self.stats.statements_compiled,
            errors = self.stats.errors_encountered,
            "Code generation completed successfully"
        );
        
        Ok(CodegenResult {
            module: self.module,
            stats: self.stats,
            debug_stats,
            errors: self.errors,
        })
    }
    
    /// Get current statistics
    pub fn statistics(&self) -> &CodegenStats {
        &self.stats
    }
    
    /// Get debug statistics
    pub fn debug_statistics(&self) -> Option<&DebugStats> {
        self.debug_metadata.as_ref().map(|d| d.statistics())
    }
    
    /// Check if debug is enabled
    pub fn debug_enabled(&self) -> bool {
        self.debug_metadata.is_some()
    }
}

/// Result of code generation
pub struct CodegenResult<'ctx> {
    pub module: Module<'ctx>,
    pub stats: CodegenStats,
    pub debug_stats: Option<DebugStats>,
    pub errors: Vec<CursedError>,
}

impl<'ctx> CodegenResult<'ctx> {
    /// Write module to object file
    pub fn write_object_file(&self, path: &Path) -> Result<(), CursedError> {
        Target::initialize_all(&Default::default());
        
        let target_triple = self.module.get_triple();
        let target = Target::from_triple(&target_triple)
            .map_err(|e| CursedError::Compile(format!("Target creation failed: {}", e)))?;
        
        let target_machine = target.create_target_machine(
            &target_triple,
            "generic",
            "",
            OptimizationLevel::O0,
            RelocMode::Default,
            CodeModel::Default,
        ).ok_or_else(|| CursedError::Compile("Target machine creation failed".to_string()))?;
        
        target_machine.write_to_file(&self.module, FileType::Object, path)
            .map_err(|e| CursedError::Compile(format!("Object file writing failed: {}", e)))?;
        
        Ok(())
    }
    
    /// Get LLVM IR as string
    pub fn to_string(&self) -> String {
        self.module.print_to_string().to_string()
    }
    
    /// Verify the module
    pub fn verify(&self) -> Result<(), String> {
        self.module.verify()
    }
}

impl fmt::Display for CodegenStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Codegen Stats: {} functions, {} variables, {} expressions, {} statements, {} errors",
            self.functions_compiled,
            self.variables_allocated,
            self.expressions_compiled,
            self.statements_compiled,
            self.errors_encountered
        )
    }
}

impl<'ctx> LlvmDebugIntegration<'ctx> for EnhancedLlvmCodegen<'ctx> {
    fn generate_function_debug(
        &mut self,
        function: FunctionValue<'ctx>,
        name: &str,
        func_decl: &FunctionDeclaration,
    ) -> Result<(), CursedError> {
        if let Some(debug) = &mut self.debug_metadata {
            debug.generate_function_debug(function, name, func_decl)?;
        }
        Ok(())
    }
    
    fn generate_variable_debug(
        &mut self,
        name: &str,
        storage: PointerValue<'ctx>,
        var_decl: &VariableDeclaration,
    ) -> Result<(), CursedError> {
        if let Some(debug) = &mut self.debug_metadata {
            debug.generate_variable_debug(name, storage, var_decl)?;
        }
        Ok(())
    }
    
    fn set_debug_location(
        &mut self,
        location: &SourceLocation,
    ) -> Result<(), CursedError> {
        if let Some(debug) = &mut self.debug_metadata {
            debug.set_debug_location_from_source(location)?;
            self.stats.debug_locations_set += 1;
        }
        Ok(())
    }
    
    fn generate_expression_debug(
        &mut self,
        expr: &dyn Expression,
        instruction: Option<InstructionValue<'ctx>>,
    ) -> Result<(), CursedError> {
        if let Some(debug) = &mut self.debug_metadata {
            debug.generate_expression_debug(expr, instruction)?;
        }
        Ok(())
    }
    
    fn enter_scope(&mut self, location: &SourceLocation) -> Result<(), CursedError> {
        if let Some(debug) = &mut self.debug_metadata {
            let file = debug.get_or_create_file(&location.file);
            debug.enter_lexical_scope(file, location.line, location.column)?;
        }
        Ok(())
    }
    
    fn exit_scope(&mut self) {
        if let Some(debug) = &mut self.debug_metadata {
            debug.exit_lexical_scope();
        }
    }
    
    fn debug_enabled(&self) -> bool {
        self.debug_metadata.is_some()
    }
}

/// Tests for enhanced LLVM code generation with debug integration
#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Literal, BinaryOperator, UnaryOperator};
    use std::path::Path;
    
    fn create_test_config() -> CodegenConfig {
        CodegenConfig {
            debug_config: DebugConfig {
                generate_debug_info: true,
                ..Default::default()
            },
            verify_module: true,
            ..Default::default()
        }
    }
    
    #[test]
    #[ignore = "Requires LLVM context - integration test"]
    fn test_enhanced_codegen_creation() {
        let context = Context::create();
        let config = create_test_config();
        
        let result = EnhancedLlvmCodegen::new(&context, Path::new("test.csd"), config);
        assert!(result.is_ok(), "Enhanced codegen creation should succeed");
        
        if let Ok(codegen) = result {
            assert!(codegen.debug_enabled());
            assert_eq!(codegen.statistics().functions_compiled, 0);
        }
    }
    
    #[test]
    fn test_codegen_config() {
        let config = CodegenConfig::default();
        assert_eq!(config.module_name, "cursed_module");
        assert_eq!(config.optimization_level, OptimizationLevel::O0);
        assert!(config.verify_module);
        assert!(!config.enable_jit);
    }
    
    #[test]
    fn test_codegen_stats_display() {
        let stats = CodegenStats {
            functions_compiled: 3,
            variables_allocated: 10,
            expressions_compiled: 25,
            statements_compiled: 15,
            debug_locations_set: 20,
            errors_encountered: 0,
        };
        
        let display = format!("{}", stats);
        assert!(display.contains("3 functions"));
        assert!(display.contains("10 variables"));
        assert!(display.contains("25 expressions"));
        assert!(display.contains("15 statements"));
        assert!(display.contains("0 errors"));
    }
}
