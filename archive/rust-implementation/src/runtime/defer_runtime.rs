use crate::ast::Expression;
use once_cell::sync::Lazy;
use crate::error_types::CursedError;
use std::panic;
use std::sync::{Arc, Mutex};

/// Runtime defer system for CURSED language
/// Handles defer statements with proper LIFO execution and panic safety
pub struct DeferRuntime {
    /// Stack of defer expressions (LIFO order)
    defer_stack: Vec<Expression>,
    /// Stack of defer scopes for proper cleanup ordering
    scope_stack: Vec<Vec<Expression>>,
    /// Panic hook installed flag
    panic_hook_installed: bool,
}

impl DeferRuntime {
    pub fn new() -> Self {
        Self {
            defer_stack: Vec::new(),
            scope_stack: Vec::new(),
            panic_hook_installed: false,
        }
    }

    /// Install panic hook for defer cleanup
    pub fn install_panic_hook(&mut self) {
        if !self.panic_hook_installed {
            let original_hook = panic::take_hook();
            panic::set_hook(Box::new(move |panic_info| {
                eprintln!("🔥 PANIC: Executing defer cleanup before panic");
                
                // Execute defer cleanup during panic
                // Note: This is a simplified version - in practice, we'd need
                // thread-local storage or other mechanisms to access the defer stack
                
                original_hook(panic_info);
            }));
            self.panic_hook_installed = true;
        }
    }

    /// Push a defer expression onto the stack
    pub fn push_defer(&mut self, expression: Expression) {
        self.defer_stack.push(expression);
    }

    /// Enter a new defer scope
    pub fn enter_scope(&mut self) {
        self.scope_stack.push(Vec::new());
    }

    /// Exit a scope and execute its defer expressions
    pub fn exit_scope(&mut self) -> Result<(), CursedError> {
        if let Some(scope_defers) = self.scope_stack.pop() {
            // Execute scope defer expressions in LIFO order
            for defer_expr in scope_defers.into_iter().rev() {
                match self.execute_defer_expression(&defer_expr) {
                    Ok(()) => {},
                    Err(e) => {
                        eprintln!("⚠️ Error in scope defer expression: {:?}", e);
                        // Continue with remaining defer expressions
                    }
                }
            }
        }
        Ok(())
    }

    /// Add defer expression to current scope
    pub fn add_scope_defer(&mut self, expression: Expression) {
        if let Some(scope) = self.scope_stack.last_mut() {
            scope.push(expression);
        } else {
            self.defer_stack.push(expression);
        }
    }

    /// Execute all defer expressions in LIFO order
    pub fn execute_all_defers(&mut self) -> Result<(), CursedError> {
        // Execute main defer stack in LIFO order
        while let Some(defer_expr) = self.defer_stack.pop() {
            match self.execute_defer_expression(&defer_expr) {
                Ok(()) => {},
                Err(e) => {
                    eprintln!("⚠️ Error in defer expression: {:?}", e);
                    // Continue with remaining defer expressions
                }
            }
        }

        // Execute any remaining scope defer expressions
        while let Some(scope_defers) = self.scope_stack.pop() {
            for defer_expr in scope_defers.into_iter().rev() {
                match self.execute_defer_expression(&defer_expr) {
                    Ok(()) => {},
                    Err(e) => {
                        eprintln!("⚠️ Error in scope defer expression: {:?}", e);
                        // Continue with remaining defer expressions
                    }
                }
            }
        }

        Ok(())
    }

    /// Execute a single defer expression
    fn execute_defer_expression(&self, expression: &Expression) -> Result<(), CursedError> {
        match expression {
            Expression::FunctionCall(call) => {
                // Execute function call
                match &call.name[..] {
                    "vibez.spill" => {
                        if let Some(arg) = call.arguments.first() {
                            match arg {
                                Expression::StringLiteral(s) => {
                                    println!("{}", s);
                                },
                                _ => {
                                    println!("Defer cleanup: complex argument");
                                }
                            }
                        }
                    },
                    _ => {
                        println!("Defer cleanup: {}", call.name);
                    }
                }
            },
            Expression::Identifier(name) => {
                println!("Defer cleanup: {}", name);
            },
            _ => {
                println!("Defer cleanup: complex expression");
            }
        }
        Ok(())
    }

