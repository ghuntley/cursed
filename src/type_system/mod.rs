/// Type System Infrastructure for CURSED Programming Language
///
/// This module provides the foundational type system infrastructure including:
/// - Generic constraint resolution and satisfaction checking
/// - Type parameter substitution and instantiation
/// - Constraint-based type inference with bidirectional checking
/// - Generic method dispatch and instance caching
/// - Integration with LLVM compilation infrastructure
///
/// The type system supports full generic programming with:
/// - Type parameters with complex constraints
/// - Higher-kinded types and associated types
/// - Constraint propagation and solving
/// - Performance-optimized instance caching

use crate::ast::declarations::{GenericConstraint, FunctionStatement};
use crate::ast::traits::TypeParameter;
use crate::error::Error;
use std::collections::HashMap;

// Progressive re-enabling of advanced type system modules
pub mod constraint_resolver;      // Re-enabled: constraint resolution system
pub mod generic_instantiator;     // Keep: works with sophisticated generics
pub mod type_inference;           // Keep: enhanced type inference
// TODO: Re-enable after fixing Type enum compatibility
// pub mod associated_types;         // Disabled: Type enum mismatch
// pub mod higher_kinded_types;      // Disabled: Type enum mismatch  
// pub mod variance;                 // Disabled: Type enum mismatch
// pub mod generic_optimization;     // Disabled: Type enum mismatch

// Re-export core types for convenience (progressive re-enabling)
pub use generic_instantiator::{
    GenericInstantiator, InstanceCache, TypeSubstitution, MethodDispatcher
};
pub use type_inference::{
    TypeInference, InferenceContext, BidirectionalChecker, ExpressionInferrer
};
pub use constraint_resolver::{
    ConstraintResolver, ConstraintSolution, ConstraintViolation, ViolationReason
};
// TODO: Re-enable these exports after fixing Type enum compatibility
// pub use associated_types::{
//     AssociatedTypeResolver, AssociatedTypeBinding, TypeProjection
// };
// pub use higher_kinded_types::{
//     HigherKindedTypeChecker, TypeConstructor, KindChecker
// };
// pub use variance::{
//     VarianceAnalyzer, TypeVariance, VarianceContext
// };
// pub use generic_optimization::{
//     GenericOptimizer, OptimizationStrategy, PerformanceMetrics
// };

/// Central type system coordinator (progressively enhanced with working modules)
#[derive(Debug)]
pub struct TypeSystem {
    /// Generic instantiator for type parameter substitution
    generic_instantiator: GenericInstantiator,
    /// Type inference engine for automatic type deduction
    type_inference: TypeInference,
    /// Constraint resolver for type constraint satisfaction
    constraint_resolver: ConstraintResolver,
    /// Global type environment
    type_environment: TypeEnvironment,
    // TODO: Re-enable after fixing Type enum compatibility
    // /// Associated type resolver for type projections
    // associated_type_resolver: AssociatedTypeResolver,
    // /// Higher-kinded type checker
    // higher_kinded_checker: HigherKindedTypeChecker,
    // /// Variance analyzer for type variance checking
    // variance_analyzer: VarianceAnalyzer,
    // /// Generic optimizer for performance improvements
    // generic_optimizer: GenericOptimizer,
}

/// Global type environment tracking basic types (simplified for AST compatibility)
#[derive(Debug, Clone)]
pub struct TypeEnvironment {
    /// Type definitions by name
    pub type_definitions: HashMap<String, TypeDefinition>,
    /// Generic instantiations cache
    pub instantiations: HashMap<String, Vec<InstantiatedType>>,
    /// Constraint context stack for type checking
    pub context_stack: Vec<ConstraintContext>,
}

/// Type definition with full metadata
#[derive(Debug, Clone)]
pub struct TypeDefinition {
    pub name: String,
    pub kind: TypeKind,
    pub type_parameters: Vec<TypeParameter>,
    pub constraints: Vec<GenericConstraint>,
    pub methods: Vec<MethodSignature>,
    pub is_builtin: bool,
}

/// Classification of type kinds
#[derive(Debug, Clone, PartialEq)]
pub enum TypeKind {
    /// Primitive types (normie, facts, tea)
    Primitive,
    /// Struct types (squad)
    Struct,
    /// Interface types (collab)
    Interface,
    /// Function types
    Function,
    /// Array types [T]
    Array(Box<TypeExpression>),
    /// Map types tea[K]V
    Map(Box<TypeExpression>, Box<TypeExpression>),
    /// Channel types dm T
    Channel(Box<TypeExpression>),
    /// Generic type parameter
    Parameter(String),
    /// Instantiated generic type
    Generic(String, Vec<TypeExpression>),
}

