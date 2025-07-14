//! REPL with Stabilized JIT Compilation
//! 
//! This module provides a Read-Eval-Print Loop (REPL) implementation
//! that uses the stabilized JIT compilation system for interactive
//! CURSED programming with real-time compilation and execution.

use std::collections::HashMap;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use crate::codegen::llvm::jit_compilation_stabilized::{StabilizedJitCompiler, JitError};
use crate::runtime::jit_runtime::{JitRuntimeConfig, OptimizationLevel, CompilationTier};
use crate::error::CursedError;

/// REPL state and configuration
#[derive(Debug)]
pub struct JitRepl {
    /// JIT compiler instance
    compiler: Arc<Mutex<StabilizedJitCompiler>>,
    /// REPL session state
    session: ReplSession,
    /// Configuration
    config: ReplConfig,
}

/// REPL session state
#[derive(Debug)]
struct ReplSession {
    /// Line number counter
    line_number: usize,
    /// Variable bindings
    variables: HashMap<String, String>,
    /// Function definitions
    functions: HashMap<String, String>,
    /// Command history
    history: Vec<String>,
    /// Session start time
    start_time: Instant,
}

/// REPL configuration
#[derive(Debug, Clone)]
pub struct ReplConfig {
    /// Enable JIT compilation
    pub enable_jit: bool,
    /// Initial optimization level
    pub optimization_level: OptimizationLevel,
    /// Compilation tier
    pub compilation_tier: CompilationTier,
    /// Enable auto-completion
    pub enable_completion: bool,
    /// Enable syntax highlighting
    pub enable_highlighting: bool,
    /// Command prompt
    pub prompt: String,
    /// Enable timing information
    pub show_timing: bool,
    /// Enable debug output
    pub debug_mode: bool,
}

impl Default for ReplConfig {
    fn default() -> Self {
        Self {
            enable_jit: true,
            optimization_level: OptimizationLevel::Basic,
            compilation_tier: CompilationTier::Tier1,
            enable_completion: false,
            enable_highlighting: false,
            prompt: "cursed> ".to_string(),
            show_timing: false,
            debug_mode: false,
        }
    }
}

impl JitRepl {
    /// Create a new REPL instance
    pub fn new(config: ReplConfig) -> Result<Self, CursedError> {
        let jit_config = JitRuntimeConfig::default();
        let compiler = StabilizedJitCompiler::new(jit_config)
            .map_err(|e| CursedError::runtime_error(&format!("Failed to create JIT compiler: {}", e)))?;
        
        let session = ReplSession {
            line_number: 0,
            variables: HashMap::new(),
            functions: HashMap::new(),
            history: Vec::new(),
            start_time: Instant::now(),
        };
        
        Ok(Self {
            compiler: Arc::new(Mutex::new(compiler)),
            session,
            config,
        })
    }
    
    /// Initialize the REPL
    pub fn initialize(&mut self) -> Result<(), CursedError> {
        let mut compiler = self.compiler.lock()
            .map_err(|_| CursedError::runtime_error("Failed to acquire compiler lock"))?;
        
        compiler.initialize()
            .map_err(|e| CursedError::runtime_error(&format!("Failed to initialize JIT compiler: {}", e)))?;
        
        if self.config.debug_mode {
            println!("🔧 JIT REPL initialized successfully");
        }
        
        Ok(())
    }
    
    /// Run the REPL
    pub fn run(&mut self) -> Result<(), CursedError> {
        self.print_welcome();
        
        loop {
            // Print prompt
            print!("{}", self.config.prompt);
            io::stdout().flush().unwrap();
            
            // Read input
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(0) => break, // EOF
                Ok(_) => {
                    let input = input.trim();
                    if input.is_empty() {
                        continue;
                    }
                    
                    // Handle special commands
                    if input.starts_with(':') {
                        self.handle_command(input)?;
                        continue;
                    }
                    
                    // Process CURSED code
                    self.process_input(input)?;
                }
                Err(e) => {
                    eprintln!("Error reading input: {}", e);
                    break;
                }
            }
        }
        
