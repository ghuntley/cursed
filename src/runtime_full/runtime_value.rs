/// Runtime value types for the CURSED programming language
/// 
/// This module provides runtime-specific value types that extend the basic Value
/// type with additional runtime context and capabilities.

use crate::runtime::value::Value;
use crate::error::SourceLocation;
use std::collections::HashMap;
use std::fmt;

/// Runtime value with additional context and metadata
#[derive(Debug, Clone)]
pub struct RuntimeValue {
    /// The underlying value
    pub value: Value,
    /// Source location where this value was created
    pub location: Option<SourceLocation>,
    /// Type information for runtime type checking
    pub type_info: Option<TypeInfo>,
    /// Metadata for debugging and runtime features
    pub metadata: HashMap<String, String>,
}

impl RuntimeValue {
    /// Create a new runtime value
    pub fn new(value: Value) -> Self {
        Self {
            value,
            location: None,
            type_info: None,
            metadata: HashMap::new(),
        }
    }

    /// Create a runtime value with location
    pub fn with_location(value: Value, location: SourceLocation) -> Self {
        Self {
            value,
            location: Some(location),
            type_info: None,
            metadata: HashMap::new(),
        }
    }

    /// Create a runtime value with type information
    pub fn with_type_info(value: Value, type_info: TypeInfo) -> Self {
        Self {
            value,
            location: None,
            type_info: Some(type_info),
            metadata: HashMap::new(),
        }
    }

    /// Add metadata to this runtime value
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Get the underlying value
    pub fn get_value(&self) -> &Value {
        &self.value
    }

    /// Get mutable reference to the underlying value
    pub fn get_value_mut(&mut self) -> &mut Value {
        &mut self.value
    }

    /// Check if this value has type information
    pub fn has_type_info(&self) -> bool {
        self.type_info.is_some()
    }

    /// Get the type name of this value
    pub fn type_name(&self) -> String {
        if let Some(type_info) = &self.type_info {
            type_info.name.clone()
        } else {
            self.value.type_name().to_string()
        }
    }

    /// Convert to string representation
    pub fn to_string(&self) -> String {
        self.value.to_string()
    }

    /// Check if this value is null
    pub fn is_null(&self) -> bool {
        self.value.is_null()
    }

    /// Get location information
    pub fn get_location(&self) -> Option<&SourceLocation> {
        self.location.as_ref()
    }
}

impl fmt::Display for RuntimeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<Value> for RuntimeValue {
    fn from(value: Value) -> Self {
        RuntimeValue::new(value)
    }
}

impl From<RuntimeValue> for Value {
    fn from(runtime_value: RuntimeValue) -> Self {
        runtime_value.value
    }
}

/// Type information for runtime values
#[derive(Debug, Clone)]
pub struct TypeInfo {
    /// Name of the type
    pub name: String,
    /// Generic type parameters if applicable
    pub type_parameters: Vec<String>,
    /// Constraints on the type
    pub constraints: Vec<String>,
    /// Whether this type is a reference type
    pub is_reference: bool,
    /// Size information if known
    pub size: Option<usize>,
}

impl TypeInfo {
    /// Create basic type info
    pub fn new(name: String) -> Self {
        Self {
            name,
            type_parameters: Vec::new(),
            constraints: Vec::new(),
            is_reference: false,
            size: None,
        }
    }

    /// Create type info with generic parameters
    pub fn with_generics(name: String, type_parameters: Vec<String>) -> Self {
        Self {
            name,
            type_parameters,
            constraints: Vec::new(),
            is_reference: false,
            size: None,
        }
    }

    /// Mark this type as a reference type
    pub fn as_reference(mut self) -> Self {
        self.is_reference = true;
        self
    }

    /// Add size information
    pub fn with_size(mut self, size: usize) -> Self {
        self.size = Some(size);
        self
    }

    /// Get the full type name including generics
    pub fn full_name(&self) -> String {
        if self.type_parameters.is_empty() {
            self.name.clone()
        } else {
            format!("{}<{}>", self.name, self.type_parameters.join(", "))
        }
    }
}

/// Common runtime value constructors
impl RuntimeValue {
    /// Create a null runtime value
    pub fn null() -> Self {
        RuntimeValue::new(Value::null())
    }

    /// Create a boolean runtime value
    pub fn bool(value: bool) -> Self {
        RuntimeValue::new(Value::bool(value))
    }

    /// Create an integer runtime value
    pub fn integer(value: i64) -> Self {
        RuntimeValue::new(Value::integer(value))
    }

    /// Create a number runtime value
    pub fn number(value: f64) -> Self {
        RuntimeValue::new(Value::number(value))
    }

    /// Create a string runtime value
    pub fn string<S: Into<String>>(value: S) -> Self {
        RuntimeValue::new(Value::string(value))
    }

    /// Create an array runtime value
    pub fn array(values: Vec<RuntimeValue>) -> Self {
        let values: Vec<Value> = values.into_iter().map(|rv| rv.value).collect();
        RuntimeValue::new(Value::array(values))
    }

    /// Create an object runtime value
    pub fn object(map: HashMap<String, RuntimeValue>) -> Self {
        let map: HashMap<String, Value> = map.into_iter()
            .map(|(k, rv)| (k, rv.value))
            .collect();
        RuntimeValue::new(Value::object(map))
    }
}

/// Specialized runtime value types
#[derive(Debug, Clone)]
pub enum SpecializedRuntimeValue {
    /// Regular runtime value
    Regular(RuntimeValue),
    /// Function value with callable information
    Function {
        name: String,
        parameters: Vec<String>,
        return_type: Option<String>,
        body: Option<Box<RuntimeValue>>,
    },
    /// Object instance with type information
    Instance {
        type_name: String,
        fields: HashMap<String, RuntimeValue>,
    },
    /// Reference to another value
    Reference {
        target: Box<RuntimeValue>,
        is_mutable: bool,
    },
    /// Channel value for goroutine communication
    Channel {
        element_type: String,
        capacity: Option<usize>,
    },
}

impl SpecializedRuntimeValue {
    /// Get the underlying runtime value if applicable
    pub fn as_runtime_value(&self) -> Option<&RuntimeValue> {
        match self {
            SpecializedRuntimeValue::Regular(rv) => Some(rv),
            SpecializedRuntimeValue::Reference { target, .. } => Some(target),
            _ => None,
        }
    }

    /// Convert to string representation
    pub fn to_string(&self) -> String {
        match self {
            SpecializedRuntimeValue::Regular(rv) => rv.to_string(),
            SpecializedRuntimeValue::Function { name, .. } => format!("function {}", name),
            SpecializedRuntimeValue::Instance { type_name, .. } => format!("instance of {}", type_name),
            SpecializedRuntimeValue::Reference { target, is_mutable } => {
                if *is_mutable {
                    format!("&mut {}", target.to_string())
                } else {
                    format!("&{}", target.to_string())
                }
            }
            SpecializedRuntimeValue::Channel { element_type, capacity } => {
                if let Some(cap) = capacity {
                    format!("chan<{}>({})", element_type, cap)
                } else {
                    format!("chan<{}>", element_type)
                }
            }
        }
    }
}

impl fmt::Display for SpecializedRuntimeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