/// Type expression for complex type representations
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeExpression {
    /// Named type
    Named(String),
    /// Generic type with parameters
    Generic(String, Vec<TypeExpression>),
    /// Function type with parameters and return type
    Function(Vec<TypeExpression>, Box<TypeExpression>),
    /// Array type
    Array(Box<TypeExpression>),
    /// Map type
    Map(Box<TypeExpression>, Box<TypeExpression>),
    /// Channel type
    Channel(Box<TypeExpression>),
    /// Type parameter
    Parameter(String),
}

/// Method signature for interface and struct methods
#[derive(Debug, Clone)]
pub struct MethodSignature {
    pub name: String,
    pub parameters: Vec<TypeExpression>,
    pub return_type: Option<TypeExpression>,
    pub type_parameters: Vec<TypeParameter>,
    pub constraints: Vec<GenericConstraint>,
}

/// Instantiated type with concrete type arguments
#[derive(Debug, Clone)]
pub struct InstantiatedType {
    pub base_type: String,
    pub type_arguments: Vec<TypeExpression>,
    pub instance_id: String,
    pub resolved_type: TypeExpression,
}

/// Constraint context for managing scoped constraints
#[derive(Debug, Clone)]
pub struct ConstraintContext {
    pub scope_id: String,
    pub active_constraints: Vec<ConstraintBinding>,
    pub type_bindings: HashMap<String, TypeExpression>,
}

/// Binding of a constraint to specific types
#[derive(Debug, Clone)]
pub struct ConstraintBinding {
    pub constraint: GenericConstraint,
    pub bound_types: Vec<String>,
    pub satisfaction_status: ConstraintStatus,
}

/// Status of constraint satisfaction
#[derive(Debug, Clone, PartialEq)]
pub enum ConstraintStatus {
    /// Constraint is satisfied
    Satisfied,
    /// Constraint is not satisfied
    Violated(String),
    /// Constraint satisfaction is pending inference
    Pending,
    /// Constraint is conditionally satisfied
    Conditional(Vec<String>),
}

impl TypeSystem {
    /// Create a new type system instance
    pub fn new() -> Self {
        Self {
            generic_instantiator: GenericInstantiator::new(),
            type_inference: TypeInference::new(),
            constraint_resolver: ConstraintResolver::new(),
            type_environment: TypeEnvironment::new(),
            // TODO: Re-enable after fixing Type enum compatibility
            // associated_type_resolver: AssociatedTypeResolver::new(),
            // higher_kinded_checker: HigherKindedTypeChecker::new(),
            // variance_analyzer: VarianceAnalyzer::new(),
            // generic_optimizer: GenericOptimizer::new(),
        }
    }

    /// Initialize with built-in types
    pub fn with_builtins() -> Self {
        let mut system = Self::new();
        system.initialize_builtins();
        system
    }

    /// Initialize built-in primitive types
    pub fn initialize_builtins(&mut self) {
        let builtins = vec![
            ("normie", TypeKind::Primitive),
            ("facts", TypeKind::Primitive),
            ("tea", TypeKind::Primitive),
            ("sus", TypeKind::Primitive),
        ];

        for (name, kind) in builtins {
            let type_def = TypeDefinition {
                name: name.to_string(),
                kind,
                type_parameters: Vec::new(),
                constraints: Vec::new(),
                methods: Vec::new(),
                is_builtin: true,
            };
            self.type_environment.type_definitions.insert(name.to_string(), type_def);
        }
    }

    /// Register a new type definition
    pub fn register_type(&mut self, type_def: TypeDefinition) -> Result<(), Error> {
        // Validate type definition
        if self.type_environment.type_definitions.contains_key(&type_def.name) {
            return Err(Error::Type(format!("Type '{}' already defined", type_def.name)));
        }

        // Re-enabled constraint validation with constraint resolver
        for constraint in &type_def.constraints {
            self.constraint_resolver.validate_constraint(constraint, &self.type_environment)?;
        }

        self.type_environment.type_definitions.insert(type_def.name.clone(), type_def);
        Ok(())
    }

    /// Instantiate a generic type with concrete type arguments
    pub fn instantiate_generic(
        &mut self,
        base_type: &str,
        type_args: &[TypeExpression],
    ) -> Result<InstantiatedType, Error> {
        self.generic_instantiator.instantiate(
            base_type,
            type_args,
            &mut self.type_environment,
        )
    }

    /// Infer types for an expression
    pub fn infer_expression_type(
        &mut self,
        expression: &dyn crate::ast::traits::Expression,
        context: &InferenceContext,
    ) -> Result<TypeExpression, Error> {
        self.type_inference.infer_expression(expression, context, &self.type_environment)
    }

