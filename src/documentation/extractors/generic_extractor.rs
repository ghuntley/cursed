//! Generic Type Information Extractor for CURSED Documentation
//! 
//! This module provides comprehensive extraction of generic type parameters,
//! constraints, bounds, and variance information for documentation generation.

use crate::ast::*;
use crate::documentation::extractors::ast_node_support::{ExpressionType, Literal};
use crate::error::Error;
use crate::documentation::extractors::ast_extractor::{GenericInfo, GenericParameter, GenericConstraint, GenericBound, Variance};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use tracing::{debug, instrument};

/// Generic type information extractor
pub struct GenericExtractor {
    /// Known constraint keywords
    constraint_keywords: HashSet<String>,
    /// Built-in traits/interfaces
    builtin_traits: HashSet<String>,
}

impl GenericExtractor {
    /// Create a new generic extractor
    #[instrument]
    pub fn new() -> Result<(), Error> {
        let mut constraint_keywords = HashSet::new();
        
        // CURSED constraint keywords
        constraint_keywords.insert("where".to_string());
        constraint_keywords.insert("implements".to_string());
        constraint_keywords.insert("extends".to_string());
        constraint_keywords.insert("super".to_string());
        constraint_keywords.insert("bounds".to_string());

        let mut builtin_traits = HashSet::new();
        
        // Common traits/interfaces
        builtin_traits.insert("Clone".to_string());
        builtin_traits.insert("Copy".to_string());
        builtin_traits.insert("Debug".to_string());
        builtin_traits.insert("Display".to_string());
        builtin_traits.insert("Eq".to_string());
        builtin_traits.insert("PartialEq".to_string());
        builtin_traits.insert("Ord".to_string());
        builtin_traits.insert("PartialOrd".to_string());
        builtin_traits.insert("Hash".to_string());
        builtin_traits.insert("Default".to_string());
        builtin_traits.insert("Send".to_string());
        builtin_traits.insert("Sync".to_string());
        builtin_traits.insert("Sized".to_string());
        builtin_traits.insert("Unpin".to_string());

        Ok(Self {
            constraint_keywords,
            builtin_traits,
        })
    }

    /// Extract generic information from function declaration
    #[instrument(skip(self, func_decl))]
    pub fn extract_function_generics(
        &self,
        func_decl: &FunctionDeclaration,
    ) -> Result<(), Error> {
        debug!("Extracting function generics for: {}", func_decl.to_string());

        let parameters = if let Some(ref generics) = func_decl.generic_params {
            self.extract_generic_parameters(generics)?
        } else {
            Vec::new()
        };

        let constraints = if let Some(ref constraints) = func_decl.constraints {
            self.extract_generic_constraints(constraints)?
        } else {
            Vec::new()
        };

        let bounds = self.extract_function_bounds(func_decl)?;

        Ok(GenericInfo {
            parameters,
            constraints,
            bounds,
        })
    }

    /// Extract generic information from struct declaration
    #[instrument(skip(self, struct_decl))]
    pub fn extract_struct_generics(
        &self,
        struct_decl: &StructDeclaration,
    ) -> Result<(), Error> {
        debug!("Extracting struct generics for: {}", struct_decl.to_string());

        let parameters = if let Some(ref generics) = struct_decl.generic_params {
            self.extract_generic_parameters(generics)?
        } else {
            Vec::new()
        };

        let constraints = if let Some(ref constraints) = struct_decl.constraints {
            self.extract_generic_constraints(constraints)?
        } else {
            Vec::new()
        };

        let bounds = self.extract_struct_bounds(struct_decl)?;

        Ok(GenericInfo {
            parameters,
            constraints,
            bounds,
        })
    }

    /// Extract generic information from interface declaration
    #[instrument(skip(self, interface_decl))]
    pub fn extract_interface_generics(
        &self,
        interface_decl: &InterfaceDeclaration,
    ) -> Result<(), Error> {
        debug!("Extracting interface generics for: {}", interface_decl.to_string());

        let parameters = if let Some(ref generics) = interface_decl.generic_params {
            self.extract_generic_parameters(generics)?
        } else {
            Vec::new()
        };

        let constraints = if let Some(ref constraints) = interface_decl.constraints {
            self.extract_generic_constraints(constraints)?
        } else {
            Vec::new()
        };

        let bounds = self.extract_interface_bounds(interface_decl)?;

        Ok(GenericInfo {
            parameters,
            constraints,
            bounds,
        })
    }

