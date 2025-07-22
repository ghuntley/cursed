//! Monomorphisation pipeline for CURSED generics
//! 
//! This module implements the complete monomorphisation system that converts
//! generic functions and types into concrete instances for code generation.

use crate::error_types::Error as CursedError;
use crate::ast::{Expression, Statement, FunctionDeclaration, StructDeclaration, Program, InterfaceStatement};
use crate::type_system::{TypeExpression, TypeEnvironment, GenericConstraint, ConstraintBinding};
use crate::type_system::constraint_resolver::{ConstraintResolver, ConstraintSolution};
use crate::type_system::generic_interfaces::{GenericInterface, GenericInterfaceChecker, InterfaceImplementation};
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};

/// Template-to-instance cache with LRU eviction policy
#[derive(Debug)]
pub struct InstanceCache {
    /// Map from instance key to cached instance
    cache: HashMap<String, MonomorphisedInstance>,
    /// LRU access order tracking
    access_order: VecDeque<String>,
    /// Maximum cache size before eviction
    max_size: usize,
    /// Cache hit statistics
    hits: usize,
    /// Cache miss statistics  
    misses: usize,
}

impl InstanceCache {
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: HashMap::new(),
            access_order: VecDeque::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }

    pub fn get(&mut self, key: &str) -> Option<MonomorphisedInstance> {
        if let Some(instance) = self.cache.get(key) {
            // Move to front for LRU
            self.access_order.retain(|k| k != key);
            self.access_order.push_front(key.to_string());
            self.hits += 1;
            Some(instance.clone())
        } else {
            self.misses += 1;
            None
        }
    }

    pub fn insert(&mut self, key: String, instance: MonomorphisedInstance) {
        // Remove if already exists to update position
        if self.cache.contains_key(&key) {
            self.access_order.retain(|k| k != &key);
        } else if self.cache.len() >= self.max_size {
            // Evict least recently used
            if let Some(lru_key) = self.access_order.pop_back() {
                self.cache.remove(&lru_key);
            }
        }

        // Insert new entry
        self.cache.insert(key.clone(), instance);
        self.access_order.push_front(key);
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.cache.contains_key(key)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &MonomorphisedInstance)> {
        self.cache.iter()
    }

    pub fn clear(&mut self) {
        self.cache.clear();
        self.access_order.clear();
        self.hits = 0;
        self.misses = 0;
    }

    pub fn get_stats(&self) -> (usize, usize, f32) {
        let total = self.hits + self.misses;
        let hit_rate = if total > 0 { self.hits as f32 / total as f32 } else { 0.0 };
        (self.hits, self.misses, hit_rate)
    }
}

/// Main monomorphisation pipeline that generates concrete AST instances
#[derive(Debug)]
pub struct MonomorphisationPipeline {
    /// Resolver for generic constraints
    constraint_resolver: ConstraintResolver,
    /// Cache of monomorphised instances with eviction policy
    instance_cache: InstanceCache,
    /// Track instantiated generic types/functions to prevent infinite recursion
    instantiation_stack: Vec<String>,
    /// Global type environment
    type_env: TypeEnvironment,
    /// Generic interface checker for interface monomorphization
    interface_checker: GenericInterfaceChecker,
}

/// A concrete instance generated from a generic declaration
#[derive(Debug, Clone)]
pub struct MonomorphisedInstance {
    /// Unique identifier for this instance
    pub instance_id: String,
    /// Original generic declaration name
    pub generic_name: String,
    /// Concrete type arguments used for instantiation
    pub type_arguments: Vec<TypeExpression>,
    /// Generated concrete AST for this instance
    pub concrete_ast: ConcreteAST,
    /// Type constraints that were satisfied for this instance
    pub satisfied_constraints: Vec<GenericConstraint>,
}

/// Concrete AST generated from generic templates
#[derive(Debug, Clone)]
pub enum ConcreteAST {
    Function(ConcreteFunctionDeclaration),
    Struct(ConcreteStructDeclaration),
    Method(ConcreteMethodDeclaration),
    Interface(ConcreteInterfaceDeclaration),
}

/// Concrete interface declaration with resolved types
#[derive(Debug, Clone)]
pub struct ConcreteInterfaceDeclaration {
    pub name: String,
    pub methods: Vec<ConcreteMethodDeclaration>,
    pub type_signature: String,
}

/// Concrete function declaration with resolved types
#[derive(Debug, Clone)]
pub struct ConcreteFunctionDeclaration {
    pub name: String,
    pub parameters: Vec<ConcreteParameter>,
    pub return_type: Option<TypeExpression>,
    pub body: Vec<Statement>,
    pub type_signature: String,
}

/// Concrete struct declaration with resolved field types
#[derive(Debug, Clone)]
pub struct ConcreteStructDeclaration {
    pub name: String,
    pub fields: Vec<ConcreteField>,
    pub methods: Vec<ConcreteMethodDeclaration>,
    pub type_signature: String,
}

/// Concrete method declaration
#[derive(Debug, Clone)]
pub struct ConcreteMethodDeclaration {
    pub name: String,
    pub receiver: Option<ConcreteParameter>,
    pub parameters: Vec<ConcreteParameter>,
    pub return_type: Option<TypeExpression>,
    pub body: Vec<Statement>,
}

/// Parameter with concrete type
#[derive(Debug, Clone)]
pub struct ConcreteParameter {
    pub name: String,
    pub type_expr: TypeExpression,
}

/// Field with concrete type
#[derive(Debug, Clone)]
pub struct ConcreteField {
    pub name: String,
    pub type_expr: TypeExpression,
}

/// Request for monomorphisation of a generic declaration
#[derive(Debug, Clone)]
pub struct InstantiationRequest {
    pub generic_name: String,
    pub type_arguments: Vec<TypeExpression>,
    pub constraints: Vec<GenericConstraint>,
    pub call_site: Option<String>,
}

