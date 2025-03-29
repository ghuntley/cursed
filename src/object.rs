// Object module is a placeholder for future implementation
// This file exists to satisfy imports in lib.rs

/// This module will contain the runtime object system
/// for direct interpretation (not using the bytecode compiler). 

/// Runtime object types to be implemented in future versions
pub enum RuntimeObject {
    /// Placeholder for future implementation
    Placeholder,
}

use std::rc::Rc;
use std::ptr::NonNull;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::fmt;
use std::fmt::{Display, Formatter, Debug};
use std::cell::RefCell;
use crate::prelude::{VecExt, StrExt, VecStrJoinExt};
// use crate::prelude_ext::{RawPtrExt, VecStrJoinExt, StrCharsExt, SliceExt};
use crate::memory::gc::Trace;
use crate::memory::Traceable;
use crate::memory::Visitor;
use crate::core::CompiledFunction;
use crate::error::Error;
use std::str;
use std::mem;

/// A location in the code for error reporting
#[derive(Debug, Clone, PartialEq)]
pub struct ErrorLocation {
    pub ip: usize,
    pub function_name: Option<String>,
    pub line: Option<usize>,
    pub column: Option<usize>,
}

impl ErrorLocation {
    pub fn new(ip: usize) -> Self {
        Self {
            ip,
            function_name: None,
            line: None,
            column: None,
        }
    }
    
    pub fn with_function(ip: usize, function_name: String) -> Self {
        Self {
            ip,
            function_name: Some(function_name),
            line: None,
            column: None,
        }
    }
    
    pub fn with_location(ip: usize, function_name: Option<String>, line: usize, column: usize) -> Self {
        Self {
            ip,
            function_name,
            line: Some(line),
            column: Some(column),
        }
    }
}

/// Object represents a runtime value
#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    Char(char),
    Array(Vec<Object>),
    HashTable(HashMap<String, Object>),
    CompiledFunction {
        ir_representation: String,
        num_locals: usize,
        num_parameters: usize,
        free_variables: Vec<Object>,
        name: Option<String>,
        is_variadic: bool,
    },
    Closure {
        function: Rc<CompiledFunction>,
        free_vars: Vec<Object>,
    },
    Builtin {
        name: String,
        function: BuiltinFunction,
    },
    Struct {
        name: String,
        fields: Vec<(String, String)>, // (name, type)
    },
    Instance {
        struct_type: Rc<Object>,
        fields: HashMap<String, Object>,
    },
    Interface {
        name: String,
        methods: Vec<(String, Vec<(String, String)>, Option<String>)>, // (method_name, parameters [(name, type)], return_type)
    },
    Method {
        receiver_type: String, // The type this method belongs to
        name: String, // Method name
        parameters: Vec<(String, String)>, // Parameters (name, type)
        return_type: Option<String>, // Optional return type
        function: Rc<CompiledFunction>, // The compiled method body
    },
    Error {
        message: String,
        error_type: Option<String>,
        stack_trace: Vec<ErrorLocation>,
    },
    Null,
}

/// Builtin function type for the CURSED language
pub type BuiltinFunction = fn(args: &[Rc<Object>]) -> Result<Rc<Object>, Error>;

impl Trace for Object {
    // Trace is an empty marker trait, so we don't need to implement any methods
}

impl Traceable for Object {
    fn trace(&self, visitor: &mut dyn Visitor) {
        match self {
            Object::Integer(_) |
            Object::Float(_) |
            Object::Boolean(_) |
            Object::String(_) |
            Object::Char(_) |
            Object::Null => {
                // These types don't contain any references to trace
            },
            Object::Array(elements) => {
                // Trace array elements
                for element in elements {
                    element.trace(visitor);
                }
            },
            Object::HashTable(entries) => {
                // Trace hash table entries
                for (_, value) in entries {
                    value.trace(visitor);
                }
            },
            Object::CompiledFunction { .. } => {
                // CompiledFunction doesn't implement Traceable
            },
            Object::Closure { function: _, free_vars } => {
                // function doesn't implement Traceable
                for var in free_vars {
                    var.trace(visitor);
                }
            },
            Object::Builtin { .. } => {
                // Builtins don't have any references to trace
            },
            Object::Struct { .. } => {
                // Type definitions don't have any references to trace
            },
            Object::Interface { .. } => {
                // Interface definitions don't have any references to trace
            },
            Object::Instance { struct_type, fields } => {
                // Trace the struct type
                struct_type.trace(visitor);
                // Trace field values
                for (_, value) in fields {
                    value.trace(visitor);
                }
            },
            Object::Error { stack_trace: _, .. } => {
                // ErrorLocation doesn't implement Traceable
            },
            Object::Method { .. } => {
                // Method doesn't contain references to trace
            },
        }
    }
    
