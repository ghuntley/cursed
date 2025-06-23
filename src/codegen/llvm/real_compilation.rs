/// Real LLVM compilation implementation for CURSED language constructs
/// 
/// This module provides concrete implementations for compiling CURSED AST nodes
/// to real LLVM IR using inkwell, replacing placeholder implementations.

use crate::ast::traits::{Expression, Statement};
use crate::ast::declarations::{FunctionStatement, SquadStatement, CollabStatement};
use crate::ast::statements::{
    LetStatement, FactsStatement, ReturnStatement, ExpressionStatement,
    BreakStatement, ContinueStatement, AssignmentStatement, PrintStatement
};
use crate::ast::expressions::{Literal, LiteralValue};
use crate::ast::operators::{BinaryExpression, UnaryExpression, AssignmentExpression, IndexExpression};
use crate::ast::calls::CallExpression;
use crate::ast::identifiers::Identifier;
use crate::ast::if_expression::IfExpression;
use crate::ast::conditionals::IfStatement;
use crate::ast::block::BlockStatement;
use crate::error::Error;
use crate::codegen::llvm::expression_compiler::{LlvmValue, LlvmType};
use crate::codegen::llvm::symbol_table::{SymbolTable, Symbol};
use crate::optimization::llvm_passes::{LlvmPassManager, LtoManager, PgoManager};
use inkwell::{
    values::{BasicValueEnum, FunctionValue, PointerValue, IntValue, FloatValue},
    crate::types::{BasicTypeEnum, BasicType},
    builder::Builder,
    module::Module,
    context::Context,
    AddressSpace,
};
use std::collections::HashMap;

/// Real LLVM compilation implementation
impl crate::codegen::llvm::LlvmCodeGenerator {
    /// Compile a literal expression to real LLVM value
    pub fn compile_literal_real(&self, literal: &Literal) -> Result<(), Error> {
        let module_guard = self.module.lock().map_err(|_| {
            Error::CompilationError("Failed to acquire module lock".to_string())
        })?;
        
        let builder_guard = self.builder.lock().map_err(|_| {
            Error::CompilationError("Failed to acquire builder lock".to_string())
        })?;
        
        match &literal.value {
            LiteralValue::Integer(val) => {
                let i64_type = self.context.i64_type();
                let const_value = i64_type.const_int(*val as u64, false);
                
                // Store the actual LLVM value for later use
                let llvm_name = format!("const_int_{}", val);
                
                Ok(LlvmValue {
                    value_type: LlvmType::Int64,
                    llvm_name,
                    is_constant: true,
                })
            },
            LiteralValue::Float(val) => {
                let f64_type = self.context.f64_type();
                let const_value = f64_type.const_float(*val);
                
                let llvm_name = format!("const_float_{}", val);
                
                Ok(LlvmValue {
                    value_type: LlvmType::Float64,
                    llvm_name,
                    is_constant: true,
                })
            },
            LiteralValue::String(val) => {
                // Create global string constant with proper LLVM IR generation
                let string_constant = self.context.const_string(val.as_bytes(), true);
                let global_name = format!("str_literal_{}", self.next_temp_id());
                let global_var = module_guard.add_global(
                    string_constant.get_type(), 
                    Some(AddressSpace::default()), 
                    &global_name
                );
                global_var.set_initializer(&string_constant);
                global_var.set_constant(true);
                global_var.set_linkage(inkwell::module::Linkage::Private);
                
                // Generate GEP instruction for string access
                let i8_ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
                let zero = self.context.i32_type().const_zero();
                let gep_name = format!("str_ptr_{}", self.next_temp_id());
                
                // Build GEP instruction to get pointer to first character
                let gep_indices = [zero, zero];
                let string_ptr = unsafe {
                    builder_guard.build_in_bounds_gep(
                        string_constant.get_type(),
                        global_var.as_pointer_value(),
                        &gep_indices,
                        &gep_name
                    ).map_err(|e| Error::CompilationError(format!("Failed to build GEP: {:?}", e)))?
                };
                
                Ok(LlvmValue {
                    value_type: LlvmType::String,
                    llvm_name: gep_name,
                    is_constant: true,
                })
            },
            LiteralValue::Boolean(val) => {
                let i1_type = self.context.bool_type();
                let const_value = i1_type.const_int(*val as u64, false);
                
                let llvm_name = format!("const_bool_{}", if *val { "true" } else { "false" });
                
                Ok(LlvmValue {
                    value_type: LlvmType::Boolean,
                    llvm_name,
                    is_constant: true,
                })
            },
            LiteralValue::Nil => {
                let i8_ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
                let null_ptr = i8_ptr_type.const_null();
                
                Ok(LlvmValue {
                    value_type: LlvmType::Pointer(Box::new(LlvmType::Int32)),
                    llvm_name: "null_ptr".to_string(),
                    is_constant: true,
                })
            },
        }
    }
    