impl MonomorphisationPipeline {
    pub fn new(type_env: TypeEnvironment) -> Self {
        let interface_checker = GenericInterfaceChecker::new(type_env.clone());
        Self {
            constraint_resolver: ConstraintResolver::new(),
            instance_cache: InstanceCache::new(1000), // Default cache size of 1000 instances
            instantiation_stack: Vec::new(),
            type_env,
            interface_checker,
        }
    }

    /// Main entry point: generate concrete instances for all generic usage
    pub fn monomorphise_program(&mut self, program: &Program) -> Result<MonomorphisedProgram, CursedError> {
        let mut instantiation_requests = Vec::new();
        
        // Collect all generic instantiation requests from the program
        self.collect_instantiation_requests(program, &mut instantiation_requests)?;
        
        // Process each request to generate concrete instances
        let mut concrete_instances = Vec::new();
        for request in instantiation_requests {
            if let Some(instance) = self.instantiate_generic(&request)? {
                concrete_instances.push(instance);
            }
        }
        
        Ok(MonomorphisedProgram {
            original_program: program.clone(),
            concrete_instances,
            instantiation_map: self.build_instantiation_map(),
        })
    }

    /// Collect all generic instantiation requests from AST nodes
    fn collect_instantiation_requests(&self, program: &Program, requests: &mut Vec<InstantiationRequest>) -> Result<(), CursedError> {
        // Traverse AST to find generic function calls and type instantiations
        for statement in &program.statements {
            self.collect_from_statement(statement, requests)?;
        }
        
        // Functions are part of statements, not a separate field
        // They are collected through Statement::Function variant
        
        Ok(())
    }

    /// Collect instantiation requests from statements
    fn collect_from_statement(&self, statement: &Statement, requests: &mut Vec<InstantiationRequest>) -> Result<(), CursedError> {
        match statement {
            Statement::Expression(expr) => {
                self.collect_from_expression(expr, requests)?;
            }
            Statement::Let(let_stmt) => {
                if let Some(ast_type) = &let_stmt.var_type {
                    let type_expr = self.convert_ast_type_to_expression(ast_type);
                    if self.is_generic_instantiation(&type_expr) {
                        requests.push(InstantiationRequest {
                            generic_name: type_expr.name.clone().unwrap_or_default(),
                            type_arguments: type_expr.parameters.clone(),
                            constraints: Vec::new(),
                            call_site: Some(format!("variable:{}", let_stmt.target.primary_name())),
                        });
                    }
                }
                
                self.collect_from_expression(&let_stmt.value, requests)?;
            }
            Statement::Return(return_stmt) => {
                if let Some(value) = &return_stmt.value {
                    self.collect_from_expression(value, requests)?;
                }
            }
            Statement::If(if_stmt) => {
                self.collect_from_expression(&if_stmt.condition, requests)?;
                for stmt in &if_stmt.then_branch {
                    self.collect_from_statement(stmt, requests)?;
                }
                if let Some(else_branch) = &if_stmt.else_branch {
                    for stmt in else_branch {
                        self.collect_from_statement(stmt, requests)?;
                    }
                }
            }
            Statement::For(for_stmt) => {
                if let Some(init) = &for_stmt.init {
                    self.collect_from_statement(init, requests)?;
                }
                if let Some(condition) = &for_stmt.condition {
                    self.collect_from_expression(condition, requests)?;
                }
                if let Some(update) = &for_stmt.update {
                    self.collect_from_expression(update, requests)?;
                }
                for stmt in &for_stmt.body {
                    self.collect_from_statement(stmt, requests)?;
                }
            }
            _ => {} // Handle other statement types as needed
        }
        
        Ok(())
    }

    /// Collect instantiation requests from expressions
    fn collect_from_expression(&self, expression: &Expression, requests: &mut Vec<InstantiationRequest>) -> Result<(), CursedError> {
        match expression {
            Expression::Call(call_expr) => {
                // Check if this is a generic function call
                if let Expression::Identifier(func_name) = &*call_expr.function {
                    if self.is_generic_function(func_name) {
                        // Extract type arguments from call context
                        let type_args = self.infer_type_arguments_from_call(call_expr)?;
                        requests.push(InstantiationRequest {
                            generic_name: func_name.clone(),
                            type_arguments: type_args,
                            constraints: self.get_function_constraints(func_name),
                            call_site: Some(format!("call:{}", func_name)),
                        });
                    }
                }
                
                // Recursively check arguments
                for arg in &call_expr.arguments {
                    self.collect_from_expression(arg, requests)?;
                }
            }
            Expression::StructLiteral(struct_literal) => {
                // Check if this is a generic struct instantiation
                if self.is_generic_struct(&struct_literal.struct_name) {
                    let type_args = self.infer_type_arguments_from_struct_literal(struct_literal)?;
                    requests.push(InstantiationRequest {
                        generic_name: struct_literal.struct_name.clone(),
                        type_arguments: type_args,
                        constraints: self.get_struct_constraints(&struct_literal.struct_name),
                        call_site: Some(format!("struct:{}", struct_literal.struct_name)),
                    });
                }
                
                // Recursively check field values
                for field in &struct_literal.fields {
                    self.collect_from_expression(&field.value, requests)?;
                }
            }
            Expression::Binary(binary_expr) => {
                self.collect_from_expression(&binary_expr.left, requests)?;
                self.collect_from_expression(&binary_expr.right, requests)?;
            }
            Expression::Unary(unary_expr) => {
                self.collect_from_expression(&unary_expr.operand, requests)?;
            }
            Expression::MemberAccess(member_access) => {
                self.collect_from_expression(&member_access.object, requests)?;
            }
            Expression::ArrayAccess(array_access) => {
                self.collect_from_expression(&array_access.array, requests)?;
                self.collect_from_expression(&array_access.index, requests)?;
            }
            _ => {} // Handle other expression types as needed
        }
        
        Ok(())
    }

    /// Collect instantiation requests from function bodies
    fn collect_from_function(&self, function: &FunctionDeclaration, requests: &mut Vec<InstantiationRequest>) -> Result<(), CursedError> {
        for statement in &function.body {
            self.collect_from_statement(statement, requests)?;
        }
        Ok(())
    }

