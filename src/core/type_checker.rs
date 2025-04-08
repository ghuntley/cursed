use crate::ast::{Statement, Expression};
use crate::ast::base::Program;
use crate::error::Error;
use std::collections::HashMap;

/// A type in the CURSED type system
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    /// Basic types
    Lit,                                 // Boolean
    Smol,                                // int8
    Mid,                                 // int16
    Normie,                              // int32
    Thicc,                               // int64
    Snack,                               // float32
    Meal,                                // float64
    Tea,                                 // string
    Sip,                                 // character (rune)
    Byte,                                // byte
    
    /// Composite types
    Array(Box<Type>, usize),             // Array[T, size]
    Slice(Box<Type>),                    // []T
    Map(Box<Type>, Box<Type>),           // tea[K]V
    Struct(String, Vec<Box<Type>>),      // squad with name and type parameters
    Interface(String, Vec<Box<Type>>),   // collab with name and type parameters
    Pointer(Box<Type>),                  // @T
    Function(Vec<Box<Type>>, Box<Type>), // slay(params) returnType
    Channel(Box<Type>),                  // dm<T>
    
    /// Type parameters for generics
    TypeParam(String),                   // Generic type parameter (T, K, etc.)
    
    /// Unknown type (for inference)
    Unknown,
}

impl ToString for Type {
    fn to_string(&self) -> String {
        match self {
            Type::Lit => "lit".to_string(),
            Type::Smol => "smol".to_string(),
            Type::Mid => "mid".to_string(), 
            Type::Normie => "normie".to_string(),
            Type::Thicc => "thicc".to_string(),
            Type::Snack => "snack".to_string(),
            Type::Meal => "meal".to_string(),
            Type::Tea => "tea".to_string(),
            Type::Sip => "sip".to_string(),
            Type::Byte => "byte".to_string(),
            
            Type::Array(t, size) => format!("[{}]{}", size, t.to_string()),
            Type::Slice(t) => format!("[]{}", t.to_string()),
            Type::Map(k, v) => format!("tea[{}]{}", k.to_string(), v.to_string()),
            Type::Struct(name, params) => {
                if params.is_empty() {
                    name.clone()
                } else {
                    let params_str = params.iter()
                        .map(|p| p.to_string())
                        .collect::<Vec<String>>()
                        .join(", ");
                    format!("{}[{}]", name, params_str)
                }
            },
            Type::Interface(name, params) => {
                if params.is_empty() {
                    name.clone()
                } else {
                    let params_str = params.iter()
                        .map(|p| p.to_string())
                        .collect::<Vec<String>>()
                        .join(", ");
                    format!("{}[{}]", name, params_str)
                }
            },
            Type::Pointer(t) => format!("@{}", t.to_string()),
            Type::Function(params, ret) => {
                let params_str = params.iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("slay({}) {}", params_str, ret.to_string())
            },
            Type::Channel(t) => format!("dm<{}>", t.to_string()),
            Type::TypeParam(name) => name.clone(),
            Type::Unknown => "unknown".to_string(),
        }
    }
}

/// Maps symbol names to their types
type TypeEnv = HashMap<String, Type>;

/// Maps type parameter names to their concrete types
type TypeMap = HashMap<String, Type>;

/// Type checker for CURSED programs
pub struct TypeChecker {
    global_env: TypeEnv,
    local_env: Vec<TypeEnv>,
    types: HashMap<String, Type>,  // Maps type names to their definitions
    current_type_params: Vec<TypeMap>, // Stack of type parameter mappings for the current context
}

impl TypeChecker {
    /// Create a new type checker
    pub fn new() -> Self {
        TypeChecker {
            global_env: HashMap::new(),
            local_env: vec![HashMap::new()],
            types: Self::initialize_builtin_types(),
            current_type_params: Vec::new(),
        }
    }
    
    /// Initialize built-in types
    fn initialize_builtin_types() -> HashMap<String, Type> {
        let mut types = HashMap::new();
        types.insert("lit".to_string(), Type::Lit);
        types.insert("smol".to_string(), Type::Smol);
        types.insert("mid".to_string(), Type::Mid);
        types.insert("normie".to_string(), Type::Normie);
        types.insert("thicc".to_string(), Type::Thicc);
        types.insert("snack".to_string(), Type::Snack);
        types.insert("meal".to_string(), Type::Meal);
        types.insert("tea".to_string(), Type::Tea);
        types.insert("sip".to_string(), Type::Sip);
        types.insert("byte".to_string(), Type::Byte);
        types
    }
    