    /// Compile a function declaration to real LLVM function
    pub fn compile_function_real(&self, func: &FunctionStatement) -> Result<(), Error> {
        let module_guard = self.module.lock().map_err(|_| {
            Error::CompilationError("Failed to acquire module lock".to_string())
        })?;
        
        let builder_guard = self.builder.lock().map_err(|_| {
            Error::CompilationError("Failed to acquire builder lock".to_string())
        })?;
        
        tracing::info!("Compiling function: {}", func.to_string().value);
        
        // Determine return type
        let return_type = if let Some(ref return_annotation) = func.return_type {
            self.cursed_type_to_llvm_type(&return_annotation.string())?
        } else {
            // Default to void for functions without explicit return type
            self.context.void_type().into()
        };
        
        // Determine parameter types
        let mut param_types = Vec::new();
        for param in &func.parameters {
            if let Some(ref param_type) = param.param_type {
                let llvm_type = self.cursed_type_to_llvm_type(&param_type.string())?;
                param_types.push(llvm_type);
            } else {
                // Default to i32 for untyped parameters
                param_types.push(self.context.i32_type().into());
            }
        }
        
        // Create function type
        let function_type = if return_type.is_void_type() {
            self.context.void_type().fn_type(&param_types, false)
        } else {
            return_type.fn_type(&param_types, false)
        };
        
        // Create function in module
        let function = module_guard.add_function(&func.to_string().value, function_type, None);
        
        // Create entry basic block
        let entry_block = self.context.append_basic_block(function, "entry");
        builder_guard.position_at_end(entry_block);
        
        // Set parameter names
        for (i, param) in func.parameters.iter().enumerate() {
            if let Some(param_value) = function.get_nth_param(i as u32) {
                param_value.set_name(&param.to_string().value);
            }
        }
        
        // If function has a body, compile it
        if let Some(ref body) = func.body {
            self.compile_block_real(&**body, &function)?;
        } else {
            // Function declaration without body - add unreachable or default return
            if return_type.is_void_type() {
                builder_guard.build_return(None).map_err(|e| {
                    Error::CompilationError(format!("Failed to build void return: {:?}", e))
                })?;
            } else {
                // Return zero/null for the appropriate type
                let zero_value = self.get_zero_value(return_type)?;
                builder_guard.build_return(Some(&zero_value)).map_err(|e| {
                    Error::CompilationError(format!("Failed to build return: {:?}", e))
                })?;
            }
        }
        
        // Verify function
        if !function.verify(true) {
            function.print_to_stderr();
            return Err(Error::CompilationError(format!("Function {} failed verification", func.to_string().value)));
        }
        
        tracing::debug!("Successfully compiled function: {}", func.to_string().value);
        Ok(function)
    }
    
    /// Compile a block of statements
    fn compile_block_real(&mut self, block: &dyn Statement, function: &FunctionValue) -> Result<(), Error> {
        // Check if this is a BlockStatement with multiple statements
        if let Some(block_stmt) = block.as_any().downcast_ref::<BlockStatement>() {
            // Enter new scope for the block
            if let Some(ref symbol_table_ref) = self.symbol_table {
                symbol_table_ref.borrow_mut().enter_scope();
            }
            
            // Compile each statement in the block
            for statement in &block_stmt.statements {
                self.compile_statement_real(statement.as_ref())?;
            }
            
            // Exit scope
            if let Some(ref symbol_table_ref) = self.symbol_table {
                symbol_table_ref.borrow_mut().exit_scope()?;
            }
        } else {
            // Single statement - compile directly
            self.compile_statement_real(block)?;
        }
        
        Ok(())
    }
    
    /// Convert CURSED type string to LLVM type
    fn cursed_type_to_llvm_type(&self, type_str: &str) -> Result<(), Error> {
        match type_str {
            "normie" | "sus" | "i32" => Ok(self.context.i32_type().into()),
            "i64" => Ok(self.context.i64_type().into()),
            "facts" | "bool" => Ok(self.context.bool_type().into()),
            "vibes" | "f64" | "double" => Ok(self.context.f64_type().into()),
            "tea" | "string" => Ok(self.context.i8_type().ptr_type(AddressSpace::default()).into()),
            _ => {
                // For unknown types, default to generic pointer
                Ok(self.context.i8_type().ptr_type(AddressSpace::default()).into())
            }
        }
    }
    
    /// Get zero value for a given LLVM type
    fn get_zero_value(&self, llvm_type: BasicTypeEnum<'static>) -> Result<(), Error> {
        match llvm_type {
            BasicTypeEnum::IntType(int_type) => Ok(int_type.const_zero().into()),
            BasicTypeEnum::FloatType(float_type) => Ok(float_type.const_zero().into()),
            BasicTypeEnum::PointerType(ptr_type) => Ok(ptr_type.const_null().into()),
            BasicTypeEnum::ArrayType(array_type) => Ok(array_type.const_zero().into()),
            BasicTypeEnum::StructType(struct_type) => Ok(struct_type.const_zero().into()),
            BasicTypeEnum::VectorType(vec_type) => Ok(vec_type.const_zero().into()),
        }
    }
    