    /// Generate a concrete instance for a generic instantiation request
    pub fn instantiate_generic(&mut self, request: &InstantiationRequest) -> Result<Option<MonomorphisedInstance>, CursedError> {
        let instance_key = self.generate_instance_key(request);
        
        // Check cache first
        if let Some(cached_instance) = self.instance_cache.get(&instance_key) {
            return Ok(Some(cached_instance));
        }
        
        // Check for infinite recursion
        if self.instantiation_stack.contains(&instance_key) {
            return Err(CursedError::RecursiveGenericInstantiation(instance_key));
        }
        
        self.instantiation_stack.push(instance_key.clone());
        
        // Resolve constraints before instantiation
        let constraint_solution = self.resolve_constraints_for_request(request)?;
        if !constraint_solution.is_satisfied {
            return Err(CursedError::ConstraintViolation(
                format!("Constraints not satisfied for {}: {:?}", 
                       request.generic_name, constraint_solution.violations)
            ));
        }
        
        // Generate concrete instance
        let instance = self.generate_concrete_instance(request, &constraint_solution)?;
        
        // Cache the result
        self.instance_cache.insert(instance_key.clone(), instance.clone());
        
        // Remove from stack
        self.instantiation_stack.pop();
        
        Ok(Some(instance))
    }

    /// Resolve constraints for an instantiation request
    fn resolve_constraints_for_request(&self, request: &InstantiationRequest) -> Result<ConstraintSolution, CursedError> {
        // Create constraint context for this instantiation
        let mut constraint_bindings = Vec::new();
        
        for (i, constraint) in request.constraints.iter().enumerate() {
            let binding = ConstraintBinding {
                constraint: constraint.clone(),
                bound_types: if i < request.type_arguments.len() {
                    vec![request.type_arguments[i].name.clone().unwrap_or_default()]
                } else {
                    vec![]
                },
                satisfaction_status: crate::type_system::ConstraintStatus::Pending,
            };
            constraint_bindings.push(binding);
        }
        
        let constraint_context = crate::type_system::ConstraintContext {
            active_constraints: constraint_bindings,
            scope_id: "monomorphisation".to_string(),
            type_bindings: HashMap::new(),
        };
        
        self.constraint_resolver.resolve_constraints(&constraint_context, &self.type_env)
            .map_err(|e| CursedError::ConstraintResolutionError(format!("{:?}", e)))
    }

    /// Generate concrete AST instance from generic template
    fn generate_concrete_instance(&self, request: &InstantiationRequest, solution: &ConstraintSolution) -> Result<MonomorphisedInstance, CursedError> {
        // Ensure all type arguments are properly specialized, not treated as void
        let specialized_args = self.ensure_type_specialization(&request.type_arguments)?;
        
        let concrete_ast = if self.is_generic_function(&request.generic_name) {
            self.instantiate_generic_function(request, solution)?
        } else if self.is_generic_struct(&request.generic_name) {
            self.instantiate_generic_struct(request, solution)?
        } else {
            return Err(CursedError::UnknownGenericType(request.generic_name.clone()));
        };
        
        Ok(MonomorphisedInstance {
            instance_id: self.generate_instance_key(request),
            generic_name: request.generic_name.clone(),
            type_arguments: specialized_args,
            concrete_ast,
            satisfied_constraints: request.constraints.clone(),
        })
    }

    /// Ensure type arguments are properly specialized instead of being treated as void
    fn ensure_type_specialization(&self, type_args: &[TypeExpression]) -> Result<Vec<TypeExpression>, CursedError> {
        let mut specialized = Vec::new();
        
        for type_arg in type_args {
            let specialized_arg = if let Some(name) = &type_arg.name {
                if name == "void" || name == "unknown" {
                    // Try to infer a better type
                    if type_arg.parameters.is_empty() {
                        // Generic parameter without specialization - use normie as fallback
                        TypeExpression::named("normie")
                    } else {
                        // Has parameters, try to specialize them
                        let specialized_params = self.ensure_type_specialization(&type_arg.parameters)?;
                        TypeExpression {
                            kind: type_arg.kind.clone(),
                            name: type_arg.name.clone(),
                            parameters: specialized_params,
                            return_type: type_arg.return_type.clone(),
                        }
                    }
                } else {
                    // Already specialized, check parameters recursively
                    if !type_arg.parameters.is_empty() {
                        let specialized_params = self.ensure_type_specialization(&type_arg.parameters)?;
                        TypeExpression {
                            kind: type_arg.kind.clone(),
                            name: type_arg.name.clone(),
                            parameters: specialized_params,
                            return_type: type_arg.return_type.clone(),
                        }
                    } else {
                        type_arg.clone()
                    }
                }
            } else {
                // No name - likely a function type, check parameters and return type
                let specialized_params = self.ensure_type_specialization(&type_arg.parameters)?;
                let specialized_return = if let Some(return_type) = &type_arg.return_type {
                    let specialized_ret = self.ensure_type_specialization(&[*return_type.clone()])?;
                    Some(Box::new(specialized_ret[0].clone()))
                } else {
                    None
                };
                
                TypeExpression {
                    kind: type_arg.kind.clone(),
                    name: type_arg.name.clone(),
                    parameters: specialized_params,
                    return_type: specialized_return,
                }
            };
            
            specialized.push(specialized_arg);
        }
        
        Ok(specialized)
    }

