//! Enhanced generic type system for CURSED
//! Provides comprehensive generic type checking, constraint resolution, and specialization

use crate::error::CursedError;
use crate::ast::{Type, FunctionStatement, Parameter};
use super::{TypeExpression, TypeEnvironment, GenericConstraint, ConstraintResolver};
use std::collections::HashMap;

/// Enhanced generic type checker with constraint support
#[derive(Debug)]
pub struct GenericTypeChecker {
    /// Type parameter bindings
    type_bindings: HashMap<String, TypeExpression>,
    
    /// Active type constraints
    constraints: Vec<GenericConstraint>,
    
    /// Constraint resolver
    resolver: ConstraintResolver,
    
    /// Specialization cache
    specializations: HashMap<String, TypeExpression>,
}

impl GenericTypeChecker {
    pub fn new() -> Self {
        Self {
            type_bindings: HashMap::new(),
            constraints: Vec::new(),
            resolver: ConstraintResolver::new(),
            specializations: HashMap::new(),
        }
    }
    
    /// Check a generic function definition
    pub fn check_generic_function(&mut self, func: &FunctionStatement, env: &TypeEnvironment) -> Result<(), CursedError> {
        // Add type parameters to binding context
        for type_param in &func.type_parameters {
            self.type_bindings.insert(type_param.name.clone(), TypeExpression::parameter(&type_param.name));
        }
        
        // Validate constraints if any
        for constraint in &self.constraints {
            self.resolver.validate_constraint(constraint, env)
                .map_err(|e| CursedError::type_error(&format!("Constraint violation: {:?}", e)))?;
        }
        
        // Check parameter types
        for param in &func.parameters {
            if let Some(param_type) = &param.param_type {
                self.check_type_usage(param_type, env)?;
            }
        }
        
        // Check return type
        if let Some(return_type) = &func.return_type {
            self.check_type_usage(return_type, env)?;
        }
        
        Ok(())
    }
    
    /// Check if a type is properly used in a generic context
    fn check_type_usage(&self, type_ast: &Type, env: &TypeEnvironment) -> Result<(), CursedError> {
        match type_ast {
            Type::Generic(name, type_args) => {
                // Check if the generic type exists
                if env.get_type(name).is_none() {
                    return Err(CursedError::TypeError(format!("Unknown generic type: {}", name)));
                }
                
                // Check type arguments
                for arg in type_args {
                    self.check_type_usage(arg, env)?;
                }
                
                Ok(())
            }
            Type::Custom(name) => {
                // Check if this is a type parameter
                if self.type_bindings.contains_key(name) {
                    Ok(())
                } else if env.get_type(name).is_some() {
                    Ok(())
                } else {
                    Err(CursedError::TypeError(format!("Unknown type: {}", name)))
                }
            }
            Type::Array(inner, _) => self.check_type_usage(inner, env),
            Type::Pointer(inner) => self.check_type_usage(inner, env),
            Type::Function(params, ret) => {
                for param in params {
                    self.check_type_usage(param, env)?;
                }
                self.check_type_usage(ret, env)
            }
            Type::Tuple(types) => {
                for t in types {
                    self.check_type_usage(t, env)?;
                }
                Ok(())
            }
            _ => Ok(())
        }
    }
    
    /// Instantiate a generic type with concrete type arguments
    pub fn instantiate_generic(&mut self, 
        generic_name: &str, 
        type_args: &[Type], 
        env: &TypeEnvironment
    ) -> Result<TypeExpression, CursedError> {
        // Create a specialization key
        let spec_key = format!("{}<{}>", generic_name, 
            type_args.iter().map(|t| t.to_string()).collect::<Vec<_>>().join(","));
        
        // Check if already specialized
        if let Some(specialized) = self.specializations.get(&spec_key) {
            return Ok(specialized.clone());
        }
        
        // Get the generic type definition
        let generic_def = env.get_type(generic_name)
            .ok_or_else(|| CursedError::TypeError(format!("Unknown generic type: {}", generic_name)))?;
        
        // Check type argument count
        if type_args.len() != generic_def.type_parameters.len() {
            return Err(CursedError::TypeError(format!(
                "Type argument count mismatch: expected {}, got {}",
                generic_def.type_parameters.len(),
                type_args.len()
            )));
        }
        
        // Create type substitution
        let mut substitution = HashMap::new();
        for (param, arg) in generic_def.type_parameters.iter().zip(type_args.iter()) {
            substitution.insert(param.clone(), self.ast_type_to_type_expr(arg));
        }
        
        // Apply substitution to create specialized type
        let specialized = self.apply_substitution(&TypeExpression::named(generic_name), &substitution);
        
        // Cache the specialization
        self.specializations.insert(spec_key, specialized.clone());
        
        Ok(specialized)
    }
    