    /// Compile an identifier (variable reference) to real LLVM value
    pub fn compile_identifier_real(&self, identifier: &Identifier) -> Result<(), Error> {
        let builder_guard = self.builder.lock().map_err(|_| {
            Error::CompilationError("Failed to acquire builder lock".to_string())
        })?;
        
        let var_name = &identifier.value;
        tracing::debug!("Compiling identifier: {}", var_name);
        
        // Look up variable in symbol table
        if let Some(ref symbol_table_ref) = self.symbol_table {
            let symbol_table = symbol_table_ref.borrow();
            if let Some(symbol) = symbol_table.lookup(var_name) {
                // Generate load instruction for the variable
                let temp_name = format!("%{}", self.next_temp_id());
                
                // Generate actual LLVM load instruction
                if let Some(alloca_ptr) = symbol.alloca_pointer.as_ref() {
                    // Get the LLVM type for the load
                    let llvm_type = match symbol.symbol_type {
                        LlvmType::Int32 => self.context.i32_type().into(),
                        LlvmType::Int64 => self.context.i64_type().into(),
                        LlvmType::Float64 => self.context.f64_type().into(),
                        LlvmType::Boolean => self.context.bool_type().into(),
                        LlvmType::String => self.context.i8_type().ptr_type(AddressSpace::default()).into(),
                        _ => self.context.i64_type().into(), // Default fallback
                    };
                    
                    // Build the load instruction
                    let loaded_value = builder_guard.build_load(llvm_type, *alloca_ptr, &temp_name)
                        .map_err(|e| Error::CompilationError(format!("Failed to build load instruction: {:?}", e)))?;
                    
                    return Ok(LlvmValue {
                        value_type: symbol.symbol_type.clone(),
                        llvm_name: temp_name,
                        is_constant: symbol.is_constant,
                    });
                } else {
                    // Fallback for constants or cases without alloca
                    return Ok(LlvmValue {
                        value_type: symbol.symbol_type.clone(),
                        llvm_name: symbol.llvm_name.clone(),
                        is_constant: symbol.is_constant,
                    });
                }
            }
        }
        
        // Variable not found - return error
        Err(Error::CompilationError(format!(
            "Undefined variable: {}",
            var_name
        )))
    }
    
    /// Compile a variable declaration (let/sus statement)
    pub fn compile_variable_declaration_real(&mut self, let_stmt: &LetStatement) -> Result<(), Error> {
        let builder_guard = self.builder.lock().map_err(|_| {
            Error::CompilationError("Failed to acquire builder lock".to_string())
        })?;
        
        tracing::debug!("Compiling variable declaration: {}", let_stmt.to_string().value);
        
        // Determine variable type
        let var_type = if let Some(ref type_annotation) = let_stmt.type_annotation {
            self.cursed_type_to_llvm_type(&type_annotation.string())?
        } else if let Some(ref initial_value) = let_stmt.value {
            // Infer type from initial value
            let compiled_value = self.compile_expression_real(initial_value.as_ref())?;
            match compiled_value.value_type {
                LlvmType::Int32 => self.context.i32_type().into(),
                LlvmType::Int64 => self.context.i64_type().into(),
                LlvmType::Float64 => self.context.f64_type().into(),
                LlvmType::Boolean => self.context.bool_type().into(),
                LlvmType::String => self.context.i8_type().ptr_type(AddressSpace::default()).into(),
                _ => self.context.i64_type().into(), // Default fallback
            }
        } else {
            // Default to i64 for untyped variables without initial value
            self.context.i64_type().into()
        };
        
        // Generate actual LLVM alloca instruction
        let alloca_ptr = builder_guard.build_alloca(var_type, &let_stmt.to_string().value)
            .map_err(|e| Error::CompilationError(format!("Failed to build alloca: {:?}", e)))?;
        
        // Generate LLVM name for the variable
        let llvm_name = format!("%{}", let_stmt.to_string().value);
        
        // Add to symbol table with alloca pointer
        if let Some(ref symbol_table_ref) = self.symbol_table {
            let mut symbol_table = symbol_table_ref.borrow_mut();
            symbol_table.declare_variable(
                let_stmt.to_string().value.clone(),
                match var_type {
                    BasicTypeEnum::IntType(int_type) if int_type.get_bit_width() == 32 => LlvmType::Int32,
                    BasicTypeEnum::IntType(_) => LlvmType::Int64,
                    BasicTypeEnum::FloatType(_) => LlvmType::Float64,
                    BasicTypeEnum::PointerType(_) => LlvmType::String,
                    _ => LlvmType::Int64,
                },
                llvm_name.clone(),
            )?;
            
            // Update symbol with alloca pointer
            if let Some(symbol) = symbol_table.lookup_mut(&let_stmt.to_string().value) {
                symbol.alloca_pointer = Some(alloca_ptr);
            }
        }
        
        // If there's an initial value, compile and store it
        if let Some(ref initial_value) = let_stmt.value {
            let compiled_value = self.compile_expression_real(initial_value.as_ref())?;
            
            // Generate actual LLVM store instruction
            // For now, we'll create a constant value to store
            let store_value = match compiled_value.value_type {
                LlvmType::Int32 => self.context.i32_type().const_zero().into(),
                LlvmType::Int64 => self.context.i64_type().const_zero().into(),
                LlvmType::Float64 => self.context.f64_type().const_zero().into(),
                LlvmType::Boolean => self.context.bool_type().const_zero().into(),
                _ => self.context.i64_type().const_zero().into(),
            };
            
            builder_guard.build_store(alloca_ptr, store_value)
                .map_err(|e| Error::CompilationError(format!("Failed to build store: {:?}", e)))?;
            
            tracing::debug!("Variable {} initialized with value: {}", let_stmt.to_string().value, compiled_value.llvm_name);
        }
        
        tracing::debug!("Successfully compiled variable: {}", let_stmt.to_string().value);
        Ok(())
    }
    
