/// JIT Executor for CURSED Programs
/// 
/// This module provides the core JIT execution functionality using LLVM's execution engine
/// to run compiled CURSED code and return actual results.

use crate::error::CursedError;
use crate::execution::{CursedValue, ValueType, ExecutionContext};
use crate::codegen::llvm::{CursedJitEngine, JitEngineConfig, JitEngineStats};
use inkwell::{context::Context, OptimizationLevel};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Main JIT executor for CURSED programs
pub struct CursedExecutor {
    llvm_context: Arc<Context>,
    jit_engine: Arc<Mutex<CursedJitEngine<'static>>>,
    config: ExecutionConfig,
    stats: ExecutionStats,
}

/// Configuration for the CURSED executor
#[derive(Debug, Clone)]
pub struct ExecutionConfig {
    /// Optimization level for JIT compilation
    pub optimization_level: OptimizationLevel,
    /// Whether to enable debug output during execution
    pub debug_output: bool,
    /// Maximum execution time in milliseconds
    pub max_execution_time_ms: u64,
    /// Whether to enable performance monitoring
    pub enable_monitoring: bool,
}

/// Statistics for execution performance
#[derive(Debug, Default, Clone)]
pub struct ExecutionStats {
    pub functions_compiled: u64,
    pub functions_executed: u64,
    pub total_execution_time_ms: u64,
    pub compilation_time_ms: u64,
    pub successful_executions: u64,
    pub failed_executions: u64,
}

/// Results from executing CURSED code
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub value: CursedValue,
    pub execution_time_ms: u64,
    pub compilation_time_ms: u64,
    pub success: bool,
}

/// Errors specific to execution
#[derive(Debug, Clone)]
pub enum ExecutionError {
    CompilationFailed(String),
    ExecutionFailed(String),
    TimeoutError(String),
    InvalidIR(String),
    FunctionNotFound(String),
    RuntimeError(String),
}

impl Default for ExecutionConfig {
    fn default() -> Self {
        Self {
            optimization_level: OptimizationLevel::O2,
            debug_output: false,
            max_execution_time_ms: 5000, // 5 second timeout
            enable_monitoring: true,
        }
    }
}

impl CursedExecutor {
    /// Create a new CURSED executor
    pub fn new() -> crate::error::Result<()> {
        Self::new_with_config(ExecutionConfig::default())
    }

    /// Create a new CURSED executor with custom configuration
    pub fn new_with_config(config: ExecutionConfig) -> crate::error::Result<()> {
        let llvm_context = Arc::new(Context::create());
        
        // Create JIT engine configuration
        let jit_config = JitEngineConfig {
            optimization_level: config.optimization_level,
            enable_function_cache: true,
            enable_performance_monitoring: config.enable_monitoring,
            max_cached_functions: 1000,
            enable_debug_info: config.debug_output,
            target_cpu: None,
            target_features: Vec::new(),
        };

        // Note: We need to handle lifetime issues with Context
        let context_ptr = Arc::as_ptr(&llvm_context);
        let leaked_context = unsafe { &*context_ptr };
        
        let jit_engine = CursedJitEngine::new(leaked_context, jit_config)
            .map_err(|e| CursedError::CompilationError(format!("Failed to create JIT engine: {}", e)))?;

        Ok(Self {
            llvm_context,
            jit_engine: Arc::new(Mutex::new(jit_engine)),
            config,
            stats: ExecutionStats::default(),
        })
    }

    /// Execute LLVM IR and return the result
    pub fn execute_ir(&mut self, ir: &str, context: &mut ExecutionContext) -> crate::error::Result<()> {
        let start_time = std::time::Instant::now();

        tracing::info!("Executing LLVM IR with JIT engine");
        tracing::debug!("IR to execute: {}", ir);

        // Analyze IR to determine execution strategy
        let execution_strategy = self.analyze_ir(ir)?;
        
        match execution_strategy {
            ExecutionStrategy::SimpleExpression(expr_type) => {
                self.execute_simple_expression(ir, expr_type)
            },
            ExecutionStrategy::Function(func_name) => {
                self.execute_function(ir, &func_name, context)
            },
            ExecutionStrategy::Program => {
                self.execute_program(ir, context)
            },
        }
    }

