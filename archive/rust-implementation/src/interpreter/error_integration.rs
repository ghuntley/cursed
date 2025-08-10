//! CURSED Interpreter Error Handling Integration
//!
//! This module integrates the yikes/shook/fam error handling system
//! with the CURSED interpreter execution engine.

use std::collections::HashMap;
use crate::error_types::{Error, Result};
use crate::runtime::cursed_error_execution::{
    CursedErrorExecution, cursed_yikes, cursed_shook, cursed_fam,
    get_cursed_error_execution, RecoveryHandler, RecoveryStrategy
};
use crate::runtime::enhanced_error_handling::CursedErrorType;
use crate::value::Value;
use crate::ast::{Expression, Statement};
use crate::runtime::goroutine::GoroutineId;

/// Interpreter error handler for CURSED error keywords
pub struct InterpreterErrorHandler {
    /// Current goroutine ID
    goroutine_id: Option<GoroutineId>,
    /// Error execution runtime
    error_execution: Option<std::sync::Arc<CursedErrorExecution>>,
    /// Current recovery handlers
    recovery_handlers: Vec<RecoveryHandler>,
    /// Error propagation enabled
    propagation_enabled: bool,
}

impl InterpreterErrorHandler {
    /// Create new interpreter error handler
    pub fn new(goroutine_id: Option<GoroutineId>) -> Self {
        Self {
            goroutine_id,
            error_execution: get_cursed_error_execution(),
            recovery_handlers: Vec::new(),
            propagation_enabled: true,
        }
    }

    /// Execute yikes error statement in interpreter
    pub fn execute_yikes(
        &self,
        error_name: &str,
        error_message: &str,
        context: HashMap<String, String>,
        file: &str,
        line: u32,
        column: u32,
    ) -> Result<Value> {
        // Create yikes error
        let cursed_error = cursed_yikes(
            error_name.to_string(),
            error_message.to_string(),
            context,
            self.goroutine_id,
            file,
            line,
            column,
        )?;

        // Return error as a value (for CURSED error handling)
        Ok(Value::Error(format!("{}", cursed_error)))
    }

    /// Execute shook error propagation in interpreter
    pub fn execute_shook(
        &self,
        source_error: CursedErrorType,
        file: &str,
        line: u32,
        column: u32,
    ) -> Result<Value> {
        // Propagate error
        let propagated_error = cursed_shook(
            source_error,
            self.goroutine_id,
            file,
            line,
            column,
        )?;

        // Return propagated error as value
        Ok(Value::Error(format!("{}", propagated_error)))
    }

    /// Execute fam recovery block in interpreter
    pub fn execute_fam<F>(
        &self,
        operation: F,
        recovery_handler: Option<RecoveryHandler>,
        file: &str,
        line: u32,
        column: u32,
    ) -> Result<Value>
    where
        F: FnOnce() -> Result<Value> + Send + 'static,
    {
        // Execute fam recovery
        let result = cursed_fam(
            operation,
            recovery_handler,
            self.goroutine_id,
            file,
            line,
            column,
        );

        match result {
            Ok(value) => Ok(value),
            Err(error) => {
                // Recovery failed, return error value
                Ok(Value::Error(format!("{}", error)))
            }
        }
    }

    /// Handle error expression evaluation
    pub fn evaluate_error_expression(
        &self,
        expr: &Expression,
        interpreter: &mut dyn InterpreterContext,
    ) -> Result<Value> {
        match expr {
            Expression::YikesError { name, message, context_expr } => {
                // Extract error name
                let error_name = match name {
                    Expression::Literal(crate::value::Value::String(s)) => s.clone(),
                    _ => return Err(Error::Runtime("Yikes error name must be a string".to_string())),
                };

                // Extract error message
                let error_message = match interpreter.evaluate_expression(message)? {
                    Value::String(s) => s,
                    other => format!("{}", other),
                };

                // Extract context if provided
                let context = if let Some(ctx_expr) = context_expr {
                    match interpreter.evaluate_expression(ctx_expr)? {
                        Value::Object(map) => {
                            let mut ctx = HashMap::new();
                            for (k, v) in map {
                                ctx.insert(k, format!("{}", v));
                            }
                            ctx
                        }
                        _ => HashMap::new(),
                    }
                } else {
                    HashMap::new()
                };

                // Execute yikes error
                self.execute_yikes(
                    &error_name,
                    &error_message,
                    context,
                    interpreter.current_file(),
                    interpreter.current_line(),
                    interpreter.current_column(),
                )
            }

            Expression::ShookPropagation { source_expr } => {
                // Evaluate source expression to get error
                let source_value = interpreter.evaluate_expression(source_expr)?;
                
                // Convert value to CURSED error type
                let source_error = match source_value {
                    Value::Error(error_str) => {
                        // Parse error string back to CursedErrorType
                        // This is simplified - in practice would need proper serialization
                        CursedErrorType::Yikes {
                            name: "propagated_error".to_string(),
                            message: error_str,
                            context: HashMap::new(),
                            stack_trace: Vec::new(),
                        }
                    }
                    _ => {
                        return Err(Error::Runtime("Shook operator requires an error value".to_string()));
                    }
                };

                // Execute shook propagation
                self.execute_shook(
                    source_error,
                    interpreter.current_file(),
                    interpreter.current_line(),
                    interpreter.current_column(),
                )
            }

            _ => Err(Error::Runtime("Not an error expression".to_string())),
        }
    }