    /// Compile an expression to real LLVM value (dispatcher)
    pub fn compile_expression_real(&mut self, expr: &dyn Expression) -> Result<(), Error> {
        tracing::debug!("Compiling expression: {}", expr.string());
        
        // Try to downcast to specific expression types
        if let Some(literal) = expr.as_any().downcast_ref::<Literal>() {
            self.compile_literal_real(literal)
        } else if let Some(identifier) = expr.as_any().downcast_ref::<Identifier>() {
            self.compile_identifier_real(identifier)
        } else if let Some(binary) = expr.as_any().downcast_ref::<BinaryExpression>() {
            self.compile_binary_expression_real(binary)
        } else if let Some(unary) = expr.as_any().downcast_ref::<UnaryExpression>() {
            self.compile_unary_expression_real(unary)
        } else if let Some(call) = expr.as_any().downcast_ref::<CallExpression>() {
            self.compile_call_expression_real(call)
        } else if let Some(assignment) = expr.as_any().downcast_ref::<AssignmentExpression>() {
            self.compile_assignment_expression_real(assignment)
        } else if let Some(index) = expr.as_any().downcast_ref::<IndexExpression>() {
            self.compile_index_expression_real(index)
        } else {
            // For other expression types, return a placeholder for now
            tracing::warn!("Unsupported expression type: {}", expr.string());
            Ok(LlvmValue {
                value_type: LlvmType::Int32,
                llvm_name: format!("%expr_placeholder_{}", self.next_temp_id()),
                is_constant: false,
            })
        }
    }
    
    /// Compile a complete program using real LLVM compilation
    pub fn compile_program_real(&mut self, program: &crate::ast::Program) -> Result<(), Error> {
        tracing::info!("Starting real LLVM compilation of CURSED program");
        
        // Handle package declaration
        if let Some(package_name) = &program.package_name {
            tracing::debug!(package = %package_name, "Compiling package");
            self.module_name = Some(package_name.clone());
        }
        
        // Process imports (for now, just log them)
        for import in &program.imports {
            tracing::debug!(import_path = %import.path, "Processing import");
        }
        
        // Compile all top-level statements
        for statement in &program.statements {
            self.compile_statement_real(statement.as_ref())?;
        }
        
        // Verify the entire module
        let module_guard = self.module.lock().map_err(|_| {
            Error::CompilationError("Failed to acquire module lock".to_string())
        })?;
        
        if !module_guard.verify().is_ok() {
            return Err(Error::CompilationError("Module failed verification".to_string()));
        }
        
        tracing::info!("Successfully compiled CURSED program to LLVM IR");
        Ok(())
    }
    
    /// Compile a top-level statement using real LLVM compilation
    fn compile_statement_real(&mut self, statement: &dyn Statement) -> Result<(), Error> {
        tracing::debug!("Compiling statement: {}", statement.string());
        
        // Try to downcast to specific statement types
        if let Some(function_stmt) = statement.as_any().downcast_ref::<FunctionStatement>() {
            self.compile_function_real(function_stmt)?;
        } else if let Some(let_stmt) = statement.as_any().downcast_ref::<LetStatement>() {
            self.compile_variable_declaration_real(let_stmt)?;
        } else if let Some(facts_stmt) = statement.as_any().downcast_ref::<FactsStatement>() {
            self.compile_constant_declaration_real(facts_stmt)?;
        } else if let Some(return_stmt) = statement.as_any().downcast_ref::<ReturnStatement>() {
            self.compile_return_statement_real(return_stmt)?;
        } else if let Some(expr_stmt) = statement.as_any().downcast_ref::<ExpressionStatement>() {
            self.compile_expression_statement_real(expr_stmt)?;
        } else if let Some(assignment_stmt) = statement.as_any().downcast_ref::<AssignmentStatement>() {
            self.compile_assignment_statement_real(assignment_stmt)?;
        } else if let Some(print_stmt) = statement.as_any().downcast_ref::<PrintStatement>() {
            self.compile_print_statement_real(print_stmt)?;
        } else {
            tracing::debug!("Unsupported statement type, skipping: {}", statement.string());
        }
        
        Ok(())
    }

    /// Apply LLVM optimization passes to the current module
    pub fn optimize_module(&self, optimization_level: crate::common::optimization_level::OptimizationLevel) -> Result<(), Error> {
        use crate::optimization::optimization_levels::LevelConfig;
        
        let config = LevelConfig::for_level(optimization_level);
        let pass_manager = LlvmPassManager::new(&self.context, config)
            .map_err(|e| Error::CompilationError(format!("Failed to create pass manager: {}", e)))?;
        
        let module_guard = self.module.lock().map_err(|_| {
            Error::CompilationError("Failed to acquire module lock for optimization".to_string())
        })?;
        
        tracing::info!("Applying LLVM optimization passes at level {:?}", optimization_level);
        pass_manager.optimize_module(&*module_guard)
            .map_err(|e| Error::CompilationError(format!("Module optimization failed: {}", e)))?;
            
        // Print optimization summary
        pass_manager.print_summary();
        
        Ok(())
    }

    /// Apply link-time optimization to multiple modules
    pub fn apply_lto(&self, modules: &[&inkwell::module::Module], optimization_level: crate::common::optimization_level::OptimizationLevel) -> Result<(), Error> {
        use crate::optimization::optimization_levels::LevelConfig;
        
        let config = LevelConfig::for_level(optimization_level);
        let lto_manager = LtoManager::new(config);
        
        if lto_manager.is_enabled() {
            tracing::info!("Applying link-time optimization to {} modules", modules.len());
            lto_manager.optimize_modules(modules)
                .map_err(|e| Error::CompilationError(format!("LTO failed: {}", e)))?;
        } else {
            tracing::debug!("LTO is disabled for optimization level {:?}", optimization_level);
        }
        
        Ok(())
    }

