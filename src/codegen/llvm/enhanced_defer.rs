use crate::ast::Expression;
use crate::error_types::CursedError;
use std::collections::HashMap;

/// Enhanced defer system for LLVM code generation
/// Provides proper cleanup block generation with exception safety
pub struct EnhancedDeferSystem {
    /// Main defer stack (LIFO order)
    defer_stack: Vec<Expression>,
    /// Cleanup labels for different exit paths
    cleanup_labels: HashMap<String, String>,
    /// Label counter for generating unique labels
    label_counter: usize,
    /// Current function name for context
    current_function: Option<String>,
}

impl EnhancedDeferSystem {
    pub fn new() -> Self {
        Self {
            defer_stack: Vec::new(),
            cleanup_labels: HashMap::new(),
            label_counter: 0,
            current_function: None,
        }
    }

    /// Set current function for context
    pub fn set_current_function(&mut self, function_name: String) {
        self.current_function = Some(function_name);
    }

    /// Add defer expression to the stack
    pub fn add_defer(&mut self, expression: Expression) {
        self.defer_stack.push(expression);
    }

    /// Generate comprehensive cleanup blocks
    pub fn generate_cleanup_blocks(&mut self, ir: &mut String) -> Result<(), CursedError> {
        if self.defer_stack.is_empty() {
            return Ok(());
        }

        // Generate normal exit cleanup
        self.generate_normal_exit_cleanup(ir)?;
        
        // Generate exception exit cleanup
        self.generate_exception_exit_cleanup(ir)?;
        
        // Generate return cleanup
        self.generate_return_cleanup(ir)?;
        
        Ok(())
    }

    /// Generate normal exit cleanup block
    fn generate_normal_exit_cleanup(&mut self, ir: &mut String) -> Result<(), CursedError> {
        let cleanup_label = self.get_next_label("normal_cleanup");
        self.cleanup_labels.insert("normal".to_string(), cleanup_label.clone());
        
        ir.push_str(&format!("  ; === NORMAL EXIT CLEANUP ===\n"));
        ir.push_str(&format!("{}:\n", cleanup_label));
        
        // Execute defer expressions in LIFO order
        for (index, defer_expr) in self.defer_stack.iter().enumerate().rev() {
            ir.push_str(&format!("  ; Defer expression {} (LIFO order)\n", index + 1));
            self.generate_defer_expression_cleanup(defer_expr, ir)?;
        }
        
        ir.push_str("  ; Normal cleanup complete\n");
        ir.push_str("  ret void\n");
        
        Ok(())
    }

    /// Generate exception exit cleanup block
    fn generate_exception_exit_cleanup(&mut self, ir: &mut String) -> Result<(), CursedError> {
        let cleanup_label = self.get_next_label("exception_cleanup");
        self.cleanup_labels.insert("exception".to_string(), cleanup_label.clone());
        
        ir.push_str(&format!("  ; === EXCEPTION EXIT CLEANUP ===\n"));
        ir.push_str(&format!("{}:\n", cleanup_label));
        
        // Execute defer expressions in LIFO order with error handling
        for (index, defer_expr) in self.defer_stack.iter().enumerate().rev() {
            ir.push_str(&format!("  ; Defer expression {} (exception mode)\n", index + 1));
            
            // Wrap in try-catch equivalent for LLVM
            let try_label = self.get_next_label("defer_try");
            let catch_label = self.get_next_label("defer_catch");
            let continue_label = self.get_next_label("defer_continue");
            
            ir.push_str(&format!("  br label %{}\n", try_label));
            ir.push_str(&format!("{}:\n", try_label));
            
            match self.generate_defer_expression_cleanup(defer_expr, ir) {
                Ok(()) => {
                    ir.push_str(&format!("  br label %{}\n", continue_label));
                },
                Err(_) => {
                    ir.push_str(&format!("  br label %{}\n", catch_label));
                }
            }
            
            ir.push_str(&format!("{}:\n", catch_label));
            ir.push_str("  ; Error in defer expression - continuing cleanup\n");
            ir.push_str(&format!("  br label %{}\n", continue_label));
            
            ir.push_str(&format!("{}:\n", continue_label));
        }
        
        ir.push_str("  ; Exception cleanup complete\n");
        ir.push_str("  call void @terminate_with_exception()\n");
        ir.push_str("  unreachable\n");
        
        Ok(())
    }

