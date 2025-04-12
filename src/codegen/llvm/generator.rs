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
use crate::codegen::MonomorphizationManager;
use crate::codegen::llvm::variables::VariableScope;

/// The LLVM code generator is responsible for generating LLVM IR from the AST
/// and providing JIT compilation capabilities.
pub struct LlvmCodeGenerator<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    file_path: PathBuf,
    /// Loop contexts for handling break and continue statements
    pub loop_contexts: Vec<LoopContext<'ctx>>,
    /// Manager for generic code specialization
    pub mono_manager: MonomorphizationManager,
    /// Variable scopes for managing variable declarations
    pub var_scopes: Vec<VariableScope<'ctx>>,
    /// Counter for generating unique string literal identifiers
    pub string_literal_count: usize,
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
            mono_manager: MonomorphizationManager::new(),
            var_scopes: Vec::new(),
            string_literal_count: 0,
        }
    }
    
    /// Compile the AST to LLVM IR.
    pub fn compile(&mut self, program: &Program) -> Result<(), String> {
        // Forward to compile_program
        self.compile_program(program)
    }
    
    /// Compile the program AST to LLVM IR.
    pub fn compile_program(&mut self, program: &Program) -> Result<(), String> {
        println!("DEBUG - compile_program: Starting compilation");
        
        // Initialize global scope
        if self.var_scopes.is_empty() {
            self.push_scope(super::variables::VariableScope::new());
        }
        
        // Register standard library functions (including puts)
        super::intrinsics::register_stdlib_functions(self.context, &self.module)?;
        
        // Create main function
        let i32_type = self.context.i32_type();
        let main_fn_type = i32_type.fn_type(&[], false);
        println!("DEBUG - compile_program: Creating main function");
        let main_function = self.module.add_function("main", main_fn_type, None);
        let entry_block = self.context.append_basic_block(main_function, "entry");
        
        // Position the builder at the end of the entry block
        self.builder.position_at_end(entry_block);
        println!("DEBUG - compile_program: Main function created, entry block positioned");
        
        // Iterate through all statements in the program and compile them
        println!("DEBUG - compile_program: Starting to compile {} statements", program.statements.len());
        for (i, statement) in program.statements.iter().enumerate() {
            println!("DEBUG - compile_program: Compiling statement {}/{}", i + 1, program.statements.len());
            match self.compile_statement(&**statement) {
                Ok(_) => println!("DEBUG - compile_program: Statement {} compiled successfully", i + 1),
                Err(e) => return Err(format!("Compilation error: {}", e)),
            }
        }
        
        // Add a default return 0 for main
        println!("DEBUG - compile_program: Adding default return 0 for main");
        self.builder.build_return(Some(&i32_type.const_int(0, false))).unwrap();
        
        // Verify that the main function exists
        if let Some(main_fn) = self.module.get_function("main") {
            println!("DEBUG - compile_program: Main function found: {}", main_fn.get_name().to_string_lossy());
        } else {
            println!("DEBUG - compile_program: WARNING - Main function NOT found after compilation!");
        }
        
        println!("DEBUG - compile_program: Compilation completed successfully");
        Ok(())
    }
    
    /// Get a reference to the LLVM module.
    pub fn module(&self) -> &Module<'ctx> {
        &self.module
    }
    
    /// Generate LLVM IR for a function statement
    pub fn generate_function(&mut self, function: &crate::ast::FunctionStatement) -> Result<inkwell::values::FunctionValue<'ctx>, Error> {
        // For now, we'll create a basic stub function with the correct name
        let name = &function.name.value;
        let return_type = self.context.i32_type();
        let param_types = vec![self.context.i32_type().into(); function.parameters.len()];
        
        let fn_type = self.context.i32_type().fn_type(&param_types, false);
        let function_value = self.module.add_function(name, fn_type, None);
        
        // Create entry basic block
        let entry = self.context.append_basic_block(function_value, "entry");
        self.builder.position_at_end(entry);
        
        // Add a simple return statement (to be replaced with actual compiled code)
        let return_value = self.context.i32_type().const_int(0, false);
        self.builder.build_return(Some(&return_value));
        
        Ok(function_value)
    }
    
    /// Generate LLVM IR for a struct statement
    pub fn generate_struct(&mut self, squad_stmt: &crate::ast::SquadStatement) -> Result<(), Error> {
        // Create a basic struct type
        let struct_name = &squad_stmt.name.value;
        let field_types = squad_stmt.fields.iter()
            .map(|_| self.context.i32_type().into())
            .collect::<Vec<_>>();
        
        let struct_type = self.context.struct_type(&field_types, false);
        
        // Since we're in generator.rs, don't have direct access to register_struct_type
        // In a real implementation, we'd register this type properly
        
        Ok(())
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
    
    /// Look up a generic function by name
    /// 
    /// This function should be implemented to look up the AST for a generic function
    /// in the compiler's symbol table or other storage mechanism.
    pub fn lookup_generic_function(&self, name: &str) -> Option<crate::ast::FunctionStatement> {
        // In a real implementation, this would look up the function in a symbol table
        // For testing purposes, we'll return None to simulate the function not being found
        None
    }
    
    /// Get the size of a type in bytes as an LLVM value
    /// 
    /// Note: This is a stub implementation for testing purposes only.
    /// The actual implementation depends on the specific inkwell API.
    pub fn get_type_size(&self, ty: &inkwell::types::BasicTypeEnum<'ctx>) -> inkwell::values::IntValue<'ctx> {
        // Since we can't directly get the size from DataLayout in the current inkwell version,
        // we'll return a hardcoded size based on the type kind for testing
        let size_bytes = match *ty {
            inkwell::types::BasicTypeEnum::ArrayType(_) => 16,
            inkwell::types::BasicTypeEnum::FloatType(_) => 4,
            inkwell::types::BasicTypeEnum::IntType(_) => 4,
            inkwell::types::BasicTypeEnum::PointerType(_) => 8,
            inkwell::types::BasicTypeEnum::StructType(_) => 32,
            inkwell::types::BasicTypeEnum::VectorType(_) => 16,
        };
        
        // Return the size as an LLVM i64 value
        self.context.i64_type().const_int(size_bytes, false)
    }
}

