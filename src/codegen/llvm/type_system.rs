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
use crate::error::Error;
use crate::type_system::{
    TypeSystem, TypeEnvironment, GenericInstantiator,
    TypeInference, TypeSubstitution, TypeExpression as TypeSystemExpression
    // Disabled imports for simplified AST compatibility:
    // ConstraintResolver, ConstraintSolution (require expanded constraint system)
};
use std::collections::HashMap;
use std::collections::HashSet;

/// LLVM Type Registry for managing compiled types
#[derive(Debug)]
pub struct LlvmTypeRegistry {
    /// Compiled struct types with LLVM representations
    struct_types: HashMap<String, CompiledStructType>,
    /// Compiled interface types with method tables
    interface_types: HashMap<String, CompiledInterfaceType>,
    /// Type dependencies for proper compilation order
    dependencies: HashMap<String, HashSet<String>>,
    /// Generic type instantiations
    generic_instances: HashMap<String, Vec<CompiledGenericInstance>>,
}

/// Compiled struct type with LLVM metadata
#[derive(Debug, Clone)]
pub struct CompiledStructType {
    pub name: String,
    pub llvm_type: String, // LLVM struct type representation
    pub fields: Vec<CompiledField>,
    pub size_bytes: usize,
    pub alignment: usize,
    pub is_generic: bool,
    pub generic_params: Vec<String>,
}

/// Compiled field with type information
#[derive(Debug, Clone)]
pub struct CompiledField {
    pub name: String,
    pub field_type: String,
    pub llvm_type: String,
    pub offset: usize,
    pub size: usize,
}

/// Compiled interface type with method dispatch table
#[derive(Debug, Clone)]
pub struct CompiledInterfaceType {
    pub name: String,
    pub methods: Vec<CompiledMethod>,
    pub vtable_type: String, // LLVM vtable type
    pub type_id: u64, // Runtime type identification
    pub is_generic: bool,
}

/// Compiled method with dispatch information
#[derive(Debug, Clone)]
pub struct CompiledMethod {
    pub name: String,
    pub signature: String,
    pub llvm_function_type: String,
    pub vtable_index: usize,
}

/// Generic type instance with concrete parameters
#[derive(Debug, Clone)]
pub struct CompiledGenericInstance {
    pub base_name: String,
    pub concrete_types: Vec<String>,
    pub instance_name: String,
    pub compiled_type: String,
}

/// Compiled generic type with constraint information
#[derive(Debug, Clone)]
pub struct CompiledGenericType {
    pub name: String,
    pub type_parameters: Vec<String>,
    pub constraints: Vec<CompiledConstraint>,
    pub instantiations: HashMap<String, CompiledGenericInstance>,
    pub llvm_template: String,
}

/// Compiled type constraint
#[derive(Debug, Clone)]
pub struct CompiledConstraint {
    pub param_name: String,
    pub constraint_type: String,
    pub constraint_methods: Vec<String>,
}

/// Type compilation context for managing state
#[derive(Debug)]
pub struct TypeCompilationContext {
    registry: LlvmTypeRegistry,
    current_module: String,
    compilation_order: Vec<String>,
    errors: Vec<String>,
    /// Integrated type system for constraint resolution and inference
    type_system: TypeSystem,
    /// Generic type compilation cache
    generic_cache: HashMap<String, CompiledGenericType>,
}

impl LlvmTypeRegistry {
    pub fn new() -> Self {
        Self {
            struct_types: HashMap::new(),
            interface_types: HashMap::new(),
            dependencies: HashMap::new(),
            generic_instances: HashMap::new(),
        }
    }

    /// Register a struct type for compilation
    pub fn register_struct(&mut self, name: String, compiled: CompiledStructType) {
        self.struct_types.insert(name, compiled);
    }

    /// Register an interface type for compilation
    pub fn register_interface(&mut self, name: String, compiled: CompiledInterfaceType) {
        self.interface_types.insert(name, compiled);
    }

    /// Get compiled struct type
    pub fn get_struct(&self, name: &str) -> Option<&CompiledStructType> {
        self.struct_types.get(name)
    }

    /// Get compiled interface type
    pub fn get_interface(&self, name: &str) -> Option<&CompiledInterfaceType> {
        self.interface_types.get(name)
    }

