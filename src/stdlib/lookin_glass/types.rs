/// Type represents the type of a value in CURSED reflection
// use crate::stdlib::lookin_glass::{Kind, StructField, Method, error::*};
use std::fmt;
use std::sync::Arc;
use std::collections::HashMap;

/// Represents the type of a value
#[derive(Debug, Clone)]
pub struct Type {
    kind: Kind,
    name: String,
    pkg_path: String,
    size: usize,
    align: usize,
    field_align: usize,
    methods: Vec<Method>,
    fields: Vec<StructField>,
    elem_type: Option<Box<Type>>,
    key_type: Option<Box<Type>>,
    len: Option<usize>,
    in_types: Vec<Type>,
    out_types: Vec<Type>,
    variadic: bool,
    comparable: bool,
}

impl Type {
    /// Create a new Type
    pub fn new(kind: Kind, name: String, pkg_path: String) -> Self {
        let size = kind.size().unwrap_or(0);
        let align = kind.align().unwrap_or(1);
        
        Self {
            kind,
            name,
            pkg_path,
            size,
            align,
            field_align: align,
            methods: Vec::new(),
            fields: Vec::new(),
            elem_type: None,
            key_type: None,
            len: None,
            in_types: Vec::new(),
            out_types: Vec::new(),
            variadic: false,
            comparable: kind.is_comparable(),
        }
    }

    /// Create a type for basic kinds
    pub fn basic(kind: Kind) -> Self {
        Self::new(kind, kind.as_str().to_string(), String::new())
    }

    /// Create an invalid type
    pub fn invalid() -> Self {
        Self::new(Kind::Invalid, "invalid".to_string(), String::new())
    }

    /// Get the kind of this type
    pub fn kind(&self) -> Kind {
        self.kind
    }

    /// Get the name of this type
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the package path
    pub fn pkg_path(&self) -> &str {
        &self.pkg_path
    }

    /// Get the size in bytes
    pub fn size(&self) -> usize {
        self.size
    }

    /// Get the alignment requirement
    pub fn align(&self) -> usize {
        self.align
    }

    /// Get the field alignment
    pub fn field_align(&self) -> usize {
        self.field_align
    }

    /// Get the string representation
    pub fn string(&self) -> String {
        if self.pkg_path.is_empty() {
            self.name.clone()
        } else {
            format!("{}.{}", self.pkg_path, self.name)
        }
    }

    /// Check if this type is comparable
    pub fn comparable(&self) -> bool {
        self.comparable
    }

    /// Check if this is a basic type
    pub fn is_basic(&self) -> bool {
        self.kind.is_basic()
    }

    /// Check if this type implements another type (interface)
    pub fn implements(&self, u: &Type) -> bool {
        if !u.kind() == Kind::Interface {
            return false;
        }

        // Check if this type has all methods required by the interface
        for required_method in &u.methods {
            if !self.has_method(&required_method.name) {
                return false;
            }
            
            // TODO: Check method signatures match
        }

        true
    }

    /// Check if this type is assignable to another type
    pub fn assignable_to(&self, u: &Type) -> bool {
        // Same type
        if self.kind == u.kind && self.name == u.name && self.pkg_path == u.pkg_path {
            return true;
        }

        // Interface implementation
        if u.kind == Kind::Interface {
            return self.implements(u);
        }

        // Basic type compatibility
        if self.kind.is_basic() && u.kind.is_basic() {
            // Allow some basic conversions
            match (self.kind, u.kind) {
                (Kind::Int, Kind::Int32) | (Kind::Int32, Kind::Int) => true,
                _ => false,
            }
        } else {
            false
        }
    }