    fn size(&self) -> usize {
        match self {
            Object::Integer(_) => std::mem::size_of::<i64>(),
            Object::Float(_) => std::mem::size_of::<f64>(),
            Object::Boolean(_) => std::mem::size_of::<bool>(),
            Object::Char(_) => std::mem::size_of::<char>(),
            Object::String(s) => std::mem::size_of::<String>() + s.len(),
            Object::Array(elements) => {
                let mut size = std::mem::size_of::<Vec<Object>>();
                for element in elements {
                    size += element.size();
                }
                size
            },
            Object::HashTable(entries) => {
                let mut size = std::mem::size_of::<HashMap<String, Object>>();
                for (key, value) in entries {
                    size += key.len() + value.size();
                }
                size
            },
            Object::CompiledFunction { ir_representation, .. } => {
                std::mem::size_of::<String>() + ir_representation.len()
            },
            Object::Closure { function, free_vars } => {
                let mut size = std::mem::size_of::<Rc<CompiledFunction>>() + function.ir_representation.len();
                for var in free_vars {
                    size += var.size();
                }
                size
            },
            Object::Builtin { name, .. } => {
                std::mem::size_of::<String>() + name.len() + std::mem::size_of::<fn(Vec<Object>) -> Result<Object, Error>>()
            },
            Object::Struct { name, fields } => {
                let mut size = std::mem::size_of::<String>() + name.len();
                for (field_name, field_type) in fields {
                    size += field_name.len() + field_type.len();
                }
                size
            },
            Object::Interface { name, methods } => {
                let mut size = std::mem::size_of::<String>() + name.len();
                for (method_name, params, return_type) in methods {
                    size += method_name.len();
                    for (param_name, param_type) in params {
                        size += param_name.len() + param_type.len();
                    }
                    if let Some(ret_type) = return_type {
                        size += ret_type.len();
                    }
                }
                size
            },
            Object::Instance { struct_type, fields } => {
                let mut size = std::mem::size_of::<Rc<Object>>() + struct_type.size();
                for (key, value) in fields {
                    size += key.len() + value.size();
                }
                size
            },
            Object::Error { message, error_type, stack_trace } => {
                let mut size = std::mem::size_of::<String>() + message.len();
                if let Some(error_type) = error_type {
                    size += error_type.len();
                }
                size += std::mem::size_of::<Vec<ErrorLocation>>() + stack_trace.len() * std::mem::size_of::<ErrorLocation>();
                size
            },
            Object::Method { receiver_type, name, parameters, return_type, function } => {
                let mut size = std::mem::size_of::<String>() + receiver_type.len() + name.len();
                for (param_name, param_type) in parameters {
                    size += param_name.len() + param_type.len();
                }
                if let Some(ret_type) = return_type {
                    size += ret_type.len();
                }
                size += std::mem::size_of::<Rc<CompiledFunction>>();
                size
            },
            Object::Null => std::mem::size_of::<()>(),
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Object::Integer(val) => write!(f, "{}", val),
            Object::Float(val) => write!(f, "{}", val),
            Object::Boolean(val) => write!(f, "{}", val),
            Object::Char(val) => write!(f, "'{}'", val),
            Object::String(val) => write!(f, "{}", val),
            Object::Array(arr) => {
                write!(f, "[")?;
                for (i, obj) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", obj)?;
                }
                write!(f, "]")
            },
            Object::HashTable(map) => {
                write!(f, "{{")?;
                for (i, (key, val)) in map.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", key, val)?;
                }
                write!(f, "}}")
            },
            Object::CompiledFunction { name, .. } => {
                if let Some(name) = name {
                    write!(f, "[Function: {}]", name)
                } else {
                    write!(f, "[Function]")
                }
            },
            Object::Closure { function, free_vars } => {
                write!(f, "closure[{}]", function.name.as_ref().unwrap_or(&"anon".to_string()))
            },
            Object::Builtin { name, .. } => {
                write!(f, "builtin[{}]", name)
            },
            Object::Struct { name, .. } => write!(f, "struct[{}]", name),
            Object::Interface { name, .. } => write!(f, "interface[{}]", name),
            Object::Instance { struct_type, .. } => {
                if let Object::Struct { name, .. } = struct_type.as_ref() {
                    write!(f, "instance[{}]", name)
                } else {
                    write!(f, "instance[unknown]")
                }
            },
            Object::Error { message, error_type, .. } => {
                if let Some(err_type) = error_type {
                    write!(f, "{}Error: {}", err_type, message)
                } else {
                    write!(f, "Error: {}", message)
                }
            },
            Object::Null => write!(f, "null"),
            Object::Method { receiver_type, name, parameters, return_type, .. } => {
                let params_str = parameters
                    .iter()
                    .map(|(param_name, param_type)| format!("{}: {}", param_name, param_type))
                    .collect::<Vec<String>>()
                    .join(", ");
                
                let return_str = match return_type {
                    Some(ret) => format!(": {}", ret),
                    None => String::new(),
                };
                
                write!(f, "method {}:{}({}){}{{ ... }}", receiver_type, name, params_str, return_str)
            },
        }
    }
}

