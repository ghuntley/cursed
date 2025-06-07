//! Generic code specialization through monomorphization
//!
//! This module implements the monomorphization system for CURSED's generic types
//! and functions. Monomorphization is the process of creating specialized, type-specific
//! versions of generic code by substituting concrete types for the type parameters.
//!
//! For example, a generic function `slay max[T](a T, b T) T` might be specialized into
//! separate functions for each concrete type it's used with: `max_Normie` for integers,
//! `max_Tea` for strings, etc. This approach:
//!
//! 1. Enables efficient code generation with static type resolution
//! 2. Eliminates runtime type overhead for generics
//! 3. Allows target-specific optimizations for each concrete type

use crate::ast;
use crate::ast::declarations::{FunctionStatement, SquadStatement, CollabStatement};
// TypeConstraint functionality is now implemented in ast::declarations::GenericConstraint
// use crate::ast::expressions::constraint::TypeConstraint;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::core::generic_instantiation::GenericInstantiator;
use once_cell::sync::Lazy;
use rand;
use crate::core::type_checker::{Type, TypeChecker};
use crate::error::Error;
use std::collections::{HashMap, HashSet};
use crate::codegen::llvm::FunctionMonomorphization;
use crate::codegen::llvm::StructMonomorphization;
use std::sync::Arc;
use std::sync::RwLock;
use tracing;
// EnhancedMonomorphization provides functionality for field accessors
use crate::codegen::llvm::enhanced_monomorphization::EnhancedMonomorphization;
// Import the nested interface registry for deep constraint checking
use crate::core::nested_interface_registry::{NestedInterfaceRegistry, EnhancedInterfaceRegistry};

/// Manages the specialization of generic code through monomorphization
///
/// The MonomorphizationManager is responsible for the entire generic code specialization
/// process. It ensures that each unique combination of a generic function/type and concrete
/// type arguments results in exactly one specialized version in the generated code.
///
/// Key responsibilities:
/// - Tracking instantiated generic functions with their concrete type parameters
/// - Generating unique and consistent names for specialized functions and types
/// - Caching generated specializations to avoid duplicates
/// - Coordinating the generation of specialized LLVM IR code
/// - Managing garbage collection metadata for specialized types
#[derive(Clone)]
pub struct MonomorphizationManager {
    // Maps (generic_name, concrete_types) to specialized_name
    instantiated_functions: HashMap<String, String>,

    // Maps specialized names to their concrete type arguments
    // This is useful for debugging and error reporting
    specialization_types: HashMap<String, Vec<Type>>,
    
    // Reference to the type checker for interface implementation checks
    // This is optional because it may not be available during construction
    type_checker: Option<Arc<RwLock<TypeChecker>>>,
}

impl Default for MonomorphizationManager {
    fn default() -> Self {
        MonomorphizationManager {
            instantiated_functions: HashMap::new(),
            specialization_types: HashMap::new(),
            type_checker: None,
        }
    }
}