    /// Instantiate a generic function with concrete types
    fn instantiate_generic_function(&self, request: &InstantiationRequest, solution: &ConstraintSolution) -> Result<ConcreteAST, CursedError> {
        // Get the generic function declaration
        let generic_func = self.get_generic_function(&request.generic_name)
            .ok_or_else(|| CursedError::UnknownGenericFunction(request.generic_name.clone()))?;
        
        // Apply type substitutions to create concrete function
        let mut concrete_func = ConcreteFunctionDeclaration {
            name: format!("{}_{}", request.generic_name, self.generate_type_suffix(&request.type_arguments)),
            parameters: Vec::new(),
            return_type: None,
            body: generic_func.body.clone(),
            type_signature: self.generate_function_signature(&request.generic_name, &request.type_arguments),
        };
        
        // Substitute type parameters in function signature
        for param in &generic_func.parameters {
            if let Some(param_type) = &param.param_type {
                concrete_func.parameters.push(ConcreteParameter {
                    name: param.name.clone(),
                    type_expr: self.substitute_type_parameters(&self.convert_ast_type_to_expression(param_type), solution)?,
                });
            }
        }
        
        if let Some(return_type) = &generic_func.return_type {
            concrete_func.return_type = Some(self.substitute_type_parameters(&self.convert_ast_type_to_expression(return_type), solution)?);
        }
        
        // Substitute type parameters in function body
        concrete_func.body = self.substitute_types_in_statements(&generic_func.body, solution)?;
        
        Ok(ConcreteAST::Function(concrete_func))
    }

    /// Instantiate a generic struct with concrete types
    fn instantiate_generic_struct(&self, request: &InstantiationRequest, solution: &ConstraintSolution) -> Result<ConcreteAST, CursedError> {
        // Get the generic struct declaration
        let generic_struct = self.get_generic_struct(&request.generic_name)
            .ok_or_else(|| CursedError::UnknownGenericStruct(request.generic_name.clone()))?;
        
        // Apply type substitutions to create concrete struct
        let mut concrete_struct = ConcreteStructDeclaration {
            name: format!("{}_{}", request.generic_name, self.generate_type_suffix(&request.type_arguments)),
            fields: Vec::new(),
            methods: Vec::new(),
            type_signature: self.generate_struct_signature(&request.generic_name, &request.type_arguments),
        };
        
        // Substitute type parameters in struct fields
        for field in &generic_struct.fields {
            if let Some(field_type) = &field.field_type {
                concrete_struct.fields.push(ConcreteField {
                    name: field.name.clone(),
                    type_expr: self.substitute_type_parameters(&self.convert_ast_type_to_expression(field_type), solution)?,
                });
            }
        }
        
        // Substitute type parameters in struct methods
        // Note: StructStatement doesn't have methods field, this would need to be handled differently
        // for method in &generic_struct.methods {
        //     let concrete_method = self.instantiate_generic_method(method, solution)?;
        //     concrete_struct.methods.push(concrete_method);
        // }
        
        Ok(ConcreteAST::Struct(concrete_struct))
    }

    /// Instantiate a generic method with concrete types
    fn instantiate_generic_method(&self, generic_method: &crate::ast::MethodDeclaration, solution: &ConstraintSolution) -> Result<ConcreteMethodDeclaration, CursedError> {
        let mut concrete_method = ConcreteMethodDeclaration {
            name: generic_method.name.clone(),
            receiver: None,
            parameters: Vec::new(),
            return_type: None,
            body: generic_method.body.clone(),
        };
        
        // Substitute receiver type
        if let Some(receiver) = &generic_method.receiver {
            concrete_method.receiver = Some(ConcreteParameter {
                name: receiver.name.clone(),
                type_expr: self.substitute_type_parameters(&self.convert_ast_type_to_expression(&receiver.receiver_type), solution)?,
            });
        }
        
        // Substitute parameter types
        for param in &generic_method.parameters {
            if let Some(param_type) = &param.param_type {
                concrete_method.parameters.push(ConcreteParameter {
                    name: param.name.clone(),
                    type_expr: self.substitute_type_parameters(&self.convert_ast_type_to_expression(param_type), solution)?,
                });
            }
        }
        
        // Substitute return type
        if let Some(return_type) = &generic_method.return_type {
            concrete_method.return_type = Some(self.substitute_type_parameters(&self.convert_ast_type_to_expression(return_type), solution)?);
        }
        
        // Substitute types in method body
        concrete_method.body = self.substitute_types_in_statements(&generic_method.body, solution)?;
        
        Ok(concrete_method)
    }

    /// Substitute type parameters with concrete types
    fn substitute_type_parameters(&self, type_expr: &TypeExpression, solution: &ConstraintSolution) -> Result<TypeExpression, CursedError> {
        if let Some(type_name) = &type_expr.name {
            // Check if this is a type parameter that needs substitution
            if let Some(substitution) = solution.substitutions.get(type_name) {
                // Validate that substitution is valid
                self.validate_type_substitution(type_name, substitution)?;
                return Ok(substitution.clone());
            }
        }
        
        // Recursively substitute in type parameters
        let mut substituted_params = Vec::new();
        for param in &type_expr.parameters {
            match self.substitute_type_parameters(param, solution) {
                Ok(substituted_param) => substituted_params.push(substituted_param),
                Err(e) => return Err(CursedError::Type(format!("Failed to substitute type parameter in {}: {}", 
                                     type_expr.name.as_ref().unwrap_or(&"unknown".to_string()), 
                                     e))),
            }
        }
        
        let substituted_return_type = if let Some(return_type) = &type_expr.return_type {
            match self.substitute_type_parameters(return_type, solution) {
                Ok(substituted_rt) => Some(Box::new(substituted_rt)),
                Err(e) => return Err(CursedError::Type(format!("Failed to substitute return type in {}: {}", 
                                     type_expr.name.as_ref().unwrap_or(&"unknown".to_string()), 
                                     e))),
            }
        } else {
            None
        };
        
        Ok(TypeExpression {
            kind: type_expr.kind.clone(),
            name: type_expr.name.clone(),
            parameters: substituted_params,
            return_type: substituted_return_type,
        })
    }

    /// Validate that a type substitution is valid
    fn validate_type_substitution(&self, type_param: &str, substitution: &TypeExpression) -> Result<(), CursedError> {
        // Check if substitution creates circular dependencies
        if self.has_circular_type_dependency(type_param, substitution) {
            return Err(CursedError::Type(format!("Circular type dependency detected: {} -> {}", type_param, 
                                 substitution.name.as_ref().unwrap_or(&"unknown".to_string()))));
        }
        
        // Check if substitution is a valid type
        if !self.is_valid_type_expression(substitution) {
            return Err(CursedError::Type(format!("Invalid type substitution: {} cannot be substituted with {}", 
                                 type_param, 
                                 substitution.name.as_ref().unwrap_or(&"unknown".to_string()))));
        }
        
        Ok(())
    }