impl Object {
    pub fn is_null(&self) -> bool {
        match self {
            Object::Null => true,
            _ => false,
        }
    }
    
    pub fn is_hashable(&self) -> bool {
        match self {
            Object::Integer(_) => true,
            Object::String(_) => true,
            Object::Boolean(_) => true,
            Object::Char(_) => true,
            Object::CompiledFunction { .. } => {
                // Functions aren't hashable
                false
            },
            _ => false,
        }
    }
    
    pub fn as_traceable(&self) -> Option<NonNull<dyn Traceable>> {
        match self {
            Object::Array(_) | 
            Object::HashTable(_) |
            Object::CompiledFunction { .. } |
            Object::Closure { .. } |
            Object::Instance { .. } => {
                // Using a safer approach for casting to trait object
                let reference: &dyn Traceable = self;
                let ptr = reference as *const dyn Traceable as *mut dyn Traceable;
                // A reference is never null, so we can safely create a NonNull
                unsafe { Some(NonNull::new_unchecked(ptr)) }
            },
            _ => None
        }
    }
    
    pub fn is_struct(&self) -> bool {
        matches!(self, Object::Struct { .. })
    }
    
    /// Get the type name of this object
    pub fn type_name(&self) -> &'static str {
        match self {
            Object::Integer(_) => "integer",
            Object::Float(_) => "float",
            Object::Boolean(_) => "boolean",
            Object::Char(_) => "char",
            Object::String(_) => "string",
            Object::Array(_) => "array",
            Object::HashTable(_) => "hash",
            Object::CompiledFunction { .. } => "function",
            Object::Closure { .. } => "closure",
            Object::Builtin { .. } => "builtin",
            Object::Struct { .. } => "struct",
            Object::Interface { .. } => "interface",
            Object::Method { .. } => "method",
            Object::Instance { .. } => "instance",
            Object::Error { .. } => "error",
            Object::Null => "null",
        }
    }
    
    /// Check if the object is of the given type
    pub fn is_type(&self, type_name: &str) -> bool {
        match (self, type_name) {
            (Object::Integer(_), "integer") => true,
            (Object::Float(_), "float") => true,
            (Object::Boolean(_), "boolean") => true,
            (Object::String(_), "string") => true,
            (Object::Char(_), "char") => true,
            (Object::Array(_), "array") => true,
            (Object::HashTable(_), "hash") => true,
            (Object::CompiledFunction { .. }, "function") => true,
            (Object::Closure { .. }, "closure") => true,
            (Object::Builtin { .. }, "builtin") => true,
            (Object::Struct { .. }, "struct") => true,
            (Object::Interface { .. }, "interface") => true,
            (Object::Method { .. }, "method") => true,
            (Object::Instance { .. }, "instance") => true,
            (Object::Error { .. }, "error") => true,
            (Object::Null, "null") => true,
            _ => false,
        }
    }
    
    pub fn is_instance(&self) -> bool {
        matches!(self, Object::Instance { .. })
    }
    
    pub fn get_field(&self, field_name: &str) -> Option<Object> {
        match self {
            Object::Instance { fields, .. } => fields.get(field_name).cloned(),
            _ => None,
        }
    }
    
    pub fn set_field(&mut self, field_name: &str, value: Object) -> Result<(), Error> {
        match self {
            Object::Instance { fields, .. } => {
                fields.insert(field_name.to_string(), value);
                Ok(())
            },
            _ => Err(Error::Runtime(format!("Cannot set field on non-instance object"))),
        }
    }
    
    pub fn as_integer(&self) -> Option<i64> {
        match self {
            Object::Integer(val) => Some(*val),
            _ => None,
        }
    }
    
    pub fn as_float(&self) -> Option<f64> {
        match self {
            Object::Float(val) => Some(*val),
            Object::Integer(val) => Some(*val as f64),
            _ => None,
        }
    }
    
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Object::Boolean(val) => Some(*val),
            _ => None,
        }
    }
    
    pub fn as_string(&self) -> Option<&String> {
        match self {
            Object::String(val) => Some(val),
            _ => None,
        }
    }
    
    pub fn is_truthy(&self) -> bool {
        match self {
            Object::Integer(n) => *n != 0,
            Object::Float(n) => *n != 0.0,
            Object::Boolean(b) => *b,
            Object::String(s) => !s.is_empty(),
            Object::Char(_) => true,
            Object::Array(a) => !a.is_empty(),
            Object::HashTable(h) => !h.is_empty(),
            Object::CompiledFunction { .. } => true,
            Object::Closure { .. } => true,
            Object::Builtin { .. } => true,
            Object::Struct { .. } => true,
            Object::Interface { .. } => true,
            Object::Method { .. } => true,
            Object::Instance { .. } => true,
            Object::Error { .. } => false,
            Object::Null => false,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Object::Integer(n) => n.to_string(),
            Object::Float(f) => f.to_string(),
            Object::Boolean(b) => b.to_string(),
            Object::String(s) => s.clone(),
            Object::Char(c) => c.to_string(),
            Object::Array(arr) => {
                let elements: Vec<String> = arr.iter().map(|obj| obj.to_string()).collect();
                format!("[{}]", elements.join(", "))
            },
            Object::HashTable(map) => {
                let entries: Vec<String> = map.iter().map(|(k, v)| format!("{}: {}", k, v.to_string())).collect();
                format!("{{{}}}", entries.join(", "))
            },
            Object::CompiledFunction { name, .. } => {
                if let Some(name) = name {
                    format!("[Function: {}]", name)
                } else {
                    "[Function]".to_string()
                }
            },
            Object::Closure { function, .. } => {
                if let Some(ref name) = function.name {
                    format!("closure[{}]", name)
                } else {
                    "closure[anonymous]".to_string()
                }
            },
            Object::Builtin { name, .. } => {
                format!("builtin function: {}", name)
            },
            Object::Struct { name, .. } => format!("struct {}", name),
            Object::Interface { name, .. } => format!("interface {}", name),
            Object::Instance { struct_type, .. } => {
                if let Object::Struct { name, .. } = struct_type.as_ref() {
                    format!("instance[{}]", name)
                } else {
                    "instance[unknown]".to_string()
                }
            },
            Object::Error { message, error_type, .. } => {
                if let Some(err_type) = error_type {
                    format!("{}Error: {}", err_type, message)
                } else {
                    format!("Error: {}", message)
                }
            },
            Object::Null => "null".to_string(),
            Object::Method { receiver_type, name, parameters, return_type, .. } => {
                let params_str = parameters
                    .iter()
                    .map(|(param_name, param_type)| format!("{}: {}", param_name, param_type))
                    .collect::<Vec<String>>()
                    .join(", ");
                
                let return_str = match return_type {
                    Some(ret) => format!(": {}", ret),
                    None => String::new(),
                };
                
                format!("method {}:{}({}){}{{ ... }}", receiver_type, name, params_str, return_str)
            },
        }
    }

    pub fn to_integer(&self) -> Option<i64> {
        match self {
            Object::Integer(val) => Some(*val),
            Object::Float(val) => Some(*val as i64),
            Object::String(val) => {
                use std::str::FromStr;
                i64::from_str(val.as_str()).ok()
            },
            _ => None,
        }
    }

    pub fn to_float(&self) -> Option<f64> {
        match self {
            Object::Float(val) => Some(*val),
            Object::Integer(val) => Some(*val as f64),
            Object::String(val) => {
                use std::str::FromStr;
                f64::from_str(val.as_str()).ok()
            },
            _ => None,
        }
    }

    pub fn to_bool(&self) -> Option<bool> {
        match self {
            Object::Boolean(val) => Some(*val),
            Object::String(val) => {
                match val.as_str() {
                    "true" => Some(true),
                    "false" => Some(false),
                    _ => None,
                }
            },
            _ => None,
        }
    }

    pub fn to_array(&self) -> Option<Vec<Object>> {
        match self {
            Object::Array(arr) => Some(arr.clone()),
            Object::String(s) => {
                // Convert string to array of character objects using std::string::String::chars
                let chars: Vec<Object> = s.as_str().chars().map(|c| Object::Char(c)).collect();
                Some(chars)
            }
            _ => None,
        }
    }

    pub fn to_hash(&self) -> Option<HashMap<String, Object>> {
        match self {
            Object::HashTable(hash) => Some(hash.clone()),
            _ => None,
        }
    }

    pub fn to_error(&self) -> Option<(String, Option<String>, Vec<ErrorLocation>)> {
        match self {
            Object::Error { message, error_type, stack_trace } => {
                Some((message.clone(), error_type.clone(), stack_trace.clone()))
            },
            _ => None,
        }
    }

    pub fn to_struct(&self) -> Option<(String, Vec<(String, String)>)> {
        match self {
            Object::Struct { name, fields } => {
                Some((name.clone(), fields.clone()))
            },
            _ => None,
        }
    }

    pub fn to_instance(&self) -> Option<(Rc<Object>, HashMap<String, Object>)> {
        match self {
            Object::Instance { struct_type, fields } => {
                Some((struct_type.clone(), fields.clone()))
            },
            _ => None,
        }
    }

    pub fn to_closure(&self) -> Option<(Rc<CompiledFunction>, Vec<Object>)> {
        match self {
            Object::Closure { function, free_vars } => {
                Some((function.clone(), free_vars.clone()))
            },
            _ => None,
        }
    }

    pub fn to_function(&self) -> Option<Rc<CompiledFunction>> {
        match self {
            Object::CompiledFunction { ir_representation, num_locals, num_parameters, free_variables, name, is_variadic } => {
                let func = CompiledFunction {
                    ir_representation: ir_representation.clone(),
                    num_locals: *num_locals,
                    num_parameters: *num_parameters,
                    free_variables: free_variables.iter().map(|obj| {
                        match obj {
                            Object::String(s) => s.clone(),
                            _ => obj.to_string()
                        }
                    }).collect(),
                    name: name.clone(),
                    is_variadic: *is_variadic,
                };
                Some(Rc::new(func))
            },
            _ => None,
        }
    }

    /// Get the chars of a string object
    pub fn chars(&self) -> Option<Vec<Object>> {
        match self {
            Object::String(s) => {
                // Use the StrCharsExt trait to access chars method
                let chars: Vec<Object> = s.as_str().chars().map(|c| Object::Char(c)).collect();
                Some(chars)
            }
            _ => None,
        }
    }

    /// Trace object references for the garbage collector
    pub fn trace_object_references(&self, visitor: &mut dyn Visitor) {
        match self {
            Object::Array(elements) => {
                for obj in elements {
                    let ptr = obj as *const Object as usize;
                    visitor.visit_ptr(ptr, crate::memory::tagged::Tag::Object);
                }
            },
            Object::HashTable(map) => {
                for (key, value) in map {
                    let key_ptr = key as *const String as usize;
                    visitor.visit_ptr(key_ptr, crate::memory::tagged::Tag::String);
                    
                    let value_ptr = value as *const Object as usize;
                    visitor.visit_ptr(value_ptr, crate::memory::tagged::Tag::Object);
                }
            },
            Object::Closure { function, free_vars } => {
                let func_ptr = Rc::as_ptr(function) as usize;
                visitor.visit_ptr(func_ptr, crate::memory::tagged::Tag::Function);
                
                for var in free_vars {
                    let var_ptr = var as *const Object as usize;
                    visitor.visit_ptr(var_ptr, crate::memory::tagged::Tag::Object);
                }
            },
            Object::Instance { struct_type, fields } => {
                let type_ptr = Rc::as_ptr(struct_type) as usize;
                visitor.visit_ptr(type_ptr, crate::memory::tagged::Tag::Object);
                
                for (_, value) in fields {
                    let value_ptr = value as *const Object as usize;
                    visitor.visit_ptr(value_ptr, crate::memory::tagged::Tag::Object);
                }
            },
            Object::Interface { .. } => {
                // Interface objects don't contain references that need tracing
            },
            Object::Method { function, .. } => {
                let func_ptr = Rc::as_ptr(function) as usize;
                visitor.visit_ptr(func_ptr, crate::memory::tagged::Tag::Function);
            },
            _ => {}
        }
    }

    /// Checks if the object is exactly of the given type
    pub fn type_check_exact(&self, type_name: &str) -> bool {
        match self {
            Object::Integer(_) => type_name == "integer",
            Object::Float(_) => type_name == "float",
            Object::Boolean(_) => type_name == "boolean",
            Object::String(_) => type_name == "string",
            Object::Char(_) => type_name == "char",
            Object::Array(_) => type_name == "array",
            Object::HashTable(_) => type_name == "hash",
            Object::CompiledFunction { .. } => type_name == "function",
            Object::Closure { .. } => type_name == "closure",
            Object::Builtin { .. } => type_name == "builtin",
            Object::Struct { .. } => type_name == "struct",
            Object::Interface { .. } => type_name == "interface",
            Object::Method { .. } => type_name == "method",
            Object::Instance { .. } => type_name == "instance",
            Object::Error { .. } => type_name == "error",
            Object::Null => type_name == "null",
        }
    }

    /// Returns a string representation of the object
    pub fn inspect(&self) -> String {
        match self {
            Object::Integer(value) => value.to_string(),
            Object::Float(value) => value.to_string(),
            Object::Boolean(value) => value.to_string(),
            Object::String(value) => format!("\"{}\"", value),
            Object::Char(value) => format!("'{}'", value),
            Object::Array(elements) => {
                let elements_str = elements
                    .iter()
                    .map(|e| e.inspect())
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("[{}]", elements_str)
            },
            Object::HashTable(pairs) => {
                let mut entries: Vec<String> = Vec::new();
                for (key, value) in pairs {
                    entries.push(format!("\"{}\": {}", key, value.inspect()));
                }
                format!("{{{}}}", entries.join(", "))
            },
            Object::CompiledFunction { name, .. } => {
                if let Some(name) = name {
                    format!("[Function: {}]", name)
                } else {
                    "[Function]".to_string()
                }
            },
            Object::Closure { function, free_vars } => {
                let free_vars_str = free_vars
                    .iter()
                    .map(|v| v.inspect())
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("Closure[function={:p}, free_vars=[{}]]", Rc::as_ptr(&function), free_vars_str)
            },
            Object::Builtin { name, .. } => {
                format!("Builtin[{}]", name)
            },
            Object::Struct { name, fields } => {
                let fields_str = fields
                    .iter()
                    .map(|(name, type_name)| format!("{}: {}", name, type_name))
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("struct {}{{ {} }}", name, fields_str)
            },
            Object::Interface { name, methods } => {
                let methods_str = methods
                    .iter()
                    .map(|(method_name, params, ret_type)| {
                        let params_str = params
                            .iter()
                            .map(|(param_name, param_type)| format!("{}: {}", param_name, param_type))
                            .collect::<Vec<String>>()
                            .join(", ");
                        
                        if let Some(return_type) = ret_type {
                            format!("{}({}): {}", method_name, params_str, return_type)
                        } else {
                            format!("{}({})", method_name, params_str)
                        }
                    })
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("interface {}{{ {} }}", name, methods_str)
            },
            Object::Method { receiver_type, name, parameters, return_type, function } => {
                let params_str = parameters
                    .iter()
                    .map(|(param_name, param_type)| format!("{}: {}", param_name, param_type))
                    .collect::<Vec<String>>()
                    .join(", ");
                
                let return_str = match return_type {
                    Some(ret) => format!(": {}", ret),
                    None => String::new(),
                };
                
                format!("method {}:{}({}){}{{ ... }}", receiver_type, name, params_str, return_str)
            },
            Object::Instance { struct_type, fields } => {
                let type_name = match &**struct_type {
                    Object::Struct { name, .. } => name.clone(),
                    _ => "Unknown".to_string(),
                };
                
                let fields_str = fields
                    .iter()
                    .map(|(name, value)| format!("{}: {}", name, value.inspect()))
                    .collect::<Vec<String>>()
                    .join(", ");
                
                format!("{}{{ {} }}", type_name, fields_str)
            },
            Object::Error { message, error_type, .. } => {
                if let Some(err_type) = error_type {
                    format!("Error: {} ({})", message, err_type)
                } else {
                    format!("Error: {}", message)
                }
            },
            Object::Null => "null".to_string(),
        }
    }
}

