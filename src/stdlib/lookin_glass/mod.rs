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
    type_of, value_of, new, zero, indirect,
    make_slice, make_map, make_chan, make_func,
    array_of, slice_of, map_of, ptr_to, chan_of, func_of,
    register_type, lookup_type, registered_types, init_type_registry
};

// Re-export enhanced utilities
pub use utilities::{
    deep_equal, deep_copy, struct_to_map, map_to_struct,
    get_tags, set_field, get_field, has_field, field_names, field_info,
    value_to_map, map_to_value
};

// Re-export VibeMapper and related types
pub use vibe_mapper::{
    VibeMapper, VibeMapperConfig,
    camel_to_snake, snake_to_camel, to_lowercase, to_uppercase
};

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
}

/// Get statistics about reflection package usage
pub fn get_reflection_statistics() -> ReflectionStatistics {
    ReflectionStatistics {
        types_created: 0, // TODO: Track statistics
        values_created: 0,
        deep_copies_performed: 0,
        struct_conversions: 0,
    }
}

/// Statistics about reflection package usage
#[derive(Debug, Clone)]
pub struct ReflectionStatistics {
    pub types_created: u64,
    pub values_created: u64,
    pub deep_copies_performed: u64,
    pub struct_conversions: u64,
}