    /// Analyze IR to determine how to execute it
    fn analyze_ir(&self, ir: &str) -> crate::error::Result<()> {
        if ir.contains("define") {
            // Contains function definitions
            if ir.contains("define i32 @main()") {
                Ok(ExecutionStrategy::Program)
            } else {
                // Extract function name
                let func_name = self.extract_function_name(ir)?;
                Ok(ExecutionStrategy::Function(func_name))
            }
        } else {
            // Simple expression or literal
            if ir.contains("i32") {
                Ok(ExecutionStrategy::SimpleExpression(ValueType::Integer))
            } else if ir.contains("double") {
                Ok(ExecutionStrategy::SimpleExpression(ValueType::Float))
            } else if ir.contains("i8*") {
                Ok(ExecutionStrategy::SimpleExpression(ValueType::String))
            } else if ir.contains("i1") {
                Ok(ExecutionStrategy::SimpleExpression(ValueType::Boolean))
            } else {
                Ok(ExecutionStrategy::SimpleExpression(ValueType::Integer))
            }
        }
    }

    /// Execute a simple expression
    fn execute_simple_expression(&mut self, ir: &str, expr_type: ValueType) -> crate::error::Result<()> {
        tracing::debug!("Executing simple expression of type: {:?}", expr_type);

        // For simple expressions, we can often evaluate them directly from the IR
        match expr_type {
            ValueType::Integer => {
                // Try to extract integer literal from IR
                if let Some(value) = self.extract_integer_literal(ir) {
                    self.stats.successful_executions += 1;
                    return Ok(CursedValue::Integer(value));
                }
                
                // Fallback: compile and execute
                self.execute_with_jit("simple_expr", ir, ValueType::Integer)
            },
            ValueType::Float => {
                // Try to extract float literal from IR
                if let Some(value) = self.extract_float_literal(ir) {
                    self.stats.successful_executions += 1;
                    return Ok(CursedValue::Float(value));
                }
                
                // Fallback: compile and execute
                self.execute_with_jit("simple_expr", ir, ValueType::Float)
            },
            ValueType::String => {
                // Try to extract string literal from IR
                if let Some(value) = self.extract_string_literal(ir) {
                    self.stats.successful_executions += 1;
                    return Ok(CursedValue::String(value));
                }
                
                // Fallback: compile and execute
                self.execute_with_jit("simple_expr", ir, ValueType::String)
            },
            ValueType::Boolean => {
                // Try to extract boolean literal from IR
                if let Some(value) = self.extract_boolean_literal(ir) {
                    self.stats.successful_executions += 1;
                    return Ok(CursedValue::Boolean(value));
                }
                
                // Fallback: compile and execute
                self.execute_with_jit("simple_expr", ir, ValueType::Boolean)
            },
            ValueType::Nil => {
                Ok(CursedValue::Nil)
            },
        }
    }

    /// Execute a function using JIT
    fn execute_function(&mut self, ir: &str, func_name: &str, _context: &mut ExecutionContext) -> crate::error::Result<()> {
        tracing::info!("Executing function: {}", func_name);
        
        self.execute_with_jit(func_name, ir, ValueType::Integer)
    }

    /// Execute a complete program
    fn execute_program(&mut self, ir: &str, _context: &mut ExecutionContext) -> crate::error::Result<()> {
        tracing::info!("Executing complete program");
        
        // Programs typically have a main function that returns an exit code
        self.execute_with_jit("main", ir, ValueType::Integer)
    }

    /// Execute using JIT compilation
    fn execute_with_jit(&mut self, func_name: &str, ir: &str, expected_type: ValueType) -> crate::error::Result<()> {
        let compilation_start = std::time::Instant::now();

        let mut jit_engine = self.jit_engine.lock().map_err(|_| {
            CursedError::CompilationError("Failed to acquire JIT engine lock".to_string())
        })?;

        // Compile the function
        jit_engine.compile_function(func_name, ir).map_err(|e| {
            self.stats.failed_executions += 1;
            CursedError::CompilationError(format!("JIT compilation failed: {}", e))
        })?;

        let compilation_time = compilation_start.elapsed();
        self.stats.compilation_time_ms += compilation_time.as_millis() as u64;
        self.stats.functions_compiled += 1;

        // Execute the function
        let execution_start = std::time::Instant::now();
        
        let result = jit_engine.execute_function(func_name).map_err(|e| {
            self.stats.failed_executions += 1;
            CursedError::RuntimeError(format!("JIT execution failed: {}", e))
        })?;

        let execution_time = execution_start.elapsed();
        self.stats.total_execution_time_ms += execution_time.as_millis() as u64;
        self.stats.functions_executed += 1;
        self.stats.successful_executions += 1;

        // Convert result based on expected type
        let cursed_value = match expected_type {
            ValueType::Integer => CursedValue::Integer(result as i64),
            ValueType::Float => CursedValue::Float(result as f64),
            ValueType::Boolean => CursedValue::Boolean(result != 0),
            ValueType::String => CursedValue::String(format!("result_{}", result)),
            ValueType::Nil => CursedValue::Nil,
        };

        if self.config.debug_output {
            tracing::info!(
                function = func_name,
                result = ?cursed_value,
                compilation_time_ms = compilation_time.as_millis(),
                execution_time_ms = execution_time.as_millis(),
                "Function executed successfully"
            );
        }

        Ok(cursed_value)
    }