    /// Check if type exists
    pub fn has_type(&self, name: &str) -> bool {
        self.struct_types.contains_key(name) || self.interface_types.contains_key(name)
    }

    /// Get all struct names
    pub fn struct_names(&self) -> Vec<String> {
        self.struct_types.keys().cloned().collect()
    }

    /// Get all interface names  
    pub fn interface_names(&self) -> Vec<String> {
        self.interface_types.keys().cloned().collect()
    }
}

impl TypeCompilationContext {
    pub fn new(module_name: String) -> Self {
        Self {
            registry: LlvmTypeRegistry::new(),
            current_module: module_name,
            compilation_order: Vec::new(),
            errors: Vec::new(),
            type_system: TypeSystem::new(),
            generic_cache: HashMap::new(),
        }
    }

    /// Create context with existing type system
    pub fn with_type_system(module_name: String, type_system: TypeSystem) -> Self {
        Self {
            registry: LlvmTypeRegistry::new(),
            current_module: module_name,
            compilation_order: Vec::new(),
            errors: Vec::new(),
            type_system,
            generic_cache: HashMap::new(),
        }
    }

    /// Compile a generic type with constraint resolution
    pub fn compile_generic_type(
        &mut self, 
        name: &str,
        type_parameters: &[String],
        constraints: &[GenericConstraint]
    ) -> Result<CompiledGenericType, Error> {
        // Validate constraints using type system (simplified for now)
        for constraint in constraints {
            self.type_system.check_constraints(
                &TypeSystemExpression::Named(constraint.constraint_name.clone()),
                constraints,
            )?;
        }

        // Compile constraints to LLVM representation
        let compiled_constraints = constraints.iter()
            .map(|c| self.compile_constraint(c))
            .collect::<Result<Vec<_>, _>>()?;

        // Generate LLVM template
        let llvm_template = self.generate_generic_template(name, type_parameters, &compiled_constraints)?;

        let compiled_generic = CompiledGenericType {
            name: name.to_string(),
            type_parameters: type_parameters.to_vec(),
            constraints: compiled_constraints,
            instantiations: HashMap::new(),
            llvm_template,
        };

        self.generic_cache.insert(name.to_string(), compiled_generic.clone());
        Ok(compiled_generic)
    }

    /// Instantiate a generic type with concrete type arguments
    pub fn instantiate_generic(
        &mut self,
        base_name: &str,
        type_args: &[String]
    ) -> Result<CompiledGenericInstance, Error> {
        // Get the generic type
        let generic_type = self.generic_cache.get(base_name)
            .ok_or_else(|| Error::TypeCompilation(format!("Generic type '{}' not found", base_name)))?
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
        }

        // Perform type substitution in LLVM template
        let instantiated_llvm = self.substitute_types_in_template(
            &generic_type.llvm_template,
            &substitutions
        )?;

        let instance = CompiledGenericInstance {
            base_name: base_name.to_string(),
            concrete_types: type_args.to_vec(),
            instance_name: instance_name.clone(),
            compiled_type: instantiated_llvm,
        };

        // Cache the instantiation
        if let Some(cached_generic) = self.generic_cache.get_mut(base_name) {
            cached_generic.instantiations.insert(instance_name, instance.clone());
        }

