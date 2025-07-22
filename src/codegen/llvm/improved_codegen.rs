//! Improved LLVM Code Generator using proper LLVM IR builders
//! 
//! This module replaces string-based IR generation with proper LLVM IR builders
//! for type safety, performance, and WebAssembly compatibility.

use crate::ast::{Program, Statement, Expression, FunctionStatement, Literal, BinaryOperator, Type};
use crate::error::{CursedError, SourceLocation};
use crate::codegen::llvm::inkwell_codegen::InkwellCodeGenerator;
use crate::codegen::llvm::register_tracker::RegisterTracker;
use inkwell::context::Context;
use inkwell::builder::Builder;
use inkwell::module::{Module, Linkage};
use inkwell::values::{BasicValueEnum, BasicValue, IntValue, FloatValue, PointerValue, FunctionValue};
use inkwell::types::{BasicTypeEnum, BasicType, IntType, FloatType, FunctionType};
use inkwell::basic_block::BasicBlock;
use inkwell::{AddressSpace, IntPredicate, FloatPredicate, OptimizationLevel};
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::targets::{Target, TargetMachine, RelocMode, CodeModel, FileType, InitializationConfig, TargetTriple};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Improved LLVM code generator that uses proper LLVM IR builders instead of string concatenation
pub struct ImprovedLlvmCodeGenerator<'ctx> {
    /// LLVM context
    context: &'ctx Context,
    
    /// LLVM module
    module: Module<'ctx>,
    
    /// LLVM IR builder
    builder: Builder<'ctx>,
    
    /// Inkwell-based code generator
    inkwell_generator: InkwellCodeGenerator<'ctx>,
    
    /// Variable storage mapping
    variables: HashMap<String, PointerValue<'ctx>>,
    
    /// Function registry
    functions: HashMap<String, FunctionValue<'ctx>>,
    
    /// Register tracker for consistent numbering
    register_tracker: RegisterTracker,
    
    /// Optimization level
    optimization_level: OptimizationLevel,
    
    /// Target triple
    target_triple: String,
    
    /// Current function context
    current_function: Option<FunctionValue<'ctx>>,
    
    /// Current basic block
    current_block: Option<BasicBlock<'ctx>>,
}

impl<'ctx> ImprovedLlvmCodeGenerator<'ctx> {
    /// Create a new improved LLVM code generator
    pub fn new(context: &'ctx Context, module_name: &str) -> Result<Self, CursedError> {
        let module = context.create_module(module_name);
        let builder = context.create_builder();
        
        // Create inkwell-based generator
        let inkwell_generator = InkwellCodeGenerator::new(context, module_name)?;
        
        // Detect target triple
        let target_triple = Self::detect_target_triple();
        
        Ok(Self {
            context,
            module,
            builder,
            inkwell_generator,
            variables: HashMap::new(),
            functions: HashMap::new(),
            register_tracker: RegisterTracker::new(),
            optimization_level: OptimizationLevel::Default,
            target_triple,
            current_function: None,
            current_block: None,
        })
    }
    
    /// Detect target triple for cross-compilation
    fn detect_target_triple() -> String {
        // Use runtime platform detection for cross-compilation
        use crate::runtime::platform::get_runtime_platform_info;
        let platform_info = get_runtime_platform_info();
        platform_info.target_triple
    }
    
    /// Set optimization level
    pub fn set_optimization_level(&mut self, level: OptimizationLevel) {
        self.optimization_level = level;
    }
    
    /// Compile a CURSED program to LLVM IR using proper IR builders
    pub fn compile_program(&mut self, program: &Program) -> Result<String, CursedError> {
        // Initialize target and machine for the current triple
        self.initialize_target()?;
        
        // Generate LLVM IR for all statements
        for statement in &program.statements {
            self.compile_statement(statement)?;
        }
        
        // Apply optimizations if enabled
        if self.optimization_level != OptimizationLevel::None {
            self.apply_optimizations()?;
        }
        
        // Verify module
        if let Err(errors) = self.module.verify() {
            return Err(CursedError::CodegenError(format!("LLVM module verification failed: {}", errors)));
        }
        
        // Return the generated LLVM IR as a string
        Ok(self.module.print_to_string().to_string())
    }
    