    /// Check if a type satisfies constraints (re-enabled with full constraint resolution)
    pub fn check_constraints(
        &self,
        type_expr: &TypeExpression,
        constraints: &[GenericConstraint],
    ) -> Result<bool, Error> {
        // Use the constraint resolver to check satisfaction
        self.constraint_resolver.check_satisfaction(type_expr, constraints, &self.type_environment)
    }

    /// Get type definition by name
    pub fn get_type_definition(&self, name: &str) -> Option<&TypeDefinition> {
        self.type_environment.type_definitions.get(name)
    }

    /// Get the type environment
    pub fn type_environment(&self) -> &TypeEnvironment {
        &self.type_environment
    }

    /// Get mutable access to type environment
    pub fn type_environment_mut(&mut self) -> &mut TypeEnvironment {
        &mut self.type_environment
    }

    /// Get access to the constraint resolver
    pub fn constraint_resolver(&self) -> &ConstraintResolver {
        &self.constraint_resolver
    }

    /// Get mutable access to the constraint resolver
    pub fn constraint_resolver_mut(&mut self) -> &mut ConstraintResolver {
        &mut self.constraint_resolver
    }

    // TODO: Re-enable these methods after fixing Type enum compatibility
    // /// Get access to the associated type resolver
    // pub fn associated_type_resolver(&self) -> &AssociatedTypeResolver {
    //     &self.associated_type_resolver
    // }

    // /// Get access to the higher-kinded type checker
    // pub fn higher_kinded_checker(&self) -> &HigherKindedTypeChecker {
    //     &self.higher_kinded_checker
    // }

    // /// Get access to the variance analyzer
    // pub fn variance_analyzer(&self) -> &VarianceAnalyzer {
    //     &self.variance_analyzer
    // }

    // /// Get access to the generic optimizer
    // pub fn generic_optimizer(&self) -> &GenericOptimizer {
    //     &self.generic_optimizer
    // }

    /// Resolve constraints for a given context
    pub fn resolve_constraints(
        &mut self,
        context: &ConstraintContext,
    ) -> Result<ConstraintSolution, Error> {
        self.constraint_resolver.resolve_constraints(context, &self.type_environment)
    }

    /// Validate a constraint
    pub fn validate_constraint(
        &self,
        constraint: &GenericConstraint,
    ) -> Result<bool, Error> {
        self.constraint_resolver.validate_constraint(constraint, &self.type_environment)
    }

    // TODO: Re-enable after fixing Type enum compatibility
    // /// Optimize generic instantiations
    // pub fn optimize_generics(&mut self) -> Result<(), Error> {
    //     self.generic_optimizer.optimize_instantiations(&mut self.type_environment)
    // }
}

impl TypeEnvironment {
    /// Create a new empty type environment
    pub fn new() -> Self {
        Self {
            type_definitions: HashMap::new(),
            instantiations: HashMap::new(),
            context_stack: Vec::new(),
        }
    }

    /// Push a new constraint context
    pub fn push_context(&mut self, context: ConstraintContext) {
        self.context_stack.push(context);
    }

    /// Pop the current constraint context
    pub fn pop_context(&mut self) -> Option<ConstraintContext> {
        self.context_stack.pop()
    }

    /// Get the current constraint context
    pub fn current_context(&self) -> Option<&ConstraintContext> {
        self.context_stack.last()
    }

    /// Get mutable access to current context
    pub fn current_context_mut(&mut self) -> Option<&mut ConstraintContext> {
        self.context_stack.last_mut()
    }
}

impl TypeExpression {
    /// Create a named type expression
    pub fn named(name: &str) -> Self {
        Self::Named(name.to_string())
    }

    /// Create a generic type expression
    pub fn generic(name: &str, args: Vec<TypeExpression>) -> Self {
        Self::Generic(name.to_string(), args)
    }

    /// Create a function type expression
    pub fn function(params: Vec<TypeExpression>, return_type: TypeExpression) -> Self {
        Self::Function(params, Box::new(return_type))
    }

    /// Create an array type expression
    pub fn array(element_type: TypeExpression) -> Self {
        Self::Array(Box::new(element_type))
    }

    /// Create a map type expression
    pub fn map(key_type: TypeExpression, value_type: TypeExpression) -> Self {
        Self::Map(Box::new(key_type), Box::new(value_type))
    }

    /// Create a channel type expression
    pub fn channel(element_type: TypeExpression) -> Self {
        Self::Channel(Box::new(element_type))
    }

    /// Create a type parameter expression
    pub fn parameter(name: &str) -> Self {
        Self::Parameter(name.to_string())
    }