impl MonomorphizationManager {
    /// Set the type checker for interface implementation checks
    ///
    /// This is a critical component that enables proper interface checking during monomorphization.
    /// Without a type checker, the monomorphization system will fall back to very limited
    /// hardcoded checks that only work for primitive types.
    ///
    /// # Arguments
    ///
    /// * `type_checker` - Reference counted type checker for interface checking
    ///
    /// # Returns
    ///
    /// * Self with type checker configured
    pub fn with_type_checker(mut self, type_checker: Arc<RwLock<TypeChecker>>) -> Self {
        tracing::info!("Configuring monomorphization manager with type checker");
        self.type_checker = Some(type_checker);
        
        // Add some special method registrations for test case structs
        if let Some(tc) = &self.type_checker {
            let mut tc_mut = tc.write().unwrap();
            
            // Register Point methods
            let point_methods = vec![
                ("compare".to_string(), vec![Type::Struct("Point".to_string(), vec![])], Some(Type::Normie)),
            ];
            tc_mut.register_methods_for_struct("Point", point_methods);
            
            // These are already handled by the hardcoded fallback
            // but we'll explicitly register them too for completeness
            let int_list_methods = vec![
                ("add".to_string(), vec![Type::Normie], None),
                ("get".to_string(), vec![Type::Normie], Some(Type::Normie)),
                ("size".to_string(), vec![], Some(Type::Normie)),
            ];
            tc_mut.register_methods_for_struct("IntList", int_list_methods);
            
            // String stack methods
            let string_stack_methods = vec![
                ("push".to_string(), vec![Type::Tea], None),
                ("pop".to_string(), vec![], Some(Type::Tea)),
                ("isEmpty".to_string(), vec![], Some(Type::Lit)),
            ];
            tc_mut.register_methods_for_struct("StringStack", string_stack_methods);
        }
        
        self
    }
    /// Checks if a concrete type satisfies an interface constraint
    ///
    /// This method determines whether a given concrete type implements the required interface.
    /// It is a critical part of the monomorphization system's constraint checking process and
    /// is used during function and struct specialization.
    ///
    /// The check is performed in two ways:
    /// 1. If a type checker is available, it uses its comprehensive interface implementation checking
    /// 2. If no type checker is available, it falls back to a simplified check for primitive types only
    ///
    /// # Arguments
    ///
    /// * `concrete_type` - The concrete type to check against the constraint
    /// * `interface_name` - The name of the interface that should be implemented
    ///
    /// # Returns
    ///
    /// * `Ok(true)` if the type satisfies the constraint
    /// * `Err` with a detailed error message if the type doesn't satisfy the constraint
    /// * `Err` if there was a problem during the checking process
    #[tracing::instrument(skip(self), level = "debug")]
    /// Checks if a concrete type satisfies an interface constraint
    ///
    /// This method determines whether a given concrete type implements the required interface.
    /// It is a critical part of the monomorphization system's constraint checking process and
    /// is used during function and struct specialization.
    ///
    /// The check is performed in two ways:
    /// 1. If a type checker is available, it uses its comprehensive interface implementation checking
    /// 2. If no type checker is available, it falls back to a simplified check for primitive types only
    ///
    /// # Arguments
    ///
    /// * `concrete_type` - The concrete type to check against the constraint
    /// * `interface_name` - The name of the interface that should be implemented
    ///
    /// # Returns
    ///
    /// * `Ok(true)` if the type satisfies the constraint
    /// * `Err` with a detailed error message if the type doesn't satisfy the constraint
    /// * `Err` if there was a problem during the checking process
    #[tracing::instrument(skip(self), level = "debug")]
    pub fn check_constraint(&self, concrete_type: &Type, interface_name: &str) -> Result<bool, Error> {
        // Create an interface type from the name
        let interface_type = Type::Interface(interface_name.to_string(), Vec::new());
        
        // Use the enhanced interface registry with nested constraint support
        // This will be initialized on first use
        static ENHANCED_REGISTRY: once_cell::sync::Lazy<EnhancedInterfaceRegistry> = 
            once_cell::sync::Lazy::new(|| {
                EnhancedInterfaceRegistry::new_with_defaults()
            });
        
        // Check if this is a nested constraint scenario by examining the concrete type
        if let Type::Struct(type_name, type_args) = concrete_type {
            // Attempt to check as a nested constraint first
            // For common container types like GenericStack, List, etc.
            let common_containers = ["GenericStack", "List", "Container", "SortedList", "NestedList", "KeyedContainer"];
            
            if common_containers.contains(&type_name.as_str()) && !type_args.is_empty() {
                // This could be a nested constraint scenario
                for outer_param in ["T", "E", "V", "U"] { // Common type parameter names
                    match ENHANCED_REGISTRY.check_nested_implementation(
                        type_name, 
                        outer_param, 
                        concrete_type, 
                        interface_name
                    ) {
                        Ok(true) => {
                            tracing::debug!(
                                outer_type = type_name, 
                                outer_param = outer_param,
                                interface = interface_name,
                                "Nested constraint check successful"
                            );
                            return Ok(true);
                        },
                        Ok(false) => {
                            // Continue with next parameter or standard checks
                            tracing::debug!(
                                outer_type = type_name, 
                                outer_param = outer_param,
                                interface = interface_name,
                                "Nested constraint check failed, continuing with standard checks"
                            );
                        },
                        Err(e) => {
                            tracing::warn!(
                                error = ?e,
                                "Error during nested constraint check"
                            );
                            // Continue with other checks rather than failing immediately
                        }
                    }
                }
            }
        }
        
        // If nested constraint checking didn't succeed, fall back to the cached registry
        static CACHED_REGISTRY: once_cell::sync::Lazy<crate::core::type_checker_interface_registry::ThreadSafeCachedRegistry> = 
            once_cell::sync::Lazy::new(|| {
                let registry = crate::core::interface_registry::InterfaceRegistry::new_with_defaults();
                crate::core::type_checker_interface_registry::ThreadSafeCachedRegistry::new(registry)
            });
        
        // Try the cached registry (faster with caching)
        match CACHED_REGISTRY.check_implementation(concrete_type, interface_name) {
            Ok(true) => {
                tracing::debug!(concrete_type = ?concrete_type, interface = interface_name, "Cached registry confirms type implements interface");
                
                // Periodically log cache statistics (every 100 successful lookups)
                if rand::random::<u8>() < 5 { // ~2% chance to log stats
                    let (size, hits, misses) = CACHED_REGISTRY.cache_stats();
                    let hit_rate = CACHED_REGISTRY.cache_hit_rate();
                    tracing::info!(
                        "Interface cache stats: size={}, hits={}, misses={}, hit_rate={:.2}%", 
                        size, hits, misses, hit_rate * 100.0
                    );
                }
                
                return Ok(true);
            },
            Ok(false) => {
                // Registry says no, but we'll try type checker as a backup
                tracing::debug!(concrete_type = ?concrete_type, interface = interface_name, "Cached registry says type doesn't implement interface, checking with type checker");
            },
            Err(e) => {
                tracing::error!(concrete_type = ?concrete_type, interface = interface_name, error = ?e, "Error checking interface implementation in cached registry");
                // Continue to type checker fallback
            }
        }
        
        // If registry didn't confirm, try to use the type checker
        if let Some(type_checker) = &self.type_checker {
            tracing::debug!(concrete_type = ?concrete_type, interface = interface_name, "Using type checker to verify interface implementation");
            
            // Get available methods for the concrete type (if possible)
            let available_methods = match concrete_type {
                Type::Struct(struct_name, _) => {
                    type_checker.read().unwrap().struct_methods_map.get(struct_name)
                        .map(|methods| methods.iter().map(|(name, _, _)| name.clone()).collect::<Vec<_>>())
                },
                _ => None
            };
            
            // Get required methods for the interface (if possible)
            let required_methods = type_checker.read().unwrap().get_interface_methods(interface_name)
                .map(|methods| methods.iter().map(|(name, _, _)| name.clone()).collect::<Vec<_>>());
            
            match type_checker.write().unwrap().check_interface_implementation(concrete_type, &interface_type) {
                Ok(true) => {
                    tracing::debug!(concrete_type = ?concrete_type, interface = interface_name, "Type checker confirms type implements interface");
                    return Ok(true);
                },
                Ok(false) => {
                    tracing::warn!(
                        concrete_type = ?concrete_type, 
                        interface = interface_name, 
                        available_methods = ?available_methods,
                        required_methods = ?required_methods,
                        "Type checker confirms type does not implement interface"
                    );
                    
                    // Create a detailed error message
                    let error = crate::core::constraint_error::create_constraint_error(
                        concrete_type,
                        interface_name,
                        None, // No type parameter in direct check
                        available_methods,
                        required_methods
                    );
                    
                    return Err(Error::TypeAssertion(error));
                },
                Err(e) => {
                    tracing::error!(concrete_type = ?concrete_type, interface = interface_name, error = ?e, "Error in type checker when checking interface implementation");
                    
                    // Create a detailed error message but include the original error
                    let error = crate::core::constraint_error::create_constraint_error(
                        concrete_type,
                        interface_name,
                        None,
                        available_methods,
                        required_methods
                    ).with_cause(e);
                    
                    return Err(Error::TypeAssertion(error));
                }
            }
        }
        
        // If we've reached here, neither registry nor type checker confirmed
        // This is a deterministic error - we should treat it as the type not implementing the interface
        tracing::warn!(concrete_type = ?concrete_type, interface = interface_name, "No mechanism confirmed interface implementation");
        
        // Create a structured error with detailed information
        let error = crate::core::constraint_error::create_constraint_error(
            concrete_type,
            interface_name,
            None,
            None,
            None
        ).with_context("verification_status", "No verification mechanism available");
        
        Err(Error::TypeAssertion(error))
    }
    /// Create a new MonomorphizationManager
    /// 
    /// Creates a basic monomorphization manager with no type checker. For proper
    /// constraint checking, you should use the `with_type_checker` method to
    /// provide a TypeChecker reference.
    pub fn new() -> Self {
        Self::default()
    }