    /// Handle error statement execution
    pub fn execute_error_statement(
        &self,
        stmt: &Statement,
        interpreter: &mut dyn InterpreterContext,
    ) -> Result<()> {
        match stmt {
            Statement::FamRecovery { try_block, catch_block, finally_block } => {
                // Execute fam recovery block
                let recovery_handler = if let Some(catch) = catch_block {
                    Some(RecoveryHandler {
                        name: "catch_handler".to_string(),
                        handler_code: format!("{:?}", catch), // Simplified
                        handler_scope: HashMap::new(),
                        priority: 1,
                    })
                } else {
                    None
                };

                let operation = || {
                    // Execute try block
                    for stmt in try_block {
                        interpreter.execute_statement(stmt)?;
                    }
                    Ok(Value::None)
                };

                // Execute fam operation
                let result = self.execute_fam(
                    operation,
                    recovery_handler,
                    interpreter.current_file(),
                    interpreter.current_line(),
                    interpreter.current_column(),
                )?;

                // Execute finally block if present
                if let Some(finally) = finally_block {
                    for stmt in finally {
                        interpreter.execute_statement(stmt)?;
                    }
                }

                // Check if recovery was successful
                match result {
                    Value::Error(_) => {
                        // Execute catch block if recovery failed
                        if let Some(catch) = catch_block {
                            for stmt in catch {
                                interpreter.execute_statement(stmt)?;
                            }
                        }
                    }
                    _ => {
                        // Operation succeeded
                    }
                }

                Ok(())
            }

            Statement::ErrorHandling { error_expr } => {
                // Execute error expression
                let _error_value = self.evaluate_error_expression(error_expr, interpreter)?;
                // Error value is created but not propagated in statement context
                Ok(())
            }

            _ => Err(Error::Runtime("Not an error handling statement".to_string())),
        }
    }

    /// Check if operation should be wrapped in error handling
    pub fn should_wrap_operation(&self, expr: &Expression) -> bool {
        match expr {
            Expression::FunctionCall { .. } => true,
            Expression::MethodCall { .. } => true,
            Expression::ArrayAccess { .. } => true,
            Expression::FieldAccess { .. } => true,
            _ => false,
        }
    }

    /// Wrap operation with automatic error propagation
    pub fn wrap_with_error_propagation<F>(
        &self,
        operation: F,
        interpreter: &mut dyn InterpreterContext,
    ) -> Result<Value>
    where
        F: FnOnce() -> Result<Value>,
    {
        if !self.propagation_enabled {
            return operation();
        }

        let result = operation();

        match result {
            Ok(value) => Ok(value),
            Err(error) => {
                // Create yikes error from runtime error
                let cursed_error = CursedErrorType::Yikes {
                    name: "runtime_error".to_string(),
                    message: format!("{}", error),
                    context: HashMap::new(),
                    stack_trace: Vec::new(),
                };

                // Propagate error using shook
                let propagated = self.execute_shook(
                    cursed_error,
                    interpreter.current_file(),
                    interpreter.current_line(),
                    interpreter.current_column(),
                )?;

                Ok(propagated)
            }
        }
    }

    /// Add recovery handler
    pub fn add_recovery_handler(&mut self, handler: RecoveryHandler) {
        self.recovery_handlers.push(handler);
    }

    /// Remove recovery handler
    pub fn remove_recovery_handler(&mut self, handler_name: &str) -> bool {
        let initial_len = self.recovery_handlers.len();
        self.recovery_handlers.retain(|h| h.name != handler_name);
        self.recovery_handlers.len() != initial_len
    }

