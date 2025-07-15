use crate::ast::Expression;
use crate::error_types::CursedError;
use std::collections::HashMap;

/// Defer cleanup system for LLVM IR generation
/// Handles proper resource cleanup with LIFO execution order
pub struct DeferCleanupSystem {
    /// Stack of defer expressions (LIFO order)
    defer_stack: Vec<Expression>,
    /// Per-scope defer expressions for exception handling
    scope_stacks: Vec<Vec<Expression>>,
    /// Current cleanup label counter
    cleanup_label_counter: usize,
}

impl DeferCleanupSystem {
    pub fn new() -> Self {
        Self {
            defer_stack: Vec::new(),
            scope_stacks: Vec::new(),
            cleanup_label_counter: 0,
        }
    }

    /// Push a defer expression onto the stack
    pub fn push_defer(&mut self, expression: Expression) {
        self.defer_stack.push(expression);
    }

    /// Enter a new scope (pushes new scope stack)
    pub fn enter_scope(&mut self) {
        self.scope_stacks.push(Vec::new());
    }

    /// Exit a scope and return defer expressions for cleanup
    pub fn exit_scope(&mut self) -> Vec<Expression> {
        self.scope_stacks.pop().unwrap_or_default()
    }

    /// Add defer expression to current scope
    pub fn add_scope_defer(&mut self, expression: Expression) {
        if let Some(scope) = self.scope_stacks.last_mut() {
            scope.push(expression);
        } else {
            self.defer_stack.push(expression);
        }
    }

    /// Generate cleanup block for normal function exit
    pub fn generate_normal_cleanup(&mut self, ir: &mut String) -> Result<(), CursedError> {
        if self.defer_stack.is_empty() {
            return Ok(());
        }

        ir.push_str("  ; === DEFER CLEANUP (NORMAL EXIT) ===\n");
        
        // Generate cleanup label
        let cleanup_label = self.get_next_cleanup_label();
        ir.push_str(&format!("{}:\n", cleanup_label));

        // Execute defer expressions in LIFO order
        for defer_expr in self.defer_stack.iter().rev() {
            ir.push_str("  ; Executing deferred expression\n");
            self.generate_defer_expression_ir(defer_expr, ir)?;
        }

        ir.push_str("  ; === END DEFER CLEANUP ===\n");
        Ok(())
    }

    /// Generate cleanup block for exception/panic exit
    pub fn generate_panic_cleanup(&mut self, ir: &mut String) -> Result<(), CursedError> {
        if self.defer_stack.is_empty() {
            return Ok(());
        }

        ir.push_str("  ; === DEFER CLEANUP (PANIC EXIT) ===\n");
        
        // Generate panic cleanup label
        let panic_cleanup_label = self.get_next_cleanup_label();
        ir.push_str(&format!("{}:\n", panic_cleanup_label));

        // Execute defer expressions in LIFO order with error handling
        for defer_expr in self.defer_stack.iter().rev() {
            ir.push_str("  ; Executing deferred expression (panic mode)\n");
            match self.generate_defer_expression_ir(defer_expr, ir) {
                Ok(()) => {
                    ir.push_str("  ; Deferred expression completed during panic\n");
                },
                Err(e) => {
                    ir.push_str(&format!("  ; Error in deferred expression during panic: {:?}\n", e));
                    ir.push_str("  ; Continuing with remaining deferred expressions\n");
                }
            }
        }

        ir.push_str("  ; === END DEFER CLEANUP (PANIC) ===\n");
        Ok(())
    }

    /// Generate cleanup for specific scope
    pub fn generate_scope_cleanup(&mut self, scope_defers: Vec<Expression>, ir: &mut String) -> Result<(), CursedError> {
        if scope_defers.is_empty() {
            return Ok(());
        }

        ir.push_str("  ; === SCOPE DEFER CLEANUP ===\n");
        
        // Execute scope defer expressions in LIFO order
        for defer_expr in scope_defers.iter().rev() {
            ir.push_str("  ; Executing scope deferred expression\n");
            self.generate_defer_expression_ir(defer_expr, ir)?;
        }

        ir.push_str("  ; === END SCOPE DEFER CLEANUP ===\n");
        Ok(())
    }