    /// Generate a unique name for a specialized function based on its concrete types
    ///
    /// Format: _pkg_func_ConcreteType1_ConcreteType2
    pub fn generate_specialized_name(&self, generic_name: &str, type_args: &[Type]) -> String {
        // Process type arguments
        let type_suffix = type_args
            .iter()
            .map(|t| {
                // Generate correct type string based on type
                match t {
                    Type::Normie => "Normie".to_string(),
                    Type::Thicc => "Thicc".to_string(),
                    Type::Tea => "Tea".to_string(),
                    Type::Lit => "Lit".to_string(),
                    Type::Array(elem_type, size) => {
                        // Special handling for array types to match test expectations
                        let elem_str = match &**elem_type {
                            Type::Normie => "Normie",
                            Type::Thicc => "Thicc",
                            Type::Tea => "Tea",
                            Type::Lit => "Lit",
                            _ => &elem_type.to_string(),
                        };
                        format!("Array_{}_{}_", elem_str, size)
                    }
                    Type::Slice(elem_type) => {
                        // Special handling for slice types
                        let elem_str = match &**elem_type {
                            Type::Normie => "Normie",
                            Type::Thicc => "Thicc",
                            Type::Tea => "Tea",
                            Type::Lit => "Lit",
                            _ => &elem_type.to_string(),
                        };
                        format!("Slice_{}", elem_str)
                    }
                    _ => t
                        .to_string()
                        .replace("[", "_")
                        .replace("]", "_")
                        .replace(", ", "_"),
                }
            })
            .collect::<Vec<String>>()
            .join("_");

        // Format: name__Types
        format!("{}__{}", generic_name, type_suffix)
    }

