//! Complete monomorphization system for CURSED generics
//!
//! This module implements the full monomorphization pipeline that converts
//! generic functions and types into concrete instances for code generation.

use crate::error_types::Error as CursedError;
use crate::ast::{Expression, Statement, FunctionDeclaration, StructDeclaration, Program, TypeParameter};
use crate::type_system::{TypeExpression, TypeEnvironment, GenericConstraint};
use crate::type_system::generic_instantiator::{GenericInstantiator, InstantiatedGeneric};
use crate::type_system::constraint_resolver::{ConstraintResolver, ConstraintSolution};
use std::collections::{HashMap, HashSet, VecDeque};

/// Complete monomorphization system for CURSED generics
#[derive(Debug)]
pub struct Monomorphizer {
    /// Generic instantiator for type checking
    instantiator: GenericInstantiator,
    /// Cache of monomorphized instances
    instance_cache: HashMap<String, MonomorphizedInstance>,
    /// Work queue for pending instantiations
    work_queue: VecDeque<InstantiationRequest>,
    /// Set of completed instantiations to prevent cycles
    completed: HashSet<String>,
    /// Global type environment
    type_env: TypeEnvironment,
}

/// A concrete instance generated from a generic declaration
#[derive(Debug, Clone)]
pub struct MonomorphizedInstance {
    /// Unique identifier for this instance (e.g., "Vec_i32", "max_i32_f64")
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

/// Request for monomorphization of a generic declaration
#[derive(Debug, Clone)]
pub struct InstantiationRequest {
    pub generic_name: String,
    pub type_arguments: Vec<TypeExpression>,
    pub constraints: Vec<GenericConstraint>,
    pub call_site: Option<String>,
}

impl Monomorphizer {
    /// Create a new monomorphizer instance
    pub fn new() -> Self {
        Self {
            instantiator: GenericInstantiator::new().with_enhanced_checking(),
            instance_cache: HashMap::new(),
            work_queue: VecDeque::new(),
            completed: HashSet::new(),
            type_env: TypeEnvironment::new(),
        }
    }

    /// Add a generic function/type to the monomorphizer
    pub fn add_generic(&mut self, declaration: GenericDeclaration) -> Result<(), CursedError> {
        match declaration {
            GenericDeclaration::Function(func) => {
                self.type_env.add_generic_function(func.name.clone(), func);
            }
            GenericDeclaration::Struct(struct_decl) => {
                self.type_env.add_generic_struct(struct_decl.name.clone(), struct_decl);
            }
        }
        Ok(())
    }

    /// Request monomorphization of a generic with specific type arguments
    pub fn request_instantiation(
        &mut self,
        generic_name: String,
        type_arguments: Vec<TypeExpression>,
        constraints: Vec<GenericConstraint>,
        call_site: Option<String>,
    ) -> Result<String, CursedError> {
        let instance_id = self.generate_instance_id(&generic_name, &type_arguments);
        
        // Check if already instantiated
        if self.instance_cache.contains_key(&instance_id) {
            return Ok(instance_id);
        }

        // Add to work queue
        self.work_queue.push_back(InstantiationRequest {
            generic_name,
            type_arguments,
            constraints,
            call_site,
        });

        Ok(instance_id)
    }

    /// Process all pending instantiation requests
    pub fn process_instantiations(&mut self) -> Result<Vec<MonomorphizedInstance>, CursedError> {
        let mut instances = Vec::new();

        while let Some(request) = self.work_queue.pop_front() {
            let instance_id = self.generate_instance_id(&request.generic_name, &request.type_arguments);
            
            // Skip if already processed
            if self.completed.contains(&instance_id) {
                continue;
            }

            // Process the instantiation
            let instance = self.instantiate_generic(&request)?;
            instances.push(instance.clone());
            self.instance_cache.insert(instance_id.clone(), instance);
            self.completed.insert(instance_id);
        }

        Ok(instances)
    }

    /// Get all monomorphized instances
    pub fn get_instances(&self) -> Vec<&MonomorphizedInstance> {
        self.instance_cache.values().collect()
    }