    /// Extract generic parameters from a list of parameter names
    #[instrument(skip(self, param_names))]
    fn extract_generic_parameters(
        &self,
        param_names: &[String],
    ) -> Result<(), Error> {
        let mut parameters = Vec::new();

        for param_name in param_names {
            // Parse parameter with potential constraints
            let (name, constraints) = self.parse_parameter_constraints(param_name)?;
            
            parameters.push(GenericParameter {
                name,
                constraints,
                default_type: None, // Would need to parse default types
                variance: self.infer_variance(param_name), // Simple variance inference
            });
        }

        Ok(parameters)
    }

    /// Parse parameter constraints from parameter string
    fn parse_parameter_constraints(&self, param_str: &str) -> Result<(), Error> {
        // Handle formats like "T: Clone + Debug" or "T where T: Clone"
        if let Some(colon_pos) = param_str.find(':') {
            let name = param_str[..colon_pos].trim().to_string();
            let constraints_str = param_str[colon_pos + 1..].trim();
            let constraints: Vec<String> = constraints_str
                .split('+')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            Ok((name, constraints))
        } else if let Some(where_pos) = param_str.find("where") {
            let name = param_str[..where_pos].trim().to_string();
            let constraints_str = param_str[where_pos + 5..].trim();
            // Parse "T: Clone + Debug" after where
            let constraints = self.parse_where_constraints(constraints_str)?;
            Ok((name, constraints))
        } else {
            Ok((param_str.trim().to_string(), Vec::new()))
        }
    }

    /// Parse where clause constraints
    fn parse_where_constraints(&self, constraints_str: &str) -> Result<(), Error> {
        // Handle multiple constraints separated by commas
        let clauses: Vec<&str> = constraints_str.split(',').collect();
        let mut constraints = Vec::new();

        for clause in clauses {
            let clause = clause.trim();
            if let Some(colon_pos) = clause.find(':') {
                let constraint_part = clause[colon_pos + 1..].trim();
                let trait_constraints: Vec<String> = constraint_part
                    .split('+')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
                constraints.extend(trait_constraints);
            }
        }

        Ok(constraints)
    }

    /// Infer variance from parameter usage (simplified)
    fn infer_variance(&self, param_name: &str) -> Option<Variance> {
        // Simple heuristics for variance inference
        if param_name.starts_with("In") {
            Some(Variance::Contravariant)
        } else if param_name.starts_with("Out") {
            Some(Variance::Covariant)
        } else {
            Some(Variance::Invariant)
        }
    }

    /// Extract generic constraints from constraint declarations
    #[instrument(skip(self, constraints))]
    fn extract_generic_constraints(
        &self,
        constraints: &[GenericConstraint],
    ) -> Result<(), Error> {
        // For now, return the constraints as-is
        // In a real implementation, we'd parse and validate them
        Ok(constraints.clone())
    }

    /// Extract bounds from function declaration
    fn extract_function_bounds(&self, func_decl: &FunctionDeclaration) -> Result<(), Error> {
        let mut bounds = Vec::new();

        // Extract bounds from parameter types
        for param in &func_decl.parameters {
            if let Some(ref param_type) = param.param_type {
                bounds.extend(self.extract_bounds_from_type_expression(param_type)?);
            }
        }

        // Extract bounds from return type
        if let Some(ref return_type) = func_decl.return_type {
            bounds.extend(self.extract_bounds_from_type_expression(return_type)?);
        }

        Ok(bounds)
    }

    /// Extract bounds from struct declaration
    fn extract_struct_bounds(&self, struct_decl: &StructDeclaration) -> Result<(), Error> {
        let mut bounds = Vec::new();

        // Extract bounds from field types
        for field in &struct_decl.fields {
            if let Some(ref field_type) = field.field_type {
                bounds.extend(self.extract_bounds_from_type_expression(field_type)?);
            }
        }

        Ok(bounds)
    }

    /// Extract bounds from interface declaration
    fn extract_interface_bounds(&self, interface_decl: &InterfaceDeclaration) -> Result<(), Error> {
        let mut bounds = Vec::new();

        // Extract bounds from method signatures
        for method in &interface_decl.methods {
            // Extract from parameters
            for param in &method.parameters {
                if let Some(ref param_type) = param.param_type {
                    bounds.extend(self.extract_bounds_from_type_expression(param_type)?);
                }
            }

            // Extract from return type
            if let Some(ref return_type) = method.return_type {
                bounds.extend(self.extract_bounds_from_type_expression(return_type)?);
            }
        }

        Ok(bounds)
    }

