//! Module for generating detailed error messages for constraint failures
//!
//! This module provides utilities for generating rich, informative error messages
//! when type parameter constraints are not satisfied.

use crate::core::type_checker::Type;
use crate::error_enhanced::{CursedError, ErrorKind};
use tracing::{debug, error, instrument};

/// Common error code prefix for constraint errors
pub const CONSTRAINT_ERROR_CODE_PREFIX: &str = "CNST";

/// Generate a detailed error message for a constraint failure
/// 
/// This function creates a rich error description that explains exactly why
/// a type does not satisfy an interface constraint, including:
/// - Missing methods
/// - Incompatible method signatures
/// - Nested constraints that aren't satisfied
/// - Suggestions for fixing the error
///
/// # Arguments
///
/// * `concrete_type` - The concrete type that failed to meet the constraint
/// * `interface_name` - The name of the interface constraint that wasn't satisfied
/// * `type_param_name` - Optional name of the type parameter (for better error context)
/// * `available_methods` - Optional list of methods actually available on the type
/// * `required_methods` - Optional list of methods required by the interface
///
/// # Returns
///
/// A `CursedError` with rich context information about the constraint failure
#[instrument(skip(available_methods, required_methods), level = "debug")]
pub fn create_constraint_error(
    concrete_type: &Type,
    interface_name: &str,
    type_param_name: Option<&str>,
    available_methods: Option<Vec<String>>, 
    required_methods: Option<Vec<String>>,
) -> CursedError {
    let mut error_msg = format!(
        "Type '{:?}' does not implement interface '{}'", 
        concrete_type, 
        interface_name
    );
    
    // Add type parameter context if available
    if let Some(param_name) = type_param_name {
        error_msg = format!(
            "Type parameter '{}' with type '{:?}' does not satisfy interface constraint '{}'",
            param_name,
            concrete_type,
            interface_name
        );
    }
    
    // Create the base error
    let mut error = CursedError::new(ErrorKind::Type, error_msg)
        .with_code(format!("{}-001", CONSTRAINT_ERROR_CODE_PREFIX))
        .with_context("concrete_type", concrete_type.to_string())
        .with_context("interface", interface_name.to_string());
    
    // Add type parameter info if available
    if let Some(param_name) = type_param_name {
        error = error.with_context("type_parameter", param_name.to_string());
    }
    
    // Add method information if available
    match (available_methods, required_methods) {
        (Some(available), Some(required)) => {
            error = error.with_context("available_methods", format!("{:?}", available));
            error = error.with_context("required_methods", format!("{:?}", required));
            
            // Compute missing methods
            let missing_methods: Vec<_> = required.iter()
                .filter(|req| !available.contains(req))
                .collect();
            
            if !missing_methods.is_empty() {
                error = error.with_context("missing_methods", format!("{:?}", missing_methods));
                
                // Add explanation about missing methods
                let missing_str = missing_methods.iter()
                    .map(|m| format!("  - {}", m))
                    .collect::<Vec<_>>()
                    .join("\n");
                
                error = error.with_context("missing_details", format!("\nMissing methods:\n{}", missing_str));
            }
        },
        (Some(available), None) => {
            error = error.with_context("available_methods", format!("{:?}", available));
        },
        (None, Some(required)) => {
            error = error.with_context("required_methods", format!("{:?}", required));
        },
        _ => {}
    }
    
    // Add suggestion for fixing the error
    let suggestion = match concrete_type {
        Type::Struct(struct_name, _) => {
            format!("Implement the '{}' interface for struct '{}'", interface_name, struct_name)
        },
        _ => {
            format!("Ensure type '{:?}' implements all methods required by the '{}' interface", 
            concrete_type, interface_name)
        }
    };
    
    error = error.with_context("suggestion", suggestion);
    
    debug!("Created constraint error: {}", error);
    error
}

/// Generate a detailed error message for a nested constraint failure
/// 
/// This function creates an error for the specific case where a generic type's
/// type parameter constraints are not satisfied.
///
/// # Arguments
///
/// * `generic_type_name` - The name of the generic type (e.g., "SortedList")
/// * `type_param_name` - The name of the type parameter (e.g., "T")
/// * `concrete_arg` - The concrete type argument that failed the constraint
/// * `interface_name` - The name of the interface constraint that wasn't satisfied
///
/// # Returns
///
/// A `CursedError` with rich context information about the nested constraint failure
#[instrument(level = "debug")]
pub fn create_nested_constraint_error(
    generic_type_name: &str,
    type_param_name: &str,
    concrete_arg: &Type,
    interface_name: &str,
) -> CursedError {
    let error_msg = format!(
        "Type argument '{:?}' for parameter '{}' in '{}' does not implement required interface '{}'",
        concrete_arg,
        type_param_name,
        generic_type_name,
        interface_name
    );
    
    let mut error = CursedError::new(ErrorKind::Type, error_msg)
        .with_code(format!("{}-002", CONSTRAINT_ERROR_CODE_PREFIX))
        .with_context("generic_type", generic_type_name.to_string())
        .with_context("type_parameter", type_param_name.to_string())
        .with_context("concrete_arg", concrete_arg.to_string())
        .with_context("interface", interface_name.to_string());
        
    // Add suggestion for fixing the error
    let suggestion = match concrete_arg {
        Type::Struct(struct_name, _) => {
            format!("Implement the '{}' interface for struct '{}' before using it as an argument to '{}'", 
                    interface_name, struct_name, generic_type_name)
        },
        _ => {
            format!("Use a type that implements the '{}' interface for parameter '{}' of '{}'", 
                    interface_name, type_param_name, generic_type_name)
        }
    };
    
    error = error.with_context("suggestion", suggestion);
    
    debug!("Created nested constraint error: {}", error);
    error
}