    /// Push a new local environment
    fn push_env(&mut self) {
        self.local_env.push(HashMap::new());
    }
    
    /// Pop the current local environment
    fn pop_env(&mut self) {
        if self.local_env.len() > 1 {
            self.local_env.pop();
        }
    }
    
    /// Get the type of a variable
    pub fn get_type(&self, name: &str) -> Option<Type> {
        // Check local environments from innermost to outermost
        for env in self.local_env.iter().rev() {
            if let Some(t) = env.get(name) {
                return Some(t.clone());
            }
        }
        
        // Check global environment
        self.global_env.get(name).cloned()
    }
    
    /// Set the type of a variable in the current environment
    fn set_type(&mut self, name: &str, ty: Type) {
        // Always set in the innermost environment
        if let Some(env) = self.local_env.last_mut() {
            env.insert(name.to_string(), ty);
        }
    }
    
    /// Check if two types are compatible
    fn types_compatible(&self, expected: &Type, actual: &Type) -> bool {
        match (expected, actual) {
            // Same concrete types are compatible
            (t1, t2) if t1 == t2 => true,
            
            // Type parameters match their concrete types
            (Type::TypeParam(param), _) => {
                // Check if we have a concrete type for this parameter
                for type_map in self.current_type_params.iter().rev() {
                    if let Some(concrete) = type_map.get(param) {
                        return self.types_compatible(concrete, actual);
                    }
                }
                
                // If no concrete type is found, assume compatibility
                // (we'll infer the type parameter from usage)
                true
            },
            
            // Check compatibility of composite types
            (Type::Array(t1, s1), Type::Array(t2, s2)) => {
                s1 == s2 && self.types_compatible(t1, t2)
            },
            
            (Type::Slice(t1), Type::Slice(t2)) => {
                self.types_compatible(t1, t2)
            },
            
            (Type::Map(k1, v1), Type::Map(k2, v2)) => {
                self.types_compatible(k1, k2) && self.types_compatible(v1, v2)
            },
            
            (Type::Struct(n1, p1), Type::Struct(n2, p2)) => {
                if n1 != n2 || p1.len() != p2.len() {
                    return false;
                }
                
                for (param1, param2) in p1.iter().zip(p2.iter()) {
                    if !self.types_compatible(param1, param2) {
                        return false;
                    }
                }
                
                true
            },
            
            // Other combinations are not compatible
            _ => false,
        }
    }
    
    /// Push a new type parameter mapping
    fn push_type_params(&mut self, params: TypeMap) {
        self.current_type_params.push(params);
    }
    
    /// Pop the current type parameter mapping
    fn pop_type_params(&mut self) {
        self.current_type_params.pop();
    }
    
    /// Convert a string type name to a Type
    fn string_to_type(&self, type_name: &str) -> Type {
        match type_name {
            "lit" => Type::Lit,
            "smol" => Type::Smol,
            "mid" => Type::Mid,
            "normie" => Type::Normie,
            "thicc" => Type::Thicc,
            "snack" => Type::Snack,
            "meal" => Type::Meal,
            "tea" => Type::Tea,
            "sip" => Type::Sip,
            "byte" => Type::Byte,
            _ => {
                // Check if it's a known type
                if let Some(ty) = self.types.get(type_name) {
                    ty.clone()
                } else {
                    // Check if it's a type parameter
                    for type_map in self.current_type_params.iter().rev() {
                        if let Some(concrete) = type_map.get(type_name) {
                            return concrete.clone();
                        }
                    }
                    
                    // Otherwise, assume it's a type parameter
                    Type::TypeParam(type_name.to_string())
                }
            }
        }
    }
    
    /// Type check a program
    pub fn check_program(&mut self, program: &Program) -> Result<(), Error> {
        // Check each statement in the program
        for statement in &program.statements {
            self.check_statement(statement)?;
        }
        
        Ok(())
    }
    
    /// Type check a statement
    fn check_statement(&mut self, statement: &Box<dyn Statement>) -> Result<(), Error> {
        // Check the type of the statement based on its concrete type
        // For each kind of statement, we need to implement type checking logic
        
        // At this stage, let's just support a basic framework without implementing
        // all statement types. We'll focus on generics functionality.
        
        Ok(())
    }
    
    /// Type check an expression
    fn check_expression(&mut self, expression: &Box<dyn Expression>) -> Result<Type, Error> {
        // At this stage, let's just support a basic framework without implementing
        // all expression types. We'll focus on generics functionality.
        
        Ok(Type::Unknown)
    }
}