        self.print_goodbye();
        Ok(())
    }
    
    /// Print welcome message
    fn print_welcome(&self) {
        println!("🎯 CURSED JIT REPL v1.0");
        println!("Type ':help' for help, ':quit' to exit");
        println!("JIT compilation is {}", if self.config.enable_jit { "enabled" } else { "disabled" });
        println!();
    }
    
    /// Print goodbye message
    fn print_goodbye(&self) {
        let session_duration = self.session.start_time.elapsed();
        println!("\n👋 REPL session ended after {:?}", session_duration);
        println!("Processed {} lines", self.session.line_number);
        
        // Show JIT statistics
        if let Ok(compiler) = self.compiler.lock() {
            if let Ok(stats) = compiler.get_statistics() {
                println!("JIT Statistics:");
                println!("  - Functions compiled: {}", stats.total_compilations);
                println!("  - Compilation time: {:?}", stats.total_compile_time);
                println!("  - Errors: {}", stats.error_count);
                println!("  - Recoveries: {}", stats.recovery_count);
            }
        }
    }
    
    /// Handle special REPL commands
    fn handle_command(&mut self, command: &str) -> Result<(), CursedError> {
        match command {
            ":help" => self.print_help(),
            ":quit" | ":q" => std::process::exit(0),
            ":stats" => self.print_stats()?,
            ":history" => self.print_history(),
            ":clear" => self.clear_session(),
            ":debug" => self.toggle_debug(),
            ":timing" => self.toggle_timing(),
            ":vars" => self.print_variables(),
            ":funcs" => self.print_functions(),
            ":reset" => self.reset_compiler()?,
            _ => println!("Unknown command: {}", command),
        }
        Ok(())
    }
    
    /// Print help information
    fn print_help(&self) {
        println!("REPL Commands:");
        println!("  :help     - Show this help message");
        println!("  :quit     - Exit the REPL");
        println!("  :stats    - Show JIT compilation statistics");
        println!("  :history  - Show command history");
        println!("  :clear    - Clear session state");
        println!("  :debug    - Toggle debug mode");
        println!("  :timing   - Toggle timing information");
        println!("  :vars     - Show defined variables");
        println!("  :funcs    - Show defined functions");
        println!("  :reset    - Reset JIT compiler");
        println!();
        println!("Enter CURSED code directly to compile and execute it.");
    }
    
    /// Print JIT statistics
    fn print_stats(&self) -> Result<(), CursedError> {
        let compiler = self.compiler.lock()
            .map_err(|_| CursedError::runtime_error("Failed to acquire compiler lock"))?;
        
        let stats = compiler.get_statistics()
            .map_err(|e| CursedError::runtime_error(&format!("Failed to get statistics: {}", e)))?;
        
        println!("JIT Compilation Statistics:");
        println!("  Total compilations: {}", stats.total_compilations);
        println!("  Total compile time: {:?}", stats.total_compile_time);
        println!("  Average compile time: {:?}", 
            if stats.total_compilations > 0 {
                stats.total_compile_time / stats.total_compilations as u32
            } else {
                std::time::Duration::from_secs(0)
            }
        );
        println!("  Errors: {}", stats.error_count);
        println!("  Recoveries: {}", stats.recovery_count);
        println!("  Cache hit ratio: {:.2}%", stats.cache_hit_ratio * 100.0);
        
        Ok(())
    }
    
    /// Print command history
    fn print_history(&self) {
        println!("Command History:");
        for (i, cmd) in self.session.history.iter().enumerate() {
            println!("  {}: {}", i + 1, cmd);
        }
    }
    
    /// Clear session state
    fn clear_session(&mut self) {
        self.session.variables.clear();
        self.session.functions.clear();
        self.session.history.clear();
        self.session.line_number = 0;
        println!("Session cleared");
    }
    
    /// Toggle debug mode
    fn toggle_debug(&mut self) {
        self.config.debug_mode = !self.config.debug_mode;
        println!("Debug mode: {}", if self.config.debug_mode { "enabled" } else { "disabled" });
    }
    
    /// Toggle timing information
    fn toggle_timing(&mut self) {
        self.config.show_timing = !self.config.show_timing;
        println!("Timing info: {}", if self.config.show_timing { "enabled" } else { "disabled" });
    }
    
    /// Print defined variables
    fn print_variables(&self) {
        println!("Defined Variables:");
        for (name, value) in &self.session.variables {
            println!("  {} = {}", name, value);
        }
    }
    
    /// Print defined functions
    fn print_functions(&self) {
        println!("Defined Functions:");
        for (name, body) in &self.session.functions {
            println!("  {} = {}", name, body);
        }
    }
    
    /// Reset JIT compiler
    fn reset_compiler(&mut self) -> Result<(), CursedError> {
        let mut compiler = self.compiler.lock()
            .map_err(|_| CursedError::runtime_error("Failed to acquire compiler lock"))?;
        
        compiler.cleanup()
            .map_err(|e| CursedError::runtime_error(&format!("Failed to cleanup compiler: {}", e)))?;
        
        compiler.initialize()
            .map_err(|e| CursedError::runtime_error(&format!("Failed to reinitialize compiler: {}", e)))?;
        
        println!("JIT compiler reset");
        Ok(())
    }
    
    /// Process user input
    fn process_input(&mut self, input: &str) -> Result<(), CursedError> {
        let start_time = Instant::now();
        
        // Add to history
        self.session.history.push(input.to_string());
        self.session.line_number += 1;
        
        // Generate unique function name for this line
        let function_name = format!("repl_line_{}", self.session.line_number);
        
        // Compile and execute the code
        let compile_result = {
        let mut compiler = self.compiler.lock()
                .map_err(|_| CursedError::runtime_error("Failed to acquire compiler lock"))?;

        compiler.compile_function(
            &function_name,
            input,
            self.config.compilation_tier,
                self.config.optimization_level,
            )
        };

        match compile_result {
        Ok(compiled_function) => {
        if self.config.debug_mode {
        println!("✅ Compiled '{}' (tier: {:?}, size: {} bytes)", 
        function_name, 
            compiled_function.tier, 
                compiled_function.code_size
            );
        }
        
        // Try to execute the function
        let exec_result = {
        let mut compiler = self.compiler.lock()
        .map_err(|_| CursedError::runtime_error("Failed to acquire compiler lock"))?;
        compiler.execute_function(&function_name, &[])
        };
        
        match exec_result {
        Ok(_) => {
            if self.config.debug_mode {
                println!("🚀 Executed '{}' successfully", function_name);
        }
        
        // Update session state if this was a variable declaration
        self.update_session_state(input);
        }
            Err(e) => {
                    println!("❌ Execution failed: {}", e);
                    
                // Try to provide helpful error message
                self.provide_error_help(&e);
            }
        }
        }
            Err(e) => {
                println!("❌ Compilation failed: {}", e);
                
                // Try to provide helpful error message
                self.provide_compilation_help(input, &e);
            }
        }
        
        // Show timing if enabled
        if self.config.show_timing {
            let elapsed = start_time.elapsed();
            println!("⏱️  Processed in {:?}", elapsed);
        }
        
        Ok(())
    }
    
    /// Update session state based on input
    fn update_session_state(&mut self, input: &str) {
        // Simple parsing to track variable declarations
        if input.starts_with("sus ") {
            if let Some(equals_pos) = input.find('=') {
                let var_part = &input[4..equals_pos].trim();
                let value_part = &input[equals_pos + 1..].trim();
                
                if let Some(space_pos) = var_part.find(' ') {
                    let var_name = &var_part[..space_pos];
                    self.session.variables.insert(var_name.to_string(), value_part.to_string());
                }
            }
        }
        
        // Track function definitions
        if input.starts_with("slay ") {
            if let Some(paren_pos) = input.find('(') {
                let func_name = &input[5..paren_pos].trim();
                self.session.functions.insert(func_name.to_string(), input.to_string());
            }
        }
    }
    
    /// Provide helpful error messages for compilation failures
    fn provide_compilation_help(&self, input: &str, error: &JitError) {
        println!("💡 Compilation Help:");
        
        match error {
            JitError::CompilationFailed(_) => {
                println!("  - Check your CURSED syntax");
                println!("  - Make sure variables are declared with 'sus'");
                println!("  - Function definitions use 'slay'");
                println!("  - Statements end with semicolons");
            }
            JitError::FunctionNotFound(_) => {
                println!("  - Function may not have been compiled correctly");
                println!("  - Try redefining the function");
            }
            _ => {
                println!("  - Try simpler code first");
                println!("  - Use ':reset' to reset the compiler");
            }
        }
        
        // Suggest corrections based on common patterns
        if input.contains("print") && !input.contains("vibez.spill") {
            println!("  - Use 'vibez.spill()' instead of 'print()'");
        }
        
        if input.contains("var ") || input.contains("let ") {
            println!("  - Use 'sus' for variable declarations");
        }
    }
    
    /// Provide helpful error messages for execution failures
    fn provide_error_help(&self, error: &JitError) {
        println!("💡 Execution Help:");
        
        match error {
            JitError::RuntimeError(_) => {
                println!("  - Check for null pointer dereferences");
                println!("  - Ensure variables are initialized");
            }
            JitError::InvalidFunctionSignature(_) => {
                println!("  - Check function parameter types");
                println!("  - Ensure return types match");
            }
            _ => {
                println!("  - Try ':reset' to reset the compiler");
                println!("  - Check for memory issues");
            }
        }
    }
}

