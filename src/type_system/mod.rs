// Type system modules for CURSED
pub mod type_inference;
pub mod generic_instantiator;
pub mod constraint_resolver;
pub mod generics_bounds_checker;
#[cfg(test)]
pub mod generics_integration_test;
pub mod associated_types;
pub mod variance;
pub mod higher_kinded_types;
pub mod generic_optimization;
pub mod generic_enhanced;
pub mod checker;
pub mod compilation_integration;
pub mod monomorphizer;
pub mod generic_constraints;
pub mod monomorphisation;
pub mod test_result_simple;
pub mod interface_compliance;
pub mod interface_inheritance;
pub mod generics_core;
// pub mod advanced_constraints;
pub mod generic_interfaces;

#[cfg(test)]
mod tests;

#[cfg(test)]
pub mod integration_tests;

#[cfg(test)]
pub mod mutable_reference_tests;

#[cfg(test)]


// Import base types from core and AST
pub use crate::core::{Type};
pub use crate::ast::{Type as AstType, Expression};

// Re-export key types
pub use type_inference::TypeInference;
pub use checker::{TypeChecker, TypeCheckError, TypeErrorKind};
pub use compilation_integration::{TypedCompilationPipeline, CompilationError, TypedProgram};
pub use test_result_simple::{TestResult, TestStatus, TestSuite, TestReport, TestResultBuilder};
pub use generic_instantiator::{GenericInstantiator, InstantiatedGeneric};
pub use constraint_resolver::{ConstraintResolver, ConstraintSolution, ConstraintViolation};
pub use generics_bounds_checker::TypeBoundsChecker;
pub use interface_compliance::{
    InterfaceComplianceChecker, InterfaceComplianceReport, InterfaceMethodRequirement,
    ConcreteMethodImplementation, ReceiverType, IncompatibleMethod,
    initialize_interface_compliance_checker, get_global_compliance_checker,
    check_global_interface_compliance, generate_global_compliance_report
};
pub use interface_inheritance::{
    InterfaceInheritanceChecker, InterfaceInheritanceTree, InterfaceCompositionOptions,
    InterfaceHierarchyInfo, InterfaceHierarchyValidationError, MethodConflict,
    initialize_interface_inheritance_checker, get_global_inheritance_checker
};

// Core type system structures
#[derive(Debug, Clone)]
pub struct TypeSystem {
    pub environment: TypeEnvironment,
    pub inference: InferenceContext,
    pub scopes: Vec<std::collections::HashMap<String, TypeExpression>>,
}

impl TypeSystem {
    pub fn new() -> Self {
        let mut environment = TypeEnvironment::new();
        
        // Add built-in types
        environment.add_builtin_type("int", TypeKind::Primitive);
        environment.add_builtin_type("string", TypeKind::Primitive);
        environment.add_builtin_type("bool", TypeKind::Primitive);
        environment.add_builtin_type("void", TypeKind::Primitive);
        
        // Add TestResult type system
        environment.add_builtin_type("TestResult", TypeKind::Struct);
        environment.add_builtin_type("TestStatus", TypeKind::Enum);
        environment.add_builtin_type("TestSuite", TypeKind::Struct);
        environment.add_builtin_type("TestReport", TypeKind::Struct);
        
        // Add built-in objects like 'vibez'
        let vibez_type = TypeDefinition {
            name: "vibez".to_string(),
            kind: TypeKind::Struct,
            type_parameters: Vec::new(),
            constraints: Vec::new(),
            methods: vec![
                MethodSignature {
                    name: "spill".to_string(),
                    parameters: vec![TypeExpression::named("tea")],
                    return_type: Some(TypeExpression::named("cap")),
                    type_parameters: Vec::new(),
                    constraints: Vec::new(),
                }
            ],
            fields: vec![], // Built-in vibez has no fields
            is_builtin: true,
            source_location: None,
        };
        environment.type_definitions.insert("vibez".to_string(), vibez_type);
        
        let mut type_system = Self {
            environment,
            inference: InferenceContext::new(),
            scopes: vec![std::collections::HashMap::new()],
        };
        
        type_system
    }
    
    pub fn enter_scope(&mut self) {
        self.scopes.push(std::collections::HashMap::new());
    }
    
    pub fn exit_scope(&mut self) {
        self.scopes.pop();
    }
    
    /// Check if a type exists in the type system
    pub fn has_type(&self, type_name: &str) -> bool {
        self.environment.type_definitions.contains_key(type_name)
    }
    
    pub fn add_variable(&mut self, name: String, type_expr: TypeExpression) {
        if let Some(current_scope) = self.scopes.last_mut() {
            current_scope.insert(name, type_expr);
        }
    }
    
