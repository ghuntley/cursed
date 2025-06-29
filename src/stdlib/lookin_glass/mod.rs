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
    INIT.call_once(|| {
        // Initialize any global state needed for reflection
        println!("LookinGlass reflection package initialized");
        initialize_type_registry();
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

/// Type registry for runtime reflection
static TYPE_REGISTRY: RwLock<HashMap<String, TypeInfo>> = RwLock::new(HashMap::new());

/// Basic type information for reflection
#[derive(Debug, Clone)]
pub struct TypeInfo {
    pub name: String,
    pub size: usize,
    pub alignment: usize,
    pub kind: TypeKind,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<MethodInfo>,
}

/// Type kind enumeration
#[derive(Debug, Clone, PartialEq)]
pub enum TypeKind {
    Primitive,
    Struct,
    Enum,
    Array,
    Pointer,
    Function,
    Interface,
}

/// Field information for structs
#[derive(Debug, Clone)]
pub struct FieldInfo {
    pub name: String,
    pub type_name: String,
    pub offset: usize,
    pub size: usize,
}

/// Method information
#[derive(Debug, Clone)]
pub struct MethodInfo {
    pub name: String,
    pub return_type: String,
    pub parameters: Vec<ParameterInfo>,
    pub is_static: bool,
}

/// Parameter information
#[derive(Debug, Clone)]
pub struct ParameterInfo {
    pub name: String,
    pub type_name: String,
}

/// Initialize the type registry
fn initialize_type_registry() {
    register_builtin_types();
}

/// Register built-in types
fn register_builtin_types() {
    let basic_types = vec![
        ("null", TypeKind::Primitive),
        ("bool", TypeKind::Primitive),
        ("i8", TypeKind::Primitive),
        ("i16", TypeKind::Primitive),
        ("i32", TypeKind::Primitive),
        ("i64", TypeKind::Primitive),
        ("u8", TypeKind::Primitive),
        ("u16", TypeKind::Primitive),
        ("u32", TypeKind::Primitive),
        ("u64", TypeKind::Primitive),
        ("f32", TypeKind::Primitive),
        ("f64", TypeKind::Primitive),
        ("string", TypeKind::Primitive),
        ("array", TypeKind::Array),
        ("object", TypeKind::Struct),
    ];
    
    if let Ok(mut registry) = TYPE_REGISTRY.write() {
        for (name, kind) in basic_types {
            let type_info = TypeInfo {
                name: name.to_string(),
                size: get_type_size(name),
                alignment: get_type_alignment(name),
                kind,
                fields: Vec::new(),
                methods: Vec::new(),
            };
            registry.insert(name.to_string(), type_info);
        }
    }
}

/// Get type size for basic types
fn get_type_size(type_name: &str) -> usize {
    match type_name {
        "bool" | "i8" | "u8" => 1,
        "i16" | "u16" => 2,
        "i32" | "u32" | "f32" => 4,
        "i64" | "u64" | "f64" => 8,
        "string" => std::mem::size_of::<String>(),
        _ => 0,
    }
}

/// Get type alignment for basic types
fn get_type_alignment(type_name: &str) -> usize {
    match type_name {
        "bool" | "i8" | "u8" => 1,
        "i16" | "u16" => 2,
        "i32" | "u32" | "f32" => 4,
        "i64" | "u64" | "f64" => 8,
        "string" => std::mem::align_of::<String>(),
        _ => 1,
    }
}

/// Register a new type
pub fn register_type(name: String, type_info: TypeInfo) -> Result<(), CursedError> {
    if let Ok(mut registry) = TYPE_REGISTRY.write() {
        registry.insert(name, type_info);
        
        // Update statistics
        if let Ok(mut stats) = REFLECTION_STATS.write() {
            stats.types_created += 1;
        }
        
        Ok(())
    } else {
        Err(CursedError::Runtime("Failed to acquire type registry lock".to_string()))
    }
}

/// Look up type information
pub fn lookup_type(name: &str) -> Option<TypeInfo> {
    if let Ok(registry) = TYPE_REGISTRY.read() {
        // Update statistics
        if let Ok(mut stats) = REFLECTION_STATS.write() {
            stats.type_lookups += 1;
        }
        
        registry.get(name).cloned()
    } else {
        None
    }
}

/// Get all registered types
pub fn registered_types() -> Vec<String> {
    if let Ok(registry) = TYPE_REGISTRY.read() {
        registry.keys().cloned().collect()
    } else {
        Vec::new()
    }
}

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