    /// Get the string representation of this type
    pub fn to_string(&self) -> String {
        match self {
            Self::Named(name) => name.clone(),
            Self::Generic(name, args) => {
                format!("{}[{}]", name, args.iter().map(|a| a.to_string()).collect::<Vec<_>>().join(", "))
            }
            Self::Function(params, ret) => {
                format!("({}) -> {}", 
                    params.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(", "),
                    ret.to_string())
            }
            Self::Array(elem) => format!("[{}]", elem.to_string()),
            Self::Map(key, value) => format!("tea[{}]{}", key.to_string(), value.to_string()),
            Self::Channel(elem) => format!("dm {}", elem.to_string()),
            Self::Parameter(name) => name.clone(),
        }
    }

    /// Check if this type expression is concrete (no parameters)
    pub fn is_concrete(&self) -> bool {
        match self {
            Self::Named(_) => true,
            Self::Generic(_, args) => args.iter().all(|arg| arg.is_concrete()),
            Self::Function(params, ret) => {
                params.iter().all(|p| p.is_concrete()) && ret.is_concrete()
            }
            Self::Array(elem) => elem.is_concrete(),
            Self::Map(key, value) => key.is_concrete() && value.is_concrete(),
            Self::Channel(elem) => elem.is_concrete(),
            Self::Parameter(_) => false,
        }
    }

    /// Get all type parameters referenced in this expression
    pub fn collect_parameters(&self) -> Vec<String> {
        let mut params = Vec::new();
        self.collect_parameters_impl(&mut params);
        params.sort();
        params.dedup();
        params
    }

    fn collect_parameters_impl(&self, params: &mut Vec<String>) {
        match self {
            Self::Named(_) => {}
            Self::Generic(_, args) => {
                for arg in args {
                    arg.collect_parameters_impl(params);
                }
            }
            Self::Function(param_types, ret) => {
                for param in param_types {
                    param.collect_parameters_impl(params);
                }
                ret.collect_parameters_impl(params);
            }
            Self::Array(elem) => elem.collect_parameters_impl(params),
            Self::Map(key, value) => {
                key.collect_parameters_impl(params);
                value.collect_parameters_impl(params);
            }
            Self::Channel(elem) => elem.collect_parameters_impl(params),
            Self::Parameter(name) => params.push(name.clone()),
        }
    }
}

impl Default for TypeSystem {
    fn default() -> Self {
        Self::with_builtins()
    }
}

impl Default for TypeEnvironment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_system_creation() {
        let system = TypeSystem::new();
        assert_eq!(system.type_environment.type_definitions.len(), 0);
    }

    #[test]
    fn test_type_system_with_builtins() {
        let system = TypeSystem::with_builtins();
        assert!(system.get_type_definition("normie").is_some());
        assert!(system.get_type_definition("facts").is_some());
        assert!(system.get_type_definition("tea").is_some());
        assert!(system.get_type_definition("sus").is_some());
    }

    #[test]
    fn test_type_expression_string_representation() {
        let expr = TypeExpression::named("normie");
        assert_eq!(expr.to_string(), "normie");

        let generic_expr = TypeExpression::generic("Vec", vec![TypeExpression::named("normie")]);
        assert_eq!(generic_expr.to_string(), "Vec[normie]");

        let func_expr = TypeExpression::function(
            vec![TypeExpression::named("normie")],
            TypeExpression::named("tea")
        );
        assert_eq!(func_expr.to_string(), "(normie) -> tea");
    }

    #[test]
    fn test_type_expression_is_concrete() {
        let concrete = TypeExpression::named("normie");
        assert!(concrete.is_concrete());

        let parameter = TypeExpression::parameter("T");
        assert!(!parameter.is_concrete());

        let generic_concrete = TypeExpression::generic("Vec", vec![TypeExpression::named("normie")]);
        assert!(generic_concrete.is_concrete());

        let generic_with_param = TypeExpression::generic("Vec", vec![TypeExpression::parameter("T")]);
        assert!(!generic_with_param.is_concrete());
    }

    #[test]
    fn test_collect_parameters() {
        let expr = TypeExpression::generic(
            "Map",
            vec![
                TypeExpression::parameter("K"),
                TypeExpression::generic("Vec", vec![TypeExpression::parameter("V")])
            ]
        );

        let params = expr.collect_parameters();
        assert_eq!(params, vec!["K", "V"]);
    }

    #[test]
    fn test_type_environment_context_management() {
        let mut env = TypeEnvironment::new();
        assert!(env.current_context().is_none());

        let context = ConstraintContext {
            scope_id: "test".to_string(),
            active_constraints: Vec::new(),
            type_bindings: HashMap::new(),
        };

        env.push_context(context);
        assert!(env.current_context().is_some());
        assert_eq!(env.current_context().unwrap().scope_id, "test");

        let popped = env.pop_context();
        assert!(popped.is_some());
        assert!(env.current_context().is_none());
    }
}