    pub fn ast_type_to_type_expression(&self, ast_type: &crate::ast::Type) -> Result<TypeExpression, TypeCheckError> {
        use crate::ast::Type;
        
        match ast_type {
            Type::Normie => Ok(TypeExpression::named("normie")),
            Type::Tea => Ok(TypeExpression::named("tea")),
            Type::Lit => Ok(TypeExpression::named("lit")),
            Type::Sip => Ok(TypeExpression::named("sip")),
            Type::Smol => Ok(TypeExpression::named("smol")),
            Type::Mid => Ok(TypeExpression::named("mid")),
            Type::Thicc => Ok(TypeExpression::named("thicc")),
            Type::Snack => Ok(TypeExpression::named("snack")),
            Type::Meal => Ok(TypeExpression::named("meal")),
            Type::Byte => Ok(TypeExpression::named("byte")),
            Type::Rune => Ok(TypeExpression::named("rune")),
            Type::Extra => Ok(TypeExpression::named("extra")),
            Type::Custom(name) => Ok(TypeExpression::named(name)),
            Type::Array(elem_type, _) => {
                let element_expr = self.ast_type_to_type_expression(elem_type)?;
                Ok(TypeExpression::array(element_expr))
            }
            Type::Slice(elem_type) => {
                let element_expr = self.ast_type_to_type_expression(elem_type)?;
                Ok(TypeExpression::array(element_expr)) // Treat slices as arrays for now
            }
            Type::Tuple(element_types) => {
                let element_exprs: Result<Vec<_>, _> = element_types.iter()
                    .map(|t| self.ast_type_to_type_expression(t))
                    .collect();
                Ok(TypeExpression::tuple(element_exprs?))
            }
            _ => Ok(TypeExpression::named("void")),
        }
    }
    
    pub fn check_expression(&mut self, expr: &crate::ast::Expression) -> Result<TypeExpression, String> {
        use crate::ast::Expression;
        
        match expr {
            Expression::Integer(_) => Ok(TypeExpression::named("normie")),
            Expression::String(_) => Ok(TypeExpression::named("tea")),
            Expression::Boolean(_) => Ok(TypeExpression::named("vibes")),
            Expression::Identifier(name) => {
                if let Some(type_def) = self.environment.type_definitions.get(name) {
                    Ok(TypeExpression::named(&type_def.name))
                } else {
                    Err(format!("Unknown identifier: {}", name))
                }
            }
            Expression::MemberAccess(member_access) => {
                let object_type = self.check_expression(&member_access.object)?;
                self.check_member_access(&object_type, &member_access.property)
            }
            Expression::Call(call_expr) => {
                self.check_call_expression(call_expr)
            }
            Expression::Map(pairs) => {
                self.check_map_pairs(pairs)
            }
            Expression::Binary(binary) => {
                let left_type = self.check_expression(&binary.left)?;
                let right_type = self.check_expression(&binary.right)?;
                self.check_binary_operation(&left_type, &binary.operator, &right_type)
            }
            Expression::Tuple(tuple_expr) => {
                let mut element_types = Vec::new();
                for element in &tuple_expr.elements {
                    element_types.push(self.check_expression(element)?);
                }
                Ok(TypeExpression::tuple(element_types))
            }
            Expression::TupleAccess(tuple_access) => {
                let tuple_type = self.check_expression(&tuple_access.tuple)?;
                self.check_tuple_access(&tuple_type, tuple_access.index)
            }
            Expression::Unary(unary_expr) => {
                let operand_type = self.check_expression(&unary_expr.operand)?;
                self.check_unary_operation(&unary_expr.operator, &operand_type)
            }
            Expression::TypeSwitch(type_switch) => {
                self.check_type_switch_expression(type_switch)
            }
            _ => Ok(TypeExpression::named("unknown")),
        }
    }
    
    fn check_member_access(&self, object_type: &TypeExpression, property: &str) -> Result<TypeExpression, String> {
        if let Some(object_name) = &object_type.name {
            if let Some(type_def) = self.environment.type_definitions.get(object_name) {
                for method in &type_def.methods {
                    if method.name == property {
                        if let Some(return_type) = &method.return_type {
                            return Ok(return_type.clone());
                        } else {
                            return Ok(TypeExpression::named("void"));
                        }
                    }
                }
                return Err(format!("Property '{}' not found on type '{}'", property, object_name));
            }
        }
        Err(format!("Cannot access property '{}' on unknown type", property))
    }
    
    fn check_call_expression(&mut self, call_expr: &crate::ast::CallExpression) -> Result<TypeExpression, String> {
        if let crate::ast::Expression::MemberAccess(member_access) = &*call_expr.function {
            let object_type = self.check_expression(&member_access.object)?;
            if let Some(object_name) = &object_type.name {
                // Clone the method information to avoid borrowing issues
                let method_info = self.environment.type_definitions.get(object_name)
                    .and_then(|type_def| {
                        type_def.methods.iter()
                            .find(|method| method.name == member_access.property)
                            .map(|method| (method.name.clone(), method.parameters.clone(), method.return_type.clone()))
                    });
                
                if let Some((method_name, parameters, return_type)) = method_info {
                    // Type check arguments
                    if call_expr.arguments.len() != parameters.len() {
                        return Err(format!("Method '{}' expects {} arguments, got {}", 
                            method_name, parameters.len(), call_expr.arguments.len()));
                    }
                    
                    // Basic argument type checking
                    for (i, arg) in call_expr.arguments.iter().enumerate() {
                        let arg_type = self.check_expression(arg)?;
                        let expected_type = &parameters[i];
                        if !self.types_compatible(&arg_type, expected_type) {
                            return Err(format!("Argument {} type mismatch: expected {:?}, got {:?}", 
                                i, expected_type, arg_type));
                        }
                    }
                    
                    return Ok(return_type.unwrap_or(TypeExpression::named("void")));
                } else {
                    return Err(format!("Method '{}' not found on type '{}'", member_access.property, object_name));
                }
            }
        }
        
        Ok(TypeExpression::named("unknown"))
    }
    
