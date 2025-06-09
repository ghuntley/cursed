//! AST nodes for multi-parameter generic declarations.
//!
//! This module handles complex generic parameter lists with constraints,
//! default values, and variance annotations.

use crate::ast::{Node, Statement, Expression};
use crate::ast::declarations::{TypeParameter, EnhancedConstraint};
use crate::lexer::token::Token;
use std::any::Any;

/// Represents variance annotations for generic type parameters
#[derive(Clone, Debug, PartialEq)]
pub enum Variance {
    Invariant,     // No annotation (default)
    Covariant,     // + annotation  
    Contravariant, // - annotation
}

impl Variance {
    /// Returns the string representation of the variance
    pub fn as_str(&self) -> &'static str {
        match self {
            Variance::Invariant => "",
            Variance::Covariant => "+",
            Variance::Contravariant => "-",
        }
    }

    /// Parses variance from a string
    pub fn from_str(s: &str) -> Self {
        match s {
            "+" => Variance::Covariant,
            "-" => Variance::Contravariant,
            _ => Variance::Invariant,
        }
    }
}

/// Enhanced type parameter with constraints, defaults, and variance
#[derive(Debug)]
pub struct EnhancedTypeParameter {
    pub token: Token,                          // The parameter token
    pub name: String,                          // Parameter name (T, U, etc.)
    pub variance: Variance,                    // Variance annotation
    pub constraints: Vec<EnhancedConstraint>,  // Type constraints
    pub default_type: Option<Box<dyn Expression>>, // Default type if not specified
    pub lifetime_bound: Option<String>,        // Lifetime bound (if any)
}

impl EnhancedTypeParameter {
    /// Creates a simple type parameter with just a name
    pub fn simple(token: Token, name: String) -> Self {
        Self {
            token,
            name,
            variance: Variance::Invariant,
            constraints: Vec::new(),
            default_type: None,
            lifetime_bound: None,
        }
    }

    /// Creates a type parameter with constraints
    pub fn with_constraints(
        token: Token,
        name: String,
        constraints: Vec<EnhancedConstraint>,
    ) -> Self {
        Self {
            token,
            name,
            variance: Variance::Invariant,
            constraints,
            default_type: None,
            lifetime_bound: None,
        }
    }

    /// Creates a type parameter with variance annotation
    pub fn with_variance(token: Token, name: String, variance: Variance) -> Self {
        Self {
            token,
            name,
            variance,
            constraints: Vec::new(),
            default_type: None,
            lifetime_bound: None,
        }
    }

    /// Adds a default type to this parameter
    pub fn with_default(mut self, default_type: Box<dyn Expression>) -> Self {
        self.default_type = Some(default_type);
        self
    }

    /// Checks if this parameter has constraints
    pub fn has_constraints(&self) -> bool {
        !self.constraints.is_empty()
    }

    /// Checks if this parameter has a default type
    pub fn has_default(&self) -> bool {
        self.default_type.is_some()
    }

    /// Checks if this parameter has variance annotation
    pub fn has_variance(&self) -> bool {
        self.variance != Variance::Invariant
    }
}

impl Node for EnhancedTypeParameter {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }

    fn string(&self) -> String {
        let mut result = String::new();

        // Add variance annotation
        if self.has_variance() {
            result.push_str(self.variance.as_str());
        }

        // Add parameter name
        result.push_str(&self.name);

        // Add constraints
        if self.has_constraints() {
            let constraint_strings: Vec<String> = self.constraints
                .iter()
                .map(|c| c.string())
                .collect();
            result.push_str(&format!(": {}", constraint_strings.join(" + ")));
        }

        // Add default type
        if let Some(ref default) = self.default_type {
            result.push_str(&format!(" = {}", default.string()));
        }

        result
    }
}

