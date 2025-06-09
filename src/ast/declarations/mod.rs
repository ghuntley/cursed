//! Declarations module for the CURSED AST
//!
//! This module contains AST nodes representing different kinds of declarations
//! including function, struct, interface, and parameter declarations.

// Submodules
pub mod enhanced_constraint;
pub mod fields;
pub mod function;
pub mod generic_constraint;
pub mod multi_param_generic;
pub mod parameter;
pub mod struct_interface;
pub mod type_parameter;
pub mod where_clause;

// Re-exports from fields module
pub use fields::Field;
pub use fields::Parameter;

// Re-exports from function module
pub use function::FunctionStatement;

// Re-exports from enhanced_constraint module  
pub use enhanced_constraint::{AssociatedType, ConstraintOperator, EnhancedConstraint, TypeBound};

// Re-exports from generic_constraint module
pub use generic_constraint::GenericConstraint;

// Re-exports from multi_param_generic module
pub use multi_param_generic::{
    ConstraintRelation, CrossParameterConstraint, EnhancedTypeParameter, 
    MultiParamGeneric, Variance
};

// Re-exports from where_clause module
pub use where_clause::WhereClause;

// Re-exports from parameter module
pub use parameter::ParameterStatement;

// Re-exports from struct_interface module
pub use struct_interface::{CollabStatement, MethodSignature, SquadStatement};

// Re-exports from type_parameter module
pub use type_parameter::TypeParameter;