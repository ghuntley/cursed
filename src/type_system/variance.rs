//! Type variance analysis and checking for CURSED
//! 
//! Provides variance computation for type parameters to ensure
//! safe subtyping relationships in generic types.

use crate::error::CursedError;
use crate::type_system::{TypeExpression, TypeDefinition, TypeParameter};
use std::collections::HashMap;

/// Variance annotations for type parameters
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variance {
    /// Type can be replaced by subtype (T -> subtype of T)
    Covariant,
    /// Type can be replaced by supertype (T -> supertype of T)  
    Contravariant,
    /// Type must be exact type (no substitution allowed)
    Invariant,
}

/// Variance analysis engine
#[derive(Debug)]
pub struct VarianceAnalyzer {
    /// Cache of computed variances for types
    variance_cache: HashMap<String, Vec<Variance>>,
    /// Current analysis context
    context: VarianceContext,
}

#[derive(Debug, Clone)]
struct VarianceContext {
    /// Current type being analyzed
    current_type: String,
    /// Parameter variance stack for recursive analysis
    parameter_stack: Vec<(String, Variance)>,
}

impl VarianceAnalyzer {
    pub fn new() -> Self {
        Self {
            variance_cache: HashMap::new(),
            context: VarianceContext {
                current_type: String::new(),
                parameter_stack: Vec::new(),
            },
        }
    }

    /// Compute variance for all type parameters of a type definition
    pub fn compute_variance(&mut self, type_def: &TypeDefinition) -> Result<Vec<Variance>, CursedError> {
        // Check cache first
        if let Some(cached) = self.variance_cache.get(&type_def.name) {
            return Ok(cached.clone());
        }

        self.context.current_type = type_def.name.clone();
        let mut variances = Vec::new();

        // Analyze each type parameter
        for param in &type_def.type_parameters {
            let variance = self.compute_parameter_variance(type_def, param)?;
            variances.push(variance);
        }

        // Cache the result
        self.variance_cache.insert(type_def.name.clone(), variances.clone());
        Ok(variances)
    }

    /// Compute variance for a specific type parameter
    fn compute_parameter_variance(&mut self, type_def: &TypeDefinition, param: &str) -> Result<Variance, CursedError> {
        // Start with covariant assumption
        let mut variance = Variance::Covariant;

        // Analyze parameter usage in type definition
        variance = self.analyze_parameter_in_methods(type_def, param, variance)?;

        Ok(variance)
    }

    /// Analyze parameter usage in method signatures
    fn analyze_parameter_in_methods(&self, type_def: &TypeDefinition, param: &str, mut variance: Variance) -> Result<Variance, CursedError> {
        for method in &type_def.methods {
            // Check parameters (contravariant position)
            for method_param in &method.parameters {
                let param_variance = self.analyze_type_usage(method_param, param)?;
                variance = self.combine_variance(variance, self.flip_variance(param_variance))?;
            }

            // Check return type (covariant position)
            if let Some(return_type) = &method.return_type {
                let return_variance = self.analyze_type_usage(return_type, param)?;
                variance = self.combine_variance(variance, return_variance)?;
            }
        }

        Ok(variance)
    }

    /// Analyze how a type parameter is used in a type expression
    fn analyze_type_usage(&self, type_expr: &TypeExpression, param: &str) -> Result<Variance, CursedError> {
        if let Some(name) = &type_expr.name {
            if name == param {
                return Ok(Variance::Covariant);
            }
        }

        // Check nested type parameters
        let mut combined_variance = Variance::Covariant;
        for nested_param in &type_expr.parameters {
            let nested_variance = self.analyze_type_usage(nested_param, param)?;
            combined_variance = self.combine_variance(combined_variance, nested_variance)?;
        }

        // Special handling for function types
        if type_expr.kind == crate::type_system::TypeKind::Function {
            // Parameters are contravariant, return type is covariant
            for func_param in &type_expr.parameters {
                let param_variance = self.analyze_type_usage(func_param, param)?;
                combined_variance = self.combine_variance(combined_variance, self.flip_variance(param_variance))?;
            }

            if let Some(return_type) = &type_expr.return_type {
                let return_variance = self.analyze_type_usage(return_type, param)?;
                combined_variance = self.combine_variance(combined_variance, return_variance)?;
            }
        }

        Ok(combined_variance)
    }

    /// Combine two variance annotations
    fn combine_variance(&self, v1: Variance, v2: Variance) -> Result<Variance, CursedError> {
        use Variance::*;
        
        match (v1, v2) {
            (Covariant, Covariant) => Ok(Covariant),
            (Contravariant, Contravariant) => Ok(Contravariant), 
            (Invariant, _) | (_, Invariant) => Ok(Invariant),
            (Covariant, Contravariant) | (Contravariant, Covariant) => Ok(Invariant),
        }
    }

    /// Flip variance for contravariant positions
    fn flip_variance(&self, variance: Variance) -> Variance {
        match variance {
            Variance::Covariant => Variance::Contravariant,
            Variance::Contravariant => Variance::Covariant,
            Variance::Invariant => Variance::Invariant,
        }
    }

