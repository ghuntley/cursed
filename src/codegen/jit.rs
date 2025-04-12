//! Just-In-Time (JIT) compilation for CURSED programs.
//!
//! This module provides JIT compilation capabilities for the CURSED language,
//! allowing code to be executed immediately after compilation without generating
//! intermediate object files or executables. It leverages LLVM's execution engine
//! to compile and run CURSED programs within the same process.
//!
//! Key features include:
//! - Dynamic compilation of CURSED programs to machine code
//! - Immediate execution of compiled programs
//! - Goroutine management for concurrent execution
//! - Integration with external runtime functions
//!
//! The JIT compiler is particularly useful for:
//! - The CURSED REPL (read-eval-print loop)
//! - Running scripts without a separate compilation step
//! - Testing and debugging during development
//! - Performance testing of small code snippets

use crate::error::Error;
use inkwell::context::Context;
use inkwell::execution_engine::ExecutionEngine;
use inkwell::module::Module;
use inkwell::values::FunctionValue;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::cell::RefCell;
use crate::codegen::llvm::LlvmCodeGenerator;

// Goroutine management
static ACTIVE_GOROUTINES: AtomicUsize = AtomicUsize::new(0);

/// Initializes the goroutine management system
///
/// This function resets the active goroutine counter to zero, preparing
/// the system for a new JIT execution. Call this before starting a new
/// CURSED program execution.
pub fn init_goroutine_manager() {
    // Reset the active goroutines counter
    ACTIVE_GOROUTINES.store(0, Ordering::SeqCst);
}

/// Registers a new active goroutine in the runtime system.
///
/// This function increments the global counter of active goroutines atomically.
/// It should be called whenever a new goroutine is started, typically at the
/// beginning of a goroutine function.
///
/// The active goroutine count is used by `wait_for_goroutines` to determine
/// when all concurrent work has completed.
pub fn register_goroutine() {
    ACTIVE_GOROUTINES.fetch_add(1, Ordering::SeqCst);
}

/// Marks a goroutine as completed in the runtime system.
///
/// This function decrements the global counter of active goroutines atomically.
/// It should be called whenever a goroutine completes its execution, typically
/// at the end of a goroutine function or when the goroutine terminates early.
///
/// Paired with `register_goroutine`, this function allows the runtime to track
/// the number of goroutines currently executing.
pub fn finish_goroutine() {
    ACTIVE_GOROUTINES.fetch_sub(1, Ordering::SeqCst);
}

/// Waits for all goroutines to complete with a specified timeout.
///
/// This function blocks until either all goroutines have completed (the active count
/// reaches zero) or the specified timeout duration has elapsed. It's useful for
/// ensuring concurrent work is finished before proceeding with sequential operations
/// or program termination.
///
/// The implementation uses a polling approach, checking the active goroutine count
/// periodically and sleeping between checks to avoid excessive CPU usage.
///
/// # Arguments
///
/// * `timeout_ms` - Maximum time to wait in milliseconds
///
/// # Returns
///
/// The number of goroutines still running when the function returns. A return value
/// of zero indicates that all goroutines completed successfully.
pub fn wait_for_goroutines(timeout_ms: u64) -> usize {
    // Simple implementation with timeout
    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_millis(timeout_ms);
    
    loop {
        let count = ACTIVE_GOROUTINES.load(Ordering::SeqCst);
        if count == 0 || start.elapsed() > timeout {
            return count;
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

/// Registers external runtime functions with the LLVM execution engine.
///
/// This function sets up the bindings between functions declared in LLVM IR and
/// their actual implementations in the runtime library. It establishes the connection
/// between compiled CURSED code and the native functions that provide runtime services.
///
/// External functions typically include:
/// - Memory management functions (allocation, collection)
/// - Channel and goroutine operations
/// - I/O operations
/// - String manipulation
///
/// # Arguments
///
/// * `_context` - The LLVM context
/// * `_module` - The LLVM module containing the function declarations
///
/// # Returns
///
/// Result<(), Error> - Success or an error if registration fails
pub fn register_external_functions(
    _context: &Context,
    _module: &Module,
) -> Result<(), Error> {
    // For now, we'll just return success
    Ok(())
}

/// Just-In-Time compiler for executing CURSED programs
///
/// JitCompiler manages the dynamic compilation and immediate execution of
/// CURSED code using LLVM's execution engine. It works with LlvmCodeGenerator
/// to produce executable machine code from CURSED's Abstract Syntax Tree (AST)
/// and runs the compiled program in the current process.
pub struct JitCompiler<'ctx> {
    context: &'ctx Context,
    execution_engine: ExecutionEngine<'ctx>,
    module_name: String,
    file_path: PathBuf,
    code_generator: Option<LlvmCodeGenerator<'ctx>>,
}

impl<'ctx> JitCompiler<'ctx> {
    /// Creates a new JIT compiler instance.
    ///
    /// This constructor initializes a JIT compiler with the provided LLVM context,
    /// execution engine, and module information. The JIT compiler will associate
    /// the compiled code with the specified module name and source file path.
    ///
    /// # Arguments
    ///
    /// * `context` - The LLVM context to use for compilation
    /// * `execution_engine` - The LLVM execution engine to compile and run code
    /// * `module_name` - The name of the LLVM module to create
    /// * `file_path` - The path to the source file being compiled (for debugging info)
    ///
    /// # Returns
    ///
    /// A new JitCompiler instance ready to compile and execute CURSED code
    pub fn new(
        context: &'ctx Context,
        execution_engine: ExecutionEngine<'ctx>,
        module_name: &str,
        file_path: PathBuf,
    ) -> Self {
        Self {
            context,
            execution_engine,
            module_name: module_name.to_string(),
            file_path,
            code_generator: None,
        }
    }
    
    /// Gets a mutable reference to the LLVM code generator.
    ///
    /// This method provides access to the code generator instance that translates
    /// CURSED AST nodes to LLVM IR. It returns an Option that may be None if the
    /// code generator hasn't been initialized yet.
    ///
    /// # Returns
    ///
    /// A mutable reference to the Option containing the LLVM code generator
    pub fn code_generator_mut(&mut self) -> &mut Option<LlvmCodeGenerator<'ctx>> {
        &mut self.code_generator
    }
    
    /// Executes the JIT-compiled program
    ///
    /// This method finds the 'main' function in the compiled LLVM module
    /// and executes it using LLVM's execution engine. The function runs
    /// in the current process and has direct access to the process memory.
    ///
    /// # Returns
    ///
    /// Result<(), String> - Ok if execution succeeds, Err with error message otherwise
    pub fn execute(&mut self) -> Result<(), String> {
        // Find and execute the main function
        match self.execution_engine.get_function_value("main") {
            Ok(main_fn) => {
                unsafe {
                    self.execution_engine.run_function(main_fn, &[]);
                }
                Ok(())
            },
            Err(_) => {
                Err("Main function not found".to_string())
            }
        }
    }
}