    /// Initialize LLVM target for the current triple
    fn initialize_target(&mut self) -> Result<(), CursedError> {
        Target::initialize_all(&InitializationConfig::default());
        
        // Set target triple for the module
        let target_triple = TargetTriple::create(&self.target_triple);
        self.module.set_triple(&target_triple);
        
        // Get target and create target machine
        let target = Target::from_triple(&target_triple)
            .map_err(|e| CursedError::CodegenError(format!("Failed to create target: {}", e)))?;
            
        let target_machine = target.create_target_machine(
            &target_triple,
            "generic",
            "",
            self.optimization_level,
            RelocMode::Default,
            CodeModel::Default,
        ).ok_or_else(|| CursedError::CodegenError("Failed to create target machine".to_string()))?;
        
        // Set data layout
        let data_layout = target_machine.get_target_data().get_data_layout();
        self.module.set_data_layout(&data_layout);
        
        Ok(())
    }
    
    /// Apply LLVM optimization passes
    fn apply_optimizations(&mut self) -> Result<(), CursedError> {
        // Use LLVM's pass manager for proper optimizations
        use inkwell::passes::{PassManager, PassManagerBuilder};
        
        let pass_manager = PassManager::create(&self.module);
        let pass_manager_builder = PassManagerBuilder::create();
        
        // Configure optimization level
        match self.optimization_level {
            OptimizationLevel::None => {
                // No optimizations
            }
            OptimizationLevel::Less => {
                pass_manager_builder.set_optimization_level(OptimizationLevel::Less);
            }
            OptimizationLevel::Default => {
                pass_manager_builder.set_optimization_level(OptimizationLevel::Default);
            }
            OptimizationLevel::Aggressive => {
                pass_manager_builder.set_optimization_level(OptimizationLevel::Aggressive);
            }
        }
        
        // Populate pass manager with optimization passes
        pass_manager_builder.populate_module_pass_manager(&pass_manager);
        
        // Run optimizations
        pass_manager.run_on(&self.module);
        
        Ok(())
    }
    
    /// Compile a statement using proper LLVM IR builders
    fn compile_statement(&mut self, statement: &Statement) -> Result<(), CursedError> {
        match statement {
            Statement::Function(func_stmt) => self.compile_function(func_stmt),
            Statement::Expression(expr) => {
                self.compile_expression(expr)?;
                Ok(())
            }
            Statement::Variable(var_stmt) => self.compile_variable_declaration(var_stmt),
            Statement::Return(ret_stmt) => self.compile_return_statement(ret_stmt),
            _ => {
                // Use inkwell generator for complex statements
                self.inkwell_generator.compile_statement(statement)
            }
        }
    }
    
    /// Compile a function declaration using proper LLVM function creation
    fn compile_function(&mut self, func_stmt: &FunctionStatement) -> Result<(), CursedError> {
        // Convert parameter types
        let mut param_types = Vec::new();
        for param in &func_stmt.parameters {
            let param_type = self.convert_type_to_llvm(&param.param_type)?;
            param_types.push(param_type.into());
        }
        
        // Convert return type
        let return_type = if let Some(ret_type) = &func_stmt.return_type {
            self.convert_type_to_llvm(ret_type)?
        } else {
            self.context.void_type().into()
        };
        
        // Create function type
        let function_type = if return_type.is_void_type() {
            self.context.void_type().fn_type(&param_types, false)
        } else {
            return_type.fn_type(&param_types, false)
        };
        
        // Create function
        let function = self.module.add_function(&func_stmt.name, function_type, Some(Linkage::External));
        self.functions.insert(func_stmt.name.clone(), function);
        
        // Create entry basic block
        let entry_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry_block);
        self.current_function = Some(function);
        self.current_block = Some(entry_block);
        
        // Allocate parameters
        for (i, param) in func_stmt.parameters.iter().enumerate() {
            let param_value = function.get_nth_param(i as u32).unwrap();
            let param_alloca = self.create_entry_alloca(&param.name, param_value.get_type())?;
            self.builder.build_store(param_alloca, param_value)
                .map_err(|e| CursedError::CodegenError(format!("Failed to store parameter: {}", e)))?;
            self.variables.insert(param.name.clone(), param_alloca);
        }
        