    /// Extract bounds from a type expression
    #[instrument(skip(self, expr))]
    fn extract_bounds_from_type_expression(
        &self,
        expr: &dyn Expression,
    ) -> Result<(), Error> {
        let mut bounds = Vec::new();

        match &expr.expr_type {
            ExpressionType::Identifier(id) => {
                // Check if this identifier represents a bound
                if self.builtin_traits.contains(&id.to_string()) {
                    bounds.push(GenericBound {
                        bound_type: id.to_string().clone(),
                        expression: id.to_string().clone(),
                        lifetime: None,
                    });
                }
            }
            ExpressionType::FunctionCall(call) => {
                // Generic type with bounds like Vec<T: Clone>
                bounds.extend(self.extract_bounds_from_type_expression(&call.function)?);
                for arg in &call.arguments {
                    bounds.extend(self.extract_bounds_from_type_expression(arg)?);
                }
            }
            ExpressionType::MemberAccess(member) => {
                // Associated types with bounds
                bounds.extend(self.extract_bounds_from_type_expression(&member.object)?);
            }
            ExpressionType::BinaryExpression(bin) => {
                // Type bounds with binary operations like T + Send
                if bin.operator == "+" {
                    bounds.extend(self.extract_bounds_from_type_expression(&bin.left)?);
                    bounds.extend(self.extract_bounds_from_type_expression(&bin.right)?);
                }
            }
            _ => {
                // Other expression types don't typically contain bounds
            }
        }

        Ok(bounds)
    }

    /// Analyze generic parameter usage to determine variance
    #[instrument(skip(self, param_name, usage_contexts))]
    pub fn analyze_parameter_variance(
        &self,
        param_name: &str,
        usage_contexts: &[VarianceContext],
    ) -> Variance {
        let mut positive_count = 0;
        let mut negative_count = 0;

        for context in usage_contexts {
            match context.variance_position {
                VariancePosition::Covariant => positive_count += 1,
                VariancePosition::Contravariant => negative_count += 1,
                VariancePosition::Invariant => {
                    // Invariant usage forces invariant variance
                    return Variance::Invariant;
                }
            }
        }

        if positive_count > 0 && negative_count > 0 {
            Variance::Invariant
        } else if positive_count > 0 {
            Variance::Covariant
        } else if negative_count > 0 {
            Variance::Contravariant
        } else {
            Variance::Invariant
        }
    }

    /// Extract constraint relationships between generic parameters
    #[instrument(skip(self, constraints))]
    pub fn extract_constraint_relationships(
        &self,
        constraints: &[GenericConstraint],
    ) -> Result<(), Error> {
        let mut relationships = Vec::new();

        for constraint in constraints {
            // Analyze constraint to determine relationship type
            let relationship_type = self.classify_constraint(&constraint.constraint_type)?;
            
            relationships.push(ConstraintRelationship {
                source_param: constraint.target_type.clone(),
                target_constraint: constraint.constraint_type.clone(),
                relationship_type,
                strength: ConstraintStrength::Required, // Default to required
            });
        }

        Ok(relationships)
    }

    /// Classify constraint type
    fn classify_constraint(&self, constraint_type: &str) -> Result<(), Error> {
        match constraint_type.to_lowercase().as_str() {
            "implements" | "impl" => Ok(ConstraintRelationshipType::Implements),
            "extends" | "super" => Ok(ConstraintRelationshipType::Extends),
            "where" => Ok(ConstraintRelationshipType::Where),
            "bounds" => Ok(ConstraintRelationshipType::Bounds),
            _ => {
                if self.builtin_traits.contains(constraint_type) {
                    Ok(ConstraintRelationshipType::TraitBound)
                } else {
                    Ok(ConstraintRelationshipType::Custom)
                }
            }
        }
    }

    /// Validate generic constraints for correctness
    #[instrument(skip(self, generic_info))]
    pub fn validate_generic_constraints(
        &self,
        generic_info: &GenericInfo,
    ) -> Result<(), Error> {
        let mut results = Vec::new();

        for constraint in &generic_info.constraints {
            let validation = self.validate_single_constraint(constraint, &generic_info.parameters)?;
            results.push(validation);
        }

        Ok(results)
    }

