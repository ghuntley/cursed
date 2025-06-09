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

use crate::codegen::llvm::LlvmCodeGenerator;
use crate::error::Error;
use inkwell::context::Context;
use inkwell::execution_engine::ExecutionEngine;
use inkwell::module::Module;
use inkwell::values::FunctionValue;
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

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
#[tracing::instrument(level = "debug")]
pub fn register_goroutine() {
    let prev_count = ACTIVE_GOROUTINES.fetch_add(1, Ordering::SeqCst);
    let new_count = prev_count + 1;
    tracing::info!(previous_count = prev_count, new_count = new_count, "Goroutine registered");
}

/// Marks a goroutine as completed in the runtime system.
///
/// This function decrements the global counter of active goroutines atomically.
/// It should be called whenever a goroutine completes its execution, typically
/// at the end of a goroutine function or when the goroutine terminates early.
///
/// Paired with `register_goroutine`, this function allows the runtime to track
/// the number of goroutines currently executing.
#[tracing::instrument(level = "debug")]
pub fn finish_goroutine() {
    let prev_count = ACTIVE_GOROUTINES.fetch_sub(1, Ordering::SeqCst);
    let new_count = prev_count - 1;
    tracing::info!(previous_count = prev_count, new_count = new_count, "Goroutine finished");
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
#[tracing::instrument(fields(timeout_ms = timeout_ms), level = "debug")]
pub fn wait_for_goroutines(timeout_ms: u64) -> usize {
    // Simple implementation with timeout
    tracing::info!(timeout_ms = timeout_ms, "Waiting for goroutines to complete");
    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_millis(timeout_ms);

    loop {
        let count = ACTIVE_GOROUTINES.load(Ordering::SeqCst);
        let elapsed = start.elapsed();
        
        if count == 0 {
            tracing::info!("All goroutines completed");
            return count;
        } else if elapsed > timeout {
            tracing::warn!(active_goroutines = count, elapsed_ms = ?elapsed.as_millis(), "Timeout waiting for goroutines");
            return count;
        }
        
        // Only log every 100ms to avoid excessive logging
        if elapsed.as_millis() % 100 < 10 {
            tracing::debug!(active_goroutines = count, elapsed_ms = ?elapsed.as_millis(), "Still waiting for goroutines");
        }
        
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

// Stdout capture for JIT execution
struct StdoutCapture {
    buffer: Arc<Mutex<String>>
}

impl StdoutCapture {
    fn new() -> Self {
        StdoutCapture {
            buffer: Arc::new(Mutex::new(String::new()))
        }
    }
    
    fn clone(&self) -> Self {
        StdoutCapture {
            buffer: Arc::clone(&self.buffer)
        }
    }
    
    fn append(&self, text: &str) {
        if let Ok(mut buffer) = self.buffer.lock() {
            buffer.push_str(text);
        }
    }
    
    fn get_content(&self) -> String {
        if let Ok(buffer) = self.buffer.lock() {
            buffer.clone()
        } else {
            String::new()
        }
    }
}

/// Maps external functions to their implementations in the execution engine.
///
/// This function sets up mappings between functions declared in the LLVM IR
/// and their actual implementations. It's essential for providing functionality
/// like I/O and memory management to CURSED programs.
///
/// # Arguments
///
/// * `execution_engine` - The LLVM execution engine to add mappings to
/// * `module` - The LLVM module containing the function declarations
///
/// # Returns
///
/// Result<(), Error> - Success or an error if mapping fails
pub fn map_external_functions(
    execution_engine: &ExecutionEngine,
    module: &Module,
) -> Result<(), Error> {
    use crate::runtime::jit_runtime::{
        cursed_print_int,
        cursed_print_string,
        cursed_print_float,
        cursed_print_bool,
        cursed_print_char
    };

    // Extract the module name (which is our package name)
    let module_name = module.get_name().to_string_lossy();
    tracing::info!(module_name = %module_name, "Mapping external functions for module");

    // Map all variations of the puts function for integers
    // 1. Standard "puts" function
    if let Some(puts_fn) = module.get_function("puts") {
        unsafe {
            let addr = cursed_print_int as usize;
            execution_engine.add_global_mapping(&puts_fn, addr);
            tracing::debug!("Mapped puts function");
        }
    }
    
    // 2. Mangled package-specific puts function: _<package>_puts
    let mangled_puts_name = format!("_{}_puts", module_name);
    if let Some(mangled_puts) = module.get_function(&mangled_puts_name) {
        unsafe {
            let addr = cursed_print_int as usize;
            execution_engine.add_global_mapping(&mangled_puts, addr);
            tracing::debug!(function = %mangled_puts_name, "Mapped mangled puts function");
        }
    }
    
    // 3. Try a few common package names in case the module name doesn't match the package name
    for pkg_name in &["minimal", "main", "test"] {
        let mangled_name = format!("_{}_puts", pkg_name);
        if mangled_name != mangled_puts_name {  // Skip if we already mapped this above
            if let Some(fn_val) = module.get_function(&mangled_name) {
                unsafe {
                    let addr = cursed_print_int as usize;
                    execution_engine.add_global_mapping(&fn_val, addr);
                    tracing::debug!(function = %mangled_name, "Mapped mangled puts function for package");
                }
            }
        }
    }

    // Map println/spill function for strings
    // Map both println and spill since either could be used
    for fn_name in &["println", "spill", "vibez_spill"] {
        if let Some(fn_val) = module.get_function(fn_name) {
            unsafe {
                let addr = cursed_print_string as usize;
                execution_engine.add_global_mapping(&fn_val, addr);
                tracing::debug!(function = %fn_name, "Mapped string print function");
            }
        }
        
        // Also check for mangled versions like _<package>_println, _<package>_spill
        let mangled_name = format!("_{}_{}", module_name, fn_name);
        if let Some(fn_val) = module.get_function(&mangled_name) {
            unsafe {
                let addr = cursed_print_string as usize;
                execution_engine.add_global_mapping(&fn_val, addr);
                tracing::debug!(function = %mangled_name, "Mapped mangled string print function");
            }
        }
        
        // Try mapping for common package names
        for pkg_name in &["minimal", "main", "test", "vibez"] {
            let pkg_mangled_name = format!("_{}_{}", pkg_name, fn_name);
            if pkg_mangled_name != mangled_name {  // Skip if we already mapped this above
                if let Some(fn_val) = module.get_function(&pkg_mangled_name) {
                    unsafe {
                        let addr = cursed_print_string as usize;
                        execution_engine.add_global_mapping(&fn_val, addr);
                        tracing::debug!(function = %pkg_mangled_name, "Mapped mangled string print function for package");
                    }
                }
            }
        }
    }
    
    // Map special print functions for different types
    let type_print_mappings = [
        ("print_float", cursed_print_float as usize),
        ("print_bool", cursed_print_bool as usize),
        ("print_char", cursed_print_char as usize),
    ];
    
    for (fn_base_name, addr) in &type_print_mappings {
        // Try unmangeled name
        if let Some(fn_val) = module.get_function(fn_base_name) {
            unsafe {
                execution_engine.add_global_mapping(&fn_val, *addr);
                tracing::debug!(function = %fn_base_name, "Mapped type-specific print function");
            }
        }
        
        // Try module-mangled name
        let mangled_name = format!("_{}_{}", module_name, fn_base_name);
        if let Some(fn_val) = module.get_function(&mangled_name) {
            unsafe {
                execution_engine.add_global_mapping(&fn_val, *addr);
                tracing::debug!(function = %mangled_name, "Mapped mangled type-specific print function");
            }
        }
        
        // Try common package names
        for pkg_name in &["minimal", "main", "test", "vibez"] {
            let pkg_mangled_name = format!("_{}_{}", pkg_name, fn_base_name);
            if let Some(fn_val) = module.get_function(&pkg_mangled_name) {
                unsafe {
                    execution_engine.add_global_mapping(&fn_val, *addr);
                    tracing::debug!(function = %pkg_mangled_name, "Mapped type-specific print function for package");
                }
            }
        }
    }

    // Map the channel runtime functions
    map_channel_functions(execution_engine, module)?;
    
    Ok(())
}

/// Maps channel-related runtime functions to their implementations.
///
/// This function maps the channel operations like create, send, receive, and close
/// to their corresponding implementations in the runtime/channel.rs module.
///
/// # Arguments
///
/// * `execution_engine` - The LLVM execution engine to add mappings to
/// * `module` - The LLVM module containing the function declarations
///
/// # Returns
///
/// Result<(), Error> - Success or an error if mapping fails
#[tracing::instrument(skip(execution_engine, module), level = "debug")]
fn map_channel_functions(
    execution_engine: &ExecutionEngine,
    module: &Module,
) -> Result<(), Error> {
    use crate::runtime::channel::{cursed_make_channel, cursed_send_to_channel, 
    cursed_receive_from_channel, cursed_close_channel, cursed_try_send_to_channel,
    cursed_try_receive_from_channel, cursed_channel_stats};
    
    tracing::debug!("Mapping channel runtime functions");
    
    // Map cursed_make_channel function
    if let Some(make_channel_fn) = module.get_function("cursed_make_channel") {
        unsafe {
            let addr = cursed_make_channel as usize;
            execution_engine.add_global_mapping(&make_channel_fn, addr);
            println!("Mapped cursed_make_channel function");
        }
    }
    
    // Map cursed_send_to_channel function
    if let Some(send_fn) = module.get_function("cursed_send_to_channel") {
        unsafe {
            let addr = cursed_send_to_channel as usize;
            execution_engine.add_global_mapping(&send_fn, addr);
            println!("Mapped cursed_send_to_channel function");
        }
    }
    
    // Map cursed_receive_from_channel function
    if let Some(receive_fn) = module.get_function("cursed_receive_from_channel") {
        unsafe {
            let addr = cursed_receive_from_channel as usize;
            execution_engine.add_global_mapping(&receive_fn, addr);
            println!("Mapped cursed_receive_from_channel function");
        }
    }
    
    // Map cursed_close_channel function
    if let Some(close_fn) = module.get_function("cursed_close_channel") {
        unsafe {
            let addr = cursed_close_channel as usize;
            execution_engine.add_global_mapping(&close_fn, addr);
            println!("Mapped cursed_close_channel function");
        }
    }
    
    // Map cursed_try_send_to_channel function
    if let Some(try_send_fn) = module.get_function("cursed_try_send_to_channel") {
        unsafe {
            let addr = cursed_try_send_to_channel as usize;
            execution_engine.add_global_mapping(&try_send_fn, addr);
            println!("Mapped cursed_try_send_to_channel function");
        }
    }
    
    // Map cursed_try_receive_from_channel function
    if let Some(try_receive_fn) = module.get_function("cursed_try_receive_from_channel") {
        unsafe {
            let addr = cursed_try_receive_from_channel as usize;
            execution_engine.add_global_mapping(&try_receive_fn, addr);
            println!("Mapped cursed_try_receive_from_channel function");
        }
    }
    
    // Map cursed_channel_stats function
    if let Some(stats_fn) = module.get_function("cursed_channel_stats") {
        unsafe {
            let addr = cursed_channel_stats as usize;
            execution_engine.add_global_mapping(&stats_fn, addr);
            println!("Mapped cursed_channel_stats function");
        }
    }
    
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
    // Track if we've mapped external functions
    functions_mapped: bool,
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
            functions_mapped: false,
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

    /// Maps external functions to their implementations in the execution engine.
    ///
    /// This method ensures that functions like `puts` are properly mapped to their
    /// native implementations before execution begins.
    ///
    /// # Returns
    ///
    /// Result<(), Error> - Success or an error if mapping fails
    #[tracing::instrument(skip(self), level = "debug")]
    pub fn map_functions(&mut self) -> Result<(), Error> {
        if self.functions_mapped {
            tracing::debug!("Functions already mapped, skipping");
            return Ok(());
        }
        
        if let Some(ref code_gen) = self.code_generator {
            map_external_functions(&self.execution_engine, code_gen.module())?;
            self.functions_mapped = true;
        } else {
            return Err(Error::from_str("No code generator available for mapping functions"));
        }
        
        Ok(())
    }

    /// Executes the JIT-compiled program
    ///
    /// This method finds the 'main' function in the compiled LLVM module
    /// and executes it using LLVM's execution engine. The function runs
    /// in the current process and has direct access to the process memory.
    ///
    /// # Returns
    ///
    /// Result<i32, Error> - The return value from main or an error
    #[tracing::instrument(skip(self), level = "info")]
    pub fn execute(&mut self) -> Result<i32, Error> {
        // Map external functions first
        tracing::debug!("Starting JIT execution process");
        self.map_functions()?;
        
        // Debug info about module functions
        if let Some(ref code_gen) = self.code_generator {
            println!("DEBUG: JitCompiler - Available functions in module:");
            let module = code_gen.module();
            module.get_functions().for_each(|f| {
                println!("DEBUG: JitCompiler - Function: {}", f.get_name().to_string_lossy());
            });
        } else {
            println!("DEBUG: JitCompiler - No code generator available to list functions");
        }
        
        // Try different variations of the main function name
        unsafe {
            // First try the _main_main function which handles dot expressions
            println!("DEBUG: JitCompiler - Trying to get function '_main_main'");
            match self.execution_engine.get_function::<unsafe extern "C" fn() -> i32>("_main_main") {
                Ok(main_fn) => {
                    println!("DEBUG: JitCompiler - Found _main_main function, calling it");
                    let result = main_fn.call();
                    println!("DEBUG: JitCompiler - _main_main function returned: {}", result);
                    return Ok(result);
                }
                Err(e) => {
                    println!("DEBUG: JitCompiler - Failed to get {} function: {}", self.module_name, e);
                }
            }
            
            // If that failed, try the unmangled name "main"
            println!("DEBUG: JitCompiler - Trying to get function 'main'");
            match self.execution_engine.get_function::<unsafe extern "C" fn() -> i32>("main") {
                Ok(main_fn) => {
                    println!("DEBUG: JitCompiler - Found main function, calling it");
                    let result = main_fn.call();
                    println!("DEBUG: JitCompiler - Main function returned: {}", result);
                    return Ok(result);
                }
                Err(e) => {
                    println!("DEBUG: JitCompiler - Failed to get main function: {}", e);
                    
                    // If that failed, try to extract the package name from the code generator
                    if let Some(ref code_gen) = self.code_generator {
                        // Try the mangled name "_<package_name>_main"
                        let package_name = code_gen.get_current_package_name();
                        let mangled_name = format!("_{}_main", package_name);
                        println!("DEBUG: JitCompiler - Trying to get function '{}'", mangled_name);
                        
                        match self.execution_engine.get_function::<unsafe extern "C" fn() -> i32>(&mangled_name) {
                            Ok(main_fn) => {
                                println!("DEBUG: JitCompiler - Found mangled main function, calling it");
                                let result = main_fn.call();
                                println!("DEBUG: JitCompiler - Mangled main function returned: {}", result);
                                return Ok(result);
                            }
                            Err(e) => {
                                println!("DEBUG: JitCompiler - Failed to get mangled main function: {}", e);
                                
                                // Last attempt: try just "_main"
                                println!("DEBUG: JitCompiler - Trying to get function '_main'");
                                match self.execution_engine.get_function::<unsafe extern "C" fn() -> i32>("_main") {
                                    Ok(main_fn) => {
                                        println!("DEBUG: JitCompiler - Found _main function, calling it");
                                        let result = main_fn.call();
                                        println!("DEBUG: JitCompiler - _main function returned: {}", result);
                                        return Ok(result);
                                    }
                                    Err(e) => {
                                        println!("DEBUG: JitCompiler - Failed to get _main function: {}", e);
                                        return Err(Error::from_str(&format!(
                                            "Failed to get main function (tried 'main', '{}', and '_main'): {}", 
                                            mangled_name, e
                                        )));
                                    }
                                }
                            }
                        }
                    } else {
                        return Err(Error::from_str("No code generator available"));
                    }
                }
            }
        }
    }
}