        Ok(instance)
    }

    /// Compile a struct declaration to LLVM types
    pub fn compile_struct(&mut self, squad: &SquadStatement) -> Result<CompiledStructType, Error> {
        let struct_name = squad.name.value.clone();
        
        // Check for circular dependencies
        if self.compilation_order.contains(&struct_name) {
            return Err(Error::TypeCompilation(format!(
                "Circular dependency detected for struct '{}'", struct_name
            )));
        }

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
        }

        // Calculate total size with padding
        let total_size = align_offset(current_offset, max_alignment);

        // Generate LLVM struct type
        let field_types: Vec<String> = compiled_fields.iter()
            .map(|f| f.llvm_type.clone())
            .collect();
        
        let llvm_type = format!("%struct.{} = type {{ {} }}", 
            struct_name, 
            field_types.join(", ")
        );

        let compiled_struct = CompiledStructType {
            name: struct_name.clone(),
            llvm_type,
            fields: compiled_fields,
            size_bytes: total_size,
            alignment: max_alignment,
            is_generic: !squad.type_parameters.is_empty(),
            generic_params: squad.type_parameters.iter().map(|p| p.name.clone()).collect(),
        };

        self.registry.register_struct(struct_name.clone(), compiled_struct.clone());
        self.compilation_order.pop();

        Ok(compiled_struct)
    }

    /// Compile an interface declaration to LLVM types
    pub fn compile_interface(&mut self, collab: &CollabStatement) -> Result<CompiledInterfaceType, Error> {
        let interface_name = collab.name.value.clone();

        // Compile methods
        let mut compiled_methods = Vec::new();
        for (index, method) in collab.methods.iter().enumerate() {
            let compiled_method = self.compile_method(method, index)?;
            compiled_methods.push(compiled_method);
        }

        // Generate vtable type
        let method_types: Vec<String> = compiled_methods.iter()
            .map(|m| format!("{}*", m.llvm_function_type))
            .collect();
        
        let vtable_type = format!("%vtable.{} = type {{ {} }}", 
            interface_name, 
            method_types.join(", ")
        );

        // Generate type ID (simple hash for now)
        let type_id = calculate_type_id(&interface_name);

        let compiled_interface = CompiledInterfaceType {
            name: interface_name.clone(),
            methods: compiled_methods,
            vtable_type,
            type_id,
            is_generic: !collab.type_parameters.is_empty(),
        };

        self.registry.register_interface(interface_name.clone(), compiled_interface.clone());

        Ok(compiled_interface)
    }

    /// Compile a field declaration
    fn compile_field(&self, field: &FieldStatement) -> Result<CompiledField, Error> {
        let field_name = field.name.value.clone();
        let type_name = field.type_name.value.clone();
        
        let (llvm_type, size) = self.map_cursed_type_to_llvm(&type_name)?;

        Ok(CompiledField {
            name: field_name,
            field_type: type_name,
            llvm_type,
            offset: 0, // Will be set during struct compilation
            size,
        })
    }

    /// Compile a method declaration
    fn compile_method(&self, method: &MethodDeclaration, vtable_index: usize) -> Result<CompiledMethod, Error> {
        let method_name = method.name.value.clone();
        
        // Build function signature
        let mut param_types = vec!["i8*".to_string()]; // self pointer
        
        for param in &method.parameters {
            let param_type = &param.param_type;
            if param_type.is_empty() {
                return Err(Error::TypeCompilation("Missing parameter type".to_string()));
            }
            let (llvm_type, _) = self.map_cursed_type_to_llvm(param_type)?;
            param_types.push(llvm_type);
        }

        // Return type
        let return_type = if let Some(ret) = &method.return_type {
            let (llvm_type, _) = self.map_cursed_type_to_llvm(&ret.string())?;
            llvm_type
        } else {
            "void".to_string()
        };

        let function_type = format!("{} ({})", return_type, param_types.join(", "));
        let signature = format!("{}({})", method_name, 
            method.parameters.iter().map(|p| p.string()).collect::<Vec<_>>().join(", ")
        );

        Ok(CompiledMethod {
            name: method_name,
            signature,
            llvm_function_type: function_type,
            vtable_index,
        })
    }

    /// Map CURSED types to LLVM types
    fn map_cursed_type_to_llvm(&self, cursed_type: &str) -> Result<(String, usize), Error> {
        match cursed_type {
            // Primitive types
            "normie" => Ok(("i64".to_string(), 8)),
            "facts" => Ok(("i1".to_string(), 1)),
            "tea" => Ok(("i8*".to_string(), 8)), // String pointer
            "sus" => Ok(("i8*".to_string(), 8)), // Generic pointer
            
            // Check for registered types
            type_name if self.registry.has_type(type_name) => {
                if let Some(struct_type) = self.registry.get_struct(type_name) {
                    Ok((format!("%struct.{}*", type_name), 8)) // Pointer to struct
                } else if let Some(_interface_type) = self.registry.get_interface(type_name) {
                    Ok(("{i8*, i8*}".to_string(), 16)) // Interface value (data ptr + vtable ptr)
                } else {
                    Err(Error::TypeCompilation(format!("Unknown type: {}", type_name)))
                }
            },
            
            // Array types
            type_name if type_name.starts_with('[') && type_name.ends_with(']') => {
                // Extract element type: [ElementType] -> ElementType
                let element_type = &type_name[1..type_name.len()-1];
                let (elem_llvm_type, elem_size) = self.map_cursed_type_to_llvm(element_type)?;
                Ok((format!("{{ i64, {}* }}", elem_llvm_type), 16)) // Length + data pointer
            },
            
            // Map types
            type_name if type_name.starts_with("tea[") => {
                // tea[KeyType]ValueType
                Ok(("i8*".to_string(), 8)) // Map pointer (simplified)
            },
            
            // Channel types
            type_name if type_name.starts_with("dm ") => {
                // dm Type -> channel type
                Ok(("i8*".to_string(), 8)) // Channel pointer
            },
            
            _ => Err(Error::TypeCompilation(format!("Unsupported type: {}", cursed_type)))
        }
    }

    /// Generate LLVM IR for all compiled types
    pub fn generate_type_definitions(&self) -> String {
        let mut ir = String::new();
        
        ir.push_str("; Type definitions generated by CURSED compiler\n\n");

        // Generate struct type definitions
        for struct_type in self.registry.struct_types.values() {
            ir.push_str(&format!("{}\n", struct_type.llvm_type));
        }

        // Generate interface vtable definitions
        for interface_type in self.registry.interface_types.values() {
            ir.push_str(&format!("{}\n", interface_type.vtable_type));
        }

        ir.push_str("\n");
        ir
    }

    /// Generate constructor functions for structs
    pub fn generate_struct_constructors(&self) -> String {
        let mut ir = String::new();
        
        for struct_type in self.registry.struct_types.values() {
            ir.push_str(&self.generate_struct_constructor(struct_type));
        }
        
        ir
    }

    /// Generate constructor for a specific struct
    fn generate_struct_constructor(&self, struct_type: &CompiledStructType) -> String {
        let mut ir = String::new();
        
        // Function signature: new_StructName(field1, field2, ...) -> %struct.StructName*
        let param_types: Vec<String> = struct_type.fields.iter()
            .map(|f| f.llvm_type.clone())
            .collect();
        
        ir.push_str(&format!(
            "define %struct.{}* @new_{}({}) {{\n",
            struct_type.name,
            struct_type.name,
            param_types.iter().enumerate()
                .map(|(i, t)| format!("{} %param{}", t, i))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        
        // Allocate memory
        ir.push_str(&format!(
            "  %ptr = call i8* @malloc(i64 {})\n",
            struct_type.size_bytes
        ));
        ir.push_str(&format!(
            "  %struct_ptr = bitcast i8* %ptr to %struct.{}*\n",
            struct_type.name
        ));
        
        // Initialize fields
        for (i, field) in struct_type.fields.iter().enumerate() {
            ir.push_str(&format!(
                "  %field_ptr{} = getelementptr inbounds %struct.{}, %struct.{}* %struct_ptr, i32 0, i32 {}\n",
                i, struct_type.name, struct_type.name, i
            ));
            ir.push_str(&format!(
                "  store {} %param{}, {}* %field_ptr{}\n",
                field.llvm_type, i, field.llvm_type, i
            ));
        }
        
        ir.push_str(&format!("  ret %struct.{}* %struct_ptr\n", struct_type.name));
        ir.push_str("}\n\n");
        
        ir
    }

    /// Generate method dispatch functions for interfaces
    pub fn generate_interface_dispatch(&self) -> String {
        let mut ir = String::new();
        
        for interface_type in self.registry.interface_types.values() {
            for method in &interface_type.methods {
                ir.push_str(&self.generate_method_dispatch(interface_type, method));
            }
        }
        
        ir
    }

    /// Generate dispatch function for a specific method
    fn generate_method_dispatch(&self, interface_type: &CompiledInterfaceType, method: &CompiledMethod) -> String {
        let mut ir = String::new();
        
        // Extract parameter types from method signature
        let param_start = method.llvm_function_type.find('(').unwrap_or(0) + 1;
        let param_end = method.llvm_function_type.rfind(')').unwrap_or(method.llvm_function_type.len());
        let params = &method.llvm_function_type[param_start..param_end];
        
        ir.push_str(&format!(
            "define {} @{}_{}({}) {{\n",
            method.llvm_function_type.split('(').next().unwrap_or("void"),
            interface_type.name,
            method.name,
            params
        ));
        
        // Extract vtable and call method
        ir.push_str("  %interface_val = alloca {i8*, i8*}\n");
        ir.push_str("  %vtable_ptr = getelementptr inbounds {i8*, i8*}, {i8*, i8*}* %interface_val, i32 0, i32 1\n");
        ir.push_str("  %vtable = load i8*, i8** %vtable_ptr\n");
        ir.push_str(&format!(
            "  %vtable_typed = bitcast i8* %vtable to %vtable.{}*\n",
            interface_type.name
        ));
        ir.push_str(&format!(
            "  %method_ptr = getelementptr inbounds %vtable.{}, %vtable.{}* %vtable_typed, i32 0, i32 {}\n",
            interface_type.name, interface_type.name, method.vtable_index
        ));
        ir.push_str(&format!(
            "  %method = load {}*, {}** %method_ptr\n",
            method.llvm_function_type, method.llvm_function_type
        ));
        
        // Call the method
        let param_names: Vec<String> = (0..params.split(',').count())
            .map(|i| format!("%param{}", i))
            .collect();
        
        ir.push_str(&format!(
            "  %result = call {} %method({})\n",
            method.llvm_function_type.split('(').next().unwrap_or("void"),
            param_names.join(", ")
        ));
        
        if method.llvm_function_type.starts_with("void") {
            ir.push_str("  ret void\n");
        } else {
            let return_type = method.llvm_function_type.split('(').next().unwrap_or("void");
            ir.push_str(&format!("  ret {} %result\n", return_type));
        }
        
        ir.push_str("}\n\n");
        
        ir
    }

    /// Get compilation errors
    pub fn get_errors(&self) -> &[String] {
        &self.errors
    }

    /// Check if compilation was successful
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Get the type registry
    pub fn registry(&self) -> &LlvmTypeRegistry {
        &self.registry
    }

    /// Compile a generic constraint to LLVM representation
    fn compile_constraint(&self, constraint: &GenericConstraint) -> Result<CompiledConstraint, Error> {
        let param_name = constraint.type_parameters.first()
            .unwrap_or(&"T".to_string()).clone();
        let constraint_type = constraint.constraint_name.clone();
        
        // Extract required methods from constraint
        let constraint_methods = match constraint.constraint_name.as_str() {
            "Comparable" => vec!["compare".to_string(), "equals".to_string()],
            "Hashable" => vec!["hash".to_string()],
            "Printable" => vec!["to_string".to_string()],
            "Numeric" => vec!["add".to_string(), "subtract".to_string(), "multiply".to_string()],
            _ => vec![], // Interface constraints would have their methods
        };

        Ok(CompiledConstraint {
            param_name,
            constraint_type,
            constraint_methods,
        })
    }

    /// Generate LLVM template for generic type
    fn generate_generic_template(
        &self,
        name: &str,
        type_parameters: &[String],
        constraints: &[CompiledConstraint]
    ) -> Result<String, Error> {
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
        }
        
        Ok(template)
    }

    /// Generate constraint checking template
    fn generate_constraint_check_template(&self, constraint: &CompiledConstraint) -> Result<String, Error> {
        let mut template = String::new();
        
        template.push_str(&format!(
            "; Constraint check template for {} : {}\n",
            constraint.param_name, constraint.constraint_type
        ));
        
        for method in &constraint.constraint_methods {
            template.push_str(&format!(
                "declare %TYPE_{}* @{}_{}_%TYPE_{}(%TYPE_{}*)\n",
                constraint.param_name.to_uppercase(),
                constraint.constraint_type.to_lowercase(),
                method,
                constraint.param_name.to_uppercase(),
                constraint.param_name.to_uppercase()
            ));
        }
        
        template.push_str("\n");
        Ok(template)
    }

    /// Validate generic constraints with concrete types
    fn validate_generic_constraints(
        &self,
        generic_type: &CompiledGenericType,
        substitutions: &HashMap<String, String>
    ) -> Result<(), Error> {
        for constraint in &generic_type.constraints {
            let concrete_type = substitutions.get(&constraint.param_name)
                .ok_or_else(|| Error::TypeCompilation(format!(
                    "Missing concrete type for parameter '{}'", constraint.param_name
                )))?;
            
            // Check if concrete type satisfies constraint
            match constraint.constraint_type.as_str() {
                "Comparable" => {
                    if !self.type_implements_comparable(concrete_type) {
                        return Err(Error::TypeCompilation(format!(
                            "Type '{}' does not implement Comparable constraint", concrete_type
                        )));
                    }
                },
                "Hashable" => {
                    if !self.type_implements_hashable(concrete_type) {
                        return Err(Error::TypeCompilation(format!(
                            "Type '{}' does not implement Hashable constraint", concrete_type
                        )));
                    }
                },
                "Numeric" => {
                    if !self.type_implements_numeric(concrete_type) {
                        return Err(Error::TypeCompilation(format!(
                            "Type '{}' does not implement Numeric constraint", concrete_type
                        )));
                    }
                },
                _ => {
                    // Check interface constraint
                    if !self.registry.has_type(&constraint.constraint_type) {
                        return Err(Error::TypeCompilation(format!(
                            "Unknown constraint type: {}", constraint.constraint_type
                        )));
                    }
                }
            }
        }
        
        Ok(())
    }

    /// Substitute types in LLVM template
    fn substitute_types_in_template(
        &self,
        template: &str,
        substitutions: &HashMap<String, String>
    ) -> Result<String, Error> {
        let mut result = template.to_string();
        
        for (param, concrete_type) in substitutions {
            let placeholder = format!("%TYPE_{}", param.to_uppercase());
            let (llvm_type, _) = self.map_cursed_type_to_llvm(concrete_type)?;
            result = result.replace(&placeholder, &llvm_type);
        }
        
        Ok(result)
    }

    /// Check if type implements Comparable
    fn type_implements_comparable(&self, type_name: &str) -> bool {
        matches!(type_name, "normie" | "tea" | "facts")
    }

    /// Check if type implements Hashable
    fn type_implements_hashable(&self, type_name: &str) -> bool {
        matches!(type_name, "normie" | "tea" | "facts")
    }

    /// Check if type implements Numeric
    fn type_implements_numeric(&self, type_name: &str) -> bool {
        matches!(type_name, "normie")
    }

    /// Get access to type system for external use
    pub fn type_system(&self) -> &TypeSystem {
        &self.type_system
    }

    /// Get mutable access to type system
    pub fn type_system_mut(&mut self) -> &mut TypeSystem {
        &mut self.type_system
    }
}