    /// Apply type substitution to a type expression
    fn apply_substitution(&self, 
        type_expr: &TypeExpression, 
        substitution: &HashMap<String, TypeExpression>
    ) -> TypeExpression {
        if let Some(name) = &type_expr.name {
            if let Some(replacement) = substitution.get(name) {
                return replacement.clone();
            }
        }
        
        TypeExpression {
            kind: type_expr.kind.clone(),
            name: type_expr.name.clone(),
            parameters: type_expr.parameters.iter()
                .map(|p| self.apply_substitution(p, substitution))
                .collect(),
            return_type: type_expr.return_type.as_ref()
                .map(|rt| Box::new(self.apply_substitution(rt, substitution))),
        }
    }
    
    /// Convert AST Type to TypeExpression
    fn ast_type_to_type_expr(&self, ast_type: &Type) -> TypeExpression {
        match ast_type {
            Type::Generic(name, args) => {
                let params = args.iter().map(|a| self.ast_type_to_type_expr(a)).collect();
                TypeExpression::generic(name, params)
            }
            Type::Custom(name) => TypeExpression::named(name),
            Type::Array(inner, _) => TypeExpression::array(self.ast_type_to_type_expr(inner)),
            Type::Pointer(inner) => TypeExpression::pointer(self.ast_type_to_type_expr(inner)),
            Type::Tuple(types) => {
                let elements = types.iter().map(|t| self.ast_type_to_type_expr(t)).collect();
                TypeExpression::tuple(elements)
            }
            Type::Normie => TypeExpression::named("normie"),
            Type::Tea => TypeExpression::named("tea"),
            Type::Lit => TypeExpression::named("lit"),
            Type::Sip => TypeExpression::named("sip"),
            Type::Smol => TypeExpression::named("smol"),
            Type::Mid => TypeExpression::named("mid"),
            Type::Thicc => TypeExpression::named("thicc"),
            Type::Snack => TypeExpression::named("snack"),
            Type::Meal => TypeExpression::named("meal"),
            Type::Byte => TypeExpression::named("byte"),
            Type::Rune => TypeExpression::named("rune"),
            Type::Extra => TypeExpression::named("extra"),
            _ => TypeExpression::named("unknown"),
        }
    }
    
    /// Add a constraint to the type checker
    pub fn add_constraint(&mut self, constraint: GenericConstraint) {
        self.constraints.push(constraint);
    }
    
    /// Clear all type bindings and constraints
    pub fn clear_context(&mut self) {
        self.type_bindings.clear();
        self.constraints.clear();
    }
    
    /// Get current type bindings
    pub fn get_type_bindings(&self) -> &HashMap<String, TypeExpression> {
        &self.type_bindings
    }
}

impl Default for GenericTypeChecker {
    fn default() -> Self {
        Self::new()
    }
}

/// Enhanced interface checker with method resolution
#[derive(Debug)]
pub struct InterfaceChecker {
    /// Interface definitions
    interfaces: HashMap<String, InterfaceDefinition>,
    