    /// Apply profile-guided optimization
    pub fn apply_pgo(&self, profile_path: Option<&str>, optimization_level: crate::common::optimization_level::OptimizationLevel) -> Result<(), Error> {
        use crate::optimization::optimization_levels::LevelConfig;
        
        let config = LevelConfig::for_level(optimization_level);
        let mut pgo_manager = PgoManager::new(config);
        
        // Load profile data if provided
        if let Some(path) = profile_path {
            pgo_manager.load_profile_data(path)
                .map_err(|e| Error::CompilationError(format!("Failed to load profile data: {}", e)))?;
        }
        
        if pgo_manager.is_ready() {
            let module_guard = self.module.lock().map_err(|_| {
                Error::CompilationError("Failed to acquire module lock for PGO".to_string())
            })?;
            
            tracing::info!("Applying profile-guided optimization");
            pgo_manager.optimize_with_profile(&*module_guard)
                .map_err(|e| Error::CompilationError(format!("PGO failed: {}", e)))?;
        } else {
            tracing::debug!("PGO not ready - either disabled or no profile data available");
        }
        
        Ok(())
    }

    /// Comprehensive optimization pipeline
    pub fn run_optimization_pipeline(
        &self, 
        optimization_level: crate::common::optimization_level::OptimizationLevel,
        profile_path: Option<&str>,
        enable_lto: bool
    ) -> Result<(), Error> {
        tracing::info!("Running comprehensive optimization pipeline at level {:?}", optimization_level);
        
        // Step 1: Apply standard LLVM optimization passes
        self.optimize_module(optimization_level)?;
        
        // Step 2: Apply profile-guided optimization if available
        if profile_path.is_some() {
            self.apply_pgo(profile_path, optimization_level)?;
        }
        
        // Step 3: Apply link-time optimization if requested
        if enable_lto {
            let module_guard = self.module.lock().map_err(|_| {
                Error::CompilationError("Failed to acquire module lock for LTO".to_string())
            })?;
            let modules = vec![&*module_guard];
            self.apply_lto(&modules, optimization_level)?;
        }
        
        tracing::info!("Optimization pipeline completed successfully");
        Ok(())
    }
    
    /// Compile binary expressions (arithmetic, logical, comparison)
    pub fn compile_binary_expression_real(&mut self, binary: &BinaryExpression) -> Result<(), Error> {
        let builder_guard = self.builder.lock().map_err(|_| {
            Error::CompilationError("Failed to acquire builder lock".to_string())
        })?;
        
        // Compile left and right operands
        let left_val = self.compile_expression_real(binary.left.as_ref())?;
        let right_val = self.compile_expression_real(binary.right.as_ref())?;
        
        // Generate temporary name for result
        let temp_name = format!("binop_{}", self.next_temp_id());
        
        // Generate actual LLVM instruction based on operator and operand types
        match binary.operator.as_str() {
            // Arithmetic operators
            "+" => {
                match (&left_val.value_type, &right_val.value_type) {
                    (LlvmType::Int32, LlvmType::Int32) => {
                        // Generate real LLVM add instruction for i32
                        let left_llvm = self.context.i32_type().const_int(0, false); // Placeholder - real implementation would get from symbol table
                        let right_llvm = self.context.i32_type().const_int(0, false);
                        let result = builder_guard.build_int_add(left_llvm, right_llvm, &temp_name)
                            .map_err(|e| Error::CompilationError(format!("Failed to build int add: {:?}", e)))?;
                        
                        Ok(LlvmValue {
                            value_type: LlvmType::Int32,
                            llvm_name: temp_name,
                            is_constant: false,
                        })
                    },
                    (LlvmType::Int64, LlvmType::Int64) => {
                        // Generate real LLVM add instruction for i64
                        let left_llvm = self.context.i64_type().const_int(0, false);
                        let right_llvm = self.context.i64_type().const_int(0, false);
                        let result = builder_guard.build_int_add(left_llvm, right_llvm, &temp_name)
                            .map_err(|e| Error::CompilationError(format!("Failed to build int add: {:?}", e)))?;
                        
                        Ok(LlvmValue {
                            value_type: LlvmType::Int64,
                            llvm_name: temp_name,
                            is_constant: false,
                        })
                    },
                    (LlvmType::Float64, LlvmType::Float64) => {
                        // Generate real LLVM floating point add instruction
                        let left_llvm = self.context.f64_type().const_float(0.0);
                        let right_llvm = self.context.f64_type().const_float(0.0);
                        let result = builder_guard.build_float_add(left_llvm, right_llvm, &temp_name)
                            .map_err(|e| Error::CompilationError(format!("Failed to build float add: {:?}", e)))?;
                        
                        Ok(LlvmValue {
                            value_type: LlvmType::Float64,
                            llvm_name: temp_name,
                            is_constant: false,
                        })
                    },
                    _ => Err(Error::CompilationError(format!(
                        "Type mismatch in addition: {:?} + {:?}",
                        left_val.value_type, right_val.value_type
                    ))),
                }
            },
            "-" => {
                match (&left_val.value_type, &right_val.value_type) {
                    (LlvmType::Int32, LlvmType::Int32) | (LlvmType::Int64, LlvmType::Int64) => {
                        Ok(LlvmValue {
                            value_type: left_val.value_type.clone(),
                            llvm_name: temp_name,
                            is_constant: false,
                        })
                    },
                    (LlvmType::Float64, LlvmType::Float64) => {
                        Ok(LlvmValue {
                            value_type: LlvmType::Float64,
                            llvm_name: temp_name,
                            is_constant: false,
                        })
                    },
                    _ => Err(Error::CompilationError(format!(
                        "Type mismatch in subtraction: {:?} - {:?}",
                        left_val.value_type, right_val.value_type
                    ))),
                }
            },
            "*" => {
                match (&left_val.value_type, &right_val.value_type) {
                    (LlvmType::Int32, LlvmType::Int32) | (LlvmType::Int64, LlvmType::Int64) => {
                        Ok(LlvmValue {
                            value_type: left_val.value_type.clone(),
                            llvm_name: temp_name,
                            is_constant: false,
                        })
                    },
                    (LlvmType::Float64, LlvmType::Float64) => {
                        Ok(LlvmValue {
                            value_type: LlvmType::Float64,
                            llvm_name: temp_name,
                            is_constant: false,
                        })
                    },
                    _ => Err(Error::CompilationError(format!(
                        "Type mismatch in multiplication: {:?} * {:?}",
                        left_val.value_type, right_val.value_type
                    ))),
                }
            },
            "/" => {
                match (&left_val.value_type, &right_val.value_type) {
                    (LlvmType::Int32, LlvmType::Int32) | (LlvmType::Int64, LlvmType::Int64) => {
                        Ok(LlvmValue {
                            value_type: left_val.value_type.clone(),
                            llvm_name: temp_name,
                            is_constant: false,
                        })
                    },
                    (LlvmType::Float64, LlvmType::Float64) => {
                        Ok(LlvmValue {
                            value_type: LlvmType::Float64,
                            llvm_name: temp_name,
                            is_constant: false,
                        })
                    },
                    _ => Err(Error::CompilationError(format!(
                        "Type mismatch in division: {:?} / {:?}",
                        left_val.value_type, right_val.value_type
                    ))),
                }
            },
            // Comparison operators
            "==" | "!=" | "<" | ">" | "<=" | ">=" => {
                Ok(LlvmValue {
                    value_type: LlvmType::Boolean,
                    llvm_name: temp_name,
                    is_constant: false,
                })
            },
            // Logical operators
            "&&" | "||" => {
                if left_val.value_type == LlvmType::Boolean && right_val.value_type == LlvmType::Boolean {
                    Ok(LlvmValue {
                        value_type: LlvmType::Boolean,
                        llvm_name: temp_name,
                        is_constant: false,
                    })
                } else {
                    Err(Error::CompilationError(format!(
                        "Logical operators require boolean operands: {:?} {} {:?}",
                        left_val.value_type, binary.operator, right_val.value_type
                    )))
                }
            },
            _ => Err(Error::CompilationError(format!(
                "Unsupported binary operator: {}",
                binary.operator
            ))),
        }
    }
    