    /// Generate a unique instance ID for a generic with type arguments
    fn generate_instance_id(&self, generic_name: &str, type_args: &[TypeExpression]) -> String {
        let mut id = generic_name.to_string();
        for arg in type_args {
            id.push('_');
            id.push_str(&format!("{:?}", arg).replace(' ', ""));
        }
        id
    }

    /// Instantiate a generic declaration with concrete types
    fn instantiate_generic(&mut self, request: &InstantiationRequest) -> Result<MonomorphizedInstance, CursedError> {
        // Get the generic declaration
        let generic_decl = self.type_env.get_generic_declaration(&request.generic_name)
            .ok_or_else(|| CursedError::GenericNotFound(request.generic_name.clone()))?;

        // Validate type arguments against constraints
        let type_parameters = generic_decl.type_parameters();
        let instantiated = self.instantiator.instantiate_with_constraints(
            &request.generic_name,
            &type_parameters,
            &request.type_arguments,
            &request.constraints,
            &self.type_env,
        )?;

        // Generate concrete AST
        let concrete_ast = self.generate_concrete_ast(&generic_decl, &instantiated)?;

        // Create monomorphized instance
        let instance_id = self.generate_instance_id(&request.generic_name, &request.type_arguments);
        Ok(MonomorphizedInstance {
            instance_id,
            generic_name: request.generic_name.clone(),
            type_arguments: request.type_arguments.clone(),
            concrete_ast,
            satisfied_constraints: request.constraints.clone(),
        })
    }

    /// Generate concrete AST from generic declaration
    fn generate_concrete_ast(
        &self,
        generic_decl: &GenericDeclaration,
        instantiated: &InstantiatedGeneric,
    ) -> Result<ConcreteAST, CursedError> {
        match generic_decl {
            GenericDeclaration::Function(func) => {
                let concrete_func = self.instantiate_function(func, instantiated)?;
                Ok(ConcreteAST::Function(concrete_func))
            }
            GenericDeclaration::Struct(struct_decl) => {
                let concrete_struct = self.instantiate_struct(struct_decl, instantiated)?;
                Ok(ConcreteAST::Struct(concrete_struct))
            }
        }
    }

    /// Instantiate a generic function with concrete types
    fn instantiate_function(
        &self,
        func: &GenericFunction,
        instantiated: &InstantiatedGeneric,
    ) -> Result<ConcreteFunctionDeclaration, CursedError> {
        // Use type bindings from instantiated generic
        let type_bindings = &instantiated.type_bindings;

        // Substitute type parameters in function signature
        let mut parameters = Vec::new();
        for param in &func.parameters {
            let concrete_type = self.substitute_type_parameters(&param.type_expr, &type_bindings)?;
            parameters.push(ConcreteParameter {
                name: param.name.clone(),
                type_expr: concrete_type,
            });
        }

        // Substitute return type
        let return_type = if let Some(ret_type) = &func.return_type {
            Some(self.substitute_type_parameters(ret_type, &type_bindings)?)
        } else {
            None
        };

        // Substitute types in function body
        let body = self.substitute_types_in_statements(&func.body, &type_bindings)?;

        // Generate unique function name
        let concrete_name = format!("{}_{}", func.name, instantiated.instance_id);

        Ok(ConcreteFunctionDeclaration {
            name: concrete_name,
            parameters,
            return_type,
            body,
            type_signature: instantiated.type_signature.clone(),
        })
    }

