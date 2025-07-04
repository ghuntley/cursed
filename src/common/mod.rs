//! Common types and utilities for CURSED - ADVANCED FEATURES ENABLED

use crate::error::CursedError;

/// Optimization levels for CURSED compilation
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OptimizationLevel {
    None,           // O0 - No optimization
    Less,           // O1 - Minimal optimization
    Default,        // O2 - Standard optimization
    Aggressive,     // O3 - Maximum optimization
    Size,           // Os - Size optimization
    SizeAggressive, // Oz - Aggressive size optimization
    Production,     // Production-level optimization
}

impl Default for OptimizationLevel {
    fn default() -> Self {
        OptimizationLevel::Default
    }
}

impl std::fmt::Display for OptimizationLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OptimizationLevel::None => write!(f, "O0"),
            OptimizationLevel::Less => write!(f, "O1"),
            OptimizationLevel::Default => write!(f, "O2"),
            OptimizationLevel::Aggressive => write!(f, "O3"),
            OptimizationLevel::Size => write!(f, "Os"),
            OptimizationLevel::SizeAggressive => write!(f, "Oz"),
            OptimizationLevel::Production => write!(f, "Production"),
        }
    }
}

/// Core value types for CURSED runtime
#[derive(Debug, Clone)]
pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Nil,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
        }
    }
}

/// Common result type for CURSED operations
pub type CursedResult<T> = Result<T, CursedError>;

/// Import error type
pub type ImportError = String;

/// Configuration for various CURSED systems
#[derive(Debug, Clone)]
pub struct Config {
    pub optimization_level: OptimizationLevel,
    pub debug_info: bool,
    pub jit_enabled: bool,
    pub gc_enabled: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            optimization_level: OptimizationLevel::default(),
            debug_info: true,
            jit_enabled: true,
            gc_enabled: true,
        }
    }
}

/// Utility functions
pub fn format_error(error: &CursedError) -> String {
    format!("CURSED Error: {}", error)
}

pub fn is_advanced_feature_enabled() -> bool {
    true // All advanced features are now enabled!
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimization_levels() {
        assert_eq!(OptimizationLevel::default().to_string(), "O2");
    }

    #[test]
    fn test_advanced_features() {
        assert!(is_advanced_feature_enabled());
    }
}
