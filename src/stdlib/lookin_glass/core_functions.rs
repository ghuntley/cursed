//! Core reflection functions for lookin_glass

use crate::error::CursedError;
use std::collections::HashMap;
use std::sync::Mutex;

/// Type information for reflection
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

/// Global type registry
static TYPE_REGISTRY: Mutex<Option<HashMap<String, TypeInfo>>> = Mutex::new(None);

/// Register a type in the registry
pub fn register_type(name: String, type_info: TypeInfo) -> Result<(), CursedError> {
    if let Ok(mut registry) = TYPE_REGISTRY.lock() {
        if registry.is_none() {
            *registry = Some(HashMap::new());
        }
        registry.as_mut().unwrap().insert(name, type_info);
        Ok(())
    } else {
        Err(CursedError::runtime_error("Failed to acquire type registry lock"))
    }
}

/// Look up a type by name
pub fn lookup_type(name: &str) -> Option<TypeInfo> {
    if let Ok(registry) = TYPE_REGISTRY.lock() {
        registry.as_ref().and_then(|r| r.get(name).cloned())
    } else {
        None
    }
}

/// Get all registered type names
pub fn registered_types() -> Vec<String> {
    if let Ok(registry) = TYPE_REGISTRY.lock() {
        registry.as_ref().map(|r| r.keys().cloned().collect()).unwrap_or_else(Vec::new)
    } else {
        Vec::new()
    }
}

/// Initialize the type registry
pub fn init_type_registry() {
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
    
    for (name, kind) in basic_types {
        let type_info = TypeInfo {
            name: name.to_string(),
            size: get_type_size(name),
            alignment: get_type_alignment(name),
            kind,
            fields: Vec::new(),
            methods: Vec::new(),
        };
        let _ = register_type(name.to_string(), type_info);
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