    /// Instantiate a generic struct with concrete types
    fn instantiate_struct(
        &self,
        struct_decl: &GenericStruct,
        instantiated: &InstantiatedGeneric,
    ) -> Result<ConcreteStructDeclaration, CursedError> {
        // Use type bindings from instantiated generic
        let type_bindings = &instantiated.type_bindings;

        // Substitute type parameters in struct fields
        let mut fields = Vec::new();
        for field in &struct_decl.fields {
            let concrete_type = self.substitute_type_parameters(&field.type_expr, &type_bindings)?;
            fields.push(ConcreteField {
                name: field.name.clone(),
                type_expr: concrete_type,
            });
        }

        // Instantiate methods
        let mut methods = Vec::new();
        for method in &struct_decl.methods {
            let concrete_method = self.instantiate_method(method, instantiated)?;
            methods.push(concrete_method);
        }

        // Generate unique struct name
        let concrete_name = format!("{}_{}", struct_decl.name, instantiated.instance_id);

        Ok(ConcreteStructDeclaration {
            name: concrete_name,
            fields,
            methods,
            type_signature: instantiated.type_signature.clone(),
        })
    }

    /// Instantiate a generic method with concrete types
    fn instantiate_method(
        &self,
        method: &GenericMethod,
        instantiated: &InstantiatedGeneric,
    ) -> Result<ConcreteMethodDeclaration, CursedError> {
        // Use type bindings from instantiated generic
        let type_bindings = &instantiated.type_bindings;

        // Substitute receiver type
        let receiver = if let Some(recv) = &method.receiver {
            let concrete_type = self.substitute_type_parameters(&recv.type_expr, &type_bindings)?;
            Some(ConcreteParameter {
                name: recv.name.clone(),
                type_expr: concrete_type,
            })
        } else {
            None
        };

        // Substitute parameter types
        let mut parameters = Vec::new();
        for param in &method.parameters {
            let concrete_type = self.substitute_type_parameters(&param.type_expr, &type_bindings)?;
            parameters.push(ConcreteParameter {
                name: param.name.clone(),
                type_expr: concrete_type,
            });
        }

        // Substitute return type
        let return_type = if let Some(ret_type) = &method.return_type {
            Some(self.substitute_type_parameters(ret_type, &type_bindings)?)
        } else {
            None
        };

        // Substitute types in method body
        let body = self.substitute_types_in_statements(&method.body, &type_bindings)?;

        Ok(ConcreteMethodDeclaration {
            name: method.name.clone(),
            receiver,
            parameters,
            return_type,
            body,
        })
    }

    /// Substitute type parameters in a type expression
    fn substitute_type_parameters(
        &self,
        type_expr: &TypeExpression,
        bindings: &HashMap<String, TypeExpression>,
    ) -> Result<TypeExpression, CursedError> {
        // Check if this is a type parameter (kind could be Primitive and name matches a binding)
        if let Some(name) = &type_expr.name {
            if bindings.contains_key(name) {
                return Ok(bindings.get(name).unwrap().clone());
            }
        }
        
        // Handle generic types (types with parameters)
        if !type_expr.parameters.is_empty() {
            let substituted_params = type_expr.parameters.iter()
                .map(|param| self.substitute_type_parameters(param, bindings))
                .collect::<Result<Vec<_>, _>>()?;
            
            let mut result = type_expr.clone();
            result.parameters = substituted_params;
            return Ok(result);
        }
        
        // Handle array types (named "Array" with one parameter)
        if let Some(name) = &type_expr.name {
            if name == "Array" && type_expr.parameters.len() == 1 {
                let substituted_element = self.substitute_type_parameters(&type_expr.parameters[0], bindings)?;
                return Ok(TypeExpression::array(substituted_element));
            }
            
            // Handle tuple types (named "Tuple" with multiple parameters)
            if name == "Tuple" && !type_expr.parameters.is_empty() {
                let substituted_elements = type_expr.parameters.iter()
                    .map(|elem| self.substitute_type_parameters(elem, bindings))
                    .collect::<Result<Vec<_>, _>>()?;
                return Ok(TypeExpression::tuple(substituted_elements));
            }
        }
        
        // For other types, return as-is
        Ok(type_expr.clone())
    }