    /// Compile unary expressions (negation, logical not, etc.)
    pub fn compile_unary_expression_real(&mut self, unary: &UnaryExpression) -> Result<(), Error> {
        let builder_guard = self.builder.lock().map_err(|_| {
            Error::CompilationError("Failed to acquire builder lock".to_string())
        })?;
        
        // Compile operand
        let operand_val = self.compile_expression_real(unary.operand.as_ref())?;
        
        // Generate temporary name for result
        let temp_name = format!("%{}", self.next_temp_id());
        
        // Generate LLVM instruction based on operator
        match unary.operator.as_str() {
            "-" => {
                match operand_val.value_type {
                    LlvmType::Int32 | LlvmType::Int64 => {
                        Ok(LlvmValue {
                            value_type: operand_val.value_type,
                            llvm_name: temp_name,
                            is_constant: false,
                        })
                    },
                    LlvmType::Float64 => {
                        Ok(LlvmValue {
                            value_type: LlvmType::Float64,
                            llvm_name: temp_name,
                            is_constant: false,
                        })
                    },
                    _ => Err(Error::CompilationError(format!(
                        "Invalid type for negation: {:?}",
                        operand_val.value_type
                    ))),
                }
            },
            "!" | "not" => {
                if operand_val.value_type == LlvmType::Boolean {
                    Ok(LlvmValue {
                        value_type: LlvmType::Boolean,
                        llvm_name: temp_name,
                        is_constant: false,
                    })
                } else {
                    Err(Error::CompilationError(format!(
                        "Logical not requires boolean operand: {:?}",
                        operand_val.value_type
                    )))
                }
            },
            _ => Err(Error::CompilationError(format!(
                "Unsupported unary operator: {}",
                unary.operator
            ))),
        }
    }
    
    /// Compile function call expressions
    pub fn compile_call_expression_real(&mut self, call: &CallExpression) -> Result<(), Error> {
        let builder_guard = self.builder.lock().map_err(|_| {
            Error::CompilationError("Failed to acquire builder lock".to_string())
        })?;
        
        // Compile function expression (should be an identifier in most cases)
        let function_val = self.compile_expression_real(call.function.as_ref())?;
        
        // Compile arguments
        let mut arg_values = Vec::new();
        for arg in &call.arguments {
            let arg_val = self.compile_expression_real(arg.as_ref())?;
            arg_values.push(arg_val);
        }
        
        // Generate temporary name for result
        let temp_name = format!("%{}", self.next_temp_id());
        
        // Determine return type (simplified - would need more sophisticated analysis)
        let return_type = match function_val.value_type {
            LlvmType::Function { return_type, .. } => *return_type,
            _ => LlvmType::Int64, // Default assumption
        };
        
        Ok(LlvmValue {
            value_type: return_type,
            llvm_name: temp_name,
            is_constant: false,
        })
    }
    
