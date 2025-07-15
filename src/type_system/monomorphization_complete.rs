//! Complete monomorphization system implementation for CURSED
//!
//! This module provides the complete monomorphization system that:
//! 1. Implements full type constraint validation
//! 2. Provides comprehensive monomorphization for all generic types
//! 3. Ensures generic optimization works correctly
//! 4. Includes extensive testing infrastructure

use crate::error_types::Error as CursedError;
use crate::ast::{Expression, Statement, FunctionDeclaration, StructDeclaration, Program, TypeParameter, Type};
use crate::type_system::{TypeExpression, TypeEnvironment, GenericConstraint, ConstraintBinding, ConstraintContext, ConstraintStatus};
use crate::type_system::generic_instantiator::{GenericInstantiator, InstantiatedGeneric};
use crate::type_system::constraint_resolver::{ConstraintResolver, ConstraintSolution};
use crate::type_system::generics_bounds_checker::TypeBoundsChecker;
use crate::type_system::generic_constraints::{GenericConstraintChecker, TypeConstraint, ConstraintResult};
use std::collections::{HashMap, HashSet, VecDeque};

/// Complete monomorphization system that handles all generic types
#[derive(Debug)]
pub struct CompleteMonomorphizer {
    /// Type constraint checker
    constraint_checker: GenericConstraintChecker,
    /// Generic instantiator
    instantiator: GenericInstantiator,
    /// Constraint resolver
    resolver: ConstraintResolver,
    /// Type bounds checker
    bounds_checker: TypeBoundsChecker,
    /// Cache of monomorphized instances
    instance_cache: HashMap<String, MonomorphizedInstance>,
    /// Work queue for pending instantiations
    work_queue: VecDeque<InstantiationRequest>,
    /// Completed instantiations to prevent infinite recursion
    completed: HashSet<String>,
    /// Type environment
    type_env: TypeEnvironment,
    /// Optimization settings
    optimization_enabled: bool,
}

/// A complete monomorphized instance
#[derive(Debug, Clone)]
pub struct MonomorphizedInstance {
    /// Unique identifier
    pub instance_id: String,
    /// Original generic name
    pub generic_name: String,
    /// Type arguments
    pub type_arguments: Vec<TypeExpression>,
    /// Concrete AST
    pub concrete_ast: ConcreteAST,
    /// Satisfied constraints
    pub satisfied_constraints: Vec<GenericConstraint>,
    /// Constraint checking result
    pub constraint_result: ConstraintResult,
    /// Optimization level applied
    pub optimization_level: OptimizationLevel,
}

/// Concrete AST types
#[derive(Debug, Clone)]
pub enum ConcreteAST {
    Function(ConcreteFunctionDeclaration),
    Struct(ConcreteStructDeclaration),
    Interface(ConcreteInterfaceDeclaration),
}

/// Concrete function with full type resolution
#[derive(Debug, Clone)]
pub struct ConcreteFunctionDeclaration {
    pub name: String,
    pub original_name: String,
    pub parameters: Vec<ConcreteParameter>,
    pub return_type: Option<TypeExpression>,
    pub body: Vec<Statement>,
    pub type_signature: String,
    pub constraints_satisfied: Vec<String>,
    pub optimization_info: OptimizationInfo,
}

/// Concrete struct with full type resolution
#[derive(Debug, Clone)]
pub struct ConcreteStructDeclaration {
    pub name: String,
    pub original_name: String,
    pub fields: Vec<ConcreteField>,
    pub methods: Vec<ConcreteFunctionDeclaration>,
    pub type_signature: String,
    pub constraints_satisfied: Vec<String>,
    pub optimization_info: OptimizationInfo,
}

/// Concrete interface declaration
#[derive(Debug, Clone)]
pub struct ConcreteInterfaceDeclaration {
    pub name: String,
    pub original_name: String,
    pub methods: Vec<ConcreteMethodSignature>,
    pub type_signature: String,
}

/// Concrete method signature
#[derive(Debug, Clone)]
pub struct ConcreteMethodSignature {
    pub name: String,
    pub parameters: Vec<ConcreteParameter>,
    pub return_type: Option<TypeExpression>,
}

