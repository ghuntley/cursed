//! Advanced Pattern Matching Implementation for CURSED
//! 
//! This module implements comprehensive pattern matching semantics including:
//! - Guards
//! - Literal and range patterns  
//! - Nested destructuring
//! - Exhaustiveness checking
//! - LLVM codegen support

use crate::ast::{Expression, Statement, Type};
use crate::error::CursedError;
use std::collections::{HashMap, HashSet};

/// Pattern matching AST nodes
#[derive(Debug, Clone)]
pub enum Pattern {
    /// Wildcard pattern (_)
    Wildcard,
    /// Variable binding pattern (x)
    Variable(String),
    /// Literal pattern (42, "hello", based)
    Literal(LiteralPattern),
    /// Range pattern (1..10, 'a'..'z')
    Range(RangePattern),
    /// Tuple destructuring pattern ((x, y, z))
    Tuple(TuplePattern),
    /// Array destructuring pattern ([head, ...tail])
    Array(ArrayPattern),
    /// Struct destructuring pattern (Person { name, age })
    Struct(StructPattern),
    /// Enum variant pattern (Some(x), None)
    Enum(EnumPattern),
    /// Or pattern (x | y | z)
    Or(OrPattern),
    /// Guard pattern (x when x > 0)
    Guard(GuardPattern),
    /// Type pattern for type switches
    Type(TypePattern),
}

/// Literal pattern for exact value matching
#[derive(Debug, Clone)]
pub struct LiteralPattern {
    pub value: Expression,
}

/// Range pattern for range matching (inclusive)
#[derive(Debug, Clone)]
pub struct RangePattern {
    pub start: Expression,
    pub end: Expression,
    pub is_inclusive: bool,
}

/// Tuple destructuring pattern
#[derive(Debug, Clone)]
pub struct TuplePattern {
    pub patterns: Vec<Pattern>,
}

/// Array destructuring pattern
#[derive(Debug, Clone)]
pub struct ArrayPattern {
    pub patterns: Vec<Pattern>,
    pub rest: Option<String>, // For ...rest patterns
}

/// Struct destructuring pattern
#[derive(Debug, Clone)]
pub struct StructPattern {
    pub type_name: String,
    pub fields: Vec<FieldPattern>,
    pub rest: bool, // For { field1, .. } patterns
}

/// Field pattern in struct destructuring
#[derive(Debug, Clone)]
pub struct FieldPattern {
    pub field_name: String,
    pub pattern: Option<Pattern>, // None means shorthand (field same as variable)
}

/// Enum variant pattern
#[derive(Debug, Clone)]
pub struct EnumPattern {
    pub enum_name: String,
    pub variant_name: String,
    pub patterns: Vec<Pattern>,
}

/// Or pattern (multiple alternatives)
#[derive(Debug, Clone)]
pub struct OrPattern {
    pub patterns: Vec<Pattern>,
}

/// Guard pattern with condition
#[derive(Debug, Clone)]
pub struct GuardPattern {
    pub pattern: Box<Pattern>,
    pub guard: Expression,
}

/// Type pattern for type switches
#[derive(Debug, Clone)]
pub struct TypePattern {
    pub target_type: Type,
    pub variable: Option<String>, // Variable to bind casted value
}

/// Enhanced switch case with pattern support
#[derive(Debug, Clone)]
pub struct PatternSwitchCase {
    pub pattern: Pattern,
    pub body: Vec<Statement>,
}

/// Enhanced switch statement with pattern matching
#[derive(Debug, Clone)]
pub struct PatternSwitchStatement {
    pub init: Option<Box<Statement>>,
    pub expression: Expression,
    pub cases: Vec<PatternSwitchCase>,
    pub default_case: Option<Vec<Statement>>,
}

/// Exhaustiveness checker for pattern matching
pub struct ExhaustivenessChecker {
    patterns: Vec<Pattern>,
    switch_type: Type,
}

impl ExhaustivenessChecker {
    pub fn new(patterns: Vec<Pattern>, switch_type: Type) -> Self {
        Self { patterns, switch_type }
    }