    /// Enable/disable error propagation
    pub fn set_propagation_enabled(&mut self, enabled: bool) {
        self.propagation_enabled = enabled;
    }

    /// Get current error context
    pub fn get_error_context(&self) -> Option<HashMap<String, String>> {
        if let Some(execution) = &self.error_execution {
            if let Some(gid) = self.goroutine_id {
                // Would get context from execution runtime
                // Simplified for now
                Some(HashMap::new())
            } else {
                None
            }
        } else {
            None
        }
    }
}

/// Trait for interpreter context integration
pub trait InterpreterContext {
    /// Evaluate an expression
    fn evaluate_expression(&mut self, expr: &Expression) -> Result<Value>;
    
    /// Execute a statement
    fn execute_statement(&mut self, stmt: &Statement) -> Result<()>;
    
    /// Get current file name
    fn current_file(&self) -> &str;
    
    /// Get current line number
    fn current_line(&self) -> u32;
    
    /// Get current column number
    fn current_column(&self) -> u32;
    
    /// Get current scope variables
    fn current_scope(&self) -> &HashMap<String, Value>;
    
    /// Set variable in current scope
    fn set_variable(&mut self, name: String, value: Value) -> Result<()>;
}

/// Error handling configuration for interpreter
#[derive(Debug, Clone)]
pub struct InterpreterErrorConfig {
    /// Enable automatic error propagation
    pub auto_propagation: bool,
    /// Enable error recovery optimization
    pub optimize_recovery: bool,
    /// Maximum error nesting depth
    pub max_error_depth: usize,
    /// Enable error context capture
    pub capture_context: bool,
}

impl Default for InterpreterErrorConfig {
    fn default() -> Self {
        Self {
            auto_propagation: true,
            optimize_recovery: true,
            max_error_depth: 50,
            capture_context: true,
        }
    }
}

/// Interpreter error execution wrapper
pub struct InterpreterErrorExecution {
    /// Error handler
    handler: InterpreterErrorHandler,
    /// Configuration
    config: InterpreterErrorConfig,
    /// Error statistics
    error_count: u64,
    /// Recovery success count
    recovery_success_count: u64,
}

impl InterpreterErrorExecution {
    /// Create new interpreter error execution
    pub fn new(goroutine_id: Option<GoroutineId>, config: InterpreterErrorConfig) -> Self {
        Self {
            handler: InterpreterErrorHandler::new(goroutine_id),
            config,
            error_count: 0,
            recovery_success_count: 0,
        }
    }

    /// Execute expression with error handling
    pub fn execute_expression_with_error_handling(
        &mut self,
        expr: &Expression,
        interpreter: &mut dyn InterpreterContext,
    ) -> Result<Value> {
        if self.handler.should_wrap_operation(expr) && self.config.auto_propagation {
            self.handler.wrap_with_error_propagation(
                || interpreter.evaluate_expression(expr),
                interpreter,
            )
        } else {
            interpreter.evaluate_expression(expr)
        }
    }

    /// Execute statement with error handling
    pub fn execute_statement_with_error_handling(
        &mut self,
        stmt: &Statement,
        interpreter: &mut dyn InterpreterContext,
    ) -> Result<()> {
        match stmt {
            Statement::FamRecovery { .. } | Statement::ErrorHandling { .. } => {
                self.handler.execute_error_statement(stmt, interpreter)
            }
            _ => {
                if self.config.auto_propagation {
                    self.handler.wrap_with_error_propagation(
                        || {
                            interpreter.execute_statement(stmt)?;
                            Ok(Value::None)
                        },
                        interpreter,
                    )?;
                    Ok(())
                } else {
                    interpreter.execute_statement(stmt)
                }
            }
        }
    }

    /// Get error statistics
    pub fn get_error_statistics(&self) -> (u64, u64) {
        (self.error_count, self.recovery_success_count)
    }

    /// Reset error statistics
    pub fn reset_statistics(&mut self) {
        self.error_count = 0;
        self.recovery_success_count = 0;
    }
}

/// Helper function to create interpreter error handler
pub fn create_interpreter_error_handler(goroutine_id: Option<GoroutineId>) -> InterpreterErrorHandler {
    InterpreterErrorHandler::new(goroutine_id)
}

/// Helper function to create interpreter error execution
pub fn create_interpreter_error_execution(
    goroutine_id: Option<GoroutineId>,
    config: Option<InterpreterErrorConfig>,
) -> InterpreterErrorExecution {
    InterpreterErrorExecution::new(goroutine_id, config.unwrap_or_default())
}
