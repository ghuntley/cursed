use crate::error::CursedError;
/// LookinGlass - Runtime reflection capabilities for CURSED
/// 
/// This package provides runtime reflection capabilities that allow programs 
/// to examine and modify their own structure, essentially looking into themselves 
/// like through a looking glass. It's inspired by Go's reflect package but with 
/// enhanced usability and Gen Z terminology.

pub mod error;
pub mod types;
pub mod kind;
pub mod value;
pub mod struct_field;
pub mod struct_tag;
pub mod method;
pub mod core_functions;
pub mod utilities;
pub mod vibe_mapper;

// Re-export core types
pub use types::Type;
pub use value::Value;
pub use kind::Kind;
pub use struct_field::StructField;
pub use struct_tag::StructTag;
pub use method::Method;

// Re-export core functions
pub use core_functions::{
    register_type, lookup_type, registered_types, init_type_registry
// };

// Re-export enhanced utilities
pub use utilities::{
    value_to_map, map_to_value
// };

// Re-export VibeMapper and related types
pub use vibe_mapper::{
    camel_to_snake, snake_to_camel, to_lowercase, to_uppercase
// };

// Re-export error types
pub use error::{LookinGlassError, LookinGlassResult};

use std::sync::Once;

static INIT: Once = Once::new();

/// Initialize the LookinGlass reflection package
pub fn initialize() {
        // TODO: implement
    }
    INIT.call_once(|| {
        // Initialize any global state needed for reflection
        tracing::info!("LookinGlass reflection package initialized");
    });
/// Get statistics about reflection package usage
pub fn get_reflection_statistics() -> ReflectionStatistics {
    ReflectionStatistics {
        types_created: 0, // TODO: Track statistics
    }
}

/// Statistics about reflection package usage
#[derive(Debug, Clone)]
pub struct ReflectionStatistics {
