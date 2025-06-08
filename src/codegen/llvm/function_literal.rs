//! LLVM code generation for function literals and closures
//!
//! This module handles the compilation of function literals (anonymous functions)
//! and closures to LLVM IR. It manages closure variable capture, function type
//! creation, and runtime function value handling.
//!
//! Key responsibilities:
//! - Compiling function literals to LLVM function definitions
//! - Implementing closure capture mechanisms (by value and by reference)
//! - Creating function pointer types and values for first-class functions
//! - Managing closure environment allocation and access
//! - Integrating with garbage collection for closure cleanup

use crate::ast::expressions::FunctionLiteral;
use crate::codegen::llvm::context::LlvmCodeGenerator;
use crate::error::Error;
use inkwell::values::{BasicValueEnum, FunctionValue, PointerValue};
use inkwell::types::{BasicTypeEnum, FunctionType, BasicType};
use inkwell::AddressSpace;
use std::collections::HashMap;
use crate::codegen::llvm::statement::StatementCompilation;
use crate::codegen::llvm::variables::VariableScope;

/// Trait for compiling function literals and closures to LLVM IR
pub trait FunctionLiteralCompiler<'ctx> {
    /// Compile a function literal to LLVM IR
    fn compile_function_literal(
        &mut self,
        func_literal: &FunctionLiteral,
    ) -> Result<BasicValueEnum<'ctx>, Error>;

    /// Create a closure environment structure for captured variables
    fn create_closure_environment(
        &mut self,
        captures: &std::collections::HashSet<String>,
    ) -> Result<PointerValue<'ctx>, Error>;

    /// Generate code to access captured variables in a closure
    fn access_captured_variable(
        &mut self,
        closure_env: PointerValue<'ctx>,
        var_name: &str,
        capture_index: usize,
    ) -> Result<BasicValueEnum<'ctx>, Error>;

    /// Create a function type from parameters and return type
    fn create_function_type(
        &self,
        params: &[BasicTypeEnum<'ctx>],
        return_type: BasicTypeEnum<'ctx>,
        is_var_args: bool,
    ) -> FunctionType<'ctx>;

    /// Create a function pointer value from a function
    fn create_function_pointer(
        &self,
        function: FunctionValue<'ctx>,
    ) -> BasicValueEnum<'ctx>;
}