    /// Generate return cleanup block
    fn generate_return_cleanup(&mut self, ir: &mut String) -> Result<(), CursedError> {
        let cleanup_label = self.get_next_label("return_cleanup");
        self.cleanup_labels.insert("return".to_string(), cleanup_label.clone());
        
        ir.push_str(&format!("  ; === RETURN CLEANUP ===\n"));
        ir.push_str(&format!("{}:\n", cleanup_label));
        
        // Execute defer expressions in LIFO order
        for (index, defer_expr) in self.defer_stack.iter().enumerate().rev() {
            ir.push_str(&format!("  ; Defer expression {} (return mode)\n", index + 1));
            self.generate_defer_expression_cleanup(defer_expr, ir)?;
        }
        
        ir.push_str("  ; Return cleanup complete\n");
        
        Ok(())
    }

    /// Generate cleanup for a single defer expression
    fn generate_defer_expression_cleanup(&self, expression: &Expression, ir: &mut String) -> Result<(), CursedError> {
        match expression {
            Expression::FunctionCall(call) => {
                ir.push_str(&format!("  ; Cleanup function call: {}\n", call.name));
                
                match &call.name[..] {
                    "vibez.spill" => {
                        if let Some(arg) = call.arguments.first() {
                            match arg {
                                Expression::StringLiteral(s) => {
                                    let str_len = s.len() + 1;
                                    ir.push_str(&format!("  call i32 (i8*, ...) @printf(i8* getelementptr ([{}x i8], [{}x i8]* @.str_defer_{}, i32 0, i32 0))\n", 
                                        str_len, str_len, self.get_string_id(s)));
                                },
                                _ => {
                                    ir.push_str("  call void @defer_complex_arg_cleanup()\n");
                                }
                            }
                        }
                    },
                    _ => {
                        // Generate call to cleanup function
                        ir.push_str(&format!("  call void @{}()\n", call.name));
                    }
                }
            },
            Expression::Identifier(name) => {
                ir.push_str(&format!("  ; Cleanup identifier: {}\n", name));
                ir.push_str(&format!("  call void @cleanup_{}()\n", name));
            },
            _ => {
                ir.push_str("  ; Generic defer expression cleanup\n");
                ir.push_str("  call void @defer_generic_cleanup()\n");
            }
        }
        
        Ok(())
    }

    /// Get next unique label
    fn get_next_label(&mut self, prefix: &str) -> String {
        let label = format!("{}.{}", prefix, self.label_counter);
        self.label_counter += 1;
        label
    }

    /// Get string ID for string literals
    fn get_string_id(&self, s: &str) -> usize {
        // Simple hash for string ID
        s.chars().map(|c| c as usize).sum::<usize>() % 1000
    }

    /// Generate cleanup call for function returns
    pub fn generate_return_cleanup_call(&self, ir: &mut String) -> Result<(), CursedError> {
        if let Some(cleanup_label) = self.cleanup_labels.get("return") {
            ir.push_str(&format!("  br label %{}\n", cleanup_label));
        }
        Ok(())
    }

    /// Generate cleanup call for normal exits
    pub fn generate_normal_cleanup_call(&self, ir: &mut String) -> Result<(), CursedError> {
        if let Some(cleanup_label) = self.cleanup_labels.get("normal") {
            ir.push_str(&format!("  br label %{}\n", cleanup_label));
        }
        Ok(())
    }

    /// Generate cleanup call for exceptions
    pub fn generate_exception_cleanup_call(&self, ir: &mut String) -> Result<(), CursedError> {
        if let Some(cleanup_label) = self.cleanup_labels.get("exception") {
            ir.push_str(&format!("  br label %{}\n", cleanup_label));
        }
        Ok(())
    }

    /// Check if there are any defer expressions
    pub fn has_defers(&self) -> bool {
        !self.defer_stack.is_empty()
    }

    /// Get defer count
    pub fn defer_count(&self) -> usize {
        self.defer_stack.len()
    }

    /// Clear all defer expressions
    pub fn clear(&mut self) {
        self.defer_stack.clear();
        self.cleanup_labels.clear();
    }
}

/// Integrate enhanced defer system with existing function compiler
pub fn integrate_enhanced_defer_system(
    defer_system: &mut EnhancedDeferSystem,
    function_ir: &mut String,
    function_name: &str,
) -> Result<(), CursedError> {
    // Set function context
    defer_system.set_current_function(function_name.to_string());
    
    // Generate all cleanup blocks
    defer_system.generate_cleanup_blocks(function_ir)?;
    
    Ok(())
}