    /// Check if this type is convertible to another type
    pub fn convertible_to(&self, u: &Type) -> bool {
        // If assignable, then convertible
        if self.assignable_to(u) {
            return true;
        }

        // Additional conversion rules
        match (self.kind, u.kind) {
            // Numeric conversions
            (a, b) if a.is_numeric() && b.is_numeric() => true,
            
            // String conversions
            (Kind::String, Kind::Slice) => true, // string to []byte
            (Kind::Slice, Kind::String) => true, // []byte to string
            
            // Pointer conversions
            (Kind::Pointer, Kind::UnsafePointer) => true,
            (Kind::UnsafePointer, Kind::Pointer) => true,
            
            _ => false,
        }
    }

    // Methods for specific kinds

    /// Get the element type (for Array, Chan, Map, Pointer, Slice)
    pub fn elem(&self) -> LookinGlassResult<Type> {
        match self.kind {
            Kind::Array | Kind::Chan | Kind::Map | Kind::Pointer | Kind::Slice => {
                self.elem_type.as_ref()
                    .map(|t| (**t).clone())
                    .ok_or_else(|| type_error("Type does not have an element type"))
            }
            _ => Err(invalid_operation(&format!("Elem() called on {}", self.kind))),
        }
    }

    /// Get the length (for Array, Slice)
    pub fn len(&self) -> LookinGlassResult<usize> {
        match self.kind {
            Kind::Array => {
                self.len.ok_or_else(|| type_error("Array type does not have length"))
            }
            _ => Err(invalid_operation(&format!("Len() called on {}", self.kind))),
        }
    }

    /// Get a struct field by index
    pub fn field(&self, i: usize) -> LookinGlassResult<StructField> {
        if self.kind != Kind::Struct {
            return Err(invalid_operation(&format!("Field() called on {}", self.kind)));
        }
        
        self.fields.get(i)
            .cloned()
            .ok_or_else(|| index_error(&format!("Field index {} out of range", i)))
    }

    /// Get a struct field by index path
    pub fn field_by_index(&self, index: &[usize]) -> LookinGlassResult<StructField> {
        if self.kind != Kind::Struct {
            return Err(invalid_operation(&format!("FieldByIndex() called on {}", self.kind)));
        }

        if index.is_empty() {
            return Err(index_error("Empty index path"));
        }

        let mut current_type = self.clone();
        let mut current_field = None;

        for &i in index {
            if current_type.kind != Kind::Struct {
                return Err(type_error("Cannot index into non-struct type"));
            }

            current_field = Some(current_type.field(i)?);
            if let Some(ref field) = current_field {
                current_type = field.field_type().clone();
            }
        }

        current_field.ok_or_else(|| field_error("No field found"))
    }

    /// Get a struct field by name
    pub fn field_by_name(&self, name: &str) -> LookinGlassResult<StructField> {
        if self.kind != Kind::Struct {
            return Err(invalid_operation(&format!("FieldByName() called on {}", self.kind)));
        }

        self.fields.iter()
            .find(|f| f.name() == name)
            .cloned()
            .ok_or_else(|| field_error(&format!("Field '{}' not found", name)))
    }

    /// Get a struct field by name using a matching function
    pub fn field_by_name_func<F>(&self, matcher: F) -> LookinGlassResult<StructField>
    where
        F: Fn(&str) -> bool,
    {
        if self.kind != Kind::Struct {
            return Err(invalid_operation(&format!("FieldByNameFunc() called on {}", self.kind)));
        }

        self.fields.iter()
            .find(|f| matcher(f.name()))
            .cloned()
            .ok_or_else(|| field_error("No matching field found"))
    }

    /// Get the number of struct fields
    pub fn num_field(&self) -> usize {
        if self.kind == Kind::Struct {
            self.fields.len()
        } else {
            0
        }
    }

    /// Get an input parameter type by index (for Func)
    pub fn in_type(&self, i: usize) -> Option<Type> {
        if self.kind == Kind::Func {
            self.in_types.get(i).cloned()
        } else {
            None
        }
    }

    /// Get the number of input parameters (for Func)
    pub fn num_in(&self) -> usize {
        if self.kind == Kind::Func {
            self.in_types.len()
        } else {
            0
        }
    }