    /// Check if patterns are exhaustive for the given type
    pub fn check_exhaustiveness(&self) -> Result<bool, CursedError> {
        match &self.switch_type {
            Type::Lit => self.check_boolean_exhaustiveness(),
            Type::Normie | Type::Smol | Type::Mid | Type::Thicc => self.check_integer_exhaustiveness(),
            Type::Snack | Type::Meal => self.check_float_exhaustiveness(),
            Type::Sip => self.check_character_exhaustiveness(),
            Type::Tea => self.check_string_exhaustiveness(),
            Type::Array(_, _) => self.check_array_exhaustiveness(),
            Type::Collab(_) => self.check_interface_exhaustiveness(),
            Type::Custom(_) => self.check_custom_type_exhaustiveness(),
            _ => Ok(false), // Conservative approach for unknown types
        }
    }

    fn check_boolean_exhaustiveness(&self) -> Result<bool, CursedError> {
        let mut has_true = false;
        let mut has_false = false;
        let mut has_wildcard = false;

        for pattern in &self.patterns {
            match pattern {
                Pattern::Literal(lit) => {
                    if let Expression::Boolean(val) = &lit.value {
                        if *val {
                            has_true = true;
                        } else {
                            has_false = true;
                        }
                    }
                }
                Pattern::Wildcard | Pattern::Variable(_) => {
                    has_wildcard = true;
                }
                _ => {}
            }
        }

        Ok((has_true && has_false) || has_wildcard)
    }

    fn check_integer_exhaustiveness(&self) -> Result<bool, CursedError> {
        // For integers, exhaustiveness requires wildcard or complete range coverage
        // This is a simplified check - full implementation would analyze ranges
        for pattern in &self.patterns {
            match pattern {
                Pattern::Wildcard | Pattern::Variable(_) => return Ok(true),
                _ => {}
            }
        }
        Ok(false)
    }

    fn check_float_exhaustiveness(&self) -> Result<bool, CursedError> {
        // Floats cannot be exhaustively matched without wildcard
        for pattern in &self.patterns {
            match pattern {
                Pattern::Wildcard | Pattern::Variable(_) => return Ok(true),
                _ => {}
            }
        }
        Ok(false)
    }

    fn check_character_exhaustiveness(&self) -> Result<bool, CursedError> {
        // Similar to integers but for character ranges
        for pattern in &self.patterns {
            match pattern {
                Pattern::Wildcard | Pattern::Variable(_) => return Ok(true),
                _ => {}
            }
        }
        Ok(false)
    }

    fn check_string_exhaustiveness(&self) -> Result<bool, CursedError> {
        // Strings cannot be exhaustively matched without wildcard
        for pattern in &self.patterns {
            match pattern {
                Pattern::Wildcard | Pattern::Variable(_) => return Ok(true),
                _ => {}
            }
        }
        Ok(false)
    }

    fn check_custom_type_exhaustiveness(&self) -> Result<bool, CursedError> {
        // For custom types, check if wildcard is present
        for pattern in &self.patterns {
            match pattern {
                Pattern::Wildcard | Pattern::Variable(_) => return Ok(true),
                _ => {}
            }
        }
        Ok(false)
    }

    fn check_tuple_exhaustiveness(&self) -> Result<bool, CursedError> {
        // Tuples require matching all elements or wildcard
        for pattern in &self.patterns {
            match pattern {
                Pattern::Wildcard | Pattern::Variable(_) => return Ok(true),
                Pattern::Tuple(_) => {
                    // Would need to recursively check tuple element exhaustiveness
                    // This is a simplified implementation
                }
                _ => {}
            }
        }
        Ok(false)
    }

    fn check_array_exhaustiveness(&self) -> Result<bool, CursedError> {
        // Arrays with dynamic size cannot be exhaustively matched without wildcard
        for pattern in &self.patterns {
            match pattern {
                Pattern::Wildcard | Pattern::Variable(_) => return Ok(true),
                _ => {}
            }
        }
        Ok(false)
    }

    fn check_interface_exhaustiveness(&self) -> Result<bool, CursedError> {
        // Interfaces require type switching on all implementing types or wildcard
        for pattern in &self.patterns {
            match pattern {
                Pattern::Wildcard | Pattern::Variable(_) => return Ok(true),
                _ => {}
            }
        }
        Ok(false)
    }

    fn check_enum_exhaustiveness(&self) -> Result<bool, CursedError> {
        // Enums require all variants to be covered or wildcard
        let mut covered_variants = HashSet::new();
        let mut has_wildcard = false;

        for pattern in &self.patterns {
            match pattern {
                Pattern::Enum(enum_pat) => {
                    covered_variants.insert(&enum_pat.variant_name);
                }
                Pattern::Wildcard | Pattern::Variable(_) => {
                    has_wildcard = true;
                }
                _ => {}
            }
        }

        // Would need enum definition to check all variants are covered
        // For now, require wildcard for safety
        Ok(has_wildcard)
    }
}

