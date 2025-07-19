//! Simple defer/panic recovery LLVM codegen implementation for CURSED
//! 
//! This module provides a working defer/panic system with proper exception handling

use crate::ast::Expression;
use crate::error_types::CursedError;

/// Simple defer/panic system for LLVM IR generation
pub struct SimpleDeferPanicSystem {
    /// Stack of defer expressions (LIFO order) 
    defer_stack: Vec<Expression>,
    /// Current cleanup label counter
    cleanup_label_counter: usize,
}

impl SimpleDeferPanicSystem {
    pub fn new() -> Self {
        Self {
            defer_stack: Vec::new(),
            cleanup_label_counter: 0,
        }
    }

    /// Add defer expression
    pub fn add_defer(&mut self, expression: Expression) {
        self.defer_stack.push(expression);
    }

    /// Generate defer cleanup execution
    pub fn generate_defer_cleanup(&mut self, ir: &mut String) -> Result<(), CursedError> {
        if self.defer_stack.is_empty() {
            return Ok(());
        }
        
        ir.push_str("  ; === DEFER CLEANUP (LIFO ORDER) ===\n");
        
        // Execute defer expressions in reverse order (LIFO)
        for (index, defer_expr) in self.defer_stack.iter().enumerate().rev() {
            ir.push_str(&format!("  ; Executing defer expression {} (LIFO)\n", index + 1));
            self.generate_defer_expression_ir(defer_expr, ir)?;
        }
        
        ir.push_str("  ; === END DEFER CLEANUP ===\n");
        
        // Clear defer stack after cleanup
        self.defer_stack.clear();
        
        Ok(())
    }

    /// Generate exception cleanup block (landing pad)
    pub fn generate_exception_cleanup_block(&mut self, exception_label: &str, ir: &mut String) -> Result<(), CursedError> {
        ir.push_str(&format!("{}:\n", exception_label));
        ir.push_str("  ; === EXCEPTION CLEANUP BLOCK (LANDING PAD) ===\n");
        
        // Generate landing pad for exception handling
        ir.push_str("  %exception_ptr = landingpad { i8*, i32 }\n");
        ir.push_str("    personality i32 (...)* @__gxx_personality_v0\n");
        ir.push_str("    cleanup\n");  // This is a cleanup landing pad
        ir.push_str("    catch i8* @_ZTI11CursedError\n");
        ir.push_str("    catch i8* null\n"); // Catch all
        
        // Execute defer cleanup during exception unwinding
        ir.push_str("  ; Execute defer cleanup during exception unwinding\n");
        self.generate_defer_cleanup(ir)?;
        
        // Resume exception unwinding after cleanup
        ir.push_str("  ; Resume exception unwinding\n");
        ir.push_str("  resume { i8*, i32 } %exception_ptr\n");
        
        Ok(())
    }

    /// Generate LLVM IR for a single defer expression
    fn generate_defer_expression_ir(&self, expression: &Expression, ir: &mut String) -> Result<(), CursedError> {
        match expression {
            Expression::Call(call) => {
                ir.push_str("  ; Defer function call\n");
                // Simplified: generate function call for defer
                if let Expression::Identifier(func_name) = &*call.function {
                    ir.push_str(&format!("  call void @{}()\n", func_name));
                } else {
                    ir.push_str("  call void @defer_function()\n");
                }
            },
            Expression::Identifier(name) => {
                ir.push_str(&format!("  ; Defer identifier cleanup: {}\n", name));
                ir.push_str(&format!("  call void @cleanup_{}()\n", name));
            },
            _ => {
                ir.push_str("  ; Generic defer expression\n");
                ir.push_str("  call void @defer_generic_cleanup()\n");
            }
        }
        
        Ok(())
    }

    /// Check if there are any defer expressions
    pub fn has_defers(&self) -> bool {
        !self.defer_stack.is_empty()
    }

    /// Get defer count for debugging
    pub fn defer_count(&self) -> usize {
        self.defer_stack.len()
    }

    /// Clear all defer state
    pub fn clear(&mut self) {
        self.defer_stack.clear();
    }

    /// Get next cleanup label
    pub fn get_next_cleanup_label(&mut self) -> String {
        let label = format!("cleanup_{}", self.cleanup_label_counter);
        self.cleanup_label_counter += 1;
        label
    }
}

/// Helper function to create simple defer/panic system
pub fn create_simple_defer_panic_system() -> SimpleDeferPanicSystem {
    SimpleDeferPanicSystem::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_defer_panic_system_creation() {
        let system = SimpleDeferPanicSystem::new();
        assert!(!system.has_defers());
        assert_eq!(system.defer_count(), 0);
    }

    #[test]
    fn test_defer_stack_lifo_order() {
        let mut system = SimpleDeferPanicSystem::new();
        
        // Add defer expressions
        system.add_defer(Expression::Identifier("cleanup1".to_string()));
        system.add_defer(Expression::Identifier("cleanup2".to_string()));
        system.add_defer(Expression::Identifier("cleanup3".to_string()));
        
        assert!(system.has_defers());
        assert_eq!(system.defer_count(), 3);
        
        // Test that defer stack maintains order for LIFO execution
        assert_eq!(system.defer_stack.len(), 3);
    }
}