    fn check_map_pairs(&mut self, map_pairs: &[(Expression, Expression)]) -> Result<TypeExpression, String> {
        if map_pairs.is_empty() {
            // Empty map - return Map with Unknown key/value types
            return Ok(TypeExpression::map(
                TypeExpression::named("unknown"), 
                TypeExpression::named("unknown")
            ));
        }
        
        // Infer key and value types from first pair
        let first_pair = &map_pairs[0];
        let key_type = self.check_expression(&first_pair.0)?;
        let value_type = self.check_expression(&first_pair.1)?;
        
        // Check that all other pairs have compatible types
        for pair in &map_pairs[1..] {
            let pair_key_type = self.check_expression(&pair.0)?;
            let pair_value_type = self.check_expression(&pair.1)?;
            
            if !self.types_compatible(&key_type, &pair_key_type) {
                return Err(format!(
                    "Map key type mismatch: expected {:?}, got {:?}",
                    key_type, pair_key_type
                ));
            }
            
            if !self.types_compatible(&value_type, &pair_value_type) {
                return Err(format!(
                    "Map value type mismatch: expected {:?}, got {:?}",
                    value_type, pair_value_type
                ));
            }
        }
        
        Ok(TypeExpression::map(key_type, value_type))
    }
    
    fn check_binary_operation(&self, left: &TypeExpression, op: &str, right: &TypeExpression) -> Result<TypeExpression, String> {
        match op {
            "+" | "-" | "*" | "/" => {
                if self.is_numeric_type(left) && self.is_numeric_type(right) {
                    Ok(left.clone())
                } else {
                    Err(format!("Arithmetic operation requires numeric types, got {:?} and {:?}", left, right))
                }
            }
            "==" | "!=" | "<" | ">" | "<=" | ">=" => {
                if self.types_compatible(left, right) {
                    Ok(TypeExpression::named("vibes"))
                } else {
                    Err(format!("Comparison requires compatible types, got {:?} and {:?}", left, right))
                }
            }
            "&&" | "||" => {
                if self.types_compatible(left, &TypeExpression::named("bool")) && 
                   self.types_compatible(right, &TypeExpression::named("bool")) {
                    Ok(TypeExpression::named("bool"))
                } else {
                    Err(format!("Logical operation requires bool types, got {:?} and {:?}", left, right))
                }
            }
            _ => Err(format!("Unknown binary operator: {}", op))
        }
    }
    
    fn types_compatible(&self, t1: &TypeExpression, t2: &TypeExpression) -> bool {
        // Enhanced compatibility check with type coercion
        if t1.name == t2.name {
            return true;
        }
        
        // Check for coercible types
        if let (Some(t1_name), Some(t2_name)) = (&t1.name, &t2.name) {
            match (t1_name.as_str(), t2_name.as_str()) {
                // Integer type coercions
                ("normie", "thicc") | ("thicc", "normie") => true,
                ("normie", "smol") | ("smol", "normie") => true,
                ("normie", "mid") | ("mid", "normie") => true,
                
                // Float type coercions
                ("snack", "meal") | ("meal", "snack") => true,
                ("drip", "meal") | ("meal", "drip") => true,
                
                // Integer to float coercions
                ("normie", "meal") | ("normie", "snack") => true,
                ("thicc", "meal") | ("smol", "snack") => true,
                
                // String type compatibility
                ("tea", "string") | ("string", "tea") => true,
                
                // Boolean type compatibility
                ("lit", "bool") | ("bool", "lit") => true,
                
                // Character type compatibility
                ("sip", "char") | ("char", "sip") => true,
                
                // Array type compatibility
                ("squad", "array") | ("array", "squad") => true,
                
                // Channel type compatibility
                ("dm", "chan") | ("chan", "dm") => true,
                
                _ => false,
            }
        } else {
            false
        }
    }
    
    fn check_tuple_access(&self, tuple_type: &TypeExpression, index: usize) -> Result<TypeExpression, String> {
        // Verify that the type is actually a tuple
        if let Some(name) = &tuple_type.name {
            if name == "Tuple" {
                // Check if the index is valid
                if index < tuple_type.parameters.len() {
                    Ok(tuple_type.parameters[index].clone())
                } else {
                    Err(format!("Tuple index {} out of bounds for tuple with {} elements", index, tuple_type.parameters.len()))
                }
            } else {
                Err(format!("Cannot access tuple element on non-tuple type: {}", name))
            }
        } else {
            Err("Cannot access tuple element on unknown type".to_string())
        }
    }
    
    fn is_numeric_type(&self, type_expr: &TypeExpression) -> bool {
        if let Some(name) = &type_expr.name {
            matches!(name.as_str(), 
                // CURSED integer types
                "normie" | "thicc" | "smol" | "mid" |
                // CURSED float types
                "snack" | "meal" | "drip" |
                // Standard types (fallback)
                "int" | "float" | "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64" | "f32" | "f64"
            )
        } else {
            false
        }
    }
    