    /// Check if a type substitution creates circular dependencies
    fn has_circular_type_dependency(&self, type_param: &str, substitution: &TypeExpression) -> bool {
        if let Some(subst_name) = &substitution.name {
            if subst_name == type_param {
                return true;
            }
            
            // Check parameters recursively
            for param in &substitution.parameters {
                if self.has_circular_type_dependency(type_param, param) {
                    return true;
                }
            }
            
            // Check return type recursively
            if let Some(return_type) = &substitution.return_type {
                if self.has_circular_type_dependency(type_param, return_type) {
                    return true;
                }
            }
        }
        
        false
    }

    /// Check if a type expression is valid
    fn is_valid_type_expression(&self, type_expr: &TypeExpression) -> bool {
        // Check if type name is valid
        if let Some(type_name) = &type_expr.name {
            // Check if it's a built-in type
            if self.is_builtin_type(type_name) {
                return true;
            }
            
            // Check if it's a defined type in the type environment
            if self.type_env.get_type(type_name).is_some() {
                return true;
            }
        }
        
        // Check parameters recursively
        for param in &type_expr.parameters {
            if !self.is_valid_type_expression(param) {
                return false;
            }
        }
        
        // Check return type recursively
        if let Some(return_type) = &type_expr.return_type {
            if !self.is_valid_type_expression(return_type) {
                return false;
            }
        }
        
        true
    }

    /// Check if a type is a built-in type
    fn is_builtin_type(&self, type_name: &str) -> bool {
        matches!(type_name, "normie" | "tea" | "lit" | "sip" | "smol" | "mid" | "thicc" | "snack" | "meal" | "void")
    }

    /// Substitute types in statement list
    fn substitute_types_in_statements(&self, statements: &[Statement], solution: &ConstraintSolution) -> Result<Vec<Statement>, CursedError> {
        let mut substituted_statements = Vec::new();
        
        for statement in statements {
            substituted_statements.push(self.substitute_types_in_statement(statement, solution)?);
        }
        
        Ok(substituted_statements)
    }

    /// Substitute types in a single statement
    fn substitute_types_in_statement(&self, statement: &Statement, solution: &ConstraintSolution) -> Result<Statement, CursedError> {
        match statement {
            Statement::Let(let_stmt) => {
                let mut new_let_stmt = let_stmt.clone();
                if let Some(var_type) = &let_stmt.var_type {
                    let type_expr = self.convert_ast_type_to_expression(var_type);
                    let substituted_type_expr = self.substitute_type_parameters(&type_expr, solution)?;
                    new_let_stmt.var_type = Some(self.convert_type_expression_to_ast(&substituted_type_expr));
                }
                new_let_stmt.value = self.substitute_types_in_expression(&let_stmt.value, solution)?;
                Ok(Statement::Let(new_let_stmt))
            }
            Statement::Expression(expr) => {
                Ok(Statement::Expression(self.substitute_types_in_expression(expr, solution)?))
            }
            Statement::Return(return_stmt) => {
                let mut new_return = return_stmt.clone();
                if let Some(value) = &return_stmt.value {
                    new_return.value = Some(self.substitute_types_in_expression(value, solution)?);
                }
                Ok(Statement::Return(new_return))
            }
            Statement::If(if_stmt) => {
                let mut new_if_stmt = if_stmt.clone();
                new_if_stmt.condition = self.substitute_types_in_expression(&if_stmt.condition, solution)?;
                new_if_stmt.then_branch = self.substitute_types_in_statements(&if_stmt.then_branch, solution)?;
                if let Some(else_body) = &if_stmt.else_branch {
                    new_if_stmt.else_branch = Some(self.substitute_types_in_statements(else_body, solution)?);
                }
                Ok(Statement::If(new_if_stmt))
            }
            Statement::While(while_stmt) => {
                let mut new_while_stmt = while_stmt.clone();
                new_while_stmt.condition = self.substitute_types_in_expression(&while_stmt.condition, solution)?;
                new_while_stmt.body = self.substitute_types_in_statements(&while_stmt.body, solution)?;
                Ok(Statement::While(new_while_stmt))
            }
            Statement::For(for_stmt) => {
                let mut new_for_stmt = for_stmt.clone();
                if let Some(init) = &for_stmt.init {
                    new_for_stmt.init = Some(Box::new(self.substitute_types_in_statement(init, solution)?));
                }
                if let Some(condition) = &for_stmt.condition {
                    new_for_stmt.condition = Some(self.substitute_types_in_expression(condition, solution)?);
                }
                if let Some(update) = &for_stmt.update {
                    new_for_stmt.update = Some(self.substitute_types_in_expression(update, solution)?);
                }
                new_for_stmt.body = self.substitute_types_in_statements(&for_stmt.body, solution)?;
                Ok(Statement::For(new_for_stmt))
            }
            Statement::Break(_) | Statement::Continue(_) => {
                // Break and continue don't have types to substitute
                Ok(statement.clone())
            }
            // Block statement not currently in AST enum - remove or handle differently
            Statement::Assignment(assign_stmt) => {
                let mut new_assign_stmt = assign_stmt.clone();
                // target is AssignmentTarget, not Expression, so we don't substitute it
                new_assign_stmt.value = self.substitute_types_in_expression(&assign_stmt.value, solution)?;
                Ok(Statement::Assignment(new_assign_stmt))
            }
            Statement::Function(func_stmt) => {
                let mut new_func_stmt = func_stmt.clone();
                // Substitute parameter types
                for param in &mut new_func_stmt.parameters {
                    if let Some(param_type) = &param.param_type {
                        let type_expr = self.convert_ast_type_to_expression(param_type);
                        let substituted_type_expr = self.substitute_type_parameters(&type_expr, solution)?;
                        param.param_type = Some(self.convert_type_expression_to_ast(&substituted_type_expr));
                    }
                }
                // Substitute return type
                if let Some(return_type) = &func_stmt.return_type {
                    let type_expr = self.convert_ast_type_to_expression(return_type);
                    let substituted_type_expr = self.substitute_type_parameters(&type_expr, solution)?;
                    new_func_stmt.return_type = Some(self.convert_type_expression_to_ast(&substituted_type_expr));
                }
                // Substitute function body
                new_func_stmt.body = self.substitute_types_in_statements(&func_stmt.body, solution)?;
                Ok(Statement::Function(new_func_stmt))
            }
            Statement::Struct(struct_stmt) => {
                let mut new_struct_stmt = struct_stmt.clone();
                // Substitute field types
                for field in &mut new_struct_stmt.fields {
                    if let Some(field_type) = &field.field_type {
                        let type_expr = self.convert_ast_type_to_expression(field_type);
                        let substituted_type_expr = self.substitute_type_parameters(&type_expr, solution)?;
                        field.field_type = Some(self.convert_type_expression_to_ast(&substituted_type_expr));
                    }
                }
                Ok(Statement::Struct(new_struct_stmt))
            }
            Statement::Defer(defer_stmt) => {
                let mut new_defer_stmt = defer_stmt.clone();
                new_defer_stmt.expression = Box::new(self.substitute_types_in_expression(&defer_stmt.expression, solution)?);
                Ok(Statement::Defer(new_defer_stmt))
            }
            Statement::PatternSwitch(pattern_switch) => {
                let mut new_pattern_switch = pattern_switch.clone();
                new_pattern_switch.expression = self.substitute_types_in_expression(&pattern_switch.expression, solution)?;
                // Substitute pattern cases
                for case in &mut new_pattern_switch.cases {
                    case.body = self.substitute_types_in_statements(&case.body, solution)?;
                }
                Ok(Statement::PatternSwitch(new_pattern_switch))
            }
            Statement::Interface(interface_stmt) => {
                let mut new_interface_stmt = interface_stmt.clone();
                // Substitute method parameter and return types
                for method in &mut new_interface_stmt.methods {
                    for param in &mut method.parameters {
                        if let Some(param_type) = &param.param_type {
                            let type_expr = self.convert_ast_type_to_expression(param_type);
                            let substituted_type_expr = self.substitute_type_parameters(&type_expr, solution)?;
                            param.param_type = Some(self.convert_type_expression_to_ast(&substituted_type_expr));
                        }
                    }
                    if let Some(return_type) = &method.return_type {
                        let type_expr = self.convert_ast_type_to_expression(return_type);
                        let substituted_type_expr = self.substitute_type_parameters(&type_expr, solution)?;
                        method.return_type = Some(self.convert_type_expression_to_ast(&substituted_type_expr));
                    }
                }
                Ok(Statement::Interface(new_interface_stmt))
            }
            _ => {
                // For unhandled statement types, clone without substitution
                // This ensures we don't crash on new statement types
                Ok(statement.clone())
            }
        }
    }