/// Helper function to align offset to specified alignment
fn align_offset(offset: usize, alignment: usize) -> usize {
    (offset + alignment - 1) & !(alignment - 1)
}

/// Calculate a simple hash-based type ID for interfaces
fn calculate_type_id(name: &str) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    name.hash(&mut hasher);
    hasher.finish()
}

/// Type casting operations
pub struct TypeCastingOperations;

impl TypeCastingOperations {
    /// Generate type assertion IR
    pub fn generate_type_assertion(
        interface_val: &str,
        target_type: &str,
        registry: &LlvmTypeRegistry,
    ) -> Result<String, Error> {
        let mut ir = String::new();
        
        // Extract type ID from interface value
        ir.push_str(&format!(
            "  %type_id_ptr = getelementptr inbounds {{i8*, i8*}}, {{i8*, i8*}}* {}, i32 0, i32 1\n",
            interface_val
        ));
        ir.push_str("  %vtable = load i8*, i8** %type_id_ptr\n");
        
        // Get expected type ID
        if let Some(interface_type) = registry.get_interface(target_type) {
            ir.push_str(&format!(
                "  %expected_id = add i64 0, {}\n",
                interface_type.type_id
            ));
        } else if let Some(_struct_type) = registry.get_struct(target_type) {
            let type_id = calculate_type_id(target_type);
            ir.push_str(&format!(
                "  %expected_id = add i64 0, {}\n",
                type_id
            ));
        } else {
            return Err(Error::TypeCompilation(format!("Unknown type for assertion: {}", target_type)));
        }
        
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
    }

