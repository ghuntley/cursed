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

    /// Get the generated LLVM IR as a string
    pub fn get_ir_string(&self) -> String {
        self.module.print_to_string().to_string()
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
            RelocMode::Default,
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