    /// Type implementations
    implementations: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct InterfaceDefinition {
    pub name: String,
    pub methods: Vec<MethodDefinition>,
    pub type_parameters: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct MethodDefinition {
    pub name: String,
    pub parameters: Vec<TypeExpression>,
    pub return_type: Option<TypeExpression>,
}

impl InterfaceChecker {
    pub fn new() -> Self {
        Self {
            interfaces: HashMap::new(),
            implementations: HashMap::new(),
        }
    }
    
    /// Register an interface definition
    pub fn register_interface(&mut self, interface: InterfaceDefinition) {
        self.interfaces.insert(interface.name.clone(), interface);
    }
    
    /// Check if a type implements an interface
    pub fn check_implementation(&self, 
        type_name: &str, 
        interface_name: &str,
        env: &TypeEnvironment
    ) -> Result<bool, CursedError> {
        // Get the interface definition
        let interface = self.interfaces.get(interface_name)
            .ok_or_else(|| CursedError::TypeError(format!("Unknown interface: {}", interface_name)))?;
        
        // Get the type definition
        let type_def = env.get_type(type_name)
            .ok_or_else(|| CursedError::TypeError(format!("Unknown type: {}", type_name)))?;
        
        // Check if all interface methods are implemented
        for method in &interface.methods {
            let has_method = type_def.methods.iter().any(|m| {
                m.name == method.name &&
                m.parameters.len() == method.parameters.len() &&
                m.return_type.as_ref().map(|rt| rt.name.clone()) == method.return_type.as_ref().map(|rt| rt.name.clone())
            });
            
            if !has_method {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
    
    /// Register a type implementation for an interface
    pub fn register_implementation(&mut self, type_name: String, interface_name: String) {
        self.implementations.entry(type_name).or_insert_with(Vec::new).push(interface_name);
    }
    
    /// Get all interfaces implemented by a type
    pub fn get_implementations(&self, type_name: &str) -> Vec<&String> {
        self.implementations.get(type_name).map(|v| v.iter().collect()).unwrap_or_default()
    }
}

impl Default for InterfaceChecker {
    fn default() -> Self {
        Self::new()
    }
}

/// Type specialization engine
#[derive(Debug)]
pub struct TypeSpecializer {
    /// Cache of specialized types
    cache: HashMap<String, TypeExpression>,
    
    /// Specialization statistics
    stats: SpecializationStats,
}

#[derive(Debug, Default)]
pub struct SpecializationStats {
    pub total_specializations: usize,
    pub cache_hits: usize,
    pub cache_misses: usize,
}

impl TypeSpecializer {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            stats: SpecializationStats::default(),
        }
    }
    
    /// Specialize a generic type with concrete arguments
    pub fn specialize(&mut self, 
        generic_type: &TypeExpression, 
        type_args: &[TypeExpression]
    ) -> Result<TypeExpression, CursedError> {
        // Create cache key
        let type_args_str: Vec<String> = type_args.iter()
            .map(|t| t.name.clone().unwrap_or_else(|| "unknown".to_string()))
            .collect();
        let cache_key = format!("{}<{}>", 
            generic_type.name.as_ref().unwrap_or("unknown"),
            type_args_str.join(",")
        );
        
        // Check cache
        if let Some(cached) = self.cache.get(&cache_key) {
            self.stats.cache_hits += 1;
            return Ok(cached.clone());
        }
        
        self.stats.cache_misses += 1;
        
        // Perform specialization
        let specialized = self.perform_specialization(generic_type, type_args)?;
        
        // Cache result
        self.cache.insert(cache_key, specialized.clone());
        self.stats.total_specializations += 1;
        
        Ok(specialized)
    }
    
    fn perform_specialization(&self, 
        generic_type: &TypeExpression, 
        type_args: &[TypeExpression]
    ) -> Result<TypeExpression, CursedError> {
        // Create substitution map
        let mut substitution = HashMap::new();
        for (i, arg) in type_args.iter().enumerate() {
            if i < generic_type.parameters.len() {
                if let Some(param_name) = &generic_type.parameters[i].name {
                    substitution.insert(param_name.clone(), arg.clone());
                }
            }
        }
        
        // Apply substitution
        Ok(self.apply_substitution_to_type(generic_type, &substitution))
    }
    
    fn apply_substitution_to_type(&self, 
        type_expr: &TypeExpression, 
        substitution: &HashMap<String, TypeExpression>
    ) -> TypeExpression {
        if let Some(name) = &type_expr.name {
            if let Some(replacement) = substitution.get(name) {
                return replacement.clone();
            }
        }
        
        TypeExpression {
            kind: type_expr.kind.clone(),
            name: type_expr.name.clone(),
            parameters: type_expr.parameters.iter()
                .map(|p| self.apply_substitution_to_type(p, substitution))
                .collect(),
            return_type: type_expr.return_type.as_ref()
                .map(|rt| Box::new(self.apply_substitution_to_type(rt, substitution))),
        }
    }
    
    /// Get specialization statistics
    pub fn get_stats(&self) -> &SpecializationStats {
        &self.stats
    }
    
    /// Clear specialization cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

impl Default for TypeSpecializer {
    fn default() -> Self {
        Self::new()
    }
}