    /// Generate type conversion IR
    pub fn generate_type_conversion(
        from_type: &str,
        to_type: &str,
        value: &str,
        registry: &LlvmTypeRegistry,
    ) -> Result<String, Error> {
        let mut ir = String::new();
        
        match (from_type, to_type) {
            // Primitive conversions
            ("normie", "tea") => {
                ir.push_str(&format!("  %str_ptr = call i8* @int_to_string(i64 {})\n", value));
            },
            ("facts", "normie") => {
                ir.push_str(&format!("  %int_val = zext i1 {} to i64\n", value));
            },
            
            // Struct to interface conversion
            (from, to) if registry.get_struct(from).is_some() && registry.get_interface(to).is_some() => {
                ir.push_str(&format!(
                    "  %interface_val = call {{i8*, i8*}} @convert_to_interface_%{}({}* {})\n",
                    to, from, value
                ));
            },
            
            _ => {
                return Err(Error::TypeCompilation(format!(
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
        base_type: &str,
        type_args: &[String],
        registry: &mut LlvmTypeRegistry,
    ) -> Result<String, Error> {
        let instance_name = format!("{}_{}", base_type, type_args.join("_"));
        
        // Check if already instantiated
        if registry.has_type(&instance_name) {
            return Ok(instance_name);
        }
        
        // Create new generic instance
        let instance = CompiledGenericInstance {
            base_name: base_type.to_string(),
            concrete_types: type_args.to_vec(),
            instance_name: instance_name.clone(),
            compiled_type: format!("struct.{}", instance_name),
        };
        
        registry.generic_instances
            .entry(base_type.to_string())
            .or_insert_with(Vec::new)
            .push(instance);
        
        Ok(instance_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::identifiers::Identifier;

    #[test]
    fn test_struct_compilation() {
        let mut context = TypeCompilationContext::new("test_module".to_string());
        
        // Create a simple struct
        let fields = vec![
            FieldStatement::new(
                "normie".to_string(),
                Identifier::new("normie".to_string(), "age".to_string()),
                Identifier::new("normie".to_string(), "normie".to_string()),
            ),
            FieldStatement::new(
                "tea".to_string(),
                Identifier::new("tea".to_string(), "name".to_string()),
                Identifier::new("tea".to_string(), "tea".to_string()),
            ),
        ];
        
        let squad = SquadStatement::new(
            "squad".to_string(),
            Identifier::new("Person".to_string(), "Person".to_string()),
            fields,
        );
        
        let result = context.compile_struct(&squad);
        assert!(result.is_ok());
        
        let compiled = result.unwrap();
        assert_eq!(compiled.name, "Person");
        assert_eq!(compiled.fields.len(), 2);
        assert!(compiled.llvm_type.contains("struct.Person"));
    }

    #[test]
    fn test_interface_compilation() {
        let mut context = TypeCompilationContext::new("test_module".to_string());
        
        // Create a simple interface
        let methods = vec![];
        
        let collab = CollabStatement::new(
            "collab".to_string(),
            Identifier::new("Drawable".to_string(), "Drawable".to_string()),
            methods,
        );
        
        let result = context.compile_interface(&collab);
        assert!(result.is_ok());
        
        let compiled = result.unwrap();
        assert_eq!(compiled.name, "Drawable");
        assert!(compiled.vtable_type.contains("vtable.Drawable"));
    }

    #[test]
    fn test_type_registry() {
        let mut registry = LlvmTypeRegistry::new();
        
        let struct_type = CompiledStructType {
            name: "TestStruct".to_string(),
            llvm_type: "%struct.TestStruct = type { i64 }".to_string(),
            fields: vec![],
            size_bytes: 8,
            alignment: 8,
            is_generic: false,
            generic_params: vec![],
        };
        
        registry.register_struct("TestStruct".to_string(), struct_type);
        
        assert!(registry.has_type("TestStruct"));
        assert!(registry.get_struct("TestStruct").is_some());
    }

    #[test]
    fn test_cursed_type_mapping() {
        let context = TypeCompilationContext::new("test".to_string());
        
        let (llvm_type, size) = context.map_cursed_type_to_llvm("normie").unwrap();
        assert_eq!(llvm_type, "i64");
        assert_eq!(size, 8);
        
        let (llvm_type, size) = context.map_cursed_type_to_llvm("facts").unwrap();
        assert_eq!(llvm_type, "i1");
        assert_eq!(size, 1);
    }
}