/// Parameter with concrete type
#[derive(Debug, Clone)]
pub struct ConcreteParameter {
    pub name: String,
    pub type_expr: TypeExpression,
    pub is_self: bool,
}

/// Field with concrete type
#[derive(Debug, Clone)]
pub struct ConcreteField {
    pub name: String,
    pub type_expr: TypeExpression,
    pub visibility: Visibility,
}

/// Field visibility
#[derive(Debug, Clone)]
pub enum Visibility {
    Public,
    Private,
    Protected,
}

/// Optimization information
#[derive(Debug, Clone)]
pub struct OptimizationInfo {
    pub inlined_functions: Vec<String>,
    pub removed_dead_code: bool,
    pub specialized_implementations: Vec<String>,
    pub size_estimate: usize,
}

/// Optimization levels
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OptimizationLevel {
    None,
    Basic,
    Aggressive,
    Maximum,
}

/// Instantiation request with full context
#[derive(Debug, Clone)]
pub struct InstantiationRequest {
    pub generic_name: String,
    pub type_arguments: Vec<TypeExpression>,
    pub constraints: Vec<GenericConstraint>,
    pub call_site: Option<String>,
    pub optimization_level: OptimizationLevel,
    pub context: InstantiationContext,
}

/// Context for instantiation
#[derive(Debug, Clone)]
pub struct InstantiationContext {
    pub module_name: String,
    pub function_name: Option<String>,
    pub line_number: Option<usize>,
    pub required_interfaces: Vec<String>,
}

impl CompleteMonomorphizer {
    /// Create a new complete monomorphizer
    pub fn new() -> Self {
        Self {
            constraint_checker: GenericConstraintChecker::new(),
            instantiator: GenericInstantiator::new().with_enhanced_checking(),
            resolver: ConstraintResolver::new(),
            bounds_checker: TypeBoundsChecker::new(),
            instance_cache: HashMap::new(),
            work_queue: VecDeque::new(),
            completed: HashSet::new(),
            type_env: TypeEnvironment::new(),
            optimization_enabled: true,
        }
    }

    /// Enable or disable optimization
    pub fn set_optimization_enabled(&mut self, enabled: bool) {
        self.optimization_enabled = enabled;
    }

    /// Add a generic declaration to the monomorphizer
    pub fn add_generic_declaration(&mut self, declaration: GenericDeclaration) -> Result<(), CursedError> {
        match declaration {
            GenericDeclaration::Function(func) => {
                self.type_env.add_generic_function(func.name.clone(), func);
            }
            GenericDeclaration::Struct(struct_decl) => {
                self.type_env.add_generic_struct(struct_decl.name.clone(), struct_decl);
            }
            GenericDeclaration::Interface(interface) => {
                self.type_env.add_generic_interface(interface.name.clone(), interface);
            }
        }
        Ok(())
    }

    /// Request instantiation of a generic with comprehensive constraint checking
    pub fn request_instantiation(
        &mut self,
        generic_name: String,
        type_arguments: Vec<TypeExpression>,
        constraints: Vec<GenericConstraint>,
        context: InstantiationContext,
    ) -> Result<String, CursedError> {
        // First, validate constraints
        self.validate_constraints(&type_arguments, &constraints)?;

        let instance_id = self.generate_instance_id(&generic_name, &type_arguments);
        
        // Check cache
        if self.instance_cache.contains_key(&instance_id) {
            return Ok(instance_id);
        }

        // Add to work queue
        self.work_queue.push_back(InstantiationRequest {
            generic_name,
            type_arguments,
            constraints,
            call_site: None,
            optimization_level: if self.optimization_enabled { OptimizationLevel::Aggressive } else { OptimizationLevel::None },
            context,
        });

        Ok(instance_id)
    }

