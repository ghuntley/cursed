//! JIT compilation and execution for CURSED language

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use inkwell::execution_engine::ExecutionEngine;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::OptimizationLevel;

use crate::codegen::llvm::LlvmCodeGenerator;
use crate::ast::Program;
use crate::error::Error;

/// Structure to manage JIT compilation and execution
pub struct JitCompiler<'ctx> {
    context: &'ctx Context,
    execution_engine: ExecutionEngine<'ctx>,
    code_generator: LlvmCodeGenerator<'ctx>,
}

impl<'ctx> JitCompiler<'ctx> {
    /// Create a new JIT compiler
    pub fn new(context: &'ctx Context, execution_engine: ExecutionEngine<'ctx>, package_name: &str, file_path: std::path::PathBuf) -> Self {
        let code_generator = LlvmCodeGenerator::new(context, package_name, file_path);
        
        JitCompiler {
            context,
            execution_engine,
            code_generator,
        }
    }
    
    /// Compile a program using the JIT compiler
    pub fn compile(&mut self, program: &Program) -> Result<(), Error> {
        // Compile the program to LLVM IR
        self.code_generator.compile_program(program)?;
        Ok(())
    }
    
    /// Execute the compiled program
    pub fn execute(&self) -> Result<i32, Error> {
        // Find and execute the main function
        unsafe {
            match self.execution_engine.get_function::<unsafe extern "C" fn() -> i32>("main") {
                Ok(main_fn) => {
                    let result = main_fn.call();
                    Ok(result)
                },
                Err(e) => Err(Error::from_str(&format!("Function 'main' not found: {}", e)))
            }
        }
    }
    
    /// Run a function by name
    pub fn run_function(&self, function_name: &str) -> Result<(), Error> {
        // Find and execute the specified function
        unsafe {
            match self.execution_engine.get_function::<unsafe extern "C" fn()>(function_name) {
                Ok(function) => {
                    function.call();
                    Ok(())
                },
                Err(e) => Err(Error::from_str(&format!("Function '{}' not found: {}", function_name, e)))
            }
        }
    }
    
    /// Get a reference to the execution engine
    pub fn get_execution_engine(&self) -> &ExecutionEngine<'ctx> {
        &self.execution_engine
    }
    
    /// Get the LLVM module
    pub fn module(&self) -> &Module<'ctx> {
        self.code_generator.module()
    }
    
    /// Get the code generator
    pub fn code_generator(&self) -> &LlvmCodeGenerator<'ctx> {
        &self.code_generator
    }
    
    /// Get a mutable reference to the code generator
    pub fn code_generator_mut(&mut self) -> &mut LlvmCodeGenerator<'ctx> {
        &mut self.code_generator
    }
}

/// Structure for managing goroutines in the JIT environment
pub struct GoroutineManager {
    goroutines: Arc<Mutex<Vec<thread::JoinHandle<()>>>>,
}

impl GoroutineManager {
    /// Create a new goroutine manager
    pub fn new() -> Self {
        GoroutineManager {
            goroutines: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    /// Launch a new goroutine
    pub fn launch<F>(&self, function: F) 
    where 
        F: FnOnce() + Send + 'static
    {
        let goroutines = Arc::clone(&self.goroutines);
        
        // Launch a new thread for the goroutine
        let handle = thread::spawn(move || {
            function();
            
            // Remove the handle when done
            let mut goroutines = goroutines.lock().unwrap();
            let thread_id = thread::current().id();
            goroutines.retain(|handle| handle.thread().id() != thread_id);
        });
        
        // Store the handle
        let mut goroutines = self.goroutines.lock().unwrap();
        goroutines.push(handle);
    }
    
    /// Wait for all goroutines to complete with a timeout
    pub fn wait_all(&self, timeout_ms: u64) -> usize {
        let mut remaining = 0;
        
        // Try joining with timeout
        {
            let mut goroutines = self.goroutines.lock().unwrap();
            let mut i = 0;
            
            while i < goroutines.len() {
                // Check if the thread has completed
                if goroutines[i].is_finished() {
                    // Thread is done, remove it
                    let _ = goroutines.remove(i).join();
                } else {
                    i += 1;
                }
            }
            
            remaining = goroutines.len();
        }
        
        // Wait a bit to give threads some time to finish
        if remaining > 0 {
            thread::sleep(Duration::from_millis(timeout_ms));
            
            // Check again
            let goroutines = self.goroutines.lock().unwrap();
            remaining = goroutines.len();
        }
        
        remaining
    }
}

/// Global goroutine manager for JIT execution
static mut GLOBAL_GOROUTINE_MANAGER: Option<GoroutineManager> = None;

/// Register external functions with the JIT execution engine
pub fn register_external_functions<'ctx>(context: &'ctx Context, module: &Module<'ctx>) -> Result<(), Error> {
    // Add function signatures for the runtime library functions
    
    // launch_goroutine function
    let void_type = context.void_type();
    let i8_ptr_type = context.i8_type().ptr_type(inkwell::AddressSpace::default());
    let fn_type = void_type.fn_type(&[i8_ptr_type.into()], false);
    let _ = module.add_function("launch_goroutine", fn_type, None);
    
    // Create channel function
    let i8_ptr_type = context.i8_type().ptr_type(inkwell::AddressSpace::default());
    let fn_type = i8_ptr_type.fn_type(&[i8_ptr_type.into()], false);
    let _ = module.add_function("create_channel", fn_type, None);
    
    // Send to channel function
    let i32_type = context.i32_type();
    let i8_ptr_type = context.i8_type().ptr_type(inkwell::AddressSpace::default());
    let fn_type = i32_type.fn_type(&[i8_ptr_type.into(), i8_ptr_type.into()], false);
    let _ = module.add_function("send_to_channel", fn_type, None);
    
    // Receive from channel function
    let i8_ptr_type = context.i8_type().ptr_type(inkwell::AddressSpace::default());
    let fn_type = i8_ptr_type.fn_type(&[i8_ptr_type.into()], false);
    let _ = module.add_function("receive_from_channel", fn_type, None);
    
    Ok(())
}

/// Initialize the global goroutine manager
pub fn init_goroutine_manager() {
    unsafe {
        GLOBAL_GOROUTINE_MANAGER = Some(GoroutineManager::new());
    }
}

/// Get a reference to the global goroutine manager
pub fn get_goroutine_manager() -> Option<&'static GoroutineManager> {
    unsafe {
        GLOBAL_GOROUTINE_MANAGER.as_ref()
    }
}

/// Launch a goroutine using the global manager
#[no_mangle]
pub extern "C" fn launch_goroutine(function_ptr: extern "C" fn()) {
    // Initialize manager if not already done
    if get_goroutine_manager().is_none() {
        init_goroutine_manager();
    }
    
    if let Some(manager) = get_goroutine_manager() {
        println!("Launching goroutine");
        manager.launch(move || {
            function_ptr();
        });
    } else {
        eprintln!("Failed to get goroutine manager");
    }
}

/// Wait for all goroutines to complete with a timeout
#[no_mangle]
pub extern "C" fn wait_for_goroutines(timeout_ms: u64) -> usize {
    if let Some(manager) = get_goroutine_manager() {
        manager.wait_all(timeout_ms)
    } else {
        0
    }
}