    /// Substitute types in an expression
    fn substitute_types_in_expression(&self, expression: &Expression, solution: &ConstraintSolution) -> Result<Expression, CursedError> {
        match expression {
            Expression::Call(call_expr) => {
                let mut new_call = call_expr.clone();
                new_call.function = Box::new(self.substitute_types_in_expression(&call_expr.function, solution)?);
                
                let mut new_args = Vec::new();
                for arg in &call_expr.arguments {
                    new_args.push(self.substitute_types_in_expression(arg, solution)?);
                }
                new_call.arguments = new_args;
                
                Ok(Expression::Call(new_call))
            }
            Expression::Binary(binary_expr) => {
                Ok(Expression::Binary(crate::ast::BinaryExpression {
                    left: Box::new(self.substitute_types_in_expression(&binary_expr.left, solution)?),
                    operator: binary_expr.operator.clone(),
                    right: Box::new(self.substitute_types_in_expression(&binary_expr.right, solution)?),
                }))
            }
            Expression::Unary(unary_expr) => {
                Ok(Expression::Unary(crate::ast::UnaryExpression {
                    operator: unary_expr.operator.clone(),
                    operand: Box::new(self.substitute_types_in_expression(&unary_expr.operand, solution)?),
                }))
            }
            Expression::MemberAccess(member_expr) => {
                let mut new_member_expr = member_expr.clone();
                new_member_expr.object = Box::new(self.substitute_types_in_expression(&member_expr.object, solution)?);
                Ok(Expression::MemberAccess(new_member_expr))
            }
            Expression::ArrayAccess(array_expr) => {
                let mut new_array_expr = array_expr.clone();
                new_array_expr.array = Box::new(self.substitute_types_in_expression(&array_expr.array, solution)?);
                new_array_expr.index = Box::new(self.substitute_types_in_expression(&array_expr.index, solution)?);
                Ok(Expression::ArrayAccess(new_array_expr))
            }
            Expression::Tuple(tuple_expr) => {
                let mut new_elements = Vec::new();
                for element in &tuple_expr.elements {
                    new_elements.push(self.substitute_types_in_expression(element, solution)?);
                }
                Ok(Expression::Tuple(crate::ast::TupleExpression {
                    elements: new_elements,
                }))
            }
            Expression::Array(array_expr) => {
                let mut new_elements = Vec::new();
                for element in array_expr {
                    new_elements.push(self.substitute_types_in_expression(element, solution)?);
                }
                Ok(Expression::Array(new_elements))
            }
            Expression::StructLiteral(struct_expr) => {
                let mut new_struct_expr = struct_expr.clone();
                // Handle struct literal field substitution
                for field in &mut new_struct_expr.fields {
                    field.value = self.substitute_types_in_expression(&field.value, solution)?;
                }
                Ok(Expression::StructLiteral(new_struct_expr))
            }
            Expression::TypeAssertion(type_assert_expr) => {
                let mut new_type_assert = type_assert_expr.clone();
                new_type_assert.value = Box::new(self.substitute_types_in_expression(&type_assert_expr.value, solution)?);
                // Substitute the asserted type
                let type_expr = self.convert_ast_type_to_expression(&type_assert_expr.target_type);
                let substituted_type_expr = self.substitute_type_parameters(&type_expr, solution)?;
                new_type_assert.target_type = self.convert_type_expression_to_ast(&substituted_type_expr);
                Ok(Expression::TypeAssertion(new_type_assert))
            }
            // Cast expression not in current AST - remove or handle differently
            Expression::Lambda(lambda_expr) => {
                let mut new_lambda = lambda_expr.clone();
                // Lambda parameters are just strings, no type substitution needed
                // Substitute lambda body
                new_lambda.body = Box::new(self.substitute_types_in_expression(&lambda_expr.body, solution)?);
                Ok(Expression::Lambda(new_lambda))
            }
            // Literals and identifiers don't need type substitution
            Expression::Literal(_) | Expression::Identifier(_) => {
                Ok(expression.clone())
            }
            // Note: Channel, Select, TypeSwitch, Match, and Range expressions
            // are not currently in the AST enum, so they are handled by the default case
            _ => {
                // For unhandled expression types, clone without substitution
                // This ensures we don't crash on new expression types
                Ok(expression.clone())
            }
        }
    }

