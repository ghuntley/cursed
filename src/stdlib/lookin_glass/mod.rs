use crate::error_types::CursedError;
use std::sync::{RwLock, Mutex};
use std::collections::HashMap;
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

// Re-export core types (if they exist)
// pub use types::Type;
// pub use value::Value;
// pub use kind::Kind;
// pub use struct_field::StructField;
// pub use struct_tag::StructTag;
// pub use method::Method;

// Re-export core functions
pub use core_functions::{
    register_type, lookup_type, registered_types, init_type_registry,
    TypeInfo, TypeKind, FieldInfo, MethodInfo, ParameterInfo
};

// Re-export enhanced utilities
// pub use utilities::{
//     value_to_map, map_to_value
// };

// Re-export VibeMapper and related types
// pub use vibe_mapper::{
//     camel_to_snake, snake_to_camel, to_lowercase, to_uppercase
// };

// Re-export error types
// pub use error::{LookinGlassError, LookinGlassResult};

use std::sync::Once;

static INIT: Once = Once::new();

/// Initialize the LookinGlass reflection package
pub fn initialize() {
    INIT.call_once(|| {
        // Initialize any global state needed for reflection
        println!("LookinGlass reflection package initialized");
        init_type_registry();
    });
}

/// Get statistics about reflection package usage
pub fn get_reflection_statistics() -> ReflectionStatistics {
    REFLECTION_STATS.read().unwrap().clone()
}

/// Statistics about reflection package usage
#[derive(Debug, Clone)]
pub struct ReflectionStatistics {
    pub types_created: u64,
    pub type_lookups: u64,
    pub value_conversions: u64,
    pub method_invocations: u64,
}

impl Default for ReflectionStatistics {
    fn default() -> Self {
        Self {
            types_created: 0,
            type_lookups: 0,
            value_conversions: 0,
            method_invocations: 0,
        }
    }
}

/// Global reflection statistics
static REFLECTION_STATS: RwLock<ReflectionStatistics> = RwLock::new(ReflectionStatistics {
    types_created: 0,
    type_lookups: 0,
    value_conversions: 0,
    method_invocations: 0,
});

/// Track value conversion
pub fn track_value_conversion() {
    if let Ok(mut stats) = REFLECTION_STATS.write() {
        stats.value_conversions += 1;
    }
}

/// Track method invocation
pub fn track_method_invocation() {
    if let Ok(mut stats) = REFLECTION_STATS.write() {
        stats.method_invocations += 1;
    }
}

/// Reset reflection statistics
pub fn reset_reflection_statistics() {
    if let Ok(mut stats) = REFLECTION_STATS.write() {
        *stats = ReflectionStatistics::default();
    }
}