    fn check_unary_operation(&self, operator: &crate::ast::UnaryOperator, operand_type: &TypeExpression) -> Result<TypeExpression, String> {
        use crate::ast::UnaryOperator;
        
        match operator {
            UnaryOperator::Not => {
                if self.types_compatible(operand_type, &TypeExpression::named("lit")) {
                    Ok(TypeExpression::named("lit"))
                } else {
                    Err(format!("Not operator requires boolean type, got {:?}", operand_type))
                }
            }
            UnaryOperator::Minus | UnaryOperator::Plus => {
                if self.is_numeric_type(operand_type) {
                    Ok(operand_type.clone())
                } else {
                    Err(format!("Unary arithmetic operator requires numeric type, got {:?}", operand_type))
                }
            }
            UnaryOperator::AddressOf => {
                // Address-of operator: @variable -> @Type
                Ok(TypeExpression::pointer(operand_type.clone()))
            }
            UnaryOperator::Dereference => {
                // Dereference operator: *pointer -> Type
                if let Some(name) = &operand_type.name {
                    if name == "Pointer" && !operand_type.parameters.is_empty() {
                        Ok(operand_type.parameters[0].clone())
                    } else {
                        Err(format!("Dereference operator requires pointer type, got {:?}", operand_type))
                    }
                } else {
                    Err(format!("Dereference operator requires pointer type, got {:?}", operand_type))
                }
            }
        }
    }

    fn check_type_switch_expression(&mut self, type_switch: &crate::ast::TypeSwitchExpression) -> Result<TypeExpression, String> {
        // Check the variable being type-switched on
        let variable_type = self.check_expression(&type_switch.variable)?;
        
        // Type check all arms and ensure they return compatible types
        let mut arm_types = Vec::new();
        
        for arm in &type_switch.arms {
            // Check the type pattern
            match &arm.type_pattern {
                crate::ast::TypePattern::Type(type_expr) => {
                    // Validate that the type exists
                    self.validate_type_exists(type_expr)?;
                }
                crate::ast::TypePattern::Interface(interface_name) => {
                    // Check that the interface exists
                    if !self.environment.type_definitions.contains_key(interface_name) {
                        return Err(format!("Unknown interface: {}", interface_name));
                    }
                }
                crate::ast::TypePattern::Wildcard => {
                    // Wildcard is always valid
                }
            }
            
            // If there's a bound variable, add it to the type environment
            // for the scope of this arm body
            let has_bound_variable = arm.bound_variable.is_some();
            
            if has_bound_variable {
                self.enter_scope();
                
                if let Some(ref bound_var) = arm.bound_variable {
                    // Determine the narrowed type based on the type pattern
                    let narrowed_type = match &arm.type_pattern {
                        crate::ast::TypePattern::Type(type_expr) => {
                            // Convert the AST type to TypeExpression for the narrowed type
                            self.ast_type_to_type_expression(type_expr).map_err(|e| e.message)?
                        }
                        crate::ast::TypePattern::Interface(interface_name) => {
                            // The bound variable has the interface type
                            TypeExpression::named(interface_name)
                        }
                        crate::ast::TypePattern::Wildcard => {
                            // For wildcard, keep the original variable type
                            variable_type.clone()
                        }
                    };
                    
                    // Add the bound variable with the narrowed type to the current scope
                    self.add_variable(bound_var.clone(), narrowed_type);
                }
            }
            
            // Type check the arm body
            let body_type = self.check_expression(&arm.body)?;
            
            // Exit the scope if we entered one for the bound variable
            if has_bound_variable {
                self.exit_scope();
            }
            arm_types.push(body_type);
        }
        
        // All arms should return the same type
        if !arm_types.is_empty() {
            let first_type = &arm_types[0];
            for arm_type in &arm_types[1..] {
                if !self.types_compatible(first_type, arm_type) {
                    return Err(format!(
                        "Type switch arms have incompatible return types: {:?} vs {:?}",
                        first_type, arm_type
                    ));
                }
            }
            Ok(first_type.clone())
        } else {
            Ok(TypeExpression::named("unknown"))
        }
    }

    fn validate_type_exists(&self, type_expr: &crate::ast::Type) -> Result<(), String> {
        match type_expr {
            crate::ast::Type::Custom(name) => {
                // Check if it's a user-defined type
                if self.environment.type_definitions.contains_key(name) {
                    Ok(())
                } else {
                    Err(format!("Unknown type: {}", name))
                }
            }
            // Built-in CURSED types are always valid
            crate::ast::Type::Normie | crate::ast::Type::Tea | crate::ast::Type::Lit |
            crate::ast::Type::Sip | crate::ast::Type::Smol | crate::ast::Type::Mid |
            crate::ast::Type::Thicc | crate::ast::Type::Snack | crate::ast::Type::Meal => Ok(()),
            _ => Ok(()), // Other type variants are assumed to be valid for now
        }
    }
}

#[derive(Debug, Clone)]
pub struct TypeEnvironment {
    pub type_definitions: std::collections::HashMap<String, TypeDefinition>,
}

#[derive(Debug, Clone)]
pub struct TypeSubstitution {
    pub mappings: std::collections::HashMap<String, TypeExpression>,
}

impl TypeSubstitution {
    pub fn new() -> Self {
        Self {
            mappings: std::collections::HashMap::new(),
        }
    }
    
    pub fn add(&mut self, from: String, to: TypeExpression) {
        self.mappings.insert(from, to);
    }
    
    pub fn apply(&self, type_expr: &TypeExpression) -> TypeExpression {
        if let Some(name) = &type_expr.name {
            if let Some(replacement) = self.mappings.get(name) {
                return replacement.clone();
            }
        }
        
        TypeExpression {
            kind: type_expr.kind.clone(),
            name: type_expr.name.clone(),
            parameters: type_expr.parameters.iter().map(|p| self.apply(p)).collect(),
            return_type: type_expr.return_type.as_ref().map(|rt| Box::new(self.apply(rt))),
        }
    }
    