    /// Get an output parameter type by index (for Func)
    pub fn out_type(&self, i: usize) -> Option<Type> {
        if self.kind == Kind::Func {
            self.out_types.get(i).cloned()
        } else {
            None
        }
    }

    /// Get the number of output parameters (for Func)
    pub fn num_out(&self) -> usize {
        if self.kind == Kind::Func {
            self.out_types.len()
        } else {
            0
        }
    }

    /// Check if this function is variadic
    pub fn is_variadic(&self) -> bool {
        self.kind == Kind::Func && self.variadic
    }

    /// Check if this is a function type
    pub fn is_func(&self) -> bool {
        self.kind == Kind::Func
    }

    /// Get the key type (for Map)
    pub fn key(&self) -> LookinGlassResult<Type> {
        if self.kind != Kind::Map {
            return Err(invalid_operation(&format!("Key() called on {}", self.kind)));
        }
        
        self.key_type.as_ref()
            .map(|t| (**t).clone())
            .ok_or_else(|| type_error("Map type does not have a key type"))
    }

    /// Get the number of methods
    pub fn num_method(&self) -> usize {
        self.methods.len()
    }

    /// Get a method by index
    pub fn method(&self, i: usize) -> LookinGlassResult<Method> {
        self.methods.get(i)
            .cloned()
            .ok_or_else(|| index_error(&format!("Method index {} out of range", i)))
    }

    /// Get a method by name
    pub fn method_by_name(&self, name: &str) -> LookinGlassResult<Method> {
        self.methods.iter()
            .find(|m| m.name() == name)
            .cloned()
            .ok_or_else(|| method_error(&format!("Method '{}' not found", name)))
    }

    /// Check if this type has a method with the given name
    pub fn has_method(&self, name: &str) -> bool {
        self.methods.iter().any(|m| m.name() == name)
    }

    // Builder methods for constructing complex types

    /// Set the element type
    pub fn with_elem(mut self, elem_type: Type) -> Self {
        self.elem_type = Some(Box::new(elem_type));
        self
    }

    /// Set the key type (for maps)
    pub fn with_key(mut self, key_type: Type) -> Self {
        self.key_type = Some(Box::new(key_type));
        self
    }

    /// Set the length (for arrays)
    pub fn with_len(mut self, len: usize) -> Self {
        self.len = Some(len);
        self
    }

    /// Add a struct field
    pub fn with_field(mut self, field: StructField) -> Self {
        if self.kind == Kind::Struct {
            self.fields.push(field);
        }
        self
    }

    /// Add multiple struct fields
    pub fn with_fields(mut self, fields: Vec<StructField>) -> Self {
        if self.kind == Kind::Struct {
            self.fields.extend(fields);
        }
        self
    }

    /// Add a method
    pub fn with_method(mut self, method: Method) -> Self {
        self.methods.push(method);
        self
    }

    /// Add multiple methods
    pub fn with_methods(mut self, methods: Vec<Method>) -> Self {
        self.methods.extend(methods);
        self
    }

    /// Set input types (for functions)
    pub fn with_in_types(mut self, in_types: Vec<Type>) -> Self {
        if self.kind == Kind::Func {
            self.in_types = in_types;
        }
        self
    }

    /// Set output types (for functions)
    pub fn with_out_types(mut self, out_types: Vec<Type>) -> Self {
        if self.kind == Kind::Func {
            self.out_types = out_types;
        }
        self
    }

    /// Set whether the function is variadic
    pub fn with_variadic(mut self, variadic: bool) -> Self {
        if self.kind == Kind::Func {
            self.variadic = variadic;
        }
        self
    }

    /// Set size and alignment
    pub fn with_size_align(mut self, size: usize, align: usize) -> Self {
        self.size = size;
        self.align = align;
        self.field_align = align;
        self
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.string())
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && 
        self.name == other.name && 
        self.pkg_path == other.pkg_path
    }
}

impl Eq for Type {}