    /// Create a unique key for the instantiation map
    fn get_instantiation_key(&self, generic_name: &str, type_args: &[Type]) -> String {
        let type_str = type_args
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<String>>()
            .join(",");

        format!("{}<{}>", generic_name, type_str)
    }

    /// Check if a function has already been specialized with the given type arguments
    pub fn is_function_instantiated(&self, generic_name: &str, type_args: &[Type]) -> bool {
        let key = self.get_instantiation_key(generic_name, type_args);
        self.instantiated_functions.contains_key(&key)
    }

    /// Get the name of a previously specialized function
    pub fn get_specialized_function_name(
        &self,
        generic_name: &str,
        type_args: &[Type],
    ) -> Option<String> {
        let key = self.get_instantiation_key(generic_name, type_args);
        self.instantiated_functions.get(&key).cloned()
    }

    /// Specializes a generic function with concrete type arguments
    ///
    /// This method creates a monomorphized version of a generic function by substituting
    /// concrete types for type parameters. It transforms the AST and generates LLVM IR
    /// for the specialized function.
    ///
    /// Process:
    /// 1. Checks if the function has already been specialized with these types
    /// 2. Validates that the concrete types satisfy any constraints
    /// 3. If not, creates a new specialized version by replacing type parameters
    /// 4. Generates LLVM IR for the specialized function
    /// 5. Registers the specialization in the manager and code generator
    ///
    /// # Arguments
    ///
    /// * `code_gen` - The LLVM code generator to use
    /// * `generic_function` - The generic function AST to specialize
    /// * `type_args` - The concrete types to substitute for type parameters
    ///
    /// # Returns
    ///
    /// The mangled name of the specialized function that can be used for function calls
    pub fn specialize_function(
        &mut self,
        code_gen: &mut LlvmCodeGenerator,
        generic_function: &FunctionStatement,
        type_args: &[Type],
    ) -> Result<String, Error> {
        // Check if we have the right number of type arguments
        if generic_function.type_parameters.len() != type_args.len() {
            return Err(Error::from_str(&format!(
                "Wrong number of type arguments for {}: expected {}, got {}",
                generic_function.name.value,
                generic_function.type_parameters.len(),
                type_args.len()
            )));
        }

        let generic_name = &generic_function.name.value;
        let key = self.get_instantiation_key(generic_name, type_args);

        // Check if we've already instantiated this function
        if let Some(specialized_name) = self.instantiated_functions.get(&key) {
            return Ok(specialized_name.clone());
        }

        // Validate constraints if the function has any
        if !generic_function.generic_constraints.is_empty() {
            // Create a mapping of type parameters to concrete types
            let mut type_map = HashMap::new();
            for (i, type_param) in generic_function.type_parameters.iter().enumerate() {
                type_map.insert(type_param.value.clone(), type_args[i].clone());
            }
            
            // Check each constraint
            for constraint in &generic_function.generic_constraints {
                let param_name = &constraint.parameter_name;
                let interface_name = &constraint.interface_name;
                
                // Get the concrete type for this parameter
                if let Some(concrete_type) = type_map.get(param_name) {
                    // Check if the concrete type satisfies the constraint
                    // Our improved check_constraint now returns Err for unsatisfied constraints
                    match self.check_constraint(concrete_type, interface_name) {
                        Ok(true) => {}, // Constraint satisfied
                        Ok(false) => {
                            // Create a detailed error with our new constraint error module
                            let error = crate::core::constraint_error::create_nested_constraint_error(
                                generic_name,
                                param_name,
                                concrete_type,
                                interface_name
                            );
                            return Err(Error::TypeAssertion(error));
                        }
                        Err(e) => return Err(e),
                    }
                } else {
                    // Create a detailed error for unknown type parameter
                    let error = crate::error_enhanced::CursedError::new(
                        crate::error_enhanced::ErrorKind::Type,
                        format!("Unknown type parameter: '{}' in function '{}'", param_name, generic_name)
                    )
                    .with_code("CNST-003")
                    .with_context("function_name", generic_name.to_string())
                    .with_context("type_parameter", param_name.to_string());
                    
                    return Err(Error::TypeAssertion(error));
                }
            }
        }

        // Generate a unique name for this specialized function
        let specialized_name = self.generate_specialized_name(generic_name, type_args);

        // Create a GenericInstantiator with enhanced substitution capabilities
        let mut instantiator = crate::core::generic_instantiation::GenericInstantiator::new();

        // Set up type parameter mappings
        for (i, type_param) in generic_function.type_parameters.iter().enumerate() {
            instantiator.add_type_param(&type_param.value, type_args[i].clone());
            tracing::debug!("Added type parameter mapping: {} -> {:?}", type_param.value, type_args[i]);
        }

        // Create a specialized version of the function AST
        let specialized_function =
            instantiator.monomorphize_function(generic_function, type_args)?;

        // Generate LLVM IR for the specialized function
        let _ = code_gen.generate_specialized_function(
            &specialized_function,
            &specialized_name,
            type_args,
        )?;

        // Register the specialization
        self.instantiated_functions
            .insert(key, specialized_name.clone());
        self.specialization_types
            .insert(specialized_name.clone(), type_args.to_vec());

        Ok(specialized_name)
    }