    /// Validate a single constraint
    fn validate_single_constraint(
        &self,
        constraint: &GenericConstraint,
        parameters: &[GenericParameter],
    ) -> Result<(), Error> {
        // Check if target type exists in parameters
        let target_exists = parameters.iter()
            .any(|param| param.to_string() == constraint.target_type);

        // Check if constraint type is valid
        let constraint_valid = self.builtin_traits.contains(&constraint.constraint_type) ||
                              self.constraint_keywords.contains(&constraint.constraint_type);

        let is_valid = target_exists && constraint_valid;
        let issues = if !is_valid {
            vec![
                if !target_exists {
                    format!("Unknown parameter: {}", constraint.target_type)
                } else {
                    String::new()
                },
                if !constraint_valid {
                    format!("Unknown constraint: {}", constraint.constraint_type)
                } else {
                    String::new()
                },
            ].into_iter().filter(|s| !s.is_empty()).collect()
        } else {
            Vec::new()
        };

        Ok(ConstraintValidationResult {
            constraint: constraint.clone(),
            is_valid,
            issues,
            suggestions: self.generate_constraint_suggestions(constraint)?,
        })
    }

    /// Generate suggestions for constraint improvements
    fn generate_constraint_suggestions(
        &self,
        constraint: &GenericConstraint,
    ) -> Result<(), Error> {
        let mut suggestions = Vec::new();

        // Suggest common traits if constraint is unrecognized
        if !self.builtin_traits.contains(&constraint.constraint_type) {
            let similar_traits: Vec<&String> = self.builtin_traits.iter()
                .filter(|&trait_name| {
                    self.string_similarity(trait_name, &constraint.constraint_type) > 0.6
                })
                .collect();

            for similar_trait in similar_traits {
                suggestions.push(format!("Did you mean '{}'?", similar_trait));
            }
        }

        Ok(suggestions)
    }

    /// Calculate string similarity (simplified Levenshtein distance)
    fn string_similarity(&self, a: &str, b: &str) -> f64 {
        if a == b {
            return 1.0;
        }

        let len_a = a.len();
        let len_b = b.len();

        if len_a == 0 || len_b == 0 {
            return 0.0;
        }

        let max_len = len_a.max(len_b);
        let distance = self.levenshtein_distance(a, b);
        
        1.0 - (distance as f64 / max_len as f64)
    }

    /// Calculate Levenshtein distance
    fn levenshtein_distance(&self, a: &str, b: &str) -> usize {
        let a_chars: Vec<char> = a.chars().collect();
        let b_chars: Vec<char> = b.chars().collect();
        let len_a = a_chars.len();
        let len_b = b_chars.len();

        let mut matrix = vec![vec![0; len_b + 1]; len_a + 1];

        for i in 0..=len_a {
            matrix[i][0] = i;
        }
        for j in 0..=len_b {
            matrix[0][j] = j;
        }

        for i in 1..=len_a {
            for j in 1..=len_b {
                let cost = if a_chars[i - 1] == b_chars[j - 1] { 0 } else { 1 };
                matrix[i][j] = std::cmp::min(
                    std::cmp::min(
                        matrix[i - 1][j] + 1,      // deletion
                        matrix[i][j - 1] + 1       // insertion
                    ),
                    matrix[i - 1][j - 1] + cost    // substitution
                );
            }
        }

        matrix[len_a][len_b]
    }
}

/// Variance context for parameter usage analysis
#[derive(Debug, Clone)]
pub struct VarianceContext {
    /// Position where parameter is used
    pub variance_position: VariancePosition,
    /// Context description
    pub context: String,
}

/// Variance position in type system
#[derive(Debug, Clone)]
pub enum VariancePosition {
    Covariant,
    Contravariant,
    Invariant,
}

/// Constraint relationship between generic parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintRelationship {
    /// Source parameter
    pub source_param: String,
    /// Target constraint
    pub target_constraint: String,
    /// Type of relationship
    pub relationship_type: ConstraintRelationshipType,
    /// Strength of constraint
    pub strength: ConstraintStrength,
}

/// Types of constraint relationships
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConstraintRelationshipType {
    Implements,
    Extends,
    Where,
    Bounds,
    TraitBound,
    Custom,
}

/// Strength of constraint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConstraintStrength {
    Required,
    Optional,
    Conditional,
}

/// Result of constraint validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintValidationResult {
    /// The constraint being validated
    pub constraint: GenericConstraint,
    /// Whether the constraint is valid
    pub is_valid: bool,
    /// Issues found
    pub issues: Vec<String>,
    /// Suggestions for improvement
    pub suggestions: Vec<String>,
}