    // Helper methods for type checking and inference

    fn is_generic_instantiation(&self, type_expr: &TypeExpression) -> bool {
        // Check if type has parameters (indicates generic instantiation)
        !type_expr.parameters.is_empty() && self.is_generic_type(&type_expr.name.as_ref().unwrap_or(&String::new()))
    }

    fn is_generic_function(&self, name: &str) -> bool {
        // Check if function is registered as generic in type environment
        self.type_env.get_generic_function(name).is_some()
    }

    fn is_generic_struct(&self, name: &str) -> bool {
        // Check if struct is registered as generic in type environment  
        self.type_env.get_generic_struct(name).is_some()
    }

    fn is_generic_type(&self, name: &str) -> bool {
        self.is_generic_function(name) || self.is_generic_struct(name)
    }

    fn get_generic_function(&self, name: &str) -> Option<&FunctionDeclaration> {
        self.type_env.get_generic_function(name)
    }

    fn get_generic_struct(&self, name: &str) -> Option<&StructDeclaration> {
        self.type_env.get_generic_struct(name)
    }

    fn get_function_constraints(&self, name: &str) -> Vec<GenericConstraint> {
        self.type_env.get_function_constraints(name).unwrap_or_default()
    }

    fn get_struct_constraints(&self, name: &str) -> Vec<GenericConstraint> {
        self.type_env.get_struct_constraints(name).unwrap_or_default()
    }

    fn infer_type_arguments_from_call(&self, call_expr: &crate::ast::CallExpression) -> Result<Vec<TypeExpression>, CursedError> {
        // Simple type inference from argument types
        let mut type_args = Vec::new();
        
        for arg in &call_expr.arguments {
            let inferred_type = self.infer_expression_type(arg)?;
            type_args.push(inferred_type);
        }
        
        Ok(type_args)
    }

    fn infer_type_arguments_from_struct_literal(&self, struct_literal: &crate::ast::StructLiteral) -> Result<Vec<TypeExpression>, CursedError> {
        // Infer from field value types
        let mut type_args = Vec::new();
        
        for field in &struct_literal.fields {
            let inferred_type = self.infer_expression_type(&field.value)?;
            type_args.push(inferred_type);
        }
        
        Ok(type_args)
    }

    fn infer_expression_type(&self, expression: &Expression) -> Result<TypeExpression, CursedError> {
        match expression {
            Expression::Integer(_) => Ok(TypeExpression::named("normie")),
            Expression::String(_) => Ok(TypeExpression::named("tea")),
            Expression::Boolean(_) => Ok(TypeExpression::named("lit")),
            Expression::Identifier(name) => {
                // Look up variable type in environment
                self.type_env.get_variable_type(name)
                    .ok_or_else(|| CursedError::UnknownVariable(name.clone()))
            }
            _ => Ok(TypeExpression::named("unknown")),
        }
    }

    fn generate_instance_key(&self, request: &InstantiationRequest) -> String {
        // Generate stable, platform-independent hash for type fingerprint
        let type_fingerprint = self.generate_type_fingerprint(&request.type_arguments);
        format!("{}_{}", request.generic_name, type_fingerprint)
    }

    /// Generate a stable type fingerprint for cross-platform builds
    fn generate_type_fingerprint(&self, type_args: &[TypeExpression]) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        
        // Hash each type argument deterministically
        for type_arg in type_args {
            self.hash_type_expression(type_arg, &mut hasher);
        }
        