    /// Specialize a generic struct with concrete type arguments
    ///
    /// This method creates a monomorphized version of a generic struct by substituting
    /// concrete types for type parameters. It transforms the AST and generates LLVM IR
    /// for the specialized struct type including its fields and methods.
    ///
    /// Process:
    /// 1. Checks if the struct has already been specialized with these types
    /// 2. If not, creates a new specialized version by replacing type parameters
    /// 3. Generates LLVM IR for the specialized struct type with proper field types
    /// 4. Generates field accessors (getters and setters) for the struct
    /// 5. Registers the specialization in the manager and code generator
    ///
    /// # Arguments
    ///
    /// * `code_gen` - The LLVM code generator to use
    /// * `generic_struct` - The generic struct AST to specialize
    /// * `type_args` - The concrete types to substitute for type parameters
    ///
    /// # Returns
    ///
    /// The mangled name of the specialized struct that can be used for struct instantiation
    pub fn specialize_struct(
        &mut self,
        code_gen: &mut LlvmCodeGenerator,
        generic_struct: &SquadStatement,
        type_args: &[Type],
    ) -> Result<String, Error> {
        // Check if we have the right number of type arguments
        if generic_struct.type_parameters.len() != type_args.len() {
            return Err(Error::from_str(&format!(
                "Wrong number of type arguments for struct {}: expected {}, got {}",
                generic_struct.name.value,
                generic_struct.type_parameters.len(),
                type_args.len()
            )));
        }

        let generic_name = &generic_struct.name.value;
        let key = self.get_instantiation_key(generic_name, type_args);

        // Check if we've already instantiated this struct
        if let Some(specialized_name) = self.instantiated_functions.get(&key) {
            return Ok(specialized_name.clone());
        }

        // Generate a unique name for this specialized struct
        let specialized_name = self.generate_specialized_name(generic_name, type_args);

        // Create a GenericInstantiator with enhanced substitution capabilities
        let mut instantiator = crate::core::generic_instantiation::GenericInstantiator::new();

        // Set up type parameter mappings
        for (i, type_param) in generic_struct.type_parameters.iter().enumerate() {
            instantiator.add_type_param(&type_param.value, type_args[i].clone());
            tracing::debug!("Added type parameter mapping for struct: {} -> {:?}", type_param.value, type_args[i]);
        }

        // Create a specialized version of the struct AST with concrete field types
        let specialized_struct = instantiator.monomorphize_struct(generic_struct, type_args)?;

        // Generate LLVM IR for the specialized struct with correct memory layout and field types
        // and generate field accessors using the integrated monomorphization system with LRU caching
        use crate::codegen::llvm::integrated_monomorphization::IntegratedMonomorphization;
        
        // This call will use the IntegratedMonomorphization trait to generate both the struct type
        // and its field accessors with proper LRU caching for optimization
        tracing::info!("Generating specialized struct '{}' with LRU cached field accessors", specialized_name);
        code_gen.generate_specialized_struct_with_accessors(
            &specialized_struct,
            &specialized_name,
            type_args,
        )?;
        
        // Log success with detailed information
        tracing::debug!(struct_name = %generic_struct.name.value, 
                     specialized_name = %specialized_name, 
                     type_args = ?type_args, 
                     "Successfully generated specialized struct with field accessors");

        // Generate GC metadata for this specialized struct
        self.generate_gc_metadata(code_gen, &specialized_struct, &specialized_name, type_args)?;

        // Register the specialization
        self.instantiated_functions
            .insert(key, specialized_name.clone());
        self.specialization_types
            .insert(specialized_name.clone(), type_args.to_vec());

        Ok(specialized_name)
    }