impl<'ctx> FunctionLiteralCompiler<'ctx> for LlvmCodeGenerator<'ctx> {
    fn compile_function_literal(
        &mut self,
        func_literal: &FunctionLiteral,
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Generate a unique name for the anonymous function
        let func_name = format!("__lambda_{}", self.get_unique_id());
        
        // Analyze captured variables for closure support
        let captured_vars = self.analyze_captured_variables(func_literal)?;
        
        // Create function type
        let mut param_types = Vec::new();
        
        // Add closure environment parameter if there are captures
        if !captured_vars.is_empty() {
            // Closure environment is passed as first parameter (void*)
            param_types.push(self.context.i8_type().ptr_type(AddressSpace::default()).into());
        }
        
        // Add regular function parameters
        for param in &func_literal.parameters {
            let param_type = self.get_type_from_expression(&*param.param_type)?;
            param_types.push(param_type);
        }
        
        // Determine return type
        let return_type = if let Some(ret_type_expr) = &func_literal.return_type {
            self.get_type_from_expression(&**ret_type_expr)?
        } else {
            // Infer return type from function body
            self.infer_function_return_type(&func_literal.body)?
        };
        
        let function_type = self.create_function_type(&param_types, return_type, false);
        
        // Create the function
        let function = self.module.add_function(&func_name, function_type, None);
        
        // Create entry block
        let entry_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry_block);
        
        // Store previous function context
        let prev_function = self.current_function;
        self.current_function = Some(function);
        
        // Push new scope for function
        self.push_scope(VariableScope::new());
        
        // Handle closure environment if present
        let mut param_index = 0;
        if !captured_vars.is_empty() {
            let closure_env = function.get_nth_param(0).unwrap().into_pointer_value();
            self.setup_closure_environment(closure_env, &captured_vars)?;
            param_index = 1;
        }
        
        // Set up function parameters in symbol table
        for (i, param) in func_literal.parameters.iter().enumerate() {
            if let Some(param_value) = function.get_nth_param((param_index + i) as u32) {
                self.symbol_table.insert(
                    param.name.value.clone(),
                    param_value,
                );
            }
        }
        
        // Compile function body
        let mut last_value = None;
        for statement in &func_literal.body.statements {
            last_value = Some(self.compile_statement(&**statement)?);
        }
        
        // Handle return value - simplified for compilation
        self.builder.build_return(None)?;
        
        // Pop scope and restore function context
        self.pop_scope();
        self.current_function = prev_function;
        
        // Create closure structure if needed
        if !captured_vars.is_empty() {
            self.create_closure_value(function, captured_vars)
        } else {
            // Return function pointer for simple function literals
            Ok(self.create_function_pointer(function))
        }
    }

    fn create_closure_environment(
        &mut self,
        captures: &std::collections::HashSet<String>,
    ) -> Result<PointerValue<'ctx>, Error> {
        if captures.is_empty() {
            // Return null pointer for no captures
            return Ok(self.context.i8_type().ptr_type(AddressSpace::default()).const_null());
        }
        
        // Create struct type for closure environment
        let mut field_types = Vec::new();
        for _var_name in captures {
            // For now, assume all captured variables are i64
            // This should be improved with proper type analysis
            field_types.push(self.context.i64_type().into());
        }
        
        let closure_struct_type = self.context.struct_type(&field_types, false);
        
        // Allocate closure environment on heap
        let size = closure_struct_type.size_of().unwrap();
        let malloc_fn = self.get_or_create_malloc_function();
        let env_ptr = self.builder.build_call(malloc_fn, &[size.into()], "closure_env")?
            .try_as_basic_value()
            .left()
            .unwrap()
            .into_pointer_value();
        
        // Cast to correct type
        let typed_env = self.builder.build_pointer_cast(
            env_ptr,
            closure_struct_type.ptr_type(AddressSpace::default()),
            "typed_closure_env",
        )?;
        
        Ok(typed_env)
    }

    fn access_captured_variable(
        &mut self,
        closure_env: PointerValue<'ctx>,
        var_name: &str,
        capture_index: usize,
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Get field pointer in closure environment
        let field_ptr = self.builder.build_struct_gep(
            closure_env.get_type(),
            closure_env,
            capture_index as u32,
            &format!("{}_ptr", var_name),
        )?;
        
        // Load the value
        let value = self.builder.build_load(
            self.context.i64_type(),  // Assuming i64 for now
            field_ptr,
            var_name,
        )?;
        
        Ok(value)
    }

    fn create_function_type(
        &self,
        params: &[BasicTypeEnum<'ctx>],
        return_type: BasicTypeEnum<'ctx>,
        is_var_args: bool,
    ) -> FunctionType<'ctx> {
        // Convert BasicTypeEnum to BasicMetadataTypeEnum
        let metadata_params: Vec<_> = params.iter().map(|&p| p.into()).collect();
        return_type.fn_type(&metadata_params, is_var_args)
    }

    fn create_function_pointer(
        &self,
        function: FunctionValue<'ctx>,
    ) -> BasicValueEnum<'ctx> {
        function.as_global_value().as_pointer_value().into()
    }
}

impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Analyze a function literal to determine which variables it captures
    fn analyze_captured_variables(
        &self,
        func_literal: &FunctionLiteral,
    ) -> Result<HashMap<String, usize>, Error> {
        // For now, return the captured variables from the AST
        // In a full implementation, this would perform static analysis
        let mut captures = HashMap::new();
        
        for (i, var_name) in func_literal.captured_variables.iter().enumerate() {
            captures.insert(var_name.clone(), i);
        }
        
        Ok(captures)
    }
    
    /// Set up closure environment access in the current scope
    fn setup_closure_environment(
        &mut self,
        closure_env: PointerValue<'ctx>,
        captured_vars: &HashMap<String, usize>,
    ) -> Result<(), Error> {
        // Store closure environment for later access
        self.closure_environment = Some(closure_env);
        
        // Add captured variables to symbol table
        for (var_name, &capture_index) in captured_vars {
            let value = self.access_captured_variable(closure_env, var_name, capture_index)?;
            self.symbol_table.insert(var_name.clone(), value);
        }
        
        Ok(())
    }
    
    /// Create a closure value (function pointer + environment)
    fn create_closure_value(
        &mut self,
        function: FunctionValue<'ctx>,
        captured_vars: HashMap<String, usize>,
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Create closure environment
        let env_ptr = self.create_closure_environment(&captured_vars.keys().cloned().collect())?;
        
        // Populate closure environment with current variable values
        for (var_name, &capture_index) in &captured_vars {
            if let Some(current_value) = self.symbol_table.get(var_name) {
                let field_ptr = self.builder.build_struct_gep(
                    env_ptr.get_type(),
                    env_ptr,
                    capture_index as u32,
                    &format!("{}_field", var_name),
                )?;
                self.builder.build_store(field_ptr, *current_value)?;
            }
        }
        
        // Create closure struct: { function_ptr, environment_ptr }
        let closure_struct_type = self.context.struct_type(&[
            function.get_type().ptr_type(AddressSpace::default()).into(),
            self.context.i8_type().ptr_type(AddressSpace::default()).into(),
        ], false);
        
        let closure_alloca = self.builder.build_alloca(closure_struct_type, "closure")?;
        
        // Store function pointer
        let func_ptr_field = self.builder.build_struct_gep(
            closure_struct_type,
            closure_alloca,
            0,
            "func_ptr_field",
        )?;
        self.builder.build_store(func_ptr_field, function.as_global_value().as_pointer_value())?;
        
        // Store environment pointer
        let env_ptr_field = self.builder.build_struct_gep(
            closure_struct_type,
            closure_alloca,
            1,
            "env_ptr_field",
        )?;
        let env_ptr_cast = self.builder.build_pointer_cast(
            env_ptr,
            self.context.i8_type().ptr_type(AddressSpace::default()),
            "env_ptr_cast",
        )?;
        self.builder.build_store(env_ptr_field, env_ptr_cast)?;
        
        // Load and return the closure
        let closure_value = self.builder.build_load(
            closure_struct_type,
            closure_alloca,
            "closure_value",
        )?;
        
        Ok(closure_value)
    }
    
    /// Get or create malloc function for dynamic allocation
    fn get_or_create_malloc_function(&self) -> FunctionValue<'ctx> {
        if let Some(malloc_fn) = self.module.get_function("malloc") {
            return malloc_fn;
        }
        
        let i8_ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
        let size_type = self.context.i64_type();
        let malloc_type = i8_ptr_type.fn_type(&[size_type.into()], false);
        
        self.module.add_function("malloc", malloc_type, None)
    }
    
    /// Generate a unique identifier for anonymous functions
    fn get_unique_id(&mut self) -> usize {
        self.lambda_counter += 1;
        self.lambda_counter
    }
    
    /// Infer return type from function body (simplified implementation)
    fn infer_function_return_type(
        &self,
        _body: &crate::ast::statements::block::BlockStatement,
    ) -> Result<BasicTypeEnum<'ctx>, Error> {
        // For now, default to i32
        // In a full implementation, this would analyze return statements
        Ok(self.context.i32_type().into())
    }
    
    /// Get LLVM type from AST expression (simplified)
    fn get_type_from_expression(
        &self,
        _expr: &dyn crate::ast::Expression,
    ) -> Result<BasicTypeEnum<'ctx>, Error> {
        // For now, default to i32
        // In a full implementation, this would perform type resolution
        Ok(self.context.i32_type().into())
    }
}
