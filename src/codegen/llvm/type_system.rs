/// LLVM Type System Compilation for CURSED
/// 
/// This module implements complete LLVM compilation for CURSED's type system including:
/// - Struct definitions (squad keyword) with field layout
/// - Interface definitions (collab keyword) with method tables
/// - Type instantiation and method dispatch
/// - Type checking and casting operations
/// - Basic generic type support
/// 
/// The implementation handles Gen Z slang syntax while generating efficient
/// standard LLVM type operations with proper memory safety and performance.

use crate::ast::declarations::{SquadStatement, CollabStatement, FieldStatement, MethodDeclaration, GenericConstraint};
use crate::ast::identifiers::Identifier;
use crate::ast::types::{TypeExpression, Type, StructType, InterfaceType};
use crate::ast::traits::{Node, Expression};
use crate::error::CursedError;
use crate::type_system::{
    TypeInference, TypeSubstitution, TypeExpression as TypeSystemExpression
    // Disabled imports for simplified AST compatibility:
    // ConstraintResolver, ConstraintSolution (require expanded constraint system)
// };

use std::collections::HashMap;
use std::collections::HashSet;

/// LLVM Type Registry for managing compiled types
#[derive(Debug)]
pub struct LlvmTypeRegistry {
    /// Compiled struct types with LLVM representations
    /// Compiled interface types with method tables
    /// Type dependencies for proper compilation order
    /// Generic type instantiations
/// Compiled struct type with LLVM metadata
#[derive(Debug, Clone)]
pub struct CompiledStructType {
    pub llvm_type: String, // LLVM struct type representation
/// Compiled field with type information
#[derive(Debug, Clone)]
pub struct CompiledField {
/// Compiled interface type with method dispatch table
#[derive(Debug, Clone)]
pub struct CompiledInterfaceType {
    pub vtable_type: String, // LLVM vtable type
    pub type_id: u64, // Runtime type identification
/// Compiled method with dispatch information
#[derive(Debug, Clone)]
pub struct CompiledMethod {
/// Generic type instance with concrete parameters
#[derive(Debug, Clone)]
pub struct CompiledGenericInstance {
/// Compiled generic type with constraint information
#[derive(Debug, Clone)]
pub struct CompiledGenericType {
/// Compiled type constraint
#[derive(Debug, Clone)]
pub struct CompiledConstraint {
/// Type compilation context for managing state
#[derive(Debug)]
pub struct TypeCompilationContext {
    /// Integrated type system for constraint resolution and inference
    /// Generic type compilation cache
impl LlvmTypeRegistry {
    pub fn new() -> Self {
        Self {
        }
    }

    /// Register a struct type for compilation
    pub fn register_struct(&mut self, name: String, compiled: CompiledStructType) {
        self.struct_types.insert(name, compiled);
    /// Register an interface type for compilation
    pub fn register_interface(&mut self, name: String, compiled: CompiledInterfaceType) {
        self.interface_types.insert(name, compiled);
    /// Get compiled struct type
    pub fn get_struct(&self, name: &str) -> Option<&CompiledStructType> {
        self.struct_types.get(name)
    /// Get compiled interface type
    pub fn get_interface(&self, name: &str) -> Option<&CompiledInterfaceType> {
        self.interface_types.get(name)
    /// Check if type exists
    pub fn has_type(&self, name: &str) -> bool {
        self.struct_types.contains_key(name) || self.interface_types.contains_key(name)
    /// Get all struct names
    pub fn struct_names(&self) -> Vec<String> {
        self.struct_types.keys().cloned().collect()
    /// Get all interface names  
    pub fn interface_names(&self) -> Vec<String> {
        self.interface_types.keys().cloned().collect()
    }
}

impl TypeCompilationContext {
    pub fn new(module_name: String) -> Self {
        Self {
        }
    }

    /// Create context with existing type system
    pub fn with_type_system(module_name: String, type_system: TypeSystem) -> Self {
        Self {
        }
    }

    /// Compile a generic type with constraint resolution
    pub fn compile_generic_type(
        constraints: &[GenericConstraint]
    ) -> crate::error::Result<()> {
        // Validate constraints using type system (simplified for now)
        for constraint in constraints {
            self.type_system.check_constraints(
            )?;
        // Compile constraints to LLVM representation
        let compiled_constraints = constraints.iter()
            .map(|c| self.compile_constraint(c))
            .collect::<Result<Vec<_>, _>>()?;

        // Generate LLVM template
        let llvm_template = self.generate_generic_template(name, type_parameters, &compiled_constraints)?;

        let compiled_generic = CompiledGenericType {

        self.generic_cache.insert(name.to_string(), compiled_generic.clone());
        Ok(compiled_generic)
    /// Instantiate a generic type with concrete type arguments
    pub fn instantiate_generic(
        type_args: &[String]
    ) -> crate::error::Result<()> {
        // Get the generic type
        let generic_type = self.generic_cache.get(base_name)
            .ok_or_else(|| CursedError::TypeCompilation(format!("Generic type '{}' not found", base_name)))?
            .clone();

        // Create type substitution map
        let substitutions: HashMap<String, String> = generic_type.type_parameters
            .iter()
            .zip(type_args.iter())
            .map(|(param, arg)| (param.clone(), arg.clone()))
            .collect();

        // Validate constraints with concrete types
        self.validate_generic_constraints(&generic_type, &substitutions)?;

        // Generate instance name
        let instance_name = format!("{}_{}", base_name, type_args.join("_"));

        // Check if already instantiated
        if let Some(existing) = generic_type.instantiations.get(&instance_name) {
            return Ok(existing.clone());
        // Perform type substitution in LLVM template
        let instantiated_llvm = self.substitute_types_in_template(
            &substitutions
        )?;

        let instance = CompiledGenericInstance {

        // Cache the instantiation
        if let Some(cached_generic) = self.generic_cache.get_mut(base_name) {
            cached_generic.instantiations.insert(instance_name, instance.clone());
        Ok(instance)
    /// Compile a struct declaration to LLVM types
    pub fn compile_struct(&mut self, squad: &SquadStatement) -> crate::error::Result<()> {
        let struct_name = squad.to_string().value.clone();
        
        // Check for circular dependencies
        if self.compilation_order.contains(&struct_name) {
            return Err(CursedError::TypeCompilation(format!(
                "Circular dependency detected for struct '{}'", struct_name
            )));
        self.compilation_order.push(struct_name.clone());

        // Compile fields
        let mut compiled_fields = Vec::new();
        let mut current_offset = 0;
        let mut max_alignment = 1;

        for field in &squad.fields {
            let compiled_field = self.compile_field(field)?;
            current_offset = align_offset(current_offset, compiled_field.size);
            
            let mut field_with_offset = compiled_field;
            field_with_offset.offset = current_offset;
            
            current_offset += field_with_offset.size;
            max_alignment = max_alignment.max(field_with_offset.size);
            
            compiled_fields.push(field_with_offset);
        // Calculate total size with padding
        let total_size = align_offset(current_offset, max_alignment);

        // Generate LLVM struct type
        let field_types: Vec<String> = compiled_fields.iter()
            .map(|f| f.llvm_type.clone())
            .collect();
        
            field_types.join(", ")
        );

        let compiled_struct = CompiledStructType {

        self.registry.register_struct(struct_name.clone(), compiled_struct.clone());
        self.compilation_order.pop();

        Ok(compiled_struct)
    /// Compile an interface declaration to LLVM types
    pub fn compile_interface(&mut self, collab: &CollabStatement) -> crate::error::Result<()> {
        let interface_name = collab.to_string().value.clone();

        // Compile methods
        let mut compiled_methods = Vec::new();
        for (index, method) in collab.methods.iter().enumerate() {
            let compiled_method = self.compile_method(method, index)?;
            compiled_methods.push(compiled_method);
        // Generate vtable type
        let method_types: Vec<String> = compiled_methods.iter()
            .map(|m| format!("{}*", m.llvm_function_type))
            .collect();
        
            method_types.join(", ")
        );

        // Generate type ID (simple hash for now)
        let type_id = calculate_type_id(&interface_name);

        let compiled_interface = CompiledInterfaceType {

        self.registry.register_interface(interface_name.clone(), compiled_interface.clone());

        Ok(compiled_interface)
    /// Compile a field declaration
    fn compile_field(&self, field: &FieldStatement) -> crate::error::Result<()> {
        let field_name = field.to_string().value.clone();
        let type_name = field.type_name.value.clone();
        
        let (llvm_type, size) = self.map_cursed_type_to_llvm(&type_name)?;

        Ok(CompiledField {
            offset: 0, // Will be set during struct compilation
        })
    /// Compile a method declaration
    fn compile_method(&self, method: &MethodDeclaration, vtable_index: usize) -> crate::error::Result<()> {
        let method_name = method.to_string().value.clone();
        
        // Build function signature
        let mut param_types = vec!["i8*".to_string()]; // self pointer
        
        for param in &method.parameters {
            let param_type = &param.param_type;
            if param_type.is_empty() {
                return Err(CursedError::TypeCompilation("Missing parameter type".to_string()));
            }
            let (llvm_type, _) = self.map_cursed_type_to_llvm(param_type)?;
            param_types.push(llvm_type);
        // Return type
        let return_type = if let Some(ret) = &method.return_type {
            let (llvm_type, _) = self.map_cursed_type_to_llvm(&ret.string())?;
            llvm_type
        } else {
            "void".to_string()

        let function_type = format!("{} ({})", return_type, param_types.join(", "));
            method.parameters.iter().map(|p| p.string()).collect::<Vec<_>>().join(", ")
        );

        Ok(CompiledMethod {
        })
    /// Map CURSED types to LLVM types
    fn map_cursed_type_to_llvm(&self, cursed_type: &str) -> crate::error::Result<()> {
        match cursed_type {
            // Primitive types
            "tea" => Ok(("i8*".to_string(), 8)), // String pointer
            "sus" => Ok(("i8*".to_string(), 8)), // Generic pointer
            
            // Check for registered types
            type_name if self.registry.has_type(type_name) => {
                if let Some(struct_type) = self.registry.get_struct(type_name) {
                    Ok((format!("%struct.{}*", type_name), 8)) // Pointer to struct
                } else if let Some(_interface_type) = self.registry.get_interface(type_name) {
                    Ok(("{i8*, i8*}".to_string(), 16)) // Interface value (data ptr + vtable ptr)
                } else {
                    Err(CursedError::TypeCompilation(format!("Unknown type: {}", type_name)))
                }
            
            // Array types
            type_name if type_name.starts_with('[') && type_name.ends_with(']') => {
                // Extract element type: [ElementType] -> ElementType
                let element_type = &type_name[1..type_name.len()-1];
                let (elem_llvm_type, elem_size) = self.map_cursed_type_to_llvm(element_type)?;
                Ok((format!("{{ i64, {}* }}", elem_llvm_type), 16)) // Length + data pointer
            
            // Map types
            type_name if type_name.starts_with("tea[") => {
                // tea[KeyType]ValueType
                Ok(("i8*".to_string(), 8)) // Map pointer (simplified)
            
            // Channel types
            type_name if type_name.starts_with("dm ") => {
                // dm Type -> channel type
                Ok(("i8*".to_string(), 8)) // Channel pointer
            
            _ => Err(CursedError::TypeCompilation(format!("Unsupported type: {}", cursed_type)))
        }
    }

    /// Generate LLVM IR for all compiled types
    pub fn generate_type_definitions(&self) -> String {
        let mut ir = String::new();
        
        ir.push_str("; Type definitions generated by CURSED compiler\n\n");

        // Generate struct type definitions
        for struct_type in self.registry.struct_types.values() {
            ir.push_str(&format!("{}\n", struct_type.llvm_type));
        // Generate interface vtable definitions
        for interface_type in self.registry.interface_types.values() {
            ir.push_str(&format!("{}\n", interface_type.vtable_type));
        ir.push_str("\n");
        ir
    /// Generate constructor functions for structs
    pub fn generate_struct_constructors(&self) -> String {
        let mut ir = String::new();
        
        for struct_type in self.registry.struct_types.values() {
            ir.push_str(&self.generate_struct_constructor(struct_type));
        ir
    /// Generate constructor for a specific struct
    fn generate_struct_constructor(&self, struct_type: &CompiledStructType) -> String {
        let mut ir = String::new();
        
        // Function signature: new_StructName(field1, field2, ...) -> %struct.StructName*
        let param_types: Vec<String> = struct_type.fields.iter()
            .map(|f| f.llvm_type.clone())
            .collect();
        
        ir.push_str(&format!(
            param_types.iter().enumerate()
                .map(|(i, t)| format!("{} %param{}", t, i))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        
        // Allocate memory
        ir.push_str(&format!(
            struct_type.size_bytes
        ));
        ir.push_str(&format!(
            struct_type.to_string()
        ));
        
        // Initialize fields
        for (i, field) in struct_type.fields.iter().enumerate() {
            ir.push_str(&format!(
                i, struct_type.to_string(), struct_type.to_string(), i
            ));
            ir.push_str(&format!(
                field.llvm_type, i, field.llvm_type, i
            ));
        ir.push_str(&format!("  ret %struct.{}* %struct_ptr\n", struct_type.to_string()));
        ir.push_str("}\n\n");
        
        ir
    /// Generate method dispatch functions for interfaces
    pub fn generate_interface_dispatch(&self) -> String {
        let mut ir = String::new();
        
        for interface_type in self.registry.interface_types.values() {
            for method in &interface_type.methods {
                ir.push_str(&self.generate_method_dispatch(interface_type, method));
            }
        }
        
        ir
    /// Generate dispatch function for a specific method
    fn generate_method_dispatch(&self, interface_type: &CompiledInterfaceType, method: &CompiledMethod) -> String {
        let mut ir = String::new();
        
        // Extract parameter types from method signature
        let param_start = method.llvm_function_type.find('(').unwrap_or(0) + 1;
        let param_end = method.llvm_function_type.rfind(')').unwrap_or(method.llvm_function_type.len());
        let params = &method.llvm_function_type[param_start..param_end];
        
        ir.push_str(&format!(
            params
        ));
        
        // Extract vtable and call method
        ir.push_str("  %interface_val = alloca {i8*, i8*}\n");
        ir.push_str("  %vtable_ptr = getelementptr inbounds {i8*, i8*}, {i8*, i8*}* %interface_val, i32 0, i32 1\n");
        ir.push_str("  %vtable = load i8*, i8** %vtable_ptr\n");
        ir.push_str(&format!(
            interface_type.to_string()
        ));
        ir.push_str(&format!(
            interface_type.to_string(), interface_type.to_string(), method.vtable_index
        ));
        ir.push_str(&format!(
            method.llvm_function_type, method.llvm_function_type
        ));
        
        // Call the method
        let param_names: Vec<String> = (0..params.split(',').count())
            .map(|i| format!("%param{}", i))
            .collect();
        
        ir.push_str(&format!(
            param_names.join(", ")
        ));
        
        if method.llvm_function_type.starts_with("void") {
            ir.push_str("  ret void\n");
        } else {
            let return_type = method.llvm_function_type.split('(').next().unwrap_or("void");
            ir.push_str(&format!("  ret {} %result\n", return_type));
        ir.push_str("}\n\n");
        
        ir
    /// Get compilation errors
    pub fn get_errors(&self) -> &[String] {
        &self.errors
    /// Check if compilation was successful
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    /// Get the type registry
    pub fn registry(&self) -> &LlvmTypeRegistry {
        &self.registry
    /// Compile a generic constraint to LLVM representation
    fn compile_constraint(&self, constraint: &GenericConstraint) -> crate::error::Result<()> {
        let param_name = constraint.type_parameters.first()
            .unwrap_or(&"T".to_string()).clone();
        let constraint_type = constraint.constraint_name.clone();
        
        // Extract required methods from constraint
        let constraint_methods = match constraint.constraint_name.as_str() {
            _ => vec![], // Interface constraints would have their methods

        Ok(CompiledConstraint {
        })
    /// Generate LLVM template for generic type
    fn generate_generic_template(
        constraints: &[CompiledConstraint]
    ) -> crate::error::Result<()> {
        let mut template = String::new();
        
        // Generate parameterized struct definition
        template.push_str(&format!("; Generic type template for {}\n", name));
        template.push_str(&format!("%struct.{}_template = type {{ ", name));
        
        // Add placeholder fields for type parameters
        let param_placeholders: Vec<String> = type_parameters.iter()
            .map(|param| format!("%TYPE_{}", param.to_uppercase()))
            .collect();
        
        template.push_str(&param_placeholders.join(", "));
        template.push_str(" }\n\n");
        
        // Generate constraint checking functions
        for constraint in constraints {
            template.push_str(&self.generate_constraint_check_template(constraint)?);
        Ok(template)
    /// Generate constraint checking template
    fn generate_constraint_check_template(&self, constraint: &CompiledConstraint) -> crate::error::Result<()> {
        let mut template = String::new();
        
        template.push_str(&format!(
            constraint.param_name, constraint.constraint_type
        ));
        
        for method in &constraint.constraint_methods {
            template.push_str(&format!(
                constraint.param_name.to_uppercase()
            ));
        template.push_str("\n");
        Ok(template)
    /// Validate generic constraints with concrete types
    fn validate_generic_constraints(
        substitutions: &HashMap<String, String>
    ) -> crate::error::Result<()> {
        for constraint in &generic_type.constraints {
            let concrete_type = substitutions.get(&constraint.param_name)
                .ok_or_else(|| CursedError::TypeCompilation(format!(
                    "Missing concrete type for parameter '{}'", constraint.param_name
                )))?;
            
            // Check if concrete type satisfies constraint
            match constraint.constraint_type.as_str() {
                "Comparable" => {
                    if !self.type_implements_comparable(concrete_type) {
                        return Err(CursedError::TypeCompilation(format!(
                            "Type '{}' does not implement Comparable constraint", concrete_type
                        )));
                    }
                "Hashable" => {
                    if !self.type_implements_hashable(concrete_type) {
                        return Err(CursedError::TypeCompilation(format!(
                            "Type '{}' does not implement Hashable constraint", concrete_type
                        )));
                    }
                "Numeric" => {
                    if !self.type_implements_numeric(concrete_type) {
                        return Err(CursedError::TypeCompilation(format!(
                            "Type '{}' does not implement Numeric constraint", concrete_type
                        )));
                    }
                _ => {
                    // Check interface constraint
                    if !self.registry.has_type(&constraint.constraint_type) {
                        return Err(CursedError::TypeCompilation(format!(
                            "Unknown constraint type: {}", constraint.constraint_type
                        )));
                    }
                }
            }
        }
        
        Ok(())
    /// Substitute types in LLVM template
    fn substitute_types_in_template(
        substitutions: &HashMap<String, String>
    ) -> crate::error::Result<()> {
        let mut result = template.to_string();
        
        for (param, concrete_type) in substitutions {
            let placeholder = format!("%TYPE_{}", param.to_uppercase());
            let (llvm_type, _) = self.map_cursed_type_to_llvm(concrete_type)?;
            result = result.replace(&placeholder, &llvm_type);
        Ok(result)
    /// Check if type implements Comparable
    fn type_implements_comparable(&self, type_name: &str) -> bool {
        matches!(type_name, "normie" | "tea" | "facts")
    /// Check if type implements Hashable
    fn type_implements_hashable(&self, type_name: &str) -> bool {
        matches!(type_name, "normie" | "tea" | "facts")
    /// Check if type implements Numeric
    fn type_implements_numeric(&self, type_name: &str) -> bool {
        matches!(type_name, "normie")
    /// Get access to type system for external use
    pub fn type_system(&self) -> &TypeSystem {
        &self.type_system
    /// Get mutable access to type system
    pub fn type_system_mut(&mut self) -> &mut TypeSystem {
        &mut self.type_system
    }
}

/// Helper function to align offset to specified alignment
fn align_offset(offset: usize, alignment: usize) -> usize {
    (offset + alignment - 1) & !(alignment - 1)
/// Calculate a simple hash-based type ID for interfaces
fn calculate_type_id(name: &str) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    name.hash(&mut hasher);
    hasher.finish()
/// Type casting operations
pub struct TypeCastingOperations;

impl TypeCastingOperations {
    /// Generate type assertion IR
    pub fn generate_type_assertion(
    ) -> crate::error::Result<()> {
        let mut ir = String::new();
        
        // Extract type ID from interface value
        ir.push_str(&format!(
            interface_val
        ));
        ir.push_str("  %vtable = load i8*, i8** %type_id_ptr\n");
        
        // Get expected type ID
        if let Some(interface_type) = registry.get_interface(target_type) {
            ir.push_str(&format!(
                interface_type.type_id
            ));
        } else if let Some(_struct_type) = registry.get_struct(target_type) {
            let type_id = calculate_type_id(target_type);
            ir.push_str(&format!(
                type_id
            ));
        } else {
            return Err(CursedError::TypeCompilation(format!("Unknown type for assertion: {}", target_type)));
        // Compare type IDs and branch
        ir.push_str("  %vtable_id = ptrtoint i8* %vtable to i64\n");
        ir.push_str("  %type_match = icmp eq i64 %vtable_id, %expected_id\n");
        ir.push_str("  br i1 %type_match, label %success, label %failure\n\n");
        
        ir.push_str("success:\n");
        ir.push_str("  ; Type assertion succeeded\n");
        ir.push_str("  br label %end\n\n");
        
        ir.push_str("failure:\n");
        ir.push_str("  ; Type assertion failed\n");
        ir.push_str("  br label %end\n\n");
        
        ir.push_str("end:\n");
        
        Ok(ir)
    /// Generate type conversion IR
    pub fn generate_type_conversion(
    ) -> crate::error::Result<()> {
        let mut ir = String::new();
        
        match (from_type, to_type) {
            // Primitive conversions
            ("normie", "tea") => {
                ir.push_str(&format!("  %str_ptr = call i8* @int_to_string(i64 {})\n", value));
            ("facts", "normie") => {
                ir.push_str(&format!("  %int_val = zext i1 {} to i64\n", value));
            
            // Struct to interface conversion
            (from, to) if registry.get_struct(from).is_some() && registry.get_interface(to).is_some() => {
                ir.push_str(&format!(
                    to, from, value
                ));
            
            _ => {
                return Err(CursedError::TypeCompilation(format!(
                    "Unsupported type conversion from {} to {}", from_type, to_type
                )));
            }
        }
        
        Ok(ir)
    }
}

/// Generic type handling
pub struct GenericTypeHandler;

impl GenericTypeHandler {
    /// Generate generic type instantiation
    pub fn instantiate_generic(
    ) -> crate::error::Result<()> {
        let instance_name = format!("{}_{}", base_type, type_args.join("_"));
        
        // Check if already instantiated
        if registry.has_type(&instance_name) {
            return Ok(instance_name);
        // Create new generic instance
        let instance = CompiledGenericInstance {
        
        registry.generic_instances
            .entry(base_type.to_string())
            .or_insert_with(Vec::new)
            .push(instance);
        
        Ok(instance_name)
    }
}