    /// Generate cleanup blocks for all exit paths
    pub fn generate_comprehensive_cleanup(&mut self, ir: &mut String) -> Result<(), CursedError> {
        // Generate normal cleanup
        self.generate_normal_cleanup(ir)?;
        
        // Generate panic cleanup
        self.generate_panic_cleanup(ir)?;
        
        // Clear defer stack after cleanup generation
        self.defer_stack.clear();
        
        Ok(())
    }

    /// Generate LLVM IR for a defer expression
    fn generate_defer_expression_ir(&self, expression: &Expression, ir: &mut String) -> Result<(), CursedError> {
        match expression {
            Expression::FunctionCall(call) => {
                // Generate function call for defer
                ir.push_str(&format!("  ; Defer function call: {}\n", call.name));
                
                // Generate the actual function call IR
                match &call.name[..] {
                    "vibez.spill" => {
                        if let Some(arg) = call.arguments.first() {
                            match arg {
                                Expression::StringLiteral(s) => {
                                    ir.push_str(&format!("  call i32 (i8*, ...) @printf(i8* getelementptr ([{}x i8], [{}x i8]* @.str, i32 0, i32 0))\n", 
                                        s.len() + 1, s.len() + 1));
                                },
                                _ => {
                                    ir.push_str("  ; Complex defer argument - generating call\n");
                                    ir.push_str("  call void @defer_cleanup_call()\n");
                                }
                            }
                        }
                    },
                    _ => {
                        // Generic function call cleanup
                        ir.push_str(&format!("  call void @{}()\n", call.name));
                    }
                }
            },
            Expression::Identifier(name) => {
                // Simple identifier cleanup
                ir.push_str(&format!("  ; Defer identifier cleanup: {}\n", name));
                ir.push_str(&format!("  call void @cleanup_{}()\n", name));
            },
            _ => {
                // Complex expression cleanup
                ir.push_str("  ; Complex defer expression - generating generic cleanup\n");
                ir.push_str("  call void @defer_generic_cleanup()\n");
            }
        }
        Ok(())
    }

    /// Get next cleanup label
    fn get_next_cleanup_label(&mut self) -> String {
        let label = format!("cleanup_{}", self.cleanup_label_counter);
        self.cleanup_label_counter += 1;
        label
    }

    /// Check if there are any defer expressions
    pub fn has_defers(&self) -> bool {
        !self.defer_stack.is_empty() || self.scope_stacks.iter().any(|scope| !scope.is_empty())
    }

    /// Get current defer count for debugging
    pub fn defer_count(&self) -> usize {
        self.defer_stack.len() + self.scope_stacks.iter().map(|s| s.len()).sum::<usize>()
    }
}

/// Defer cleanup context for tracking cleanup state
pub struct DeferContext {
    pub cleanup_system: DeferCleanupSystem,
    pub in_panic: bool,
    pub cleanup_generated: bool,
}

impl DeferContext {
    pub fn new() -> Self {
        Self {
            cleanup_system: DeferCleanupSystem::new(),
            in_panic: false,
            cleanup_generated: false,
        }
    }

    /// Mark context as in panic state
    pub fn enter_panic(&mut self) {
        self.in_panic = true;
    }

    /// Generate appropriate cleanup based on context
    pub fn generate_cleanup(&mut self, ir: &mut String) -> Result<(), CursedError> {
        if self.cleanup_generated {
            return Ok(());
        }

        if self.in_panic {
            self.cleanup_system.generate_panic_cleanup(ir)?;
        } else {
            self.cleanup_system.generate_normal_cleanup(ir)?;
        }

        self.cleanup_generated = true;
        Ok(())
    }
}
