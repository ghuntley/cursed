//! LLVM code generator for the Cursed compiler

use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::BasicValueEnum;
use inkwell::types::BasicType;
use inkwell::OptimizationLevel;
use std::path::PathBuf;
use crate::error::Error;
use crate::ast::base::Program;
use crate::ast::{Node, Statement, Expression};

/// The LLVM code generator is responsible for generating LLVM IR from the AST
/// and providing JIT compilation capabilities.
pub struct LlvmCodeGenerator<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    file_path: PathBuf,
}

impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Create a new LLVM code generator.
    pub fn new(context: &'ctx Context, module_name: &str, file_path: PathBuf) -> Self {
        let module = context.create_module(module_name);
        let builder = context.create_builder();
        
        LlvmCodeGenerator {
            context,
            module,
            builder,
            file_path,
        }
    }
    
    /// Compile the AST to LLVM IR.
    pub fn compile(&mut self, program: &Program) -> Result<(), String> {
        // Forward to compile_program
        self.compile_program(program)
    }
    
    /// Compile the program AST to LLVM IR.
    pub fn compile_program(&mut self, program: &Program) -> Result<(), String> {
        // Initialize string helpers
        // Define any required global string helpers here if needed
        
        // Create main function
        let i32_type = self.context.i32_type();
        let main_fn_type = i32_type.fn_type(&[], false);
        let main_function = self.module.add_function("main", main_fn_type, None);
        let entry_block = self.context.append_basic_block(main_function, "entry");
        
        // Position the builder at the end of the entry block
        self.builder.position_at_end(entry_block);
        
        // Add a default return 0 for main
        self.builder.build_return(Some(&i32_type.const_int(0, false))).unwrap();
        
        Ok(())
    }
    
    /// Get a reference to the LLVM module.
    pub fn module(&self) -> &Module<'ctx> {
        &self.module
    }
    
    /// Get a mutable reference to the LLVM module.
    pub fn module_mut(&mut self) -> &mut Module<'ctx> {
        &mut self.module
    }
    
    /// Get a reference to the builder.
    pub fn builder(&self) -> &Builder<'ctx> {
        &self.builder
    }
    
    /// Get a mutable reference to the builder.
    pub fn builder_mut(&mut self) -> &mut Builder<'ctx> {
        &mut self.builder
    }
    
    /// Get a reference to the module - alias for module() for compatibility.
    pub fn get_module(&self) -> &Module<'ctx> {
        self.module()
    }
    
    /// Create a function in the module.
    pub fn create_function(
        &self,
        name: &str,
        param_types: &[inkwell::types::BasicMetadataTypeEnum<'ctx>],
        return_type: inkwell::types::BasicTypeEnum<'ctx>,
        variadic: bool
    ) -> inkwell::values::FunctionValue<'ctx> {
        let fn_type = return_type.fn_type(param_types, variadic);
        self.module.add_function(name, fn_type, None)
    }
}