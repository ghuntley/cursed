//! Simple type switch LLVM codegen for CURSED compiler
//! 
//! This module provides basic runtime type checking for type switches.

use crate::ast::{Type, TypePattern, TypeSwitchExpression, TypeSwitchArm};
use crate::error::CursedError;

/// Generate type ID for runtime type checking
pub fn get_runtime_type_id(type_expr: &Type) -> u32 {
    match type_expr {
        Type::Normie => 1,
        Type::Smol => 2,
        Type::Mid => 3,
        Type::Thicc => 4,
        Type::Snack => 6,
        Type::Meal => 7,
        Type::Tea => 8,
        Type::Lit => 9,
        Type::Sip => 10,
        Type::Byte => 11,
        Type::Rune => 12,
        Type::Extra => 13,
        Type::Array(_, _) => 100,
        Type::Slice(_) => 101,
        Type::Pointer(_) => 102,
        Type::Collab(_) => 103,
        Type::Function(_, _) => 105,
        Type::Generic(_, _) => 106,
        Type::Dm(_) => 107,
        _ => 999, // Unknown type
    }
}

/// Generate LLVM type string for type casting
pub fn get_llvm_type_string(type_expr: &Type) -> &'static str {
    match type_expr {
        Type::Normie => "i32",
        Type::Smol => "i8",
        Type::Mid => "i16", 
        Type::Thicc => "i64",
        Type::Snack => "float",
        Type::Meal => "double",
        Type::Tea => "i8",
        Type::Lit => "i1",
        Type::Sip => "i8",
        Type::Byte => "i8",
        Type::Rune => "i32",
        Type::Extra => "{ double, double }",
        _ => "i8", // Generic fallback
    }
}

/// Check if type is an integer type
pub fn is_integer_type(type_expr: &Type) -> bool {
    matches!(type_expr, Type::Normie | Type::Smol | Type::Mid | Type::Thicc | Type::Byte | Type::Rune)
}

/// Check if type is a float type
pub fn is_float_type(type_expr: &Type) -> bool {
    matches!(type_expr, Type::Snack | Type::Meal | Type::Extra)
}

/// Generate runtime type checking function declarations
pub fn generate_runtime_type_declarations() -> String {
    let mut ir = String::new();
    
    // Runtime type info structure
    ir.push_str("%runtime_type_info = type { i32, i8*, i64, i64 }\n");
    
    // Declare runtime functions
    ir.push_str("declare %runtime_type_info* @cursed_get_runtime_type_info(i8*)\n");
    ir.push_str("declare i1 @cursed_implements_interface(i8*, i8*)\n");
    ir.push_str("declare i1 @cursed_is_integer_type(i8*)\n");
    ir.push_str("declare i1 @cursed_is_float_type(i8*)\n");
    ir.push_str("declare i1 @cursed_is_string_type(i8*)\n");
    ir.push_str("declare i1 @cursed_is_boolean_type(i8*)\n");
    ir.push_str("declare void @cursed_panic(i8*)\n");
    
    // Error message strings
    ir.push_str("@str_typeswitch_unhandled = private unnamed_addr constant [30 x i8] c\"Unhandled type switch case\\00\"\n");
    
    ir
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_type_ids() {
        assert_eq!(get_runtime_type_id(&Type::Normie), 1);
        assert_eq!(get_runtime_type_id(&Type::Tea), 8);
        assert_eq!(get_runtime_type_id(&Type::Lit), 9);
    }
    
    #[test]
    fn test_llvm_types() {
        assert_eq!(get_llvm_type_string(&Type::Normie), "i32");
        assert_eq!(get_llvm_type_string(&Type::Tea), "i8");
        assert_eq!(get_llvm_type_string(&Type::Lit), "i1");
    }
    
    #[test]
    fn test_type_checking() {
        assert!(is_integer_type(&Type::Normie));
        assert!(!is_integer_type(&Type::Tea));
        assert!(is_float_type(&Type::Snack));
        assert!(!is_float_type(&Type::Normie));
    }
}