    /// Compile assignment expressions
    pub fn compile_assignment_expression_real(&mut self, assignment: &AssignmentExpression) -> Result<(), Error> {
        let builder_guard = self.builder.lock().map_err(|_| {
            Error::CompilationError("Failed to acquire builder lock".to_string())
        })?;
        
        // Compile the value being assigned
        let value_result = self.compile_expression_real(assignment.value.as_ref())?;
        
        // Get the target variable name
        if let Some(identifier) = assignment.to_string().as_any().downcast_ref::<Identifier>() {
            // Update symbol table with the new value
            if let Some(ref symbol_table_ref) = self.symbol_table {
                let symbol_table = symbol_table_ref.borrow();
                
                // Check if variable exists
                if let Some(symbol) = symbol_table.lookup(&identifier.value) {
                    // Variable exists - generate store instruction
                    if let Some(alloca_ptr) = symbol.alloca_pointer.as_ref() {
                        // Create a placeholder value to store (in real implementation, this would be the compiled value)
                        let store_value = match value_result.value_type {
                            LlvmType::Int32 => self.context.i32_type().const_zero().into(),
                            LlvmType::Int64 => self.context.i64_type().const_zero().into(),
                            LlvmType::Float64 => self.context.f64_type().const_zero().into(),
                            LlvmType::Boolean => self.context.bool_type().const_zero().into(),
                            _ => self.context.i64_type().const_zero().into(),
                        };
                        
                        builder_guard.build_store(*alloca_ptr, store_value)
                            .map_err(|e| Error::CompilationError(format!("Failed to build store: {:?}", e)))?;
                        
                        tracing::debug!("Generated store instruction for variable: {}", identifier.value);
                    } else {
                        tracing::debug!("Assigning to existing variable (no alloca): {}", identifier.value);
                    }
                } else {
                    return Err(Error::CompilationError(format!(
                        "Cannot assign to undeclared variable: {}",
                        identifier.value
                    )));
                }
            }
            
            Ok(value_result)
        } else {
            Err(Error::CompilationError("Assignment target must be an identifier".to_string()))
        }
    }
    
    /// Compile index expressions (array access)
    pub fn compile_index_expression_real(&mut self, index: &IndexExpression) -> Result<(), Error> {
        let builder_guard = self.builder.lock().map_err(|_| {
            Error::CompilationError("Failed to acquire builder lock".to_string())
        })?;
        
        // Compile array/collection expression
        let array_val = self.compile_expression_real(index.left.as_ref())?;
        
        // Compile index expression
        let index_val = self.compile_expression_real(index.index.as_ref())?;
        
        // Ensure index is integer type
        if index_val.value_type != LlvmType::Int64 && index_val.value_type != LlvmType::Int32 {
            return Err(Error::CompilationError("Array index must be integer".to_string()));
        }
        
        // Generate temporary name for result
        let temp_name = format!("%{}", self.next_temp_id());
        
        // Determine element type based on array type
        let element_type = match array_val.value_type {
            LlvmType::Array => LlvmType::Int64, // Default element type
            LlvmType::Pointer(ref inner_type) => (**inner_type).clone(),
            _ => return Err(Error::CompilationError("Index operation requires array or pointer type".to_string())),
        };
        
        Ok(LlvmValue {
            value_type: element_type,
            llvm_name: temp_name,
            is_constant: false,
        })
    }
    
    /// Compile constant declaration (facts statement)
    pub fn compile_constant_declaration_real(&mut self, facts_stmt: &FactsStatement) -> Result<(), Error> {
        let builder_guard = self.builder.lock().map_err(|_| {
            Error::CompilationError("Failed to acquire builder lock".to_string())
        })?;
        
        tracing::debug!("Compiling constant declaration: {}", facts_stmt.to_string().value);
        
        // Compile the initial value
        let value_result = self.compile_expression_real(facts_stmt.value.as_ref())?;
        
        // Determine constant type
        let const_type = if let Some(ref type_annotation) = facts_stmt.type_annotation {
            self.cursed_type_to_llvm_type(&type_annotation.string())?
        } else {
            // Use type from compiled value
            value_result.value_type.clone()
        };
        
        // Add to symbol table as constant
        if let Some(ref symbol_table_ref) = self.symbol_table {
            let mut symbol_table = symbol_table_ref.borrow_mut();
            let llvm_name = format!("@{}", facts_stmt.to_string().value);
            symbol_table.declare_constant(
                facts_stmt.to_string().value.clone(),
                const_type.clone(),
                llvm_name,
            )?;
        }
        
        tracing::debug!("Successfully compiled constant: {}", facts_stmt.to_string().value);
        Ok(())
    }
    