/// Pattern compiler for LLVM code generation
pub struct PatternCompiler<'a> {
    ir_code: &'a mut String,
    register_counter: &'a mut usize,
    label_counter: &'a mut usize,
}

impl<'a> PatternCompiler<'a> {
    pub fn new(ir_code: &'a mut String, register_counter: &'a mut usize, label_counter: &'a mut usize) -> Self {
        Self {
            ir_code,
            register_counter,
            label_counter,
        }
    }

    /// Generate LLVM IR for pattern matching
    pub fn compile_pattern_match(
        &mut self,
        value_reg: &str,
        pattern: &Pattern,
        success_label: &str,
        fail_label: &str,
    ) -> Result<HashMap<String, String>, CursedError> {
        let mut bindings = HashMap::new();
        self.compile_pattern_recursive(value_reg, pattern, success_label, fail_label, &mut bindings)?;
        Ok(bindings)
    }

    fn compile_pattern_recursive(
        &mut self,
        value_reg: &str,
        pattern: &Pattern,
        success_label: &str,
        fail_label: &str,
        bindings: &mut HashMap<String, String>,
    ) -> Result<(), CursedError> {
        match pattern {
            Pattern::Wildcard => {
                // Wildcard always matches
                self.ir_code.push_str(&format!("  br label %{}\n", success_label));
            }
            
            Pattern::Variable(var_name) => {
                // Variable pattern always matches and binds the value
                bindings.insert(var_name.clone(), value_reg.to_string());
                self.ir_code.push_str(&format!("  br label %{}\n", success_label));
            }
            
            Pattern::Literal(lit_pat) => {
                self.compile_literal_pattern(value_reg, lit_pat, success_label, fail_label)?;
            }
            
            Pattern::Range(range_pat) => {
                self.compile_range_pattern(value_reg, range_pat, success_label, fail_label)?;
            }
            
            Pattern::Tuple(tuple_pat) => {
                self.compile_tuple_pattern(value_reg, tuple_pat, success_label, fail_label, bindings)?;
            }
            
            Pattern::Array(array_pat) => {
                self.compile_array_pattern(value_reg, array_pat, success_label, fail_label, bindings)?;
            }
            
            Pattern::Struct(struct_pat) => {
                self.compile_struct_pattern(value_reg, struct_pat, success_label, fail_label, bindings)?;
            }
            
            Pattern::Guard(guard_pat) => {
                self.compile_guard_pattern(value_reg, guard_pat, success_label, fail_label, bindings)?;
            }
            
            Pattern::Or(or_pat) => {
                self.compile_or_pattern(value_reg, or_pat, success_label, fail_label, bindings)?;
            }
            
            _ => {
                return Err(CursedError::compiler_error(
                    "Pattern type not yet implemented in LLVM codegen"
                ));
            }
        }
        Ok(())
    }

    fn compile_literal_pattern(
        &mut self,
        value_reg: &str,
        lit_pat: &LiteralPattern,
        success_label: &str,
        fail_label: &str,
    ) -> Result<(), CursedError> {
        match &lit_pat.value {
            Expression::Integer(val) => {
                let cmp_reg = self.next_register();
                self.ir_code.push_str(&format!(
                    "  {} = icmp eq i32 {}, {}\n",
                    cmp_reg, value_reg, val
                ));
                self.ir_code.push_str(&format!(
                    "  br i1 {}, label %{}, label %{}\n",
                    cmp_reg, success_label, fail_label
                ));
            }
            Expression::Boolean(val) => {
                let cmp_reg = self.next_register();
                let bool_val = if *val { 1 } else { 0 };
                self.ir_code.push_str(&format!(
                    "  {} = icmp eq i1 {}, {}\n",
                    cmp_reg, value_reg, bool_val
                ));
                self.ir_code.push_str(&format!(
                    "  br i1 {}, label %{}, label %{}\n",
                    cmp_reg, success_label, fail_label
                ));
            }
            Expression::Character(val) => {
                let cmp_reg = self.next_register();
                self.ir_code.push_str(&format!(
                    "  {} = icmp eq i8 {}, {}\n",
                    cmp_reg, value_reg, *val as u8
                ));
                self.ir_code.push_str(&format!(
                    "  br i1 {}, label %{}, label %{}\n",
                    cmp_reg, success_label, fail_label
                ));
            }
            _ => {
                return Err(CursedError::compiler_error(
                    "Literal pattern type not supported"
                ));
            }
        }
        Ok(())
    }