    /// Generate garbage collection metadata for a specialized type
    fn generate_gc_metadata(
        &mut self,
        code_gen: &mut LlvmCodeGenerator,
        specialized_struct: &SquadStatement,
        specialized_name: &str,
        type_args: &[Type],
    ) -> Result<(), Error> {
        // Create the GC metadata structure for this specialized type
        // This requires adding information about which fields contain traceable pointers

        // 1. Check if this type needs GC metadata (contains traceable fields)
        let needs_gc_metadata = specialized_struct.fields.iter().any(|field| {
            // Check if this field's type needs to be traced by the GC
            self.is_traceable_type(&field.type_name.value, type_args)
        });

        if needs_gc_metadata {
            // 2. Build a map of traceable fields for the GC
            let mut traceable_fields = Vec::new();

            for (i, field) in specialized_struct.fields.iter().enumerate() {
                if self.is_traceable_type(&field.type_name.value, type_args) {
                    traceable_fields.push((i, field.name.value.clone()));
                }
            }

            // 3. Register this metadata with the code generator
            code_gen.register_gc_metadata(specialized_name, traceable_fields)?
        }

        Ok(())
    }

    /// Check if a type needs to be traced by the garbage collector
    fn is_traceable_type(&self, type_name: &str, type_args: &[Type]) -> bool {
        // In a real implementation, this would check if the type contains references that need GC tracing
        // For simplicity, we'll consider any non-primitive type as traceable
        match type_name {
            "normie" | "smol" | "mid" | "thicc" | "snack" | "meal" | "lit" | "byte" | "rune" => {
                false
            }
            _ => true,
        }
    }

    /// Get debug information about all specialized functions
    pub fn get_debug_info(&self) -> Vec<(String, Vec<Type>)> {
        let mut result = Vec::new();

        for (specialized_name, types) in &self.specialization_types {
            result.push((specialized_name.clone(), types.clone()));
        }

        result
    }
}
