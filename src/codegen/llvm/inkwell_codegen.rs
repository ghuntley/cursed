//! Inkwell-based LLVM Code Generator for CURSED
//! 
//! This module provides the main entry point for type-safe LLVM code generation
//! using the inkwell LLVM bindings instead of string-based IR generation.

use crate::ast::{Program, Statement, Expression, FunctionStatement};
use crate::error::CursedError;
use crate::codegen::llvm::inkwell_statement_generator::InkwellStatementGenerator;
use crate::codegen::llvm::inkwell_expression_compiler::InkwellExpressionCompiler;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::{Module, Linkage};
use inkwell::values::{BasicValueEnum, BasicValue, IntValue, FloatValue, PointerValue, FunctionValue, BasicMetadataValueEnum};
use inkwell::types::{BasicTypeEnum, BasicType, IntType, FloatType, FunctionType};
use inkwell::basic_block::BasicBlock;
use inkwell::{AddressSpace, IntPredicate, FloatPredicate, OptimizationLevel};
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::targets::{Target, TargetMachine, RelocMode, CodeModel, FileType, InitializationConfig, TargetTriple};
use std::collections::HashMap;
use std::path::Path;

/// Main inkwell-based LLVM code generator for CURSED
pub struct InkwellCodeGenerator<'ctx> {
    /// LLVM context
    context: &'ctx Context,
    
    /// LLVM module
    module: Module<'ctx>,
    
    /// LLVM IR builder
    builder: Builder<'ctx>,
    
    /// Statement generator
    statement_generator: InkwellStatementGenerator<'ctx>,
    
    /// Expression compiler
    expression_compiler: InkwellExpressionCompiler<'ctx>,
    
    /// Compiled functions registry
    functions: HashMap<String, FunctionValue<'ctx>>,
    
    /// Global variables
    globals: HashMap<String, PointerValue<'ctx>>,
    
    /// Optimization level
    optimization_level: OptimizationLevel,
    
    /// Target triple
    target_triple: String,
}

impl<'ctx> InkwellCodeGenerator<'ctx> {
    /// Create a new inkwell code generator
    pub fn new(context: &'ctx Context, module_name: &str) -> Result<Self, CursedError> {
        let module = context.create_module(module_name);
        let builder = context.create_builder();
        let statement_generator = InkwellStatementGenerator::new(context, module_name);
        let expression_compiler = InkwellExpressionCompiler::new(context, &builder);
        
        Ok(Self {
            context,
            module,
            builder,
            statement_generator,
            expression_compiler,
            functions: HashMap::new(),
            globals: HashMap::new(),
            optimization_level: OptimizationLevel::None,
            target_triple: "x86_64-unknown-linux-gnu".to_string(),
        })
    }

    /// Set the optimization level
    pub fn set_optimization_level(&mut self, level: OptimizationLevel) {
        self.optimization_level = level;
    }

    /// Set the target triple
    pub fn set_target_triple(&mut self, triple: String) {
        self.target_triple = triple.clone();
        let target_triple = triple.as_str().try_into().unwrap();
        self.module.set_triple(&target_triple);
    }

