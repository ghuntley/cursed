use crate::error::CursedError;
use crate::minimal_ast::*;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::*;
use inkwell::targets::*;
use inkwell::types::*;
use inkwell::OptimizationLevel;
use std::collections::HashMap;

pub struct MinimalCodegen<'ctx> {
impl<'ctx> MinimalCodegen<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str) -> Self {
        let module = context.create_module(module_name);
        let builder = context.create_builder();
        
        Self {
        }
    }
    
    pub fn compile_program(&mut self, program: &Program) -> crate::error::Result<()> {
        // Create a main function
        let i32_type = self.context.i32_type();
        let fn_type = i32_type.fn_type(&[], false);
        let function = self.module.add_function("main", fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");
        
        self.builder.position_at_end(basic_block);
        
        // Compile all statements
        for statement in &program.statements {
            self.compile_statement(statement)?;
        // Return 0 from main
        let zero = i32_type.const_int(0, false);
        self.builder.build_return(Some(&zero)).map_err(|e| {
            CursedError::Compile(format!("Failed to build return: {}", e))
        })?;
        
        Ok(())
    fn compile_statement(&mut self, statement: &Statement) -> crate::error::Result<()> {
        match statement {
            Statement::Facts(name, expr) => {
                let value = self.compile_expression(expr)?;
                
                // For now, just create a global variable
                let global = self.module.add_global(value.get_type(), None, name);
                global.set_initializer(&value);
                
                Ok(())
            }
            Statement::Slay(name, _params, body) => {
                // Create a function - if name is "main", make it return i32, otherwise void
                let (fn_type, returns_value) = if name == "main" {
                    let i32_type = self.context.i32_type();
                    (i32_type.fn_type(&[], false), true)
                } else {
                    let void_type = self.context.void_type();
                    (void_type.fn_type(&[], false), false)
                
                let function = self.module.add_function(name, fn_type, None);
                let basic_block = self.context.append_basic_block(function, "entry");
                
                let old_insertion_point = self.builder.get_insert_block();
                self.builder.position_at_end(basic_block);
                
                let mut last_value = None;
                
                // Compile function body
                for stmt in body {
                    match stmt {
                        Statement::Expression(expr) => {
                            last_value = Some(self.compile_expression(expr)?);
                        }
                        Statement::Facts(var_name, expr) => {
                            // Create local variable
                            let value = self.compile_expression(expr)?;
                            let alloca = self.builder.build_alloca(value.get_type(), var_name).map_err(|e| {
                                CursedError::Compile(format!("Failed to create alloca: {}", e))
                            })?;
                            self.builder.build_store(alloca, value).map_err(|e| {
                                CursedError::Compile(format!("Failed to store value: {}", e))
                            })?;
                            self.variables.insert(var_name.clone(), alloca);
                        }
                        _ => {
                            self.compile_statement(stmt)?;
                        }
                    }
                // Build return
                if returns_value && name == "main" {
                    // For main function, return 0
                    let zero = self.context.i32_type().const_int(0, false);
                    self.builder.build_return(Some(&zero)).map_err(|e| {
                        CursedError::Compile(format!("Failed to build return: {}", e))
                    })?;
                } else {
                    self.builder.build_return(None).map_err(|e| {
                        CursedError::Compile(format!("Failed to build return: {}", e))
                    })?;
                // Restore insertion point
                if let Some(block) = old_insertion_point {
                    self.builder.position_at_end(block);
                Ok(())
            }
            Statement::Expression(expr) => {
                self.compile_expression(expr)?;
                Ok(())
            }
        }
    fn compile_expression(&mut self, expression: &Expression) -> crate::error::Result<BasicValueEnum<'ctx>> {
        match expression {
            Expression::String(s) => {
                let string_value = self.context.const_string(s.as_bytes(), false);
                Ok(string_value.into())
            }
            Expression::Integer(i) => {
                let int_value = self.context.i64_type().const_int(*i as u64, false);
                Ok(int_value.into())
            }
            Expression::Boolean(b) => {
                let bool_value = self.context.bool_type().const_int(if *b { 1 } else { 0 }, false);
                Ok(bool_value.into())
            }
            Expression::Identifier(name) => {
                // For now, just check if we have the variable and return the global value
                if let Some(global) = self.module.get_global(name) {
                    // Return the global value directly (it's a constant initializer)
                    Ok(global.get_initializer().unwrap())
                } else {
                    // Return a placeholder if variable not found
                    let int_value = self.context.i64_type().const_int(42, false);
                    Ok(int_value.into())
                }
            }
            Expression::FunctionCall(name, _args) => {
                // For now, just return a placeholder
                let int_value = self.context.i64_type().const_int(0, false);
                Ok(int_value.into())
            }
        }
    pub fn emit_llvm_ir(&self) -> String {
        self.module.print_to_string().to_string()
    pub fn write_to_file(&self, filename: &str) -> crate::error::Result<()> {
        self.module.print_to_file(filename).map_err(|e| {
            CursedError::Compile(format!("Failed to write LLVM IR to file: {}", e))
        })
    pub fn compile_to_object_file(&self, target_triple: &str, filename: &str) -> crate::error::Result<()> {
        Target::initialize_all(&InitializationConfig::default());
        
        let target = Target::from_triple(&TargetTriple::create(target_triple))
            .map_err(|e| CursedError::Compile(format!("Failed to create target: {}", e)))?;
            
        let target_machine = target.create_target_machine(
        ).ok_or_else(|| CursedError::Compile("Failed to create target machine".to_string()))?;
        
        target_machine.write_to_file(&self.module, FileType::Object, filename.as_ref())
            .map_err(|e| CursedError::Compile(format!("Failed to write object file: {}", e)))
    }
}

pub fn compile_cursed_to_llvm(program: &Program, module_name: &str) -> crate::error::Result<String> {
    let context = Context::create();
    let mut codegen = MinimalCodegen::new(&context, module_name);
    
    codegen.compile_program(program)?;
    Ok(codegen.emit_llvm_ir())
pub fn compile_cursed_to_object(program: &Program, module_name: &str, output_file: &str) -> crate::error::Result<()> {
    let context = Context::create();
    let mut codegen = MinimalCodegen::new(&context, module_name);
    
    codegen.compile_program(program)?;
    
    // Get the target triple for the current platform
    let target_triple = TargetMachine::get_default_triple();
    codegen.compile_to_object_file(target_triple.as_str().to_str().unwrap(), output_file)
pub fn compile_cursed_to_executable(program: &Program, module_name: &str, output_file: &str) -> crate::error::Result<()> {
    let context = Context::create();
    let mut codegen = MinimalCodegen::new(&context, module_name);
    
    codegen.compile_program(program)?;
    
    // Get the target triple for the current platform
    let target_triple = TargetMachine::get_default_triple();
    
    // First create an object file
    let temp_obj = format!("{}.o", output_file);
    codegen.compile_to_object_file(target_triple.as_str().to_str().unwrap(), &temp_obj)?;
    
    // Then link it into an executable using gcc
    let link_result = std::process::Command::new("gcc")
        .arg(&temp_obj)
        .arg("-o")
        .arg(output_file)
        .output();
    
    // Clean up temporary object file
    let _ = std::fs::remove_file(&temp_obj);
    
    match link_result {
        Ok(output) => {
            if output.status.success() {
                Ok(())
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                Err(CursedError::Compile(format!("Linking failed: {}", stderr)))
            }
        }
        Err(e) => Err(CursedError::Compile(format!("Failed to run linker: {}", e)))
    }
}