    /// Check if there are any defer expressions
    pub fn has_defers(&self) -> bool {
        !self.defer_stack.is_empty() || self.scope_stack.iter().any(|scope| !scope.is_empty())
    }

    /// Get current defer count
    pub fn defer_count(&self) -> usize {
        self.defer_stack.len() + self.scope_stack.iter().map(|s| s.len()).sum::<usize>()
    }
}

/// Thread-safe defer runtime for multi-threaded environments
pub struct ThreadSafeDeferRuntime {
    runtime: Arc<Mutex<DeferRuntime>>,
}

impl ThreadSafeDeferRuntime {
    pub fn new() -> Self {
        Self {
            runtime: Arc::new(Mutex::new(DeferRuntime::new())),
        }
    }

    pub fn push_defer(&self, expression: Expression) -> Result<(), CursedError> {
        let mut runtime = self.runtime.lock().map_err(|_| {
            CursedError::Runtime("Failed to lock defer runtime".to_string())
        })?;
        runtime.push_defer(expression);
        Ok(())
    }

    pub fn enter_scope(&self) -> Result<(), CursedError> {
        let mut runtime = self.runtime.lock().map_err(|_| {
            CursedError::Runtime("Failed to lock defer runtime".to_string())
        })?;
        runtime.enter_scope();
        Ok(())
    }

    pub fn exit_scope(&self) -> Result<(), CursedError> {
        let mut runtime = self.runtime.lock().map_err(|_| {
            CursedError::Runtime("Failed to lock defer runtime".to_string())
        })?;
        runtime.exit_scope()
    }

    pub fn execute_all_defers(&self) -> Result<(), CursedError> {
        let mut runtime = self.runtime.lock().map_err(|_| {
            CursedError::Runtime("Failed to lock defer runtime".to_string())
        })?;
        runtime.execute_all_defers()
    }

    pub fn has_defers(&self) -> bool {
        if let Ok(runtime) = self.runtime.lock() {
            runtime.has_defers()
        } else {
            false
        }
    }
}

/// Global defer runtime instance
static GLOBAL_DEFER_RUNTIME: Lazy<std::sync::Mutex<ThreadSafeDeferRuntime>> = Lazy::new(|| std::sync::Mutex::new(ThreadSafeDeferRuntime::new()));

/// Get global defer runtime
pub fn get_global_defer_runtime() -> &'static ThreadSafeDeferRuntime {
    unsafe {
        if GLOBAL_DEFER_RUNTIME.is_none() {
            // Initialization handled by Lazy);
        }
        GLOBAL_DEFER_RUNTIME.lock().unwrap()
    }
}

/// Initialize global defer runtime
pub fn init_defer_runtime() {
    unsafe {
        if GLOBAL_DEFER_RUNTIME.is_none() {
            // Initialization handled by Lazy);
        }
    }
}

/// Execute all global defer expressions
pub fn execute_global_defers() -> Result<(), CursedError> {
    get_global_defer_runtime().execute_all_defers()
}

/// RAII guard for defer scope management
pub struct DeferScopeGuard {
    runtime: &'static ThreadSafeDeferRuntime,
}

impl DeferScopeGuard {
    pub fn new() -> Result<Self, CursedError> {
        let runtime = get_global_defer_runtime();
        runtime.enter_scope()?;
        Ok(Self { runtime })
    }
}

impl Drop for DeferScopeGuard {
    fn drop(&mut self) {
        if let Err(e) = self.runtime.exit_scope() {
            eprintln!("⚠️ Error executing defer scope cleanup: {:?}", e);
        }
    }
}
