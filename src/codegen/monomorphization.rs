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
use crate::ast::declarations::{FunctionStatement, SquadStatement};
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::core::generic_instantiation::GenericInstantiator;
use crate::core::type_checker::Type;
use crate::error::Error;
use std::collections::HashMap;

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
pub struct MonomorphizationManager {
    // Maps (generic_name, concrete_types) to specialized_name
    instantiated_functions: HashMap<String, String>,

    // Maps specialized names to their concrete type arguments
    // This is useful for debugging and error reporting
    specialization_types: HashMap<String, Vec<Type>>,
}

impl Default for MonomorphizationManager {
    fn default() -> Self {
        MonomorphizationManager {
            instantiated_functions: HashMap::new(),
            specialization_types: HashMap::new(),
        }
    }
}

impl MonomorphizationManager {
    /// Create a new MonomorphizationManager
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
    /// 2. If not, creates a new specialized version by replacing type parameters
    /// 3. Generates LLVM IR for the specialized function
    /// 4. Registers the specialization in the manager and code generator
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

        // Generate a unique name for this specialized function
        let specialized_name = self.generate_specialized_name(generic_name, type_args);

        // Create a GenericInstantiator to handle type parameter substitution
        let mut instantiator = GenericInstantiator::new();

        // Set up type parameter mappings
        for (i, type_param) in generic_function.type_parameters.iter().enumerate() {
            instantiator.add_type_param(&type_param.value, type_args[i].clone());
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

        // Create a GenericInstantiator to handle type parameter substitution
        let mut instantiator = GenericInstantiator::new();

        // Set up type parameter mappings
        for (i, type_param) in generic_struct.type_parameters.iter().enumerate() {
            instantiator.add_type_param(&type_param.value, type_args[i].clone());
        }

        // Create a specialized version of the struct AST
        let specialized_struct = instantiator.monomorphize_struct(generic_struct, type_args)?;

        // Generate LLVM IR for the specialized struct with correct memory layout
        // In the real implementation, we would generate a proper LLVM struct type here
        // Let's use a helper method to create the struct type
        let _ = code_gen.generate_specialized_struct(
            &specialized_struct,
            &specialized_name,
            type_args,
        )?;

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
