use crate::ast::Type;

/// Temporary helper to convert Type enum to string for backward compatibility
pub fn type_to_string(t: &Option<Type>) -> Option<String> {
    t.as_ref().map(|t| format!("{}", t))
}

/// Convert Type enum to string with default fallback
pub fn type_to_string_with_default(t: &Option<Type>, default: &str) -> String {
    t.as_ref().map(|t| format!("{}", t)).unwrap_or_else(|| default.to_string())
}