impl Default for Object {
    fn default() -> Self {
        Object::Null
    }
}

impl From<i64> for Object {
    fn from(val: i64) -> Self {
        Object::Integer(val)
    }
}

impl From<f64> for Object {
    fn from(val: f64) -> Self {
        Object::Float(val)
    }
}

impl From<bool> for Object {
    fn from(val: bool) -> Self {
        Object::Boolean(val)
    }
}

impl From<String> for Object {
    fn from(val: String) -> Self {
        Object::String(val)
    }
}

impl From<&str> for Object {
    fn from(val: &str) -> Self {
        Object::String(val.to_string())
    }
}

impl From<Vec<Object>> for Object {
    fn from(val: Vec<Object>) -> Self {
        Object::Array(val)
    }
}

impl From<HashMap<String, Object>> for Object {
    fn from(val: HashMap<String, Object>) -> Self {
        Object::HashTable(val)
    }
}

impl From<Rc<CompiledFunction>> for Object {
    fn from(val: Rc<CompiledFunction>) -> Self {
        Object::CompiledFunction {
            ir_representation: val.ir_representation.clone(),
            num_locals: val.num_locals,
            num_parameters: val.num_parameters,
            free_variables: val.free_variables.iter()
                .map(|s| Object::String(s.clone()))
                .collect(),
            name: val.name.clone(),
            is_variadic: val.is_variadic,
        }
    }
}