/// Multi-parameter generic declaration with enhanced features
#[derive(Clone, Debug)]
pub struct MultiParamGeneric {
    pub token: Token,                               // Opening bracket token [
    pub parameters: Vec<EnhancedTypeParameter>,     // Type parameters with constraints
    pub cross_constraints: Vec<CrossParameterConstraint>, // Constraints between parameters
}

impl MultiParamGeneric {
    /// Creates a new multi-parameter generic declaration
    pub fn new(
        token: Token,
        parameters: Vec<EnhancedTypeParameter>,
    ) -> Self {
        Self {
            token,
            parameters,
            cross_constraints: Vec::new(),
        }
    }

    /// Adds cross-parameter constraints to this generic declaration
    pub fn with_cross_constraints(
        mut self,
        constraints: Vec<CrossParameterConstraint>,
    ) -> Self {
        self.cross_constraints = constraints;
        self
    }

    /// Checks if this generic declaration is empty
    pub fn is_empty(&self) -> bool {
        self.parameters.is_empty()
    }

    /// Gets the number of type parameters
    pub fn parameter_count(&self) -> usize {
        self.parameters.len()
    }

    /// Checks if any parameters have constraints
    pub fn has_constraints(&self) -> bool {
        self.parameters.iter().any(|p| p.has_constraints()) || !self.cross_constraints.is_empty()
    }

    /// Gets all parameter names
    pub fn parameter_names(&self) -> Vec<String> {
        self.parameters.iter().map(|p| p.name.clone()).collect()
    }
}

impl Node for MultiParamGeneric {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }

    fn string(&self) -> String {
        if self.parameters.is_empty() {
            return String::new();
        }

        let param_strings: Vec<String> = self.parameters
            .iter()
            .map(|param| param.string())
            .collect();

        let mut result = format!("[{}]", param_strings.join(", "));

        // Add cross-parameter constraints if any
        if !self.cross_constraints.is_empty() {
            let constraint_strings: Vec<String> = self.cross_constraints
                .iter()
                .map(|c| c.string())
                .collect();
            result.push_str(&format!(" where {}", constraint_strings.join(", ")));
        }

        result
    }
}

impl Statement for MultiParamGeneric {
    fn statement_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Represents constraints between multiple type parameters
/// Example: `where T: Into<U>, U: Display`
#[derive(Clone, Debug)]
pub struct CrossParameterConstraint {
    pub token: Token,                    // The constraint token
    pub source_param: String,            // Source parameter name
    pub target_param: String,            // Target parameter name
    pub relationship: ConstraintRelation, // Type of relationship
}

impl CrossParameterConstraint {
    /// Creates a new cross-parameter constraint
    pub fn new(
        token: Token,
        source_param: String,
        target_param: String,
        relationship: ConstraintRelation,
    ) -> Self {
        Self {
            token,
            source_param,
            target_param,
            relationship,
        }
    }
}

impl Node for CrossParameterConstraint {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }

    fn string(&self) -> String {
        match &self.relationship {
            ConstraintRelation::Into => {
                format!("{}: Into<{}>", self.source_param, self.target_param)
            }
            ConstraintRelation::From => {
                format!("{}: From<{}>", self.target_param, self.source_param)
            }
            ConstraintRelation::Same => {
                format!("{} = {}", self.source_param, self.target_param)
            }
            ConstraintRelation::Subtype => {
                format!("{} <: {}", self.source_param, self.target_param)
            }
        }
    }
}

/// Types of relationships between type parameters
#[derive(Clone, Debug, PartialEq)]
pub enum ConstraintRelation {
    Into,    // T: Into<U>
    From,    // U: From<T>
    Same,    // T = U (type equality)
    Subtype, // T <: U (subtype relationship)
}

impl ConstraintRelation {
    /// Returns the string representation of the relation
    pub fn as_str(&self) -> &'static str {
        match self {
            ConstraintRelation::Into => "Into",
            ConstraintRelation::From => "From",
            ConstraintRelation::Same => "=",
            ConstraintRelation::Subtype => "<:",
        }
    }
}