    pub fn unify(&mut self, t1: &TypeExpression, t2: &TypeExpression) -> Result<(), String> {
        match (&t1.name, &t2.name) {
            (Some(n1), Some(n2)) if n1 == n2 => {
                // Same named types, check parameters
                if t1.parameters.len() != t2.parameters.len() {
                    return Err(format!("Parameter count mismatch: {} vs {}", t1.parameters.len(), t2.parameters.len()));
                }
                
                for (p1, p2) in t1.parameters.iter().zip(t2.parameters.iter()) {
                    self.unify(p1, p2)?;
                }
                
                if let (Some(rt1), Some(rt2)) = (&t1.return_type, &t2.return_type) {
                    self.unify(rt1, rt2)?;
                }
                
                Ok(())
            }
            (Some(name), _) => {
                // t1 is a type variable, bind it to t2
                self.add(name.clone(), t2.clone());
                Ok(())
            }
            (_, Some(name)) => {
                // t2 is a type variable, bind it to t1
                self.add(name.clone(), t1.clone());
                Ok(())
            }
            _ => {
                // Both are concrete types, must match exactly
                if t1 == t2 {
                    Ok(())
                } else {
                    Err(format!("Cannot unify {:?} with {:?}", t1, t2))
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeExpression {
    pub kind: TypeKind,
    pub name: Option<String>,
    pub parameters: Vec<TypeExpression>,
    pub return_type: Option<Box<TypeExpression>>,
}

impl TypeExpression {
    pub fn named(name: &str) -> Self {
        Self {
            kind: TypeKind::Primitive,
            name: Some(name.to_string()),
            parameters: Vec::new(),
            return_type: None,
        }
    }
    
    pub fn parameter(name: &str) -> Self {
        Self {
            kind: TypeKind::Primitive,
            name: Some(name.to_string()),
            parameters: Vec::new(),
            return_type: None,
        }
    }
    
    pub fn generic(name: &str, params: Vec<TypeExpression>) -> Self {
        Self {
            kind: TypeKind::Struct,
            name: Some(name.to_string()),
            parameters: params,
            return_type: None,
        }
    }
    
    pub fn function(params: Vec<TypeExpression>, return_type: TypeExpression) -> Self {
        Self {
            kind: TypeKind::Function,
            name: None,
            parameters: params,
            return_type: Some(Box::new(return_type)),
        }
    }
    
    pub fn array(element_type: TypeExpression) -> Self {
        Self {
            kind: TypeKind::Struct,
            name: Some("Array".to_string()),
            parameters: vec![element_type],
            return_type: None,
        }
    }
    
    pub fn map(key_type: TypeExpression, value_type: TypeExpression) -> Self {
        Self {
            kind: TypeKind::Struct,
            name: Some("Map".to_string()),
            parameters: vec![key_type, value_type],
            return_type: None,
        }
    }
    
    pub fn tuple(element_types: Vec<TypeExpression>) -> Self {
        Self {
            kind: TypeKind::Struct,
            name: Some("Tuple".to_string()),
            parameters: element_types,
            return_type: None,
        }
    }
    
    pub fn is_tuple(&self) -> bool {
        self.name.as_ref().map_or(false, |n| n == "Tuple")
    }
    
    pub fn get_tuple_types(&self) -> Option<&Vec<TypeExpression>> {
        if self.is_tuple() {
            Some(&self.parameters)
        } else {
            None
        }
    }
    
    pub fn pointer(inner_type: TypeExpression) -> Self {
        Self {
            kind: TypeKind::Primitive,
            name: Some("Pointer".to_string()),
            parameters: vec![inner_type],
            return_type: None,
        }
    }
    
    pub fn mutable_reference(pointee_type: TypeExpression) -> Self {
        Self {
            kind: TypeKind::Reference(Box::new(pointee_type), true),
            name: Some("&mut".to_string()),
            parameters: Vec::new(),
            return_type: None,
        }
    }
    
    pub fn immutable_reference(pointee_type: TypeExpression) -> Self {
        Self {
            kind: TypeKind::Reference(Box::new(pointee_type), false),
            name: Some("&".to_string()),
            parameters: Vec::new(),
            return_type: None,
        }
    }
    
    pub fn mutable_pointer(pointee_type: TypeExpression) -> Self {
        Self {
            kind: TypeKind::Pointer(Box::new(pointee_type), true),
            name: Some("*mut".to_string()),
            parameters: Vec::new(),
            return_type: None,
        }
    }
    
    pub fn immutable_pointer(pointee_type: TypeExpression) -> Self {
        Self {
            kind: TypeKind::Pointer(Box::new(pointee_type), false),
            name: Some("*".to_string()),
            parameters: Vec::new(),
            return_type: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TypeDefinition {
    pub name: String,
    pub kind: TypeKind,
    pub type_parameters: Vec<String>,
    pub constraints: Vec<GenericConstraint>,
    pub methods: Vec<MethodSignature>,
    pub fields: Vec<crate::ast::StructField>, // For struct field storage
    pub is_builtin: bool,
    pub source_location: Option<crate::error::SourceLocation>,
}

#[derive(Debug, Clone)]
pub enum TypeKind {
    Struct,
    Enum,
    Interface,
    Function,
    Primitive,
    Generic(GenericInfo),
    Named(String),
    Reference(Box<TypeExpression>, bool), // (pointee_type, is_mutable)
    Pointer(Box<TypeExpression>, bool),   // (pointee_type, is_mutable)
}

impl Eq for TypeKind {}

impl std::hash::Hash for TypeKind {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            TypeKind::Struct => 0.hash(state),
            TypeKind::Enum => 1.hash(state),
            TypeKind::Interface => 2.hash(state),
            TypeKind::Function => 3.hash(state),
            TypeKind::Primitive => 4.hash(state),
            TypeKind::Generic(_) => 5.hash(state), // Simple hash for generics
            TypeKind::Named(name) => {
                6.hash(state);
                name.hash(state);
            }
            TypeKind::Reference(pointee, is_mutable) => {
                7.hash(state);
                pointee.hash(state);
                is_mutable.hash(state);
            }
            TypeKind::Pointer(pointee, is_mutable) => {
                8.hash(state);
                pointee.hash(state);
                is_mutable.hash(state);
            }
        }
    }
}

impl PartialEq for TypeKind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TypeKind::Struct, TypeKind::Struct) => true,
            (TypeKind::Enum, TypeKind::Enum) => true,
            (TypeKind::Interface, TypeKind::Interface) => true,
            (TypeKind::Function, TypeKind::Function) => true,
            (TypeKind::Primitive, TypeKind::Primitive) => true,
            (TypeKind::Named(a), TypeKind::Named(b)) => a == b,
            (TypeKind::Generic(_), TypeKind::Generic(_)) => false, // Don't compare generic info
            (TypeKind::Reference(a_pointee, a_mut), TypeKind::Reference(b_pointee, b_mut)) => {
                a_pointee == b_pointee && a_mut == b_mut
            }
            (TypeKind::Pointer(a_pointee, a_mut), TypeKind::Pointer(b_pointee, b_mut)) => {
                a_pointee == b_pointee && a_mut == b_mut
            }
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GenericInfo {
    pub function_declaration: Option<crate::ast::FunctionDeclaration>,
    pub struct_declaration: Option<crate::ast::StructDeclaration>,
    pub constraints: Vec<GenericConstraint>,
}

#[derive(Debug, Clone)]
pub struct InstantiatedType {
    pub base_type: TypeExpression,
    pub type_arguments: Vec<TypeExpression>,
    pub substitutions: TypeSubstitution,
}

impl InstantiatedType {
    pub fn new(base_type: TypeExpression, type_arguments: Vec<TypeExpression>) -> Self {
        let mut substitutions = TypeSubstitution::new();
        
        // Apply type arguments to base type if it has parameters
        if base_type.parameters.len() == type_arguments.len() {
            for (i, param) in base_type.parameters.iter().enumerate() {
                if let Some(param_name) = &param.name {
                    substitutions.add(param_name.clone(), type_arguments[i].clone());
                }
            }
        }
        
        Self {
            base_type,
            type_arguments,
            substitutions,
        }
    }
    
    pub fn instantiate(&self) -> TypeExpression {
        self.substitutions.apply(&self.base_type)
    }
}

#[derive(Debug, Clone)]
pub struct MethodSignature {
    pub name: String,
    pub parameters: Vec<TypeExpression>,
    pub return_type: Option<TypeExpression>,
    pub type_parameters: Vec<String>,
    pub constraints: Vec<GenericConstraint>,
}

#[derive(Debug, Clone)]
pub struct ConstraintContext {
    pub scope_id: String,
    pub active_constraints: Vec<ConstraintBinding>,
    pub type_bindings: std::collections::HashMap<String, TypeExpression>,
}

#[derive(Debug, Clone)]
pub struct GenericConstraint {
    pub constraint_name: String,
    pub type_parameters: Vec<String>,
    pub bounds: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ConstraintBinding {
    pub constraint: GenericConstraint,
    pub bound_types: Vec<String>,
    pub satisfaction_status: ConstraintStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConstraintStatus {
    /// Constraint is waiting to be validated
    Pending,
    /// Constraint has been validated and satisfied
    Resolved,
    /// Constraint validation failed
    Failed(String),
    /// Constraint is partially satisfied but needs more information
    Partial(Vec<String>),
    /// Constraint is being recursively checked (cycle detection)
    InProgress,
    /// Constraint validation was skipped due to configuration
    Skipped(String),
}

#[derive(Debug, Clone)]
pub struct InferenceContext {
    pub substitutions: TypeSubstitution,
    pub type_vars: std::collections::HashMap<String, TypeExpression>,
    pub next_var_id: usize,
}

impl InferenceContext {
    pub fn new() -> Self {
        Self {
            substitutions: TypeSubstitution::new(),
            type_vars: std::collections::HashMap::new(),
            next_var_id: 0,
        }
    }
    
    pub fn fresh_type_var(&mut self) -> TypeExpression {
        let var_name = format!("T{}", self.next_var_id);
        self.next_var_id += 1;
        TypeExpression::named(&var_name)
    }
    
    pub fn bind_type_var(&mut self, var_name: &str, type_expr: TypeExpression) {
        self.type_vars.insert(var_name.to_string(), type_expr.clone());
        self.substitutions.add(var_name.to_string(), type_expr);
    }
    
    pub fn resolve_type(&self, type_expr: &TypeExpression) -> TypeExpression {
        self.substitutions.apply(type_expr)
    }
}

#[derive(Debug, Clone)]
pub struct ConstraintTracker {
    /// Track status of type constraints by ID
    pub constraint_status: std::collections::HashMap<String, ConstraintStatus>,
    /// Track constraint dependencies
    pub constraint_dependencies: std::collections::HashMap<String, Vec<String>>,
    /// Track constraint resolution order
    pub resolution_order: Vec<String>,
    /// Track constraints currently being validated (cycle detection)
    pub in_progress: std::collections::HashSet<String>,
}

impl ConstraintTracker {
    pub fn new() -> Self {
        Self {
            constraint_status: std::collections::HashMap::new(),
            constraint_dependencies: std::collections::HashMap::new(),
            resolution_order: Vec::new(),
            in_progress: std::collections::HashSet::new(),
        }
    }

    /// Register a new constraint for tracking
    pub fn register_constraint(&mut self, constraint_id: String, dependencies: Vec<String>) {
        self.constraint_status.insert(constraint_id.clone(), ConstraintStatus::Pending);
        self.constraint_dependencies.insert(constraint_id, dependencies);
    }

    /// Update constraint status
    pub fn update_status(&mut self, constraint_id: &str, status: ConstraintStatus) -> Result<(), String> {
        if !self.constraint_status.contains_key(constraint_id) {
            return Err(format!("Unknown constraint: {}", constraint_id));
        }

        // Check for invalid transitions
        let current_status = &self.constraint_status[constraint_id];
        match (current_status, &status) {
            (ConstraintStatus::Resolved, ConstraintStatus::Failed(_)) => {
                return Err("Cannot transition from Resolved to Failed".to_string());
            }
            (ConstraintStatus::Failed(_), ConstraintStatus::Resolved) => {
                return Err("Cannot transition from Failed to Resolved".to_string());
            }
            _ => {}
        }

        self.constraint_status.insert(constraint_id.to_string(), status);
        
        // Add to resolution order if resolved
        if matches!(self.constraint_status[constraint_id], ConstraintStatus::Resolved) {
            if !self.resolution_order.contains(&constraint_id.to_string()) {
                self.resolution_order.push(constraint_id.to_string());
            }
        }

        Ok(())
    }

    /// Start validating a constraint (cycle detection)
    pub fn start_validation(&mut self, constraint_id: &str) -> Result<(), String> {
        if self.in_progress.contains(constraint_id) {
            return Err(format!("Circular dependency detected for constraint: {}", constraint_id));
        }

        self.in_progress.insert(constraint_id.to_string());
        self.update_status(constraint_id, ConstraintStatus::InProgress)?;
        Ok(())
    }

    /// Finish validating a constraint
    pub fn finish_validation(&mut self, constraint_id: &str, final_status: ConstraintStatus) -> Result<(), String> {
        self.in_progress.remove(constraint_id);
        self.update_status(constraint_id, final_status)?;
        Ok(())
    }

    /// Get all constraints with a specific status
    pub fn get_constraints_with_status(&self, status: &ConstraintStatus) -> Vec<String> {
        self.constraint_status
            .iter()
            .filter(|(_, s)| std::mem::discriminant(*s) == std::mem::discriminant(status))
            .map(|(id, _)| id.clone())
            .collect()
    }

    /// Check if all constraints are resolved
    pub fn all_resolved(&self) -> bool {
        self.constraint_status
            .values()
            .all(|status| matches!(status, ConstraintStatus::Resolved))
    }

    /// Get constraints ready for validation (dependencies resolved)
    pub fn get_ready_constraints(&self) -> Vec<String> {
        self.constraint_status
            .iter()
            .filter(|(id, status)| {
                matches!(status, ConstraintStatus::Pending) &&
                self.dependencies_resolved(id)
            })
            .map(|(id, _)| id.clone())
            .collect()
    }

    /// Check if all dependencies of a constraint are resolved
    fn dependencies_resolved(&self, constraint_id: &str) -> bool {
        if let Some(deps) = self.constraint_dependencies.get(constraint_id) {
            deps.iter().all(|dep| {
                self.constraint_status
                    .get(dep)
                    .map(|status| matches!(status, ConstraintStatus::Resolved))
                    .unwrap_or(false)
            })
        } else {
            true // No dependencies
        }
    }

    /// Get constraint validation summary
    pub fn get_summary(&self) -> ConstraintValidationSummary {
        let total = self.constraint_status.len();
        let resolved = self.get_constraints_with_status(&ConstraintStatus::Resolved).len();
        let failed = self.constraint_status
            .values()
            .filter(|status| matches!(status, ConstraintStatus::Failed(_)))
            .count();
        let pending = self.get_constraints_with_status(&ConstraintStatus::Pending).len();

        ConstraintValidationSummary {
            total,
            resolved,
            failed,
            pending,
            in_progress: self.in_progress.len(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConstraintValidationSummary {
    pub total: usize,
    pub resolved: usize,
    pub failed: usize,
    pub pending: usize,
    pub in_progress: usize,
}
impl Default for TypeSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeEnvironment {
    pub fn new() -> Self {
        Self {
            type_definitions: std::collections::HashMap::new(),
        }
    }
    
    pub fn add_builtin_type(&mut self, name: &str, kind: TypeKind) {
        let type_def = TypeDefinition {
            name: name.to_string(),
            kind,
            type_parameters: Vec::new(),
            constraints: Vec::new(),
            methods: Vec::new(),
            fields: Vec::new(), // Built-in types have no fields by default
            is_builtin: true,
            source_location: None,
        };
        self.type_definitions.insert(name.to_string(), type_def);
    }
    
    pub fn add_type_definition(&mut self, type_def: TypeDefinition) {
        self.type_definitions.insert(type_def.name.clone(), type_def);
    }
    
    pub fn get_type(&self, name: &str) -> Option<&TypeDefinition> {
        self.type_definitions.get(name)
    }
    
    // Generic storage and retrieval functions
    pub fn get_generic_function(&self, name: &str) -> Option<&crate::ast::FunctionDeclaration> {
        if let Some(type_def) = self.type_definitions.get(name) {
            if let TypeKind::Generic(ref generic_info) = type_def.kind {
                if let Some(ref function_decl) = generic_info.function_declaration {
                    return Some(function_decl);
                }
            }
        }
        None
    }

    pub fn get_generic_struct(&self, name: &str) -> Option<&crate::ast::StructDeclaration> {
        if let Some(type_def) = self.type_definitions.get(name) {
            if let TypeKind::Generic(ref generic_info) = type_def.kind {
                if let Some(ref struct_decl) = generic_info.struct_declaration {
                    return Some(struct_decl);
                }
            }
        }
        None
    }

    pub fn get_function_constraints(&self, name: &str) -> Option<Vec<GenericConstraint>> {
        if let Some(type_def) = self.type_definitions.get(name) {
            if let TypeKind::Generic(ref generic_info) = type_def.kind {
                return Some(generic_info.constraints.clone());
            }
        }
        None
    }

    pub fn get_struct_constraints(&self, name: &str) -> Option<Vec<GenericConstraint>> {
        if let Some(type_def) = self.type_definitions.get(name) {
            if let TypeKind::Generic(ref generic_info) = type_def.kind {
                return Some(generic_info.constraints.clone());
            }
        }
        None
    }

    pub fn get_variable_type(&self, name: &str) -> Option<TypeExpression> {
        if let Some(type_def) = self.type_definitions.get(name) {
            Some(TypeExpression {
                kind: TypeKind::Named(name.to_string()),
                name: Some(name.to_string()),
                parameters: Vec::new(),
                return_type: None,
            })
        } else {
            None
        }
    }

    pub fn store_generic_function(&mut self, name: &str, function_decl: crate::ast::FunctionDeclaration, constraints: Vec<GenericConstraint>) {
        let generic_info = GenericInfo {
            function_declaration: Some(function_decl),
            struct_declaration: None,
            constraints,
        };
        
        let type_def = TypeDefinition {
            name: name.to_string(),
            kind: TypeKind::Generic(generic_info),
            type_parameters: Vec::new(),
            constraints: Vec::new(),
            methods: Vec::new(),
            fields: Vec::new(),
            is_builtin: false,
            source_location: None,
        };
        
        self.type_definitions.insert(name.to_string(), type_def);
    }

    pub fn store_generic_struct(&mut self, name: &str, struct_decl: crate::ast::StructDeclaration, constraints: Vec<GenericConstraint>) {
        let generic_info = GenericInfo {
            function_declaration: None,
            struct_declaration: Some(struct_decl),
            constraints,
        };
        
        let type_def = TypeDefinition {
            name: name.to_string(),
            kind: TypeKind::Generic(generic_info),
            type_parameters: Vec::new(),
            constraints: Vec::new(),
            methods: Vec::new(),
            fields: Vec::new(),
            is_builtin: false,
            source_location: None,
        };
        
        self.type_definitions.insert(name.to_string(), type_def);
    }
}

impl Default for TypeEnvironment {
    fn default() -> Self {
        Self::new()
    }
}
// impl Default for TypeInference {
//     fn default() -> Self {
//         Self {}
//     }
// }

// Additional type system types
#[derive(Debug, Clone)]
pub struct TypeParameter {
    pub name: String,
    pub bounds: Vec<GenericConstraint>,
}

// Additional re-exports for enhanced functionality
pub use generic_enhanced::{GenericTypeChecker, InterfaceChecker, TypeSpecializer};
pub use constraint_resolver::{
    ViolationReason, TypeUnifier, ConstraintPropagator, ConstraintGraph, ConstraintNode
};
// pub use crate::type_system::advanced_constraints::{
//     AdvancedConstraint, AdvancedConstraintChecker, ConstraintDependencyGraph, ConstraintNode as AdvancedConstraintNode
// };
pub use crate::type_system::generic_interfaces::{
    GenericInterface, GenericTypeParameter, AssociatedType, InterfaceMethod, 
    Variance, InterfaceImplementation, GenericInterfaceChecker,
    InterfaceHierarchy
};
pub use crate::type_system::higher_kinded_types::{
    Kind, TypeConstructor, KindedTypeParameter, HigherKindedConstraint,
    HigherKindedTypeSystem, TypeConstructorInstance
};

// New generics modules
pub use crate::type_system::monomorphizer::{
    Monomorphizer, MonomorphizedInstance, ConcreteAST, InstantiationRequest
};
pub use crate::type_system::generic_constraints::{
    GenericConstraintChecker, ConstraintResult, TypeConstraint, WhereClause
};
pub use crate::type_system::generics_core::GenericsCore;