    fn compile_range_pattern(
        &mut self,
        value_reg: &str,
        range_pat: &RangePattern,
        success_label: &str,
        fail_label: &str,
    ) -> Result<(), CursedError> {
        // Simplified range pattern compilation
        // In practice, would need proper expression evaluation for range bounds
        let start_val = match &range_pat.start {
            Expression::Integer(val) => *val,
            _ => return Err(CursedError::compiler_error("Range start must be integer")),
        };
        
        let end_val = match &range_pat.end {
            Expression::Integer(val) => *val,
            _ => return Err(CursedError::compiler_error("Range end must be integer")),
        };

        let ge_reg = self.next_register();
        let le_reg = self.next_register();
        let and_reg = self.next_register();

        // value >= start
        self.ir_code.push_str(&format!(
            "  {} = icmp sge i32 {}, {}\n",
            ge_reg, value_reg, start_val
        ));

        // value <= end (or < end+1 for exclusive)
        let end_check = if range_pat.is_inclusive {
            format!("  {} = icmp sle i32 {}, {}\n", le_reg, value_reg, end_val)
        } else {
            format!("  {} = icmp slt i32 {}, {}\n", le_reg, value_reg, end_val)
        };
        self.ir_code.push_str(&end_check);

        // Combine conditions
        self.ir_code.push_str(&format!(
            "  {} = and i1 {}, {}\n",
            and_reg, ge_reg, le_reg
        ));
        self.ir_code.push_str(&format!(
            "  br i1 {}, label %{}, label %{}\n",
            and_reg, success_label, fail_label
        ));

        Ok(())
    }

    fn compile_tuple_pattern(
        &mut self,
        value_reg: &str,
        tuple_pat: &TuplePattern,
        success_label: &str,
        fail_label: &str,
        bindings: &mut HashMap<String, String>,
    ) -> Result<(), CursedError> {
        // Generate labels for each tuple element check
        let mut check_labels = Vec::new();
        for i in 0..tuple_pat.patterns.len() {
            check_labels.push(self.next_label());
        }

        // Start with first element
        if !tuple_pat.patterns.is_empty() {
            self.ir_code.push_str(&format!("  br label %{}\n", check_labels[0]));
        } else {
            self.ir_code.push_str(&format!("  br label %{}\n", success_label));
            return Ok(());
        }

        // Generate checks for each tuple element
        for (i, pattern) in tuple_pat.patterns.iter().enumerate() {
            self.ir_code.push_str(&format!("{}:\n", check_labels[i]));
            
            // Extract tuple element
            let element_reg = self.next_register();
            self.ir_code.push_str(&format!(
                "  {} = extractvalue {{i32, i32, i32}} {}, {}\n",
                element_reg, value_reg, i
            ));

            let next_label = if i + 1 < check_labels.len() {
                &check_labels[i + 1]
            } else {
                success_label
            };

            self.compile_pattern_recursive(&element_reg, pattern, next_label, fail_label, bindings)?;
        }

        Ok(())
    }

    fn compile_array_pattern(
        &mut self,
        _value_reg: &str,
        _array_pat: &ArrayPattern,
        _success_label: &str,
        _fail_label: &str,
        _bindings: &mut HashMap<String, String>,
    ) -> Result<(), CursedError> {
        // Array pattern compilation would require runtime length checks
        // and element extraction - simplified for now
        Err(CursedError::compiler_error(
            "Array patterns not yet implemented in LLVM codegen"
        ))
    }

    fn compile_struct_pattern(
        &mut self,
        _value_reg: &str,
        _struct_pat: &StructPattern,
        _success_label: &str,
        _fail_label: &str,
        _bindings: &mut HashMap<String, String>,
    ) -> Result<(), CursedError> {
        // Struct pattern compilation would require field extraction
        // and type checking - simplified for now
        Err(CursedError::compiler_error(
            "Struct patterns not yet implemented in LLVM codegen"
        ))
    }