        // Generate stable TypeId from hash
        let hash_value = hasher.finish();
        format!("T{:016x}", hash_value)
    }

    /// Hash a type expression deterministically for fingerprint generation
    fn hash_type_expression(&self, type_expr: &TypeExpression, hasher: &mut impl Hasher) {
        // Hash the type expression as a string representation
        format!("{:?}", type_expr).hash(hasher);
        
        // Hash the name if present
        if let Some(name) = &type_expr.name {
            name.hash(hasher);
        }
        
        // Hash parameters recursively
        type_expr.parameters.len().hash(hasher);
        for param in &type_expr.parameters {
            self.hash_type_expression(param, hasher);
        }
        
        // Hash return type if present
        if let Some(return_type) = &type_expr.return_type {
            self.hash_type_expression(return_type, hasher);
        }
    }

    fn generate_type_suffix(&self, type_args: &[TypeExpression]) -> String {
        type_args.iter()
            .map(|t| t.name.as_deref().unwrap_or("unknown"))
            .collect::<Vec<_>>()
            .join("_")
    }

    fn generate_function_signature(&self, name: &str, type_args: &[TypeExpression]) -> String {
        format!("{}[{}]", name, type_args.iter()
            .map(|t| t.name.as_deref().unwrap_or("unknown"))
            .collect::<Vec<_>>()
            .join(", "))
    }

    fn generate_struct_signature(&self, name: &str, type_args: &[TypeExpression]) -> String {
        format!("{}[{}]", name, type_args.iter()
            .map(|t| t.name.as_deref().unwrap_or("unknown"))
            .collect::<Vec<_>>()
            .join(", "))
    }

    fn build_instantiation_map(&self) -> HashMap<String, String> {
        self.instance_cache.iter()
            .map(|(key, instance)| (instance.generic_name.clone(), key.clone()))
            .collect()
    }

    /// Convert AST Type to TypeExpression for monomorphization
    fn convert_ast_type_to_expression(&self, ast_type: &crate::ast::Type) -> TypeExpression {
        use crate::ast::Type;
        
        match ast_type {
            Type::Normie => TypeExpression::named("normie"),
            Type::Tea => TypeExpression::named("tea"),
            Type::Lit => TypeExpression::named("lit"),
            Type::Sip => TypeExpression::named("sip"),
            Type::Smol => TypeExpression::named("smol"),
            Type::Mid => TypeExpression::named("mid"),
            Type::Thicc => TypeExpression::named("thicc"),
            Type::Snack => TypeExpression::named("snack"),
            Type::Meal => TypeExpression::named("meal"),

            Type::Integer => TypeExpression::named("normie"),
            Type::Float => TypeExpression::named("meal"),
            Type::String => TypeExpression::named("tea"),
            Type::Boolean => TypeExpression::named("lit"),
            Type::Void => TypeExpression::named("void"),
            Type::Custom(name) => TypeExpression::named(name),
            Type::Array(element_type, _) => {
                TypeExpression::array(self.convert_ast_type_to_expression(element_type))
            }
            Type::Slice(element_type) => {
                TypeExpression::generic("slice", vec![self.convert_ast_type_to_expression(element_type)])
            }
            Type::Function(params, return_type) => {
                let param_types: Vec<TypeExpression> = params.iter()
                    .map(|p| self.convert_ast_type_to_expression(p))
                    .collect();
                TypeExpression::function(param_types, self.convert_ast_type_to_expression(return_type))
            }
            _ => TypeExpression::named("unknown"),
        }
    }

    /// Convert TypeExpression back to AST Type
    fn convert_type_expression_to_ast(&self, type_expr: &TypeExpression) -> crate::ast::Type {
        use crate::ast::Type;
        
        if let Some(name) = &type_expr.name {
            match name.as_str() {
                "normie" => Type::Normie,
                "tea" => Type::Tea,
                "lit" => Type::Lit,
                "sip" => Type::Sip,
                "smol" => Type::Smol,
                "mid" => Type::Mid,
                "thicc" => Type::Thicc,
                "snack" => Type::Snack,
                "meal" => Type::Meal,

                "void" => Type::Void,
                "Array" if type_expr.parameters.len() == 1 => {
                    Type::Array(
                        Box::new(self.convert_type_expression_to_ast(&type_expr.parameters[0])),
                        None
                    )
                }
                "slice" if type_expr.parameters.len() == 1 => {
                    Type::Slice(Box::new(self.convert_type_expression_to_ast(&type_expr.parameters[0])))
                }
                _ => Type::Custom(name.clone()),
            }
        } else {
            Type::Custom("unknown".to_string())
        }
    }
}

/// Result of the monomorphisation process
#[derive(Debug)]
pub struct MonomorphisedProgram {
    /// Original program AST
    pub original_program: Program,
    /// Generated concrete instances
    pub concrete_instances: Vec<MonomorphisedInstance>,
    /// Mapping from generic names to concrete instance IDs
    pub instantiation_map: HashMap<String, String>,
}

impl MonomorphisedProgram {
    /// Get all concrete functions generated from generics
    pub fn get_concrete_functions(&self) -> Vec<&ConcreteFunctionDeclaration> {
        self.concrete_instances.iter()
            .filter_map(|instance| {
                match &instance.concrete_ast {
                    ConcreteAST::Function(func) => Some(func),
                    _ => None,
                }
            })
            .collect()
    }

    /// Get all concrete structs generated from generics
    pub fn get_concrete_structs(&self) -> Vec<&ConcreteStructDeclaration> {
        self.concrete_instances.iter()
            .filter_map(|instance| {
                match &instance.concrete_ast {
                    ConcreteAST::Struct(struct_decl) => Some(struct_decl),
                    _ => None,
                }
            })
            .collect()
    }

    /// Get concrete instance by generic name and type arguments
    pub fn get_instance(&self, generic_name: &str, type_args: &[TypeExpression]) -> Option<&MonomorphisedInstance> {
        let type_suffix = type_args.iter()
            .map(|t| t.name.as_deref().unwrap_or("unknown"))
            .collect::<Vec<_>>()
            .join("_");
        let instance_key = format!("{}_{}", generic_name, type_suffix);
        
        self.concrete_instances.iter()
            .find(|instance| instance.instance_id == instance_key)
    }
}

// TypeEnvironment extensions are now implemented in mod.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_monomorphisation() {
        let type_env = TypeEnvironment::new();
        let mut pipeline = MonomorphisationPipeline::new(type_env);
        
        let request = InstantiationRequest {
            generic_name: "identity".to_string(),
            type_arguments: vec![TypeExpression::named("normie")],
            constraints: Vec::new(),
            call_site: Some("test".to_string()),
        };
        
        let instance_key = pipeline.generate_instance_key(&request);
        assert_eq!(instance_key, "identity_normie");
    }

    #[test]
    fn test_type_suffix_generation() {
        let type_env = TypeEnvironment::new();
        let pipeline = MonomorphisationPipeline::new(type_env);
        
        let type_args = vec![
            TypeExpression::named("normie"),
            TypeExpression::named("tea"),
        ];
        
        let suffix = pipeline.generate_type_suffix(&type_args);
        assert_eq!(suffix, "normie_tea");
    }

    #[test]
    fn test_function_signature_generation() {
        let type_env = TypeEnvironment::new();
        let pipeline = MonomorphisationPipeline::new(type_env);
        
        let type_args = vec![
            TypeExpression::named("normie"),
            TypeExpression::named("tea"),
        ];
        
        let signature = pipeline.generate_function_signature("map", &type_args);
        assert_eq!(signature, "map[normie, tea]");
    }
}
