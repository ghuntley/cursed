//! LLVM Code Generator Context
//! Contains the main LlvmCodeGenerator struct and its core functionality

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

// LLVM 17 compatible imports
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{BasicValueEnum, FunctionValue, PointerValue};
use inkwell::types::{BasicType, BasicTypeEnum};
use inkwell::basic_block::BasicBlock;

use crate::ast::base::Program;
use super::types::*;
use super::errors::*;

/// Manages the state for LLVM Intermediate Representation generation.
pub struct LlvmCodeGenerator<'ctx> {
    pub(crate) context: &'ctx Context,
    pub(crate) module: Module<'ctx>,
    pub(crate) builder: Builder<'ctx>,
    pub(crate) variables: HashMap<String, (PointerValue<'ctx>, BasicTypeEnum<'ctx>)>, 
    pub(crate) current_function: Option<FunctionValue<'ctx>>,
    pub(crate) functions: HashMap<String, FunctionValue<'ctx>>, 
    pub(crate) current_package_name: String,
    pub(crate) imported_packages: HashMap<String, ImportedPackageInfo<'ctx>>,
    pub(crate) current_file_path: PathBuf,
    // Struct types mapping: package name -> struct name -> LLVM struct type
    pub(crate) struct_types: HashMap<String, HashMap<String, inkwell::types::StructType<'ctx>>>,
    // Loop control flow tracking
    pub(crate) loop_exit_blocks: Vec<BasicBlock<'ctx>>,
}

impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Creates a new LlvmCodeGenerator instance.
    pub fn new(context: &'ctx Context, module_name: &str, initial_file_path: PathBuf) -> Self {
        let module = context.create_module(module_name);
        let builder = context.create_builder();
        let current_package_name = module_name.to_string(); 

        LlvmCodeGenerator {
            context,
            module,
            builder,
            variables: HashMap::new(),
            current_function: None,
            functions: HashMap::new(),
            current_package_name,
            imported_packages: HashMap::new(),
            current_file_path: initial_file_path,
            struct_types: HashMap::new(),
            loop_exit_blocks: Vec::new(),
        }
    }
    
    /// Mangles a symbol name with its package name according to `_<package>_<symbol>`.
    pub fn mangle_name(&self, package_name: &str, symbol_name: &str) -> String {
        format!("_{}_{}", package_name, symbol_name)
    }
    
    /// Helper to create an alloca instruction in the entry block of the current function.
    /// Allocas should typically be grouped in the entry block for optimal SSA form via mem2reg.
    pub fn create_entry_block_alloca<T: BasicType<'ctx>>(
        &self,
        llvm_type: T,
        name: &str,
    ) -> PointerValue<'ctx> {
        // Create a temporary builder positioned at the beginning of the entry block
        let builder = self.context.create_builder();
        let entry_block = self.current_function.unwrap().get_first_basic_block().unwrap();

        match entry_block.get_first_instruction() {
            Some(first_instr) => builder.position_before(&first_instr),
            None => builder.position_at_end(entry_block),
        }

        builder.build_alloca(llvm_type, name).unwrap()
    }
    
    /// Compiles the program into LLVM IR.
    pub fn compile(&mut self, program: &Program) -> Result<(), String> {
        self.compile_program(program)
    }
    
    /// Alias for compile to maintain backward compatibility
    pub fn compile_program(&mut self, program: &Program) -> Result<(), String> {
        // Initialize string helpers using the implementation from string.rs
        // Use self instead of trying to access through the module
        self.init_string_helpers();
        
        // Create a main function (assuming top-level code runs in main for now)
        let i32_type = self.context.i32_type();
        let main_fn_type = i32_type.fn_type(&[], false);
        let main_function = self.module.add_function("main", main_fn_type, None);
        let entry_block = self.context.append_basic_block(main_function, "entry");

        // Set current function context and position builder
        self.current_function = Some(main_function);
        self.builder.position_at_end(entry_block);
        self.variables.clear(); // Clear variables for the new function scope

        // Flag to track if a return statement has been added
        let mut has_return = false;

        // Compile all statements in the program
        for stmt in &program.statements {
            match stmt.as_any().downcast_ref::<crate::ast::statements::declarations::ReturnStatement>() {
                Some(_) => has_return = true,
                None => {}
            }
            self.compile_statement(stmt.as_ref())?;
        }

        // Add a default return 0 for main if no return statement was added
        if !has_return && self.builder.get_insert_block().is_some() {
            self.builder.build_return(Some(&i32_type.const_int(0, false))).unwrap();
        } else if !has_return {
            // This case might happen if the program is empty or control flow is complex.
            // Let's re-position to the last block if no block is set.
            if let Some(last_block) = main_function.get_last_basic_block() {
                self.builder.position_at_end(last_block);
                // Check if the block is already terminated
                if last_block.get_terminator().is_none() {
                    self.builder.build_return(Some(&i32_type.const_int(0, false))).unwrap();
                }
            } else {
                // Should not happen if entry block was created
                return Err("Main function has no basic blocks!".to_string());
            }
        }

        // Clear current function context
        self.current_function = None;

        // Optional: Verify the generated module
        if let Err(err) = self.module.verify() {
            return Err(format!("LLVM module verification failed: {}\n{}", err.to_string(), self.module.print_to_string()));
        }

        Ok(())
    }
    
    // Add getter methods for the private fields
    
    /// Get a reference to the current LLVM module
    pub fn module(&self) -> &Module<'ctx> {
        &self.module
    }
    
    /// Get a reference to the LLVM context
    pub fn context(&self) -> &'ctx Context {
        self.context
    }
    
    /// Get a reference to the LLVM builder
    pub fn builder(&self) -> &Builder<'ctx> {
        &self.builder
    }
    
    /// Get a mutable reference to the LLVM builder
    pub fn builder_mut(&mut self) -> &mut Builder<'ctx> {
        &mut self.builder
    }
    
    /// Get the current package name
    pub fn current_package_name(&self) -> &str {
        &self.current_package_name
    }
    
    /// Get the current file path
    pub fn current_file_path(&self) -> &Path {
        &self.current_file_path
    }
    
    /// Check if we're currently in a function context
    pub fn in_function(&self) -> bool {
        self.current_function.is_some()
    }
    
    /// Get the current function, if any
    pub fn current_function(&self) -> Option<FunctionValue<'ctx>> {
        self.current_function
    }
    
    // Initialize the string helper functions
    // This is now implemented in string.rs
    // pub fn init_string_helpers(&mut self) {
    //     // Implement string helpers initialization logic
    //     // This would be moved from the original file
    // }
    
    // Getter for module (used in tests)
    pub fn get_module(&self) -> &Module<'ctx> {
        &self.module
    }
}

// Import of other module implementations is done in respective files