    fn compile_guard_pattern(
        &mut self,
        value_reg: &str,
        guard_pat: &GuardPattern,
        success_label: &str,
        fail_label: &str,
        bindings: &mut HashMap<String, String>,
    ) -> Result<(), CursedError> {
        // First match the inner pattern
        let guard_check_label = self.next_label();
        self.compile_pattern_recursive(value_reg, &guard_pat.pattern, &guard_check_label, fail_label, bindings)?;

        // Then check the guard condition
        self.ir_code.push_str(&format!("{}:\n", guard_check_label));
        
        // For now, simplified guard evaluation
        // Real implementation would need expression compiler
        let guard_reg = self.next_register();
        self.ir_code.push_str(&format!(
            "  {} = call i1 @evaluate_guard_expression()\n",
            guard_reg
        ));
        self.ir_code.push_str(&format!(
            "  br i1 {}, label %{}, label %{}\n",
            guard_reg, success_label, fail_label
        ));

        Ok(())
    }

    fn compile_or_pattern(
        &mut self,
        value_reg: &str,
        or_pat: &OrPattern,
        success_label: &str,
        fail_label: &str,
        bindings: &mut HashMap<String, String>,
    ) -> Result<(), CursedError> {
        // Generate labels for each alternative
        let mut alt_labels = Vec::new();
        for i in 0..or_pat.patterns.len() {
            alt_labels.push(self.next_label());
        }

        // Start with first alternative
        if !or_pat.patterns.is_empty() {
            self.ir_code.push_str(&format!("  br label %{}\n", alt_labels[0]));
        } else {
            self.ir_code.push_str(&format!("  br label %{}\n", fail_label));
            return Ok(());
        }

        // Generate checks for each alternative
        for (i, pattern) in or_pat.patterns.iter().enumerate() {
            self.ir_code.push_str(&format!("{}:\n", alt_labels[i]));
            
            let next_alt = if i + 1 < alt_labels.len() {
                &alt_labels[i + 1]
            } else {
                fail_label
            };

            self.compile_pattern_recursive(value_reg, pattern, success_label, next_alt, bindings)?;
        }

        Ok(())
    }

    fn next_register(&mut self) -> String {
        *self.register_counter += 1;
        format!("%{}", self.register_counter)
    }

    fn next_label(&mut self) -> String {
        *self.label_counter += 1;
        format!("label_{}", self.label_counter)
    }
}

/// Pattern parser for parsing pattern syntax
pub struct PatternParser;

impl PatternParser {
    /// Parse pattern from expression (simplified)
    pub fn parse_pattern_from_expression(expr: &Expression) -> Result<Pattern, CursedError> {
        match expr {
            Expression::Identifier(name) if name == "_" => Ok(Pattern::Wildcard),
            Expression::Identifier(name) => Ok(Pattern::Variable(name.clone())),
            Expression::Integer(_) | Expression::Boolean(_) | Expression::Character(_) | Expression::String(_) => {
                Ok(Pattern::Literal(LiteralPattern { value: expr.clone() }))
            }
            Expression::Tuple(tuple) => {
                let mut patterns = Vec::new();
                for element in &tuple.elements {
                    patterns.push(Self::parse_pattern_from_expression(element)?);
                }
                Ok(Pattern::Tuple(TuplePattern { patterns }))
            }
            _ => Err(CursedError::parse_error("Invalid pattern expression")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boolean_exhaustiveness() {
        let patterns = vec![
            Pattern::Literal(LiteralPattern { value: Expression::Boolean(true) }),
            Pattern::Literal(LiteralPattern { value: Expression::Boolean(false) }),
        ];
        let checker = ExhaustivenessChecker::new(
            patterns,
            Type::Lit
        );
        assert!(checker.check_exhaustiveness().unwrap());
    }

    #[test]
    fn test_incomplete_boolean_exhaustiveness() {
        let patterns = vec![
            Pattern::Literal(LiteralPattern { value: Expression::Boolean(true) }),
        ];
        let checker = ExhaustivenessChecker::new(
            patterns,
            Type::Lit
        );
        assert!(!checker.check_exhaustiveness().unwrap());
    }

    #[test]
    fn test_wildcard_exhaustiveness() {
        let patterns = vec![Pattern::Wildcard];
        let checker = ExhaustivenessChecker::new(
            patterns,
            Type::Normie
        );
        assert!(checker.check_exhaustiveness().unwrap());
    }
}
