//! Type checking for CURSED

use crate::error_types::Result;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    Bool,
    String,
    Void,
    Array(Box<Type>),
    Function(Vec<Type>, Box<Type>),
    Custom(String),
    Unknown,
    // CURSED-specific types
    Normie,              // Standard/basic integer type (normie)
    Tea,                 // String/information type (tea)
    Lit,                 // Boolean/truth type (lit)
    Sip,                 // Character type (sip)
    Squad(Box<Type>),    // Array/collection type (squad)
    Collab(String),      // Interface type (collab)
    Dm(Box<Type>),       // Channel type (dm<T>)
}

pub struct TypeChecker {
    types: std::collections::HashMap<String, Type>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            types: std::collections::HashMap::new(),
        }
    }

    pub fn check_type(&self, name: &str) -> Option<&Type> {
        self.types.get(name)
    }

    pub fn add_type(&mut self, name: String, type_def: Type) {
        self.types.insert(name, type_def);
    }

    pub fn type_check(&self, expression: &str) -> Result<Type> {
        // Enhanced type checking logic with CURSED-specific types
        match expression {
            // Boolean literals
            "true" | "based" => Ok(Type::Lit),
            "false" | "cap" => Ok(Type::Lit),
            "cringe" => Ok(Type::Unknown), // nil/null
            
            // String literals
            s if s.starts_with('"') && s.ends_with('"') => Ok(Type::Tea),
            
            // Character literals
            s if s.starts_with('\'') && s.ends_with('\'') && s.len() == 3 => Ok(Type::Sip),
            
            // Integer literals (various CURSED types)
            s if s.parse::<i8>().is_ok() => Ok(Type::Normie), // Default to normie for small integers
            s if s.parse::<i32>().is_ok() => Ok(Type::Normie),
            s if s.parse::<i64>().is_ok() => Ok(Type::Normie),
            
            // Float literals
            s if s.parse::<f32>().is_ok() => Ok(Type::Float),
            s if s.parse::<f64>().is_ok() => Ok(Type::Float),
            
            // Array literals
            s if s.starts_with('[') && s.ends_with(']') => {
                // Parse array type - for now return Array of Unknown
                Ok(Type::Squad(Box::new(Type::Unknown)))
            }
            
            // Map literals
            s if s.starts_with('{') && s.ends_with('}') && s.contains(':') => {
                // Map literal detected - return Map type
                Ok(Type::Custom("Map".to_string()))
            }
            
            // Channel types
            s if s.starts_with("chan ") => {
                // Channel type - extract inner type
                let inner_type = &s[5..];
                let inner = self.type_check(inner_type)?;
                Ok(Type::Dm(Box::new(inner)))
            }
            
            // Identifiers - check if they're known types
            s if s.chars().all(|c| c.is_alphabetic() || c == '_') => {
                // Check if this is a known variable type
                if let Some(t) = self.check_type(s) {
                    Ok(t.clone())
                } else {
                    // Unknown identifier - could be a variable name
                    Ok(Type::Unknown)
                }
            }
            
            _ => Ok(Type::Unknown),
        }
    }
}
