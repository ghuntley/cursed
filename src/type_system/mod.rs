// Type system modules for CURSED
pub mod type_inference;
pub mod generic_instantiator;
pub mod constraint_resolver;
pub mod associated_types;
pub mod variance;
pub mod higher_kinded_types;

// Import base types from core and AST
pub use crate::core::{Type, TypeChecker};
pub use crate::ast::types::Type as AstType;

// Re-export key types
pub use type_inference::TypeInference;

// Core type system structures
#[derive(Debug, Clone)]
pub struct TypeSystem {
    pub checker: TypeChecker,
}

#[derive(Debug, Clone)]
pub struct TypeEnvironment {
    // Type environment for inference
}

// TypeInference is re-exported from type_inference module

#[derive(Debug, Clone)]
pub struct TypeSubstitution {
    // Type substitution
}

#[derive(Debug, Clone)]
pub struct TypeExpression {
    // Type expression
}

#[derive(Debug, Clone)]
pub struct TypeDefinition {
    // Type definition
}

#[derive(Debug, Clone)]
pub struct TypeKind {
    // Type kind
}

#[derive(Debug, Clone)]
pub struct InstantiatedType {
    pub base_type: String,
    pub type_arguments: Vec<TypeExpression>,
}

#[derive(Debug, Clone)]
pub struct MethodSignature {
    // Method signature
}

#[derive(Debug, Clone)]
pub struct ConstraintContext {
    pub constraints: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ConstraintBinding {
    pub name: String,
    pub bound_type: TypeExpression,
}

#[derive(Debug, Clone)]
pub enum ConstraintStatus {
    Satisfied,
    Unsatisfied,
    Pending,
}

#[derive(Debug, Clone)]
pub struct InferenceContext {
    // Inference context
}

impl Default for TypeSystem {
    fn default() -> Self {
        Self {
            checker: TypeChecker::new(),
        }
    }
}

impl Default for TypeEnvironment {
    fn default() -> Self {
        Self {}
    }
}

impl Default for TypeInference {
    fn default() -> Self {
        Self {}
    }
}

// Re-exports
pub use generic_instantiator::GenericInstantiator;