        // Compile function body
        if let Some(body) = &func_stmt.body {
            for statement in &body.statements {
                self.compile_statement(statement)?;
            }
        }
        
        // Add return if not already present
        if !self.current_block_has_terminator() {
            if return_type.is_void_type() {
                self.builder.build_return(None)
                    .map_err(|e| CursedError::CodegenError(format!("Failed to build return: {}", e)))?;
            } else {
                // Return default value
                let default_value = self.get_default_value_for_type(return_type)?;
                self.builder.build_return(Some(&default_value))
                    .map_err(|e| CursedError::CodegenError(format!("Failed to build return: {}", e)))?;
            }
        }
        
        // Verify function
        if !function.verify(true) {
            return Err(CursedError::CodegenError(format!("Function '{}' verification failed", func_stmt.name)));
        }
        
        Ok(())
    }
    
    /// Compile an expression using proper LLVM value creation
    fn compile_expression(&mut self, expression: &Expression) -> Result<BasicValueEnum<'ctx>, CursedError> {
        match expression {
            Expression::Integer(val) => {
                let int_type = self.context.i32_type();
                Ok(int_type.const_int(*val as u64, false).into())
            }
            Expression::Float(val) => {
                let float_type = self.context.f64_type();
                Ok(float_type.const_float(*val).into())
            }
            Expression::String(val) => {
                let string_value = self.builder.build_global_string_ptr(val, "str")
                    .map_err(|e| CursedError::CodegenError(format!("Failed to create string: {}", e)))?;
                Ok(string_value.as_pointer_value().into())
            }
            Expression::Boolean(val) => {
                let bool_type = self.context.bool_type();
                Ok(bool_type.const_int(if *val { 1 } else { 0 }, false).into())
            }
            Expression::Identifier(name) => {
                if let Some(alloca) = self.variables.get(name) {
                    let loaded_value = self.builder.build_load(*alloca, name)
                        .map_err(|e| CursedError::CodegenError(format!("Failed to load variable: {}", e)))?;
                    Ok(loaded_value)
                } else {
                    Err(CursedError::CodegenError(format!("Undefined variable: {}", name)))
                }
            }
            Expression::Binary(binary_expr) => {
                let left = self.compile_expression(&binary_expr.left)?;
                let right = self.compile_expression(&binary_expr.right)?;
                self.compile_binary_operation(left, &binary_expr.operator, right)
            }
            Expression::Call(call_expr) => {
                self.compile_function_call(&call_expr.function, &call_expr.arguments)
            }
            _ => {
                // Use inkwell generator for complex expressions
                self.inkwell_generator.compile_expression(expression)
            }
        }
    }
    
    /// Compile binary operations using proper LLVM instruction builders
    fn compile_binary_operation(
        &mut self,
        left: BasicValueEnum<'ctx>,
        op: &BinaryOperator,
        right: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, CursedError> {
        match (left, right) {
            (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => {
                let result = match op {
                    BinaryOperator::Add => self.builder.build_int_add(l, r, "add"),
                    BinaryOperator::Subtract => self.builder.build_int_sub(l, r, "sub"),
                    BinaryOperator::Multiply => self.builder.build_int_mul(l, r, "mul"),
                    BinaryOperator::Divide => self.builder.build_int_signed_div(l, r, "div"),
                    BinaryOperator::Equal => self.builder.build_int_compare(IntPredicate::EQ, l, r, "eq"),
                    BinaryOperator::NotEqual => self.builder.build_int_compare(IntPredicate::NE, l, r, "ne"),
                    BinaryOperator::LessThan => self.builder.build_int_compare(IntPredicate::SLT, l, r, "lt"),
                    BinaryOperator::LessThanOrEqual => self.builder.build_int_compare(IntPredicate::SLE, l, r, "le"),
                    BinaryOperator::GreaterThan => self.builder.build_int_compare(IntPredicate::SGT, l, r, "gt"),
                    BinaryOperator::GreaterThanOrEqual => self.builder.build_int_compare(IntPredicate::SGE, l, r, "ge"),
                    _ => return Err(CursedError::CodegenError(format!("Unsupported integer operation: {:?}", op))),
                };
                result.map(|v| v.into()).map_err(|e| CursedError::CodegenError(format!("Failed to build integer operation: {}", e)))
            }
            (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => {
                let result = match op {
                    BinaryOperator::Add => self.builder.build_float_add(l, r, "fadd"),
                    BinaryOperator::Subtract => self.builder.build_float_sub(l, r, "fsub"),
                    BinaryOperator::Multiply => self.builder.build_float_mul(l, r, "fmul"),
                    BinaryOperator::Divide => self.builder.build_float_div(l, r, "fdiv"),
                    BinaryOperator::Equal => self.builder.build_float_compare(FloatPredicate::OEQ, l, r, "feq"),
                    BinaryOperator::NotEqual => self.builder.build_float_compare(FloatPredicate::ONE, l, r, "fne"),
                    BinaryOperator::LessThan => self.builder.build_float_compare(FloatPredicate::OLT, l, r, "flt"),
                    BinaryOperator::LessThanOrEqual => self.builder.build_float_compare(FloatPredicate::OLE, l, r, "fle"),
                    BinaryOperator::GreaterThan => self.builder.build_float_compare(FloatPredicate::OGT, l, r, "fgt"),
                    BinaryOperator::GreaterThanOrEqual => self.builder.build_float_compare(FloatPredicate::OGE, l, r, "fge"),
                    _ => return Err(CursedError::CodegenError(format!("Unsupported float operation: {:?}", op))),
                };
                result.map(|v| v.into()).map_err(|e| CursedError::CodegenError(format!("Failed to build float operation: {}", e)))
            }
            _ => Err(CursedError::CodegenError("Type mismatch in binary operation".to_string())),
        }
    }
    
    /// Compile function calls using proper LLVM call instructions
    fn compile_function_call(
        &mut self,
        function_expr: &Expression,
        arguments: &[Expression],
    ) -> Result<BasicValueEnum<'ctx>, CursedError> {
        // Get function name
        let function_name = match function_expr {
            Expression::Identifier(name) => name,
            _ => return Err(CursedError::CodegenError("Invalid function call target".to_string())),
        };
        
        // Get function from registry
        let function = self.functions.get(function_name)
            .ok_or_else(|| CursedError::CodegenError(format!("Undefined function: {}", function_name)))?;
        
        // Compile arguments
        let mut arg_values = Vec::new();
        for arg in arguments {
            let arg_value = self.compile_expression(arg)?;
            arg_values.push(arg_value.into());
        }
        
        // Build call instruction
        let call_result = self.builder.build_call(*function, &arg_values, "call")
            .map_err(|e| CursedError::CodegenError(format!("Failed to build call: {}", e)))?;
        
        // Return result if function returns a value
        match call_result.try_as_basic_value() {
            inkwell::values::Either::Left(value) => Ok(value),
            inkwell::values::Either::Right(_) => {
                // Void function, return unit value
                let unit_type = self.context.i8_type();
                Ok(unit_type.const_int(0, false).into())
            }
        }
    }
    
    /// Compile variable declarations using proper LLVM alloca instructions
    fn compile_variable_declaration(&mut self, var_stmt: &crate::ast::VariableStatement) -> Result<(), CursedError> {
        // Compile initial value if present
        let initial_value = if let Some(init) = &var_stmt.initializer {
            Some(self.compile_expression(init)?)
        } else {
            None
        };
        
        // Determine type
        let var_type = if let Some(value) = &initial_value {
            value.get_type()
        } else if let Some(type_annotation) = &var_stmt.variable_type {
            self.convert_type_to_llvm(type_annotation)?
        } else {
            return Err(CursedError::CodegenError("Cannot determine variable type".to_string()));
        };
        
        // Create alloca
        let alloca = self.create_entry_alloca(&var_stmt.name, var_type)?;
        
        // Store initial value if present
        if let Some(value) = initial_value {
            self.builder.build_store(alloca, value)
                .map_err(|e| CursedError::CodegenError(format!("Failed to store initial value: {}", e)))?;
        }
        
        // Add to variable registry
        self.variables.insert(var_stmt.name.clone(), alloca);
        
        Ok(())
    }
    
    /// Compile return statements using proper LLVM return instructions
    fn compile_return_statement(&mut self, ret_stmt: &crate::ast::ReturnStatement) -> Result<(), CursedError> {
        if let Some(expr) = &ret_stmt.value {
            let return_value = self.compile_expression(expr)?;
            self.builder.build_return(Some(&return_value))
                .map_err(|e| CursedError::CodegenError(format!("Failed to build return: {}", e)))?;
        } else {
            self.builder.build_return(None)
                .map_err(|e| CursedError::CodegenError(format!("Failed to build void return: {}", e)))?;
        }
        Ok(())
    }
    
    /// Convert CURSED types to LLVM types
    fn convert_type_to_llvm(&self, cursed_type: &Type) -> Result<BasicTypeEnum<'ctx>, CursedError> {
        match cursed_type {
            Type::Integer => Ok(self.context.i32_type().into()),
            Type::Float => Ok(self.context.f64_type().into()),
            Type::Boolean => Ok(self.context.bool_type().into()),
            Type::String => Ok(self.context.i8_type().ptr_type(AddressSpace::default()).into()),
            Type::Character => Ok(self.context.i8_type().into()),
            Type::Void => Ok(self.context.void_type().into()),
            _ => Err(CursedError::CodegenError(format!("Unsupported type: {:?}", cursed_type))),
        }
    }
    
    /// Create an alloca in the entry block
    fn create_entry_alloca(&self, name: &str, var_type: BasicTypeEnum<'ctx>) -> Result<PointerValue<'ctx>, CursedError> {
        let current_function = self.current_function
            .ok_or_else(|| CursedError::CodegenError("No current function for alloca".to_string()))?;
        
        let entry_block = current_function.get_first_basic_block()
            .ok_or_else(|| CursedError::CodegenError("No entry block for alloca".to_string()))?;
        
        let builder = self.context.create_builder();
        builder.position_at_end(entry_block);
        
        builder.build_alloca(var_type, name)
            .map_err(|e| CursedError::CodegenError(format!("Failed to create alloca: {}", e)))
    }
    
    /// Check if current block has a terminator
    fn current_block_has_terminator(&self) -> bool {
        if let Some(block) = self.current_block {
            block.get_terminator().is_some()
        } else {
            false
        }
    }
    
    /// Get default value for a type
    fn get_default_value_for_type(&self, var_type: BasicTypeEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, CursedError> {
        match var_type {
            BasicTypeEnum::IntType(int_type) => Ok(int_type.const_int(0, false).into()),
            BasicTypeEnum::FloatType(float_type) => Ok(float_type.const_float(0.0).into()),
            BasicTypeEnum::PointerType(ptr_type) => Ok(ptr_type.const_null().into()),
            BasicTypeEnum::StructType(struct_type) => Ok(struct_type.const_zero().into()),
            BasicTypeEnum::VectorType(vec_type) => Ok(vec_type.const_zero().into()),
            BasicTypeEnum::ArrayType(array_type) => Ok(array_type.const_zero().into()),
        }
    }
    
    /// Get the generated LLVM module
    pub fn get_module(&self) -> &Module<'ctx> {
        &self.module
    }
    
    /// Print LLVM IR to string
    pub fn print_to_string(&self) -> String {
        self.module.print_to_string().to_string()
    }
}

/// Backward compatibility structure for migration
pub struct ImprovedLlvmCodeGeneratorWrapper {
    /// Internal state for compatibility
    pub ir_code: String,
}

impl ImprovedLlvmCodeGeneratorWrapper {
    /// Create new wrapper for compatibility with existing code
    pub fn new() -> Result<Self, CursedError> {
        Ok(Self {
            ir_code: String::new(),
        })
    }
    
    /// Compile using improved LLVM generator
    pub fn compile(&mut self, source: &str) -> Result<String, CursedError> {
        let context = Context::create();
        let mut generator = ImprovedLlvmCodeGenerator::new(&context, "cursed_module")?;
        
        // Parse source
        let mut parser = crate::parser::new_parser(source)?;
        let program = parser.parse_program()?;
        
        // Generate LLVM IR
        let ir = generator.compile_program(&program)?;
        self.ir_code = ir.clone();
        
        Ok(ir)
    }
    
    /// Get module for backward compatibility
    pub fn module(&self) -> crate::codegen::llvm::main::MockModule {
        crate::codegen::llvm::main::MockModule::new(self.ir_code.clone())
    }
}