impl Drop for JitRepl {
    fn drop(&mut self) {
        // Ensure cleanup is performed
        if let Ok(mut compiler) = self.compiler.lock() {
            let _ = compiler.cleanup();
        }
    }
}

/// Create and run a JIT REPL with default configuration
pub fn run_jit_repl() -> Result<(), CursedError> {
    let config = ReplConfig::default();
    let mut repl = JitRepl::new(config)?;
    repl.initialize()?;
    repl.run()
}

/// Create and run a JIT REPL with custom configuration
pub fn run_jit_repl_with_config(config: ReplConfig) -> Result<(), CursedError> {
    let mut repl = JitRepl::new(config)?;
    repl.initialize()?;
    repl.run()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    
    #[test]
    fn test_repl_creation() {
        let config = ReplConfig::default();
        let repl = JitRepl::new(config);
        assert!(repl.is_ok());
    }
    
    #[test]
    fn test_repl_initialization() {
        let config = ReplConfig::default();
        let mut repl = JitRepl::new(config).unwrap();
        let result = repl.initialize();
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_session_state_update() {
        let config = ReplConfig::default();
        let mut repl = JitRepl::new(config).unwrap();
        
        // Test variable declaration tracking
        repl.update_session_state("sus x normie = 42");
        assert!(repl.session.variables.contains_key("x"));
        
        // Test function definition tracking
        repl.update_session_state("slay test() { vibez.spill(\"hello\") }");
        assert!(repl.session.functions.contains_key("test"));
    }
    
    #[test]
    fn test_command_handling() {
        let config = ReplConfig::default();
        let mut repl = JitRepl::new(config).unwrap();
        
        // Test help command
        let result = repl.handle_command(":help");
        assert!(result.is_ok());
        
        // Test debug toggle
        let initial_debug = repl.config.debug_mode;
        repl.handle_command(":debug").unwrap();
        assert_ne!(repl.config.debug_mode, initial_debug);
    }
    
    #[test]
    fn test_error_help() {
        let config = ReplConfig::default();
        let repl = JitRepl::new(config).unwrap();
        
        let error = JitError::CompilationFailed("test error".to_string());
        repl.provide_compilation_help("invalid code", &error);
        
        let error = JitError::RuntimeError("test error".to_string());
        repl.provide_error_help(&error);
    }
}