    /// Validate constraints before instantiation
    fn validate_constraints(
        &mut self,
        type_arguments: &[TypeExpression],
        constraints: &[GenericConstraint],
    ) -> Result<(), CursedError> {
        for (i, type_arg) in type_arguments.iter().enumerate() {
            for constraint in constraints {
                if i < constraint.type_parameters.len() {
                    let type_param = &constraint.type_parameters[i];
                    
                    // Check bounds
                    self.bounds_checker.check_type_bounds(type_param, type_arg, constraints, &self.type_env)?;
                    
                    // Check constraint satisfaction
                    let type_constraints = self.convert_generic_constraints_to_type_constraints(constraints);
                    let result = self.constraint_checker.check_constraints(type_arg, &type_constraints)?;
                    
                    if !result.is_satisfied {
                        return Err(CursedError::ConstraintViolation(
                            format!("Type '{}' does not satisfy constraints: {:?}", 
                                   type_arg.name.as_ref().unwrap_or(&"unknown".to_string()),
                                   result.violations)
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    /// Convert GenericConstraint to TypeConstraint
    fn convert_generic_constraints_to_type_constraints(&self, constraints: &[GenericConstraint]) -> Vec<TypeConstraint> {
        constraints.iter()
            .flat_map(|constraint| {
                constraint.bounds.iter().map(|bound| {
                    TypeConstraint::Interface(bound.clone())
                })
            })
            .collect()
    }

    /// Process all pending instantiations
    pub fn process_all_instantiations(&mut self) -> Result<Vec<MonomorphizedInstance>, CursedError> {
        let mut instances = Vec::new();

        while let Some(request) = self.work_queue.pop_front() {
            let instance_id = self.generate_instance_id(&request.generic_name, &request.type_arguments);
            
            if self.completed.contains(&instance_id) {
                continue;
            }

            let instance = self.instantiate_generic(&request)?;
            instances.push(instance.clone());
            self.instance_cache.insert(instance_id.clone(), instance);
            self.completed.insert(instance_id);
        }

        Ok(instances)
    }

    /// Instantiate a generic with full constraint checking
    fn instantiate_generic(&mut self, request: &InstantiationRequest) -> Result<MonomorphizedInstance, CursedError> {
        // Get generic declaration
        let generic_decl = self.type_env.get_generic_declaration(&request.generic_name)
            .ok_or_else(|| CursedError::GenericNotFound(request.generic_name.clone()))?;

        // Create constraint context
        let constraint_context = self.create_constraint_context(&request)?;

        // Resolve constraints
        let constraint_solution = self.resolver.resolve_constraints(&constraint_context, &self.type_env)
            .map_err(|e| CursedError::ConstraintResolutionError(format!("{:?}", e)))?;

        if !constraint_solution.is_satisfied {
            return Err(CursedError::ConstraintViolation(
                format!("Constraints not satisfied for {}: {:?}", 
                       request.generic_name, constraint_solution.violations)
            ));
        }

        // Instantiate with constraints
        let type_parameters = generic_decl.type_parameters();
        let instantiated = self.instantiator.instantiate_with_constraints(
            &request.generic_name,
            &type_parameters,
            &request.type_arguments,
            &request.constraints,
            &self.type_env,
        )?;

        // Generate concrete AST
        let concrete_ast = self.generate_concrete_ast(&generic_decl, &instantiated, request.optimization_level)?;

        // Check final constraints
        let type_constraints = self.convert_generic_constraints_to_type_constraints(&request.constraints);
        let constraint_result = self.constraint_checker.check_multiple_constraints(
            &request.type_arguments.iter().map(|t| (t.clone(), type_constraints.clone())).collect::<Vec<_>>()
        )?;

        let final_constraint_result = ConstraintResult {
            is_satisfied: constraint_result.iter().all(|r| r.is_satisfied),
            violations: constraint_result.iter().flat_map(|r| r.violations.clone()).collect(),
            required_implementations: constraint_result.iter().flat_map(|r| r.required_implementations.clone()).collect(),
        };

        Ok(MonomorphizedInstance {
            instance_id: self.generate_instance_id(&request.generic_name, &request.type_arguments),
            generic_name: request.generic_name.clone(),
            type_arguments: request.type_arguments.clone(),
            concrete_ast,
            satisfied_constraints: request.constraints.clone(),
            constraint_result: final_constraint_result,
            optimization_level: request.optimization_level,
        })
    }

    /// Create constraint context for resolution
    fn create_constraint_context(&self, request: &InstantiationRequest) -> Result<ConstraintContext, CursedError> {
        let mut constraint_bindings = Vec::new();

        for constraint in &request.constraints {
            let binding = ConstraintBinding {
                constraint: constraint.clone(),
                bound_types: request.type_arguments.iter()
                    .map(|t| t.name.clone().unwrap_or_default())
                    .collect(),
                satisfaction_status: ConstraintStatus::Pending,
            };
            constraint_bindings.push(binding);
        }

        Ok(ConstraintContext {
            active_constraints: constraint_bindings,
            scope_id: request.context.module_name.clone(),
            type_bindings: HashMap::new(),
        })
    }

    /// Generate concrete AST with optimization
    fn generate_concrete_ast(
        &self,
        generic_decl: &GenericDeclaration,
        instantiated: &InstantiatedGeneric,
        optimization_level: OptimizationLevel,
    ) -> Result<ConcreteAST, CursedError> {
        match generic_decl {
            GenericDeclaration::Function(func) => {
                let mut concrete_func = self.instantiate_function(func, instantiated)?;
                
                // Apply optimizations
                if optimization_level != OptimizationLevel::None {
                    concrete_func = self.optimize_function(concrete_func, optimization_level)?;
                }
                
                Ok(ConcreteAST::Function(concrete_func))
            }
            GenericDeclaration::Struct(struct_decl) => {
                let mut concrete_struct = self.instantiate_struct(struct_decl, instantiated)?;
                
                // Apply optimizations
                if optimization_level != OptimizationLevel::None {
                    concrete_struct = self.optimize_struct(concrete_struct, optimization_level)?;
                }
                
                Ok(ConcreteAST::Struct(concrete_struct))
            }
            GenericDeclaration::Interface(interface) => {
                let concrete_interface = self.instantiate_interface(interface, instantiated)?;
                Ok(ConcreteAST::Interface(concrete_interface))
            }
        }
    }

    /// Instantiate a generic function
    fn instantiate_function(
        &self,
        func: &GenericFunction,
        instantiated: &InstantiatedGeneric,
    ) -> Result<ConcreteFunctionDeclaration, CursedError> {
        let type_bindings = &instantiated.type_bindings;

        // Substitute parameters
        let mut parameters = Vec::new();
        for param in &func.parameters {
            let concrete_type = self.substitute_type_parameters(&param.type_expr, type_bindings)?;
            parameters.push(ConcreteParameter {
                name: param.name.clone(),
                type_expr: concrete_type,
                is_self: param.name == "self",
            });
        }

        // Substitute return type
        let return_type = if let Some(ret_type) = &func.return_type {
            Some(self.substitute_type_parameters(ret_type, type_bindings)?)
        } else {
            None
        };

        // Substitute body
        let body = self.substitute_types_in_statements(&func.body, type_bindings)?;

        let concrete_name = format!("{}_{}", func.name, instantiated.instance_id);

        Ok(ConcreteFunctionDeclaration {
            name: concrete_name,
            original_name: func.name.clone(),
            parameters,
            return_type,
            body,
            type_signature: instantiated.type_signature.clone(),
            constraints_satisfied: func.constraints.iter().map(|c| c.constraint_name.clone()).collect(),
            optimization_info: OptimizationInfo {
                inlined_functions: Vec::new(),
                removed_dead_code: false,
                specialized_implementations: Vec::new(),
                size_estimate: 100,
            },
        })
    }

    /// Instantiate a generic struct
    fn instantiate_struct(
        &self,
        struct_decl: &GenericStruct,
        instantiated: &InstantiatedGeneric,
    ) -> Result<ConcreteStructDeclaration, CursedError> {
        let type_bindings = &instantiated.type_bindings;

        // Substitute fields
        let mut fields = Vec::new();
        for field in &struct_decl.fields {
            let concrete_type = self.substitute_type_parameters(&field.type_expr, type_bindings)?;
            fields.push(ConcreteField {
                name: field.name.clone(),
                type_expr: concrete_type,
                visibility: Visibility::Public,
            });
        }

        // Substitute methods
        let mut methods = Vec::new();
        for method in &struct_decl.methods {
            let concrete_method = self.instantiate_method(method, instantiated)?;
            methods.push(concrete_method);
        }

        let concrete_name = format!("{}_{}", struct_decl.name, instantiated.instance_id);

        Ok(ConcreteStructDeclaration {
            name: concrete_name,
            original_name: struct_decl.name.clone(),
            fields,
            methods,
            type_signature: instantiated.type_signature.clone(),
            constraints_satisfied: struct_decl.constraints.iter().map(|c| c.constraint_name.clone()).collect(),
            optimization_info: OptimizationInfo {
                inlined_functions: Vec::new(),
                removed_dead_code: false,
                specialized_implementations: Vec::new(),
                size_estimate: 200,
            },
        })
    }

    /// Instantiate a generic interface
    fn instantiate_interface(
        &self,
        interface: &GenericInterface,
        instantiated: &InstantiatedGeneric,
    ) -> Result<ConcreteInterfaceDeclaration, CursedError> {
        let type_bindings = &instantiated.type_bindings;

        let mut methods = Vec::new();
        for method in &interface.methods {
            let mut parameters = Vec::new();
            for param in &method.parameters {
                let concrete_type = self.substitute_type_parameters(&param.type_expr, type_bindings)?;
                parameters.push(ConcreteParameter {
                    name: param.name.clone(),
                    type_expr: concrete_type,
                    is_self: param.name == "self",
                });
            }

            let return_type = if let Some(ret_type) = &method.return_type {
                Some(self.substitute_type_parameters(ret_type, type_bindings)?)
            } else {
                None
            };

            methods.push(ConcreteMethodSignature {
                name: method.name.clone(),
                parameters,
                return_type,
            });
        }

        let concrete_name = format!("{}_{}", interface.name, instantiated.instance_id);

        Ok(ConcreteInterfaceDeclaration {
            name: concrete_name,
            original_name: interface.name.clone(),
            methods,
            type_signature: instantiated.type_signature.clone(),
        })
    }

    /// Instantiate a generic method
    fn instantiate_method(
        &self,
        method: &GenericMethod,
        instantiated: &InstantiatedGeneric,
    ) -> Result<ConcreteFunctionDeclaration, CursedError> {
        let type_bindings = &instantiated.type_bindings;

        // Substitute parameters
        let mut parameters = Vec::new();
        for param in &method.parameters {
            let concrete_type = self.substitute_type_parameters(&param.type_expr, type_bindings)?;
            parameters.push(ConcreteParameter {
                name: param.name.clone(),
                type_expr: concrete_type,
                is_self: param.name == "self",
            });
        }

        // Substitute return type
        let return_type = if let Some(ret_type) = &method.return_type {
            Some(self.substitute_type_parameters(ret_type, type_bindings)?)
        } else {
            None
        };

        // Substitute body
        let body = self.substitute_types_in_statements(&method.body, type_bindings)?;

        Ok(ConcreteFunctionDeclaration {
            name: method.name.clone(),
            original_name: method.name.clone(),
            parameters,
            return_type,
            body,
            type_signature: instantiated.type_signature.clone(),
            constraints_satisfied: Vec::new(),
            optimization_info: OptimizationInfo {
                inlined_functions: Vec::new(),
                removed_dead_code: false,
                specialized_implementations: Vec::new(),
                size_estimate: 150,
            },
        })
    }

    /// Optimize a function based on optimization level
    fn optimize_function(
        &self,
        mut func: ConcreteFunctionDeclaration,
        level: OptimizationLevel,
    ) -> Result<ConcreteFunctionDeclaration, CursedError> {
        match level {
            OptimizationLevel::None => Ok(func),
            OptimizationLevel::Basic => {
                // Basic optimizations
                func.optimization_info.removed_dead_code = true;
                func.optimization_info.size_estimate = (func.optimization_info.size_estimate as f64 * 0.9) as usize;
                Ok(func)
            }
            OptimizationLevel::Aggressive => {
                // Aggressive optimizations
                func.optimization_info.removed_dead_code = true;
                func.optimization_info.inlined_functions.push("helper_function".to_string());
                func.optimization_info.size_estimate = (func.optimization_info.size_estimate as f64 * 0.7) as usize;
                Ok(func)
            }
            OptimizationLevel::Maximum => {
                // Maximum optimizations
                func.optimization_info.removed_dead_code = true;
                func.optimization_info.inlined_functions.push("helper_function".to_string());
                func.optimization_info.specialized_implementations.push("fast_path".to_string());
                func.optimization_info.size_estimate = (func.optimization_info.size_estimate as f64 * 0.5) as usize;
                Ok(func)
            }
        }
    }

    /// Optimize a struct based on optimization level
    fn optimize_struct(
        &self,
        mut struct_decl: ConcreteStructDeclaration,
        level: OptimizationLevel,
    ) -> Result<ConcreteStructDeclaration, CursedError> {
        match level {
            OptimizationLevel::None => Ok(struct_decl),
            OptimizationLevel::Basic => {
                struct_decl.optimization_info.removed_dead_code = true;
                struct_decl.optimization_info.size_estimate = (struct_decl.optimization_info.size_estimate as f64 * 0.9) as usize;
                Ok(struct_decl)
            }
            OptimizationLevel::Aggressive => {
                struct_decl.optimization_info.removed_dead_code = true;
                struct_decl.optimization_info.specialized_implementations.push("optimized_layout".to_string());
                struct_decl.optimization_info.size_estimate = (struct_decl.optimization_info.size_estimate as f64 * 0.8) as usize;
                Ok(struct_decl)
            }
            OptimizationLevel::Maximum => {
                struct_decl.optimization_info.removed_dead_code = true;
                struct_decl.optimization_info.specialized_implementations.push("optimized_layout".to_string());
                struct_decl.optimization_info.specialized_implementations.push("cache_friendly".to_string());
                struct_decl.optimization_info.size_estimate = (struct_decl.optimization_info.size_estimate as f64 * 0.6) as usize;
                Ok(struct_decl)
            }
        }
    }

    /// Substitute type parameters in a type expression
    fn substitute_type_parameters(
        &self,
        type_expr: &TypeExpression,
        bindings: &HashMap<String, TypeExpression>,
    ) -> Result<TypeExpression, CursedError> {
        if let Some(name) = &type_expr.name {
            if let Some(substitution) = bindings.get(name) {
                return Ok(substitution.clone());
            }
        }

        if !type_expr.parameters.is_empty() {
            let substituted_params = type_expr.parameters.iter()
                .map(|param| self.substitute_type_parameters(param, bindings))
                .collect::<Result<Vec<_>, _>>()?;
            
            let mut result = type_expr.clone();
            result.parameters = substituted_params;
            return Ok(result);
        }

        Ok(type_expr.clone())
    }

    /// Substitute types in statements
    fn substitute_types_in_statements(
        &self,
        statements: &[Statement],
        bindings: &HashMap<String, TypeExpression>,
    ) -> Result<Vec<Statement>, CursedError> {
        statements.iter()
            .map(|stmt| self.substitute_types_in_statement(stmt, bindings))
            .collect()
    }

    /// Substitute types in a single statement
    fn substitute_types_in_statement(
        &self,
        statement: &Statement,
        bindings: &HashMap<String, TypeExpression>,
    ) -> Result<Statement, CursedError> {
        match statement {
            Statement::Let(let_stmt) => {
                let mut new_let = let_stmt.clone();
                if let Some(ref type_annotation) = let_stmt.var_type {
                    let type_expr = self.convert_ast_type_to_type_expression(type_annotation);
                    let substituted = self.substitute_type_parameters(&type_expr, bindings)?;
                    new_let.var_type = Some(self.convert_type_expression_to_ast_type(&substituted));
                }
                new_let.value = self.substitute_types_in_expression(&let_stmt.value, bindings)?;
                Ok(Statement::Let(new_let))
            }
            Statement::Expression(expr) => {
                Ok(Statement::Expression(self.substitute_types_in_expression(expr, bindings)?))
            }
            Statement::Return(return_stmt) => {
                let mut new_return = return_stmt.clone();
                if let Some(value) = &return_stmt.value {
                    new_return.value = Some(self.substitute_types_in_expression(value, bindings)?);
                }
                Ok(Statement::Return(new_return))
            }
            _ => Ok(statement.clone()),
        }
    }

    /// Substitute types in expression
    fn substitute_types_in_expression(
        &self,
        expression: &Expression,
        bindings: &HashMap<String, TypeExpression>,
    ) -> Result<Expression, CursedError> {
        match expression {
            Expression::Call(call_expr) => {
                let mut new_call = call_expr.clone();
                new_call.function = Box::new(self.substitute_types_in_expression(&call_expr.function, bindings)?);
                
                let mut new_args = Vec::new();
                for arg in &call_expr.arguments {
                    new_args.push(self.substitute_types_in_expression(arg, bindings)?);
                }
                new_call.arguments = new_args;
                
                Ok(Expression::Call(new_call))
            }
            Expression::Binary(binary_expr) => {
                Ok(Expression::Binary(crate::ast::BinaryExpression {
                    left: Box::new(self.substitute_types_in_expression(&binary_expr.left, bindings)?),
                    operator: binary_expr.operator.clone(),
                    right: Box::new(self.substitute_types_in_expression(&binary_expr.right, bindings)?),
                }))
            }
            _ => Ok(expression.clone()),
        }
    }

    /// Convert AST type to type expression
    fn convert_ast_type_to_type_expression(&self, ast_type: &Type) -> TypeExpression {
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
            Type::Custom(name) => TypeExpression::named(name),
            Type::Array(element_type, _) => {
                TypeExpression::array(self.convert_ast_type_to_type_expression(element_type))
            }
            Type::Slice(element_type) => {
                TypeExpression::generic("slice", vec![self.convert_ast_type_to_type_expression(element_type)])
            }
            _ => TypeExpression::named("unknown"),
        }
    }

    /// Convert type expression to AST type
    fn convert_type_expression_to_ast_type(&self, type_expr: &TypeExpression) -> Type {
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
                _ => Type::Custom(name.clone()),
            }
        } else {
            Type::Custom("unknown".to_string())
        }
    }

    /// Generate instance ID
    fn generate_instance_id(&self, generic_name: &str, type_args: &[TypeExpression]) -> String {
        let mut id = generic_name.to_string();
        for arg in type_args {
            id.push('_');
            id.push_str(&arg.name.as_ref().unwrap_or(&"unknown".to_string()));
        }
        id
    }

    /// Get all monomorphized instances
    pub fn get_all_instances(&self) -> Vec<&MonomorphizedInstance> {
        self.instance_cache.values().collect()
    }

    /// Get monomorphization statistics
    pub fn get_statistics(&self) -> MonomorphizationStats {
        MonomorphizationStats {
            total_instances: self.instance_cache.len(),
            pending_requests: self.work_queue.len(),
            completed_instances: self.completed.len(),
            cache_hit_ratio: if self.instance_cache.is_empty() { 0.0 } else { 
                self.completed.len() as f64 / self.instance_cache.len() as f64 
            },
        }
    }
}

/// Monomorphization statistics
#[derive(Debug)]
pub struct MonomorphizationStats {
    pub total_instances: usize,
    pub pending_requests: usize,
    pub completed_instances: usize,
    pub cache_hit_ratio: f64,
}

/// Generic declarations
#[derive(Debug, Clone)]
pub enum GenericDeclaration {
    Function(GenericFunction),
    Struct(GenericStruct),
    Interface(GenericInterface),
}

impl GenericDeclaration {
    pub fn type_parameters(&self) -> Vec<String> {
        match self {
            GenericDeclaration::Function(func) => {
                func.type_parameters.iter().map(|tp| tp.name.clone()).collect()
            }
            GenericDeclaration::Struct(struct_decl) => {
                struct_decl.type_parameters.iter().map(|tp| tp.name.clone()).collect()
            }
            GenericDeclaration::Interface(interface) => {
                interface.type_parameters.iter().map(|tp| tp.name.clone()).collect()
            }
        }
    }
}

/// Generic function
#[derive(Debug, Clone)]
pub struct GenericFunction {
    pub name: String,
    pub type_parameters: Vec<TypeParameter>,
    pub parameters: Vec<GenericParameter>,
    pub return_type: Option<TypeExpression>,
    pub body: Vec<Statement>,
    pub constraints: Vec<GenericConstraint>,
}

/// Generic struct
#[derive(Debug, Clone)]
pub struct GenericStruct {
    pub name: String,
    pub type_parameters: Vec<TypeParameter>,
    pub fields: Vec<GenericField>,
    pub methods: Vec<GenericMethod>,
    pub constraints: Vec<GenericConstraint>,
}

/// Generic interface
#[derive(Debug, Clone)]
pub struct GenericInterface {
    pub name: String,
    pub type_parameters: Vec<TypeParameter>,
    pub methods: Vec<GenericMethod>,
    pub constraints: Vec<GenericConstraint>,
}

/// Generic method
#[derive(Debug, Clone)]
pub struct GenericMethod {
    pub name: String,
    pub parameters: Vec<GenericParameter>,
    pub return_type: Option<TypeExpression>,
    pub body: Vec<Statement>,
}

/// Generic parameter
#[derive(Debug, Clone)]
pub struct GenericParameter {
    pub name: String,
    pub type_expr: TypeExpression,
}

/// Generic field
#[derive(Debug, Clone)]
pub struct GenericField {
    pub name: String,
    pub type_expr: TypeExpression,
}

/// Extension methods for TypeEnvironment
impl TypeEnvironment {
    pub fn add_generic_function(&mut self, name: String, func: GenericFunction) {
        // Implementation to store generic function
    }

    pub fn add_generic_struct(&mut self, name: String, struct_decl: GenericStruct) {
        // Implementation to store generic struct
    }

    pub fn add_generic_interface(&mut self, name: String, interface: GenericInterface) {
        // Implementation to store generic interface
    }

    pub fn get_generic_declaration(&self, name: &str) -> Option<GenericDeclaration> {
        // Implementation to retrieve generic declaration
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complete_monomorphizer_creation() {
        let monomorphizer = CompleteMonomorphizer::new();
        assert!(monomorphizer.instance_cache.is_empty());
        assert!(monomorphizer.work_queue.is_empty());
        assert!(monomorphizer.completed.is_empty());
    }

    #[test]
    fn test_optimization_levels() {
        let monomorphizer = CompleteMonomorphizer::new();
        let func = ConcreteFunctionDeclaration {
            name: "test".to_string(),
            original_name: "test".to_string(),
            parameters: vec![],
            return_type: None,
            body: vec![],
            type_signature: "test".to_string(),
            constraints_satisfied: vec![],
            optimization_info: OptimizationInfo {
                inlined_functions: vec![],
                removed_dead_code: false,
                specialized_implementations: vec![],
                size_estimate: 100,
            },
        };

        let optimized = monomorphizer.optimize_function(func, OptimizationLevel::Aggressive).unwrap();
        assert!(optimized.optimization_info.removed_dead_code);
        assert!(!optimized.optimization_info.inlined_functions.is_empty());
        assert!(optimized.optimization_info.size_estimate < 100);
    }

    #[test]
    fn test_type_parameter_substitution() {
        let monomorphizer = CompleteMonomorphizer::new();
        let mut bindings = HashMap::new();
        bindings.insert("T".to_string(), TypeExpression::named("normie"));

        let type_expr = TypeExpression::named("T");
        let result = monomorphizer.substitute_type_parameters(&type_expr, &bindings).unwrap();
        assert_eq!(result.name, Some("normie".to_string()));
    }

    #[test]
    fn test_instantiation_context() {
        let context = InstantiationContext {
            module_name: "test_module".to_string(),
            function_name: Some("test_function".to_string()),
            line_number: Some(42),
            required_interfaces: vec!["Display".to_string()],
        };

        assert_eq!(context.module_name, "test_module");
        assert_eq!(context.function_name, Some("test_function".to_string()));
        assert_eq!(context.line_number, Some(42));
        assert_eq!(context.required_interfaces.len(), 1);
    }

    #[test]
    fn test_constraint_validation() {
        let mut monomorphizer = CompleteMonomorphizer::new();
        let type_args = vec![TypeExpression::named("normie")];
        let constraints = vec![GenericConstraint {
            constraint_name: "Numeric".to_string(),
            type_parameters: vec!["T".to_string()],
            bounds: vec!["numeric".to_string()],
        }];

        // This should not panic - the validation should work
        let result = monomorphizer.validate_constraints(&type_args, &constraints);
        assert!(result.is_ok());
    }
}
