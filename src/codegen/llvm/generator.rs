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
    /// Loop contexts for handling break and continue statements
    pub loop_contexts: Vec<LoopContext<'ctx>>,
}

/// Represents a loop context for handling break and continue statements
#[derive(Clone)]
pub struct LoopContext<'ctx> {
    /// The name of the loop (for debugging)
    pub name: String,
    /// The continuation block (where to jump for continue statements)
    pub continue_block: inkwell::basic_block::BasicBlock<'ctx>,
    /// The exit block (where to jump for break statements)
    pub break_block: inkwell::basic_block::BasicBlock<'ctx>,
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
            loop_contexts: Vec::new(),
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
    
    /// Get a reference to the context.
    pub fn context(&self) -> &'ctx Context {
        self.context
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
    
    /// Push a new loop context onto the stack.
    /// This is used to handle break and continue statements within loops.
    pub fn push_loop_context(
        &mut self,
        name: &str,
    ) -> Result<(), Error> {
        // Create blocks for continue and break
        let current_fn = self.builder().get_insert_block()
            .ok_or_else(|| Error::codegen("No current block".to_string()))?
            .get_parent()
            .ok_or_else(|| Error::codegen("No parent function".to_string()))?;
        
        let continue_block = self.context().append_basic_block(current_fn, &format!("{}.continue", name));
        let break_block = self.context().append_basic_block(current_fn, &format!("{}.break", name));
        
        // Push the new context
        self.loop_contexts.push(LoopContext {
            name: name.to_string(),
            continue_block,
            break_block,
        });
        
        Ok(())
    }
    
    /// Pop the current loop context from the stack.
    pub fn pop_loop_context(&mut self) -> Option<LoopContext<'ctx>> {
        self.loop_contexts.pop()
    }
    
    /// Get the current loop context, if any.
    pub fn current_loop_context(&self) -> Option<&LoopContext<'ctx>> {
        self.loop_contexts.last()
    }
}