    /// Get the LLVM module
    pub fn module(&self) -> &Module<'ctx> {
        &self.module
    }

    /// Get the LLVM module (mutable)
    pub fn module_mut(&mut self) -> &mut Module<'ctx> {
        &mut self.module
    }

    /// Compile a complete CURSED program
    pub fn compile_program(&mut self, program: &Program) -> Result<(), CursedError> {
        // First, declare all functions (forward declarations)
        for statement in &program.statements {
            if let Statement::Function(func_stmt) = statement {
                self.declare_function(func_stmt)?;
            }
        }

        // Generate runtime function declarations
        self.generate_runtime_declarations()?;

        // Collect non-function statements for main function
        let mut top_level_statements = Vec::new();
        let mut has_main_function = false;

        // Second pass: Generate function definitions and collect top-level statements
        for statement in &program.statements {
            match statement {
                Statement::Function(func_stmt) => {
                    if func_stmt.name == "main" {
                        has_main_function = true;
                    }
                    self.compile_function(func_stmt)?;
                },
                _ => {
                    // Collect non-function statements for main function
                    top_level_statements.push(statement);
                }
            }
        }

        // Generate main function if needed
        if !has_main_function || !top_level_statements.is_empty() {
            self.generate_main_function(&top_level_statements, has_main_function)?;
        }

        // Verify the module
        if let Err(error) = self.module.verify() {
            return Err(CursedError::CompilerError(format!("Module verification failed: {}", error)));
        }

        Ok(())
    }

    /// Declare a function (forward declaration)
    fn declare_function(&mut self, func_stmt: &FunctionStatement) -> Result<FunctionValue<'ctx>, CursedError> {
        // Convert parameter types
        let mut param_types = Vec::new();
        for param in &func_stmt.parameters {
            let param_type = if let Some(param_type) = &param.param_type {
                self.convert_cursed_type_to_llvm(param_type)?
            } else {
                self.context.i32_type().into() // Default to i32
            };
            param_types.push(param_type.into());
        }

        // Convert return type
        let return_type = if let Some(ret_type) = &func_stmt.return_type {
            Some(self.convert_cursed_type_to_llvm(ret_type)?)
        } else {
            None // void function
        };

        // Create function type
        let function_type = if let Some(ret_type) = return_type {
            ret_type.fn_type(&param_types, false)
        } else {
            self.context.void_type().fn_type(&param_types, false)
        };

        // Add function to module
        let function = self.module.add_function(&func_stmt.name, function_type, None);

        // Set parameter names
        for (i, param) in func_stmt.parameters.iter().enumerate() {
            if let Some(param_value) = function.get_nth_param(i as u32) {
                param_value.set_name(&param.name);
            }
        }

        // Register the function
        self.functions.insert(func_stmt.name.clone(), function);

        Ok(function)
    }

    /// Compile a function definition
    fn compile_function(&mut self, func_stmt: &FunctionStatement) -> Result<(), CursedError> {
        let function = self.functions.get(&func_stmt.name)
            .ok_or_else(|| CursedError::CompilerError(format!("Function '{}' not declared", func_stmt.name)))?
            .clone();

        // Create entry basic block
        let entry_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry_block);

        // Set current function context
        self.statement_generator.set_current_function(function);
        self.expression_compiler.set_current_function(function);

        // Handle function parameters
        for (i, param) in func_stmt.parameters.iter().enumerate() {
            if let Some(param_value) = function.get_nth_param(i as u32) {
                // Create alloca for parameter and store the parameter value
                let param_type = param_value.get_type();
                let alloca = self.builder.build_alloca(param_type, &param.name)
                    .map_err(|e| CursedError::CompilerError(format!("Failed to allocate parameter '{}': {:?}", param.name, e)))?;

                self.builder.build_store(alloca, param_value)
                    .map_err(|e| CursedError::CompilerError(format!("Failed to store parameter '{}': {:?}", param.name, e)))?;

                // Register the parameter as a variable
                self.statement_generator.add_variable(param.name.clone(), alloca);
                self.expression_compiler.add_variable(param.name.clone(), alloca);
            }
        }

        // Compile function body
        for statement in &func_stmt.body {
            self.statement_generator.compile_statement(statement)?;
        }

        // Add default return if the function doesn't already have a terminator
        if !self.has_terminator() {
            if func_stmt.return_type.is_some() {
                // Function has return type but no explicit return - add default return
                let default_value = self.get_default_value_for_type(&func_stmt.return_type)?;
                self.builder.build_return(Some(&default_value))
                    .map_err(|e| CursedError::CompilerError(format!("Failed to build default return: {:?}", e)))?;
            } else {
                // Void function
                self.builder.build_return(None)
                    .map_err(|e| CursedError::CompilerError(format!("Failed to build void return: {:?}", e)))?;
            }
        }

        // Clear function context
        self.statement_generator.clear_current_function();

        Ok(())
    }

    /// Generate main function with top-level statements
    fn generate_main_function(&mut self, top_level_statements: &[&Statement], has_main: bool) -> Result<(), CursedError> {
        if has_main && top_level_statements.is_empty() {
            return Ok(()); // Main already exists and no top-level statements
        }

        let main_function = if has_main {
            // Get existing main function
            self.functions.get("main").unwrap().clone()
        } else {
            // Create new main function
            let i32_type = self.context.i32_type();
            let main_type = i32_type.fn_type(&[], false);
            let main_function = self.module.add_function("main", main_type, None);
            self.functions.insert("main".to_string(), main_function);
            main_function
        };

        if !top_level_statements.is_empty() {
            // Create entry block for main if it doesn't exist
            let entry_block = if has_main {
                // Insert statements at the beginning of existing main
                main_function.get_first_basic_block().unwrap()
            } else {
                self.context.append_basic_block(main_function, "entry")
            };

            self.builder.position_at_end(entry_block);
            self.statement_generator.set_current_function(main_function);

            // Compile top-level statements
            for statement in top_level_statements {
                self.statement_generator.compile_statement(statement)?;
            }

            if !has_main {
                // Add return 0 for generated main
                let zero = self.context.i32_type().const_zero();
                self.builder.build_return(Some(&zero))
                    .map_err(|e| CursedError::CompilerError(format!("Failed to build main return: {:?}", e)))?;
            }

            self.statement_generator.clear_current_function();
        }

        Ok(())
    }

    /// Generate runtime function declarations
    fn generate_runtime_declarations(&mut self) -> Result<(), CursedError> {
        let i32_type = self.context.i32_type();
        let i8_type = self.context.i8_type();
        let i64_type = self.context.i64_type();
        let double_type = self.context.f64_type();
        let void_type = self.context.void_type();
        let i8_ptr_type = i8_type.ptr_type(AddressSpace::default());

        // Printf function
        let printf_type = i32_type.fn_type(&[i8_ptr_type.into()], true);
        self.module.add_function("printf", printf_type, None);

        // Puts function
        let puts_type = i32_type.fn_type(&[i8_ptr_type.into()], false);
        self.module.add_function("puts", puts_type, None);

        // Memory allocation functions
        let malloc_type = i8_ptr_type.fn_type(&[i64_type.into()], false);
        self.module.add_function("malloc", malloc_type, None);

        let free_type = void_type.fn_type(&[i8_ptr_type.into()], false);
        self.module.add_function("free", free_type, None);

        // String functions
        let strlen_type = i64_type.fn_type(&[i8_ptr_type.into()], false);
        self.module.add_function("strlen", strlen_type, None);

        let strcpy_type = i8_ptr_type.fn_type(&[i8_ptr_type.into(), i8_ptr_type.into()], false);
        self.module.add_function("strcpy", strcpy_type, None);

        // CURSED runtime functions
        let i32_to_string_type = i8_ptr_type.fn_type(&[i32_type.into()], false);
        self.module.add_function("i32_to_string", i32_to_string_type, None);

        let char_to_string_type = i8_ptr_type.fn_type(&[i8_type.into()], false);
        self.module.add_function("char_to_string", char_to_string_type, None);

        let string_concat_type = i8_ptr_type.fn_type(&[i8_ptr_type.into(), i8_ptr_type.into()], false);
        self.module.add_function("string_concat", string_concat_type, None);

        // Tea functions (CURSED's print system)
        let tea_type = i8_ptr_type.fn_type(&[i64_type.into()], false);
        self.module.add_function("tea", tea_type, None);

        let tea_float_type = i8_ptr_type.fn_type(&[double_type.into()], false);
        self.module.add_function("tea_float", tea_float_type, None);

        let tea_bool_type = i8_ptr_type.fn_type(&[i32_type.into()], false);
        self.module.add_function("tea_bool", tea_bool_type, None);

        Ok(())
    }

    /// Convert CURSED type to LLVM type
    fn convert_cursed_type_to_llvm(&self, cursed_type: &crate::ast::Type) -> Result<BasicTypeEnum<'ctx>, CursedError> {
        use crate::ast::Type;
        
        match cursed_type {
            Type::Integer | Type::Normie => Ok(self.context.i32_type().into()),
            Type::Float => Ok(self.context.f64_type().into()),
            Type::String | Type::Tea => Ok(self.context.i8_type().ptr_type(AddressSpace::default()).into()),
            Type::Boolean | Type::Lit => Ok(self.context.bool_type().into()),
            Type::Sip => Ok(self.context.i8_type().into()),
            Type::Smol => Ok(self.context.i8_type().into()),
            Type::Mid => Ok(self.context.i16_type().into()),
            Type::Thicc => Ok(self.context.i64_type().into()),
            Type::Snack => Ok(self.context.f32_type().into()),
            Type::Meal => Ok(self.context.f64_type().into()),
            Type::Byte => Ok(self.context.i8_type().into()),
            Type::Rune => Ok(self.context.i32_type().into()),
            Type::Void => Err(CursedError::CompilerError("Void type cannot be used as basic type".to_string())),
            _ => Err(CursedError::CompilerError(format!("Unsupported type conversion: {:?}", cursed_type)))
        }
    }

    /// Get default value for a type
    fn get_default_value_for_type(&self, type_opt: &Option<crate::ast::Type>) -> Result<BasicValueEnum<'ctx>, CursedError> {
        if let Some(cursed_type) = type_opt {
            use crate::ast::Type;
            
            match cursed_type {
                Type::Integer | Type::Normie => Ok(self.context.i32_type().const_zero().into()),
                Type::Float => Ok(self.context.f64_type().const_zero().into()),
                Type::Boolean | Type::Lit => Ok(self.context.bool_type().const_zero().into()),
                Type::Sip | Type::Smol | Type::Byte => Ok(self.context.i8_type().const_zero().into()),
                Type::Mid => Ok(self.context.i16_type().const_zero().into()),
                Type::Thicc => Ok(self.context.i64_type().const_zero().into()),
                Type::Snack => Ok(self.context.f32_type().const_zero().into()),
                Type::Meal => Ok(self.context.f64_type().const_zero().into()),
                Type::Rune => Ok(self.context.i32_type().const_zero().into()),
                Type::String | Type::Tea => {
                    let ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
                    Ok(ptr_type.const_null().into())
                },
                _ => Err(CursedError::CompilerError(format!("Cannot get default value for type: {:?}", cursed_type)))
            }
        } else {
            Err(CursedError::CompilerError("Cannot get default value for void type".to_string()))
        }
    }

    /// Check if current basic block has terminator
    fn has_terminator(&self) -> bool {
        if let Some(block) = self.builder.get_insert_block() {
            block.get_terminator().is_some()
        } else {
            false
        }
    }

    /// Compile a single statement
    pub fn compile_statement(&mut self, statement: &Statement) -> Result<(), CursedError> {
        match statement {
            Statement::Function(func_stmt) => {
                self.compile_function(func_stmt)?;
            }
            Statement::Expression(expr) => {
                self.compile_expression(expr)?;
            }
            Statement::Variable(var_stmt) => {
                self.compile_variable_statement(var_stmt)?;
            }
            Statement::Return(ret_stmt) => {
                self.compile_return_statement(ret_stmt)?;
            }
            Statement::If(if_stmt) => {
                self.compile_if_statement(if_stmt)?;
            }
            Statement::While(while_stmt) => {
                self.compile_while_statement(while_stmt)?;
            }
            _ => {
                // For complex statements, use the statement generator
                return Err(CursedError::CompilerError(format!("Statement type not yet implemented in inkwell backend: {:?}", statement)));
            }
        }
        Ok(())
    }

    /// Compile an expression and return its value
    pub fn compile_expression(&mut self, expression: &Expression) -> Result<BasicValueEnum<'ctx>, CursedError> {
        self.expression_compiler.compile_expression(expression)
    }

    /// Compile a variable statement
    fn compile_variable_statement(&mut self, var_stmt: &crate::ast::VariableStatement) -> Result<(), CursedError> {
        // Create alloca for the variable
        let var_type = if let Some(init_expr) = &var_stmt.initializer {
            // Infer type from initializer
            let init_value = self.compile_expression(init_expr)?;
            init_value.get_type()
        } else if let Some(type_annotation) = &var_stmt.variable_type {
            self.convert_cursed_type_to_llvm(type_annotation)?
        } else {
            return Err(CursedError::CompilerError("Cannot determine variable type".to_string()));
        };

        let alloca = self.builder.build_alloca(var_type, &var_stmt.name)
            .map_err(|e| CursedError::CompilerError(format!("Failed to create alloca: {}", e)))?;
        
        // Store initial value if present
        if let Some(init_expr) = &var_stmt.initializer {
            let init_value = self.compile_expression(init_expr)?;
            self.builder.build_store(alloca, init_value)
                .map_err(|e| CursedError::CompilerError(format!("Failed to store initial value: {}", e)))?;
        }

        // Add to expression compiler's variable scope
        self.expression_compiler.add_variable(var_stmt.name.clone(), alloca);
        
        Ok(())
    }

    /// Compile a return statement
    fn compile_return_statement(&mut self, ret_stmt: &crate::ast::ReturnStatement) -> Result<(), CursedError> {
        if let Some(expr) = &ret_stmt.value {
            let return_value = self.compile_expression(expr)?;
            self.builder.build_return(Some(&return_value))
                .map_err(|e| CursedError::CompilerError(format!("Failed to build return: {}", e)))?;
        } else {
            self.builder.build_return(None)
                .map_err(|e| CursedError::CompilerError(format!("Failed to build void return: {}", e)))?;
        }
        Ok(())
    }

    /// Compile an if statement
    fn compile_if_statement(&mut self, if_stmt: &crate::ast::IfStatement) -> Result<(), CursedError> {
        let condition = self.compile_expression(&if_stmt.condition)?;
        
        let current_function = self.builder.get_insert_block()
            .and_then(|bb| bb.get_parent())
            .ok_or_else(|| CursedError::CompilerError("No current function for if statement".to_string()))?;

        // Create basic blocks
        let then_block = self.context.append_basic_block(current_function, "if.then");
        let else_block = if if_stmt.else_body.is_some() {
            Some(self.context.append_basic_block(current_function, "if.else"))
        } else {
            None
        };
        let merge_block = self.context.append_basic_block(current_function, "if.end");

        // Build conditional branch
        if let Some(else_bb) = else_block {
            self.builder.build_conditional_branch(condition.into_int_value(), then_block, else_bb)
                .map_err(|e| CursedError::CompilerError(format!("Failed to build conditional branch: {}", e)))?;
        } else {
            self.builder.build_conditional_branch(condition.into_int_value(), then_block, merge_block)
                .map_err(|e| CursedError::CompilerError(format!("Failed to build conditional branch: {}", e)))?;
        }

        // Generate then block
        self.builder.position_at_end(then_block);
        for stmt in &if_stmt.then_body.statements {
            self.compile_statement(stmt)?;
        }
        if !self.has_terminator() {
            self.builder.build_unconditional_branch(merge_block)
                .map_err(|e| CursedError::CompilerError(format!("Failed to build branch: {}", e)))?;
        }

        // Generate else block if present
        if let (Some(else_bb), Some(else_body)) = (else_block, &if_stmt.else_body) {
            self.builder.position_at_end(else_bb);
            for stmt in &else_body.statements {
                self.compile_statement(stmt)?;
            }
            if !self.has_terminator() {
                self.builder.build_unconditional_branch(merge_block)
                    .map_err(|e| CursedError::CompilerError(format!("Failed to build branch: {}", e)))?;
            }
        }

        // Continue with merge block
        self.builder.position_at_end(merge_block);
        
        Ok(())
    }

    /// Compile a while statement
    fn compile_while_statement(&mut self, while_stmt: &crate::ast::WhileStatement) -> Result<(), CursedError> {
        let current_function = self.builder.get_insert_block()
            .and_then(|bb| bb.get_parent())
            .ok_or_else(|| CursedError::CompilerError("No current function for while statement".to_string()))?;

        // Create basic blocks
        let condition_block = self.context.append_basic_block(current_function, "while.cond");
        let body_block = self.context.append_basic_block(current_function, "while.body");
        let exit_block = self.context.append_basic_block(current_function, "while.end");

        // Jump to condition block
        self.builder.build_unconditional_branch(condition_block)
            .map_err(|e| CursedError::CompilerError(format!("Failed to build branch: {}", e)))?;

        // Generate condition block
        self.builder.position_at_end(condition_block);
        let condition = self.compile_expression(&while_stmt.condition)?;
        self.builder.build_conditional_branch(condition.into_int_value(), body_block, exit_block)
            .map_err(|e| CursedError::CompilerError(format!("Failed to build conditional branch: {}", e)))?;

        // Generate body block
        self.builder.position_at_end(body_block);
        for stmt in &while_stmt.body.statements {
            self.compile_statement(stmt)?;
        }
        if !self.has_terminator() {
            self.builder.build_unconditional_branch(condition_block)
                .map_err(|e| CursedError::CompilerError(format!("Failed to build branch: {}", e)))?;
        }

        // Continue with exit block
        self.builder.position_at_end(exit_block);
        
        Ok(())
    }

    /// Get the generated LLVM IR as a string
    pub fn get_ir_string(&self) -> String {
        self.module.print_to_string().to_string()
    }
    
    /// Set target triple for compilation
    pub fn set_target_triple(&mut self, triple: &str) {
        self.target_triple = triple.to_string();
        self.module.set_triple(&TargetTriple::create(triple));
    }
    
    /// Enable WebAssembly-specific optimizations
    pub fn enable_wasm_optimizations(&mut self) {
        // WebAssembly-specific optimizations will be applied during LLVM passes
        // This method serves as a marker for WASM-specific configuration
    }
    
    /// Compile AST to WebAssembly-optimized LLVM IR
    pub fn compile_to_wasm_ir(&mut self, program: &Program) -> Result<String, CursedError> {
        // Configure for WebAssembly target
        self.set_target_triple("wasm32-unknown-unknown");
        
        // Add WebAssembly-specific function exports
        self.add_wasm_exports();
        
        // Compile the program
        self.compile(program)?;
        
        // Return the generated IR
        Ok(self.get_ir_string())
    }
    
    /// Add WebAssembly-specific function exports with enhanced features
    fn add_wasm_exports(&mut self) {
        // Add export directives for WebAssembly
        // This will make functions visible from the WASM module
        
        // Memory management exports for WASM
        let i32_type = self.context.i32_type();
        let void_type = self.context.void_type();
        let i8_ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
        
        // Enhanced memory allocation function
        let malloc_type = i8_ptr_type.fn_type(&[i32_type.into()], false);
        self.module.add_function("__wasm_malloc", malloc_type, Some(Linkage::External));
        
        // Enhanced memory deallocation function
        let free_type = void_type.fn_type(&[i8_ptr_type.into()], false);
        self.module.add_function("__wasm_free", free_type, Some(Linkage::External));
        
        // Memory management utilities
        let memory_grow_type = void_type.fn_type(&[i32_type.into()], false);
        self.module.add_function("__wasm_memory_grow", memory_grow_type, Some(Linkage::External));
        
        let memory_size_type = i32_type.fn_type(&[], false);
        self.module.add_function("__wasm_memory_size", memory_size_type, Some(Linkage::External));
        
        // Bulk memory operations
        let memory_fill_type = void_type.fn_type(&[
            i8_ptr_type.into(),
            self.context.i8_type().into(),
            i32_type.into()
        ], false);
        self.module.add_function("__wasm_memory_fill", memory_fill_type, Some(Linkage::External));
        
        let memory_copy_type = void_type.fn_type(&[
            i8_ptr_type.into(),
            i8_ptr_type.into(),
            i32_type.into()
        ], false);
        self.module.add_function("__wasm_memory_copy", memory_copy_type, Some(Linkage::External));
        
        // SIMD support functions (conditional on target features)
        if self.has_simd_support() {
            let v128_type = self.context.i8_type().vec_type(16);
            let v128_ptr_type = v128_type.ptr_type(AddressSpace::default());
            
            let simd_load_type = v128_type.fn_type(&[v128_ptr_type.into()], false);
            self.module.add_function("__wasm_v128_load", simd_load_type, Some(Linkage::External));
            
            let simd_store_type = void_type.fn_type(&[v128_ptr_type.into(), v128_type.into()], false);
            self.module.add_function("__wasm_v128_store", simd_store_type, Some(Linkage::External));
        }
        
        // Threading support (conditional on target features)
        if self.has_threading_support() {
            let atomic_wait_type = i32_type.fn_type(&[
                i32_type.ptr_type(AddressSpace::default()).into(),
                i32_type.into(),
                self.context.i64_type().into()
            ], false);
            self.module.add_function("__wasm_atomic_wait32", atomic_wait_type, Some(Linkage::External));
            
            let atomic_notify_type = i32_type.fn_type(&[
                i32_type.ptr_type(AddressSpace::default()).into(),
                i32_type.into()
            ], false);
            self.module.add_function("__wasm_atomic_notify", atomic_notify_type, Some(Linkage::External));
        }
    }
    
    /// Check if SIMD support is available
    fn has_simd_support(&self) -> bool {
        // In a real implementation, this would check target features
        // For now, assume SIMD is available
        true
    }
    
    /// Check if threading support is available
    fn has_threading_support(&self) -> bool {
        // In a real implementation, this would check target features
        // For now, assume threading is available
        true
    }
    
    /// Enhanced WASM IR compilation with debugging support
    pub fn compile_to_wasm_ir_with_debug(&mut self, program: &Program, debug_info: bool) -> Result<String, CursedError> {
        // Configure for WebAssembly target with debug information
        self.set_target_triple("wasm32-unknown-unknown");
        
        // Enable debug information if requested
        if debug_info {
            self.enable_debug_info();
        }
        
        // Add WebAssembly-specific function exports
        self.add_wasm_exports();
        
        // Add debug helper functions
        if debug_info {
            self.add_debug_functions();
        }
        
        // Compile the program
        self.compile(program)?;
        
        // Return the generated IR with debug annotations
        let mut ir = self.get_ir_string();
        
        if debug_info {
            ir = self.add_debug_metadata(ir);
        }
        
        Ok(ir)
    }
    
    /// Add debug helper functions for WASM
    fn add_debug_functions(&mut self) {
        let i32_type = self.context.i32_type();
        let void_type = self.context.void_type();
        let i8_ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
        
        // Debug trace function
        let debug_trace_type = void_type.fn_type(&[
            i8_ptr_type.into(),
            i32_type.into()
        ], false);
        self.module.add_function("__wasm_debug_trace", debug_trace_type, Some(Linkage::External));
        
        // Performance profiling function
        let profile_enter_type = void_type.fn_type(&[i8_ptr_type.into()], false);
        self.module.add_function("__wasm_profile_enter", profile_enter_type, Some(Linkage::External));
        
        let profile_exit_type = void_type.fn_type(&[i8_ptr_type.into()], false);
        self.module.add_function("__wasm_profile_exit", profile_exit_type, Some(Linkage::External));
        
        // Memory debugging functions
        let debug_alloc_type = i8_ptr_type.fn_type(&[
            i32_type.into(),
            i8_ptr_type.into(),
            i32_type.into()
        ], false);
        self.module.add_function("__wasm_debug_alloc", debug_alloc_type, Some(Linkage::External));
        
        let debug_free_type = void_type.fn_type(&[
            i8_ptr_type.into(),
            i8_ptr_type.into(),
            i32_type.into()
        ], false);
        self.module.add_function("__wasm_debug_free", debug_free_type, Some(Linkage::External));
    }
    
    /// Enable debug information generation
    fn enable_debug_info(&mut self) {
        // In a real implementation, this would configure LLVM debug info
        // For now, this is a placeholder for debug configuration
    }
    
    /// Add debug metadata to IR
    fn add_debug_metadata(&self, mut ir: String) -> String {
        // Add debug metadata annotations to the IR
        let debug_header = format!(
            "; Debug Information for CURSED WebAssembly Module\n\
             ; Generated with debug support enabled\n\
             !llvm.dbg.cu = !{{!{}}}\n\
             !llvm.module.flags = !{{{{!1, !2}}}}\n\
             \n\
             !0 = distinct !DICompileUnit(language: DW_LANG_C99, file: !3, producer: \"CURSED Compiler\", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug)\n\
             !1 = !{{i32 2, !\"Dwarf Version\", i32 4}}\n\
             !2 = !{{i32 2, !\"Debug Info Version\", i32 3}}\n\
             !3 = !DIFile(filename: \"cursed_module.csd\", directory: \".\")\n\n"
        );
        
        // Insert debug metadata at the beginning
        ir = format!("{}{}", debug_header, ir);
        
        ir
    }

    /// Compile to object file
    pub fn compile_to_object_file(&self, output_path: &Path) -> Result<(), CursedError> {
        // Initialize targets
        Target::initialize_all(&InitializationConfig::default());

        let target_triple = self.module.get_triple();
        let target = Target::from_triple(&target_triple)
            .map_err(|e| CursedError::CompilerError(format!("Failed to create target from triple: {}", e)))?;

        let target_machine = target.create_target_machine(
            &target_triple,
            "generic",
            "",
            self.optimization_level,
            RelocMode::PIC,
            CodeModel::Default,
        ).ok_or_else(|| CursedError::CompilerError("Failed to create target machine".to_string()))?;

        target_machine.write_to_file(&self.module, FileType::Object, output_path)
            .map_err(|e| CursedError::CompilerError(format!("Failed to write object file: {}", e)))?;

        Ok(())
    }

    /// Create execution engine for JIT compilation
    pub fn create_execution_engine(&self) -> Result<ExecutionEngine<'ctx>, CursedError> {
        self.module.create_jit_execution_engine(self.optimization_level)
            .map_err(|e| CursedError::CompilerError(format!("Failed to create execution engine: {}", e)))
    }

    /// Add a variable to the current scope (for use by statement generator)
    pub fn add_variable(&mut self, name: String, alloca: PointerValue<'ctx>) {
        self.statement_generator.add_variable(name.clone(), alloca);
        self.expression_compiler.add_variable(name, alloca);
    }

    /// Get a function by name
    pub fn get_function(&self, name: &str) -> Option<FunctionValue<'ctx>> {
        self.functions.get(name).copied()
    }

    /// Add a global variable
    pub fn add_global(&mut self, name: String, global: PointerValue<'ctx>) {
        self.globals.insert(name, global);
    }
}

/// Helper trait for backwards compatibility with existing string-based API
impl<'ctx> InkwellCodeGenerator<'ctx> {
    /// Backward compatibility method that returns IR as string (like the old API)
    pub fn compile(&mut self, source: &str) -> Result<String, CursedError> {
        // Parse the source code
        let mut parser = crate::parser::new_parser(source)?;
        let program = parser.parse_program()?;
        
        // Compile the program
        self.compile_program(&program)?;
        
        // Return the generated IR as string
        Ok(self.get_ir_string())
    }

    /// Compile AST directly (backward compatibility)
    pub fn compile_ast(&mut self, program: &Program) -> Result<String, CursedError> {
        self.compile_program(program)?;
        Ok(self.get_ir_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::context::Context;

    #[test]
    fn test_create_inkwell_codegen() {
        let context = Context::create();
        let codegen = InkwellCodeGenerator::new(&context, "test_module");
        
        assert!(codegen.is_ok());
        if let Ok(codegen) = codegen {
            assert_eq!(codegen.functions.len(), 0);
            assert_eq!(codegen.globals.len(), 0);
        }
    }

    #[test]
    fn test_module_creation() {
        let context = Context::create();
        let codegen = InkwellCodeGenerator::new(&context, "test_module").unwrap();
        
        let module_name = codegen.module().get_name().to_str().unwrap();
        assert_eq!(module_name, "test_module");
    }

    #[test]
    fn test_runtime_declarations() {
        let context = Context::create();
        let mut codegen = InkwellCodeGenerator::new(&context, "test_module").unwrap();
        
        // Generate runtime declarations
        let result = codegen.generate_runtime_declarations();
        assert!(result.is_ok());
        
        // Check that some runtime functions were declared
        let module = codegen.module();
        assert!(module.get_function("printf").is_some());
        assert!(module.get_function("puts").is_some());
        assert!(module.get_function("malloc").is_some());
    }

    #[test]
    fn test_get_ir_string() {
        let context = Context::create();
        let codegen = InkwellCodeGenerator::new(&context, "test_module").unwrap();
        
        let ir_string = codegen.get_ir_string();
        assert!(ir_string.contains("test_module"));
    }
}