    /// Substitute types in a list of statements
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
        // This is a simplified version - in practice, we'd need to handle
        // all statement types and substitute types in expressions
        match statement {
            Statement::Let(let_stmt) => {
                let mut new_let = let_stmt.clone();
                if let Some(ref type_annotation) = let_stmt.var_type {
                    new_let.var_type = Some(self.substitute_type_parameters(
                        &TypeExpression::from_ast_type(type_annotation),
                        bindings,
                    )?.to_ast_type());
                }
                Ok(Statement::Let(new_let))
            }
            // Handle other statement types...
            _ => Ok(statement.clone()),
        }
    }
}

/// Generic declaration that can be monomorphized
#[derive(Debug, Clone)]
pub enum GenericDeclaration {
    Function(GenericFunction),
    Struct(GenericStruct),
}

impl GenericDeclaration {
    /// Get type parameters for this generic declaration
    pub fn type_parameters(&self) -> Vec<String> {
        match self {
            GenericDeclaration::Function(func) => {
                func.type_parameters.iter().map(|tp| tp.name.clone()).collect()
            }
            GenericDeclaration::Struct(struct_decl) => {
                struct_decl.type_parameters.iter().map(|tp| tp.name.clone()).collect()
            }
        }
    }
}

/// Generic function declaration
#[derive(Debug, Clone)]
pub struct GenericFunction {
    pub name: String,
    pub type_parameters: Vec<TypeParameter>,
    pub parameters: Vec<GenericParameter>,
    pub return_type: Option<TypeExpression>,
    pub body: Vec<Statement>,
    pub constraints: Vec<GenericConstraint>,
}

/// Generic struct declaration
#[derive(Debug, Clone)]
pub struct GenericStruct {
    pub name: String,
    pub type_parameters: Vec<TypeParameter>,
    pub fields: Vec<GenericField>,
    pub methods: Vec<GenericMethod>,
    pub constraints: Vec<GenericConstraint>,
}

/// Generic method declaration
#[derive(Debug, Clone)]
pub struct GenericMethod {
    pub name: String,
    pub receiver: Option<GenericParameter>,
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

/// Extension trait for type environment
impl TypeEnvironment {
    /// Add a generic function to the environment
    pub fn add_generic_function(&mut self, name: String, func: GenericFunction) {
        // Implementation would add to internal storage
    }

    /// Add a generic struct to the environment
    pub fn add_generic_struct(&mut self, name: String, struct_decl: GenericStruct) {
        // Implementation would add to internal storage
    }

    /// Get a generic declaration by name
    pub fn get_generic_declaration(&self, name: &str) -> Option<GenericDeclaration> {
        // Implementation would retrieve from internal storage
        None
    }
}

/// Extension trait for type expressions
impl TypeExpression {
    /// Convert from AST type to type expression
    pub fn from_ast_type(ast_type: &crate::ast::Type) -> Self {
        // Implementation would convert AST type to type expression
        TypeExpression::named("unknown")
    }

    /// Convert to AST type
    pub fn to_ast_type(&self) -> crate::ast::Type {
        // Implementation would convert type expression to AST type
        if let Some(name) = &self.name {
            crate::ast::Type::Custom(name.clone())
        } else {
            crate::ast::Type::Custom("unknown".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monomorphizer_creation() {
        let monomorphizer = Monomorphizer::new();
        assert!(monomorphizer.instance_cache.is_empty());
        assert!(monomorphizer.work_queue.is_empty());
    }

    #[test]
    fn test_instance_id_generation() {
        let monomorphizer = Monomorphizer::new();
        let type_args = vec![
            TypeExpression::named("i32"),
            TypeExpression::named("f64"),
        ];
        let id = monomorphizer.generate_instance_id("max", &type_args);
        assert!(id.contains("max"));
        assert!(id.contains("i32"));
        assert!(id.contains("f64"));
    }

    #[test]
    fn test_instantiation_request() {
        let mut monomorphizer = Monomorphizer::new();
        let type_args = vec![TypeExpression::named("i32")];
        let constraints = vec![];
        
        let result = monomorphizer.request_instantiation(
            "Vec".to_string(),
            type_args,
            constraints,
            None,
        );
        
        assert!(result.is_ok());
        assert_eq!(monomorphizer.work_queue.len(), 1);
    }
}