    /// Extract integer literal from IR
    fn extract_integer_literal(&self, ir: &str) -> Option<i64> {
        // Look for patterns like "ret i32 42" or "i64 100"
        if let Some(captures) = regex::Regex::new(r"ret i\d+ (\d+)")
            .ok()?
            .captures(ir) {
            captures.get(1)?.as_str().parse().ok()
        } else if let Some(captures) = regex::Regex::new(r"i\d+ (\d+)")
            .ok()?
            .captures(ir) {
            captures.get(1)?.as_str().parse().ok()
        } else {
            None
        }
    }

    /// Extract float literal from IR
    fn extract_float_literal(&self, ir: &str) -> Option<f64> {
        // Look for patterns like "ret double 3.14" or "double 2.5"
        if let Some(captures) = regex::Regex::new(r"ret double ([0-9]*\.?[0-9]+)")
            .ok()?
            .captures(ir) {
            captures.get(1)?.as_str().parse().ok()
        } else if let Some(captures) = regex::Regex::new(r"double ([0-9]*\.?[0-9]+)")
            .ok()?
            .captures(ir) {
            captures.get(1)?.as_str().parse().ok()
        } else {
            None
        }
    }

    /// Extract string literal from IR
    fn extract_string_literal(&self, ir: &str) -> Option<String> {
        // Look for string constants in IR
        if let Some(captures) = regex::Regex::new(r#"@str_\w+ = .*"([^"]+)""#)
            .ok()?
            .captures(ir) {
            Some(captures.get(1)?.as_str().to_string())
        } else {
            None
        }
    }

    /// Extract boolean literal from IR
    fn extract_boolean_literal(&self, ir: &str) -> Option<bool> {
        // Look for patterns like "ret i1 true" or "i1 false"
        if ir.contains("true") {
            Some(true)
        } else if ir.contains("false") {
            Some(false)
        } else if ir.contains("i1 1") {
            Some(true)
        } else if ir.contains("i1 0") {
            Some(false)
        } else {
            None
        }
    }

    /// Extract function name from IR
    fn extract_function_name(&self, ir: &str) -> crate::error::Result<()> {
        if let Some(captures) = regex::Regex::new(r"define.*@([a-zA-Z_][a-zA-Z0-9_]*)")
            .map_err(|e| CursedError::Parse(format!("Regex error: {}", e)))?
            .captures(ir) {
            Ok(captures.get(1)
                .ok_or_else(|| CursedError::Parse("Could not extract function name".to_string()))?
                .as_str()
                .to_string())
        } else {
            Err(CursedError::Parse("No function definition found in IR".to_string()))
        }
    }

    /// Get execution statistics
    pub fn get_stats(&self) -> ExecutionStats {
        self.stats.clone()
    }

    /// Reset execution statistics
    pub fn reset_stats(&mut self) {
        self.stats = ExecutionStats::default();
    }

    /// Update configuration
    pub fn update_config(&mut self, config: ExecutionConfig) {
        self.config = config;
    }

    /// Check if JIT engine is available
    pub fn is_available(&self) -> bool {
        self.jit_engine.lock().is_ok()
    }
}

/// Execution strategy based on IR analysis
#[derive(Debug, Clone)]
enum ExecutionStrategy {
    SimpleExpression(ValueType),
    Function(String),
    Program,
}

// impl From<ExecutionError> for CursedError {
//     fn from(err: ExecutionError) -> Self {
//         match err {
//             ExecutionError::CompilationFailed(msg) => CursedError::CompilationError(msg),
//             ExecutionError::ExecutionFailed(msg) => CursedError::RuntimeError(msg),
//             ExecutionError::TimeoutError(msg) => CursedError::RuntimeError(format!("Timeout: {}", msg)),
//             ExecutionError::InvalidIR(msg) => CursedError::Parse(format!("Invalid IR: {}", msg)),
//             ExecutionError::FunctionNotFound(msg) => CursedError::CompilationError(format!("Function not found: {}", msg)),
//             ExecutionError::RuntimeError(msg) => CursedError::RuntimeError(msg),
//         }
//     }
// }