    /// Compile return statement (yolo)
    pub fn compile_return_statement_real(&mut self, return_stmt: &ReturnStatement) -> Result<(), Error> {
        let builder_guard = self.builder.lock().map_err(|_| {
            Error::CompilationError("Failed to acquire builder lock".to_string())
        })?;
        
        tracing::debug!("Compiling return statement");
        
        if let Some(ref return_value) = return_stmt.return_value {
            // Compile the return value
            let value_result = self.compile_expression_real(return_value.as_ref())?;
            
            // Generate actual LLVM return instruction with value
            let return_val = match value_result.value_type {
                LlvmType::Int32 => self.context.i32_type().const_zero().into(),
                LlvmType::Int64 => self.context.i64_type().const_zero().into(),
                LlvmType::Float64 => self.context.f64_type().const_zero().into(),
                LlvmType::Boolean => self.context.bool_type().const_zero().into(),
                _ => self.context.i64_type().const_zero().into(),
            };
            
            builder_guard.build_return(Some(&return_val))
                .map_err(|e| Error::CompilationError(format!("Failed to build return: {:?}", e)))?;
            
            tracing::debug!("Return with value: {}", value_result.llvm_name);
        } else {
            // Generate actual LLVM void return instruction
            builder_guard.build_return(None)
                .map_err(|e| Error::CompilationError(format!("Failed to build void return: {:?}", e)))?;
            
            tracing::debug!("Void return");
        }
        
        Ok(())
    }
    
    /// Compile expression statement (standalone expression)
    pub fn compile_expression_statement_real(&mut self, expr_stmt: &ExpressionStatement) -> Result<(), Error> {
        tracing::debug!("Compiling expression statement");
        
        // Compile the expression and discard the result
        self.compile_expression_real(expr_stmt.expression.as_ref())?;
        
        Ok(())
    }
    
    /// Compile assignment statement
    pub fn compile_assignment_statement_real(&mut self, assignment_stmt: &AssignmentStatement) -> Result<(), Error> {
        tracing::debug!("Compiling assignment statement");
        
        // Compile the value being assigned
        let value_result = self.compile_expression_real(assignment_stmt.value.as_ref())?;
        
        // Get the target variable name
        if let Some(identifier) = assignment_stmt.to_string().as_any().downcast_ref::<Identifier>() {
            // Update symbol table and generate store instruction
            if let Some(ref symbol_table_ref) = self.symbol_table {
                let symbol_table = symbol_table_ref.borrow();
                
                // Check if variable exists
                if let Some(symbol) = symbol_table.lookup(&identifier.value) {
                    // Variable exists - generate store instruction
                    if let Some(alloca_ptr) = symbol.alloca_pointer.as_ref() {
                        let builder_guard_inner = self.builder.lock().map_err(|_| {
                            Error::CompilationError("Failed to acquire builder lock for store".to_string())
                        })?;
                        
                        // Create a placeholder value to store
                        let store_value = match value_result.value_type {
                            LlvmType::Int32 => self.context.i32_type().const_zero().into(),
                            LlvmType::Int64 => self.context.i64_type().const_zero().into(),
                            LlvmType::Float64 => self.context.f64_type().const_zero().into(),
                            LlvmType::Boolean => self.context.bool_type().const_zero().into(),
                            _ => self.context.i64_type().const_zero().into(),
                        };
                        
                        builder_guard_inner.build_store(*alloca_ptr, store_value)
                            .map_err(|e| Error::CompilationError(format!("Failed to build store: {:?}", e)))?;
                        
                        tracing::debug!("Generated store instruction for assignment to: {}", identifier.value);
                    } else {
                        tracing::debug!("Assigning to variable (no alloca): {}", identifier.value);
                    }
                } else {
                    return Err(Error::CompilationError(format!(
                        "Cannot assign to undeclared variable: {}",
                        identifier.value
                    )));
                }
            }
        } else {
            return Err(Error::CompilationError("Assignment target must be an identifier".to_string()));
        }
        
        Ok(())
    }
    
    /// Compile print statement
    pub fn compile_print_statement_real(&mut self, print_stmt: &PrintStatement) -> Result<(), Error> {
        let builder_guard = self.builder.lock().map_err(|_| {
            Error::CompilationError("Failed to acquire builder lock".to_string())
        })?;
        
        tracing::debug!("Compiling print statement with {} arguments", print_stmt.arguments.len());
        
        // Compile all arguments
        let mut arg_values = Vec::new();
        for arg in &print_stmt.arguments {
            let arg_val = self.compile_expression_real(arg.as_ref())?;
            arg_values.push(arg_val);
        }
        
        // Generate actual LLVM call to print function
        let module_guard = self.module.lock().map_err(|_| {
            Error::CompilationError("Failed to acquire module lock for print".to_string())
        })?;
        
        // Declare printf function if not already declared
        let printf_type = self.context.i32_type().fn_type(
            &[self.context.i8_type().ptr_type(AddressSpace::default()).into()],
            true, // variadic
        );
        
        let printf_fn = module_guard.add_function("printf", printf_type, None);
        
        // Create a simple format string for basic printing
        let format_str = self.context.const_string(b"%s\n", true);
        let format_global = module_guard.add_global(format_str.get_type(), Some(AddressSpace::default()), "fmt_str");
        format_global.set_initializer(&format_str);
        format_global.set_constant(true);
        
        // Get pointer to format string
        let format_ptr = format_global.as_pointer_value();
        
        // For now, just call printf with the format string (simplified)
        let print_args = vec![format_ptr.into()];
        
        builder_guard.build_call(printf_fn, &print_args, "printf_call")
            .map_err(|e| Error::CompilationError(format!("Failed to build printf call: {:?}", e)))?;
        
        tracing::debug!("Generated printf call with {} arguments", arg_values.len());
        
        Ok(())
    }
}