impl From<(String, Option<String>, Vec<ErrorLocation>)> for Object {
    fn from(val: (String, Option<String>, Vec<ErrorLocation>)) -> Self {
        Object::Error {
            message: val.0,
            error_type: val.1,
            stack_trace: val.2,
        }
    }
}

impl From<(String, Vec<(String, String)>)> for Object {
    fn from(val: (String, Vec<(String, String)>)) -> Self {
        Object::Struct {
            name: val.0,
            fields: val.1,
        }
    }
}

impl From<(Rc<Object>, HashMap<String, Object>)> for Object {
    fn from(val: (Rc<Object>, HashMap<String, Object>)) -> Self {
        Object::Instance {
            struct_type: val.0,
            fields: val.1,
        }
    }
}

impl From<(Rc<CompiledFunction>, Vec<Object>)> for Object {
    fn from(val: (Rc<CompiledFunction>, Vec<Object>)) -> Self {
        Object::Closure {
            function: val.0,
            free_vars: val.1,
        }
    }
}

impl From<char> for Object {
    fn from(val: char) -> Self {
        Object::Char(val)
    }
}

// Extension method for accessing Object as traceable
pub trait ObjectTraceableExt {
    fn as_traceable(&self) -> Option<NonNull<dyn Traceable>>;
}

impl ObjectTraceableExt for Object {
    fn as_traceable(&self) -> Option<NonNull<dyn Traceable>> {
        match self {
            Object::Array(_) | 
            Object::HashTable(_) |
            Object::CompiledFunction { .. } |
            Object::Closure { .. } |
            Object::Instance { .. } => {
                // Using a safer approach for casting to trait object
                let reference: &dyn Traceable = self;
                let ptr = reference as *const dyn Traceable as *mut dyn Traceable;
                // A reference is never null, so we can safely create a NonNull
                unsafe { Some(NonNull::new_unchecked(ptr)) }
            },
            _ => None
        }
    }
}