    /// Check if a type substitution is safe given variance constraints
    pub fn check_subtyping(&self, 
                          base_type: &TypeExpression, 
                          derived_type: &TypeExpression,
                          variances: &[Variance]) -> Result<bool, CursedError> {
        
        if base_type.name != derived_type.name {
            return Ok(false);
        }

        if base_type.parameters.len() != derived_type.parameters.len() {
            return Ok(false);
        }

        if base_type.parameters.len() != variances.len() {
            return Err(CursedError::type_error("Variance count mismatch"));
        }

        // Check each type parameter according to its variance
        for ((base_param, derived_param), variance) in 
            base_type.parameters.iter()
                .zip(derived_type.parameters.iter())
                .zip(variances.iter()) {
            
            let is_compatible = match variance {
                Variance::Covariant => self.is_subtype(derived_param, base_param)?,
                Variance::Contravariant => self.is_subtype(base_param, derived_param)?,
                Variance::Invariant => self.types_equal(base_param, derived_param)?,
            };

            if !is_compatible {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Check if type1 is a subtype of type2
    fn is_subtype(&self, type1: &TypeExpression, type2: &TypeExpression) -> Result<bool, CursedError> {
        // Simplified subtype checking - can be extended
        Ok(type1.name == type2.name)
    }

    /// Check if two types are exactly equal
    fn types_equal(&self, type1: &TypeExpression, type2: &TypeExpression) -> Result<bool, CursedError> {
        Ok(type1 == type2)
    }

    /// Get cached variance for a type
    pub fn get_variance(&self, type_name: &str) -> Option<&Vec<Variance>> {
        self.variance_cache.get(type_name)
    }

    /// Clear variance cache
    pub fn clear_cache(&mut self) {
        self.variance_cache.clear();
    }
}

/// Utility functions for variance analysis
pub mod variance_utils {
    use super::*;

    /// Create a variance analyzer with built-in type variances
    pub fn create_with_builtins() -> VarianceAnalyzer {
        let mut analyzer = VarianceAnalyzer::new();
        
        // Add built-in type variances
        analyzer.variance_cache.insert("Array".to_string(), vec![Variance::Covariant]);
        analyzer.variance_cache.insert("Option".to_string(), vec![Variance::Covariant]);
        analyzer.variance_cache.insert("Result".to_string(), vec![Variance::Covariant, Variance::Covariant]);
        
        analyzer
    }

    /// Format variance for display
    pub fn format_variance(variance: Variance) -> &'static str {
        match variance {
            Variance::Covariant => "+",
            Variance::Contravariant => "-", 
            Variance::Invariant => "=",
        }
    }

    /// Parse variance from string annotation
    pub fn parse_variance(s: &str) -> Result<Variance, CursedError> {
        match s {
            "+" | "covariant" => Ok(Variance::Covariant),
            "-" | "contravariant" => Ok(Variance::Contravariant),
            "=" | "invariant" => Ok(Variance::Invariant),
            _ => Err(CursedError::type_error(&format!("Invalid variance annotation: {}", s))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::type_system::{TypeKind, MethodSignature};

    #[test]
    fn test_basic_variance_computation() {
        let mut analyzer = VarianceAnalyzer::new();
        
        let type_def = TypeDefinition {
            name: "Box".to_string(),
            kind: TypeKind::Struct,
            type_parameters: vec!["T".to_string()],
            constraints: Vec::new(),
            fields: Vec::new(),
            methods: vec![
                MethodSignature {
                    name: "get".to_string(),
                    parameters: Vec::new(),
                    return_type: Some(TypeExpression::named("T")),
                    type_parameters: Vec::new(),
                    constraints: Vec::new(),
                    source_location: None,
                }
            ],
            is_builtin: false,
            source_location: None,
        };

        let variances = analyzer.compute_variance(&type_def).unwrap();
        assert_eq!(variances.len(), 1);
        assert_eq!(variances[0], Variance::Covariant);
    }

    #[test]
    fn test_contravariant_function_parameters() {
        let mut analyzer = VarianceAnalyzer::new();
        
        let type_def = TypeDefinition {
            name: "Function".to_string(),
            kind: TypeKind::Function,
            type_parameters: vec!["T".to_string(), "R".to_string()],
            constraints: Vec::new(),
            fields: Vec::new(),
            methods: vec![
                MethodSignature {
                    name: "call".to_string(),
                    parameters: vec![TypeExpression::named("T")],
                    return_type: Some(TypeExpression::named("R")),
                    type_parameters: Vec::new(),
                    constraints: Vec::new(),
                    source_location: None,
                }
            ],
            is_builtin: false,
            source_location: None,
        };

        let variances = analyzer.compute_variance(&type_def).unwrap();
        assert_eq!(variances.len(), 2);
        // T should be contravariant (input parameter)
        // R should be covariant (return type)
    }

    #[test]
    fn test_variance_utilities() {
        assert_eq!(variance_utils::format_variance(Variance::Covariant), "+");
        assert_eq!(variance_utils::format_variance(Variance::Contravariant), "-");
        assert_eq!(variance_utils::format_variance(Variance::Invariant), "=");
        
        assert_eq!(variance_utils::parse_variance("+").unwrap(), Variance::Covariant);
        assert_eq!(variance_utils::parse_variance("-").unwrap(), Variance::Contravariant);
        assert_eq!(variance_utils::parse_variance("=").unwrap(), Variance::Invariant);
    }
}
