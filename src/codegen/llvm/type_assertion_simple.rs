//! Simple Type Assertion Implementation for CURSED
//!
//! This module provides a simplified type assertion implementation that focuses
//! on the essential functionality without complex type checking.

use crate::ast::{Expression, Type, TypeAssertionExpression};
use crate::error::CursedError;
use std::collections::HashMap;

/// Simple type assertion compiler
pub struct SimpleTypeAssertionCompiler {
    pub ir_code: String,
    pub register_counter: usize,
}

impl SimpleTypeAssertionCompiler {
    pub fn new() -> Self {
        Self {
            ir_code: String::new(),
            register_counter: 0,
        }
    }

    /// Get next register name
    fn next_register(&mut self) -> String {
        let reg = format!("%type_assert_{}", self.register_counter);
        self.register_counter += 1;
        reg
    }

    /// Compile type assertion expression (simplified)
    pub fn compile_type_assertion(&mut self, type_assertion: &TypeAssertionExpression, value_reg: &str) -> Result<String, CursedError> {
        // Get type IDs for runtime checking
        let source_type_id = self.get_simple_type_id(&type_assertion.value);
        let target_type_id = self.get_simple_type_id_from_type(&type_assertion.target_type);
        
        if type_assertion.is_safe {
            // Safe type assertion - return default value on failure
            self.compile_safe_type_assertion(value_reg, source_type_id, target_type_id)
        } else {
            // Unsafe type assertion - panic on failure
            self.compile_unsafe_type_assertion(value_reg, source_type_id, target_type_id)
        }
    }

    /// Compile safe type assertion
    fn compile_safe_type_assertion(&mut self, value_reg: &str, source_type_id: u32, target_type_id: u32) -> Result<String, CursedError> {
        let check_reg = self.next_register();
        let result_reg = self.next_register();
        let cast_reg = self.next_register();
        let default_reg = self.next_register();
        
        // Generate type check
        self.ir_code.push_str(&format!("  {} = call i1 @cursed_check_type_compatibility(i8* {}, i32 {}, i32 {})\n", 
                                       check_reg, value_reg, source_type_id, target_type_id));
        
        // Generate cast or default value
        self.ir_code.push_str(&format!("  {} = call i8* @cursed_cast_type(i8* {}, i32 {}, i32 {})\n", 
                                       cast_reg, value_reg, source_type_id, target_type_id));
        
        self.ir_code.push_str(&format!("  {} = call i8* @cursed_null_value()\n", default_reg));
        
        // Select result based on type check
        self.ir_code.push_str(&format!("  {} = select i1 {}, i8* {}, i8* {}\n", 
                                       result_reg, check_reg, cast_reg, default_reg));
        
        Ok(result_reg)
    }

    /// Compile unsafe type assertion
    fn compile_unsafe_type_assertion(&mut self, value_reg: &str, source_type_id: u32, target_type_id: u32) -> Result<String, CursedError> {
        let check_reg = self.next_register();
        let result_reg = self.next_register();
        let panic_block = format!("type_assert_panic_{}", self.register_counter);
        let success_block = format!("type_assert_success_{}", self.register_counter);
        
        // Generate type check
        self.ir_code.push_str(&format!("  {} = call i1 @cursed_check_type_compatibility(i8* {}, i32 {}, i32 {})\n", 
                                       check_reg, value_reg, source_type_id, target_type_id));
        
        // Branch on type check result
        self.ir_code.push_str(&format!("  br i1 {}, label %{}, label %{}\n", 
                                       check_reg, success_block, panic_block));
        
        // Panic block
        self.ir_code.push_str(&format!("{}:\n", panic_block));
        self.ir_code.push_str(&format!("  call void @cursed_panic_type_assertion(i32 {}, i32 {})\n", 
                                       source_type_id, target_type_id));
        self.ir_code.push_str("  unreachable\n");
        
        // Success block
        self.ir_code.push_str(&format!("{}:\n", success_block));
        self.ir_code.push_str(&format!("  {} = call i8* @cursed_cast_type(i8* {}, i32 {}, i32 {})\n", 
                                       result_reg, value_reg, source_type_id, target_type_id));
        
        Ok(result_reg)
    }

    /// Get simple type ID from expression
    fn get_simple_type_id(&self, expr: &Expression) -> u32 {
        match expr {
            Expression::Integer(_) => 1,   // Integer
            Expression::Float(_) => 2,     // Float
            Expression::String(_) => 3,    // String
            Expression::Boolean(_) => 4,   // Boolean
            _ => 999,                      // Unknown
        }
    }

    /// Get simple type ID from Type enum
    fn get_simple_type_id_from_type(&self, type_: &Type) -> u32 {
        match type_ {
            Type::Integer | Type::Normie => 1,
            Type::Float | Type::Snack | Type::Meal => 2,
            Type::String | Type::Tea => 3,
            Type::Boolean | Type::Lit => 4,
            Type::Byte => 5,
            Type::Sip => 6,
            _ => 999,
        }
    }
}

impl Default for SimpleTypeAssertionCompiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_type_ids() {
        let compiler = SimpleTypeAssertionCompiler::new();
        
        assert_eq!(compiler.get_simple_type_id_from_type(&Type::Integer), 1);
        assert_eq!(compiler.get_simple_type_id_from_type(&Type::Float), 2);
        assert_eq!(compiler.get_simple_type_id_from_type(&Type::String), 3);
        assert_eq!(compiler.get_simple_type_id_from_type(&Type::Boolean), 4);
    }

    #[test]
    fn test_register_generation() {
        let mut compiler = SimpleTypeAssertionCompiler::new();
        
        assert_eq!(compiler.next_register(), "%type_assert_0");
        assert_eq!(compiler.next_register(), "%type_assert_1");
        assert_eq!(compiler.next_register(), "%type_assert_2");
    }
}
