// Object module is a placeholder for future implementation
// This file exists to satisfy imports in lib.rs

/// This module will contain the runtime object system
/// for direct interpretation (not using the bytecode compiler).
/// 
/// Currently, the main implementation uses Objects defined in compiler/mod.rs. 

/// Runtime object types to be implemented in future versions
pub enum RuntimeObject {
    /// Placeholder for future implementation
    Placeholder,
}

use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::ptr::NonNull;
use crate::prelude::{VecExt, StrExt};
use crate::prelude_ext::{RawPtrExt, VecStrJoinExt, StrCharsExt, SliceExt};
use crate::memory::gc::Trace;
use crate::memory::Traceable;
use crate::memory::Visitor;
use crate::memory::tagged::{TaggedDynPtr};
use crate::compiler::CompiledFunction;
use crate::vm::ErrorLocation;
use crate::error::Error;
use std::str;
use std::cell::RefCell;
use num_traits::{FromPrimitive, ToPrimitive};
use std::cell::{Ref, RefMut};

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
    CompiledFunction(Rc<CompiledFunction>),
    Closure {
        function: Rc<CompiledFunction>,
        free_vars: Vec<Object>,
    },
    Struct {
        name: String,
        fields: Vec<(String, String)>, // (name, type)
    },
    Instance {
        struct_type: Rc<Object>,
        fields: HashMap<String, Object>,
    },
    Error {
        message: String,
        error_type: Option<String>,
        stack_trace: Vec<ErrorLocation>,
    },
    Null,
}

impl Trace for Object {
}

impl Traceable for Object {
    fn trace(&self, visitor: &mut dyn Visitor) {
        match self {
            Object::Array(arr) => {
                for obj in arr {
                    visitor.visit(obj);
                }
            },
            Object::HashTable(map) => {
                for obj in map.values() {
                    visitor.visit(obj);
                }
            },
            Object::Closure { function, free_vars } => {
                visitor.visit_ptr(Rc::as_ptr(function) as usize, crate::memory::tagged::Tag::Function);
                for var in free_vars {
                    visitor.visit(var);
                }
            },
            Object::Instance { struct_type, fields } => {
                visitor.visit(struct_type.as_ref());
                for (_, value) in fields.iter() {
                    visitor.visit(value);
                }
            },
            _ => {}
        }
    }
    
    fn size(&self) -> usize {
        match self {
            Object::Integer(_) => std::mem::size_of::<i64>(),
            Object::Float(_) => std::mem::size_of::<f64>(),
            Object::Boolean(_) => std::mem::size_of::<bool>(),
            Object::Char(_) => std::mem::size_of::<char>(),
            Object::String(s) => std::mem::size_of::<String>() + s.capacity(),
            Object::Array(a) => std::mem::size_of::<Vec<Object>>() + (a.capacity() * std::mem::size_of::<Object>()),
            Object::HashTable(h) => {
                let key_size = h.keys().map(|k| k.capacity()).sum::<usize>();
                std::mem::size_of::<HashMap<String, Object>>() + 
                    (h.capacity() * (std::mem::size_of::<String>() + std::mem::size_of::<Object>())) +
                    key_size
            },
            Object::CompiledFunction(_) => std::mem::size_of::<Rc<CompiledFunction>>() + std::mem::size_of::<CompiledFunction>(),
            Object::Closure { function: _, free_vars } => {
                std::mem::size_of::<Rc<CompiledFunction>>() + 
                std::mem::size_of::<Vec<Object>>() + 
                (free_vars.capacity() * std::mem::size_of::<Object>())
            },
            Object::Struct { name, fields } => {
                std::mem::size_of::<String>() + name.capacity() +
                std::mem::size_of::<Vec<(String, String)>>() + 
                fields.iter().map(|(n, t)| n.capacity() + t.capacity() + std::mem::size_of::<(String, String)>()).sum::<usize>()
            },
            Object::Instance { struct_type: _, fields } => {
                std::mem::size_of::<Rc<Object>>() +
                std::mem::size_of::<HashMap<String, Object>>() +
                fields.keys().map(|k| k.capacity()).sum::<usize>() +
                (fields.capacity() * (std::mem::size_of::<String>() + std::mem::size_of::<Object>()))
            },
            Object::Error { message, error_type, stack_trace } => {
                std::mem::size_of::<String>() + message.capacity() +
                (error_type.as_ref().map_or(0, |t| t.capacity()) + std::mem::size_of::<Option<String>>()) +
                std::mem::size_of::<Vec<ErrorLocation>>() +
                (stack_trace.capacity() * std::mem::size_of::<ErrorLocation>())
            },
            Object::Null => 0,
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
            Object::CompiledFunction(func) => {
                write!(f, "function[{}]", func.name.as_ref().unwrap_or(&"anon".to_string()))
            },
            Object::Closure { function, free_vars } => {
                write!(f, "closure[{}]", function.name.as_ref().unwrap_or(&"anon".to_string()))
            },
            Object::Struct { name, .. } => write!(f, "struct[{}]", name),
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
            _ => false,
        }
    }
    
    pub fn as_traceable(&self) -> Option<NonNull<dyn Traceable>> {
        match self {
            Object::Array(_) | 
            Object::HashTable(_) |
            Object::CompiledFunction(_) |
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
            Object::Boolean(val) => *val,
            Object::Integer(val) => *val != 0,
            Object::Float(val) => *val != 0.0,
            Object::String(val) => !val.is_empty(),
            Object::Array(arr) => !arr.is_empty(),
            Object::HashTable(hash) => !hash.is_empty(),
            Object::Null => false,
            _ => true,
        }
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
            Object::CompiledFunction(_) => "function",
            Object::Closure { .. } => "closure",
            Object::Struct { .. } => "struct",
            Object::Instance { .. } => "instance",
            Object::Error { .. } => "error",
            Object::Null => "null",
        }
    }

    /// Check if the object is of the given type
    pub fn is_type(&self, type_name: &str) -> bool {
        match self {
            Object::Integer(_) => type_name == "integer",
            Object::Float(_) => type_name == "float",
            Object::Boolean(_) => type_name == "boolean",
            Object::Char(_) => type_name == "char",
            Object::String(_) => type_name == "string",
            Object::Array(_) => type_name == "array",
            Object::HashTable(_) => type_name == "hash",
            Object::CompiledFunction(_) => type_name == "function",
            Object::Closure { .. } => type_name == "closure",
            Object::Struct { .. } => type_name == "struct",
            Object::Instance { .. } => type_name == "instance",
            Object::Error { .. } => type_name == "error",
            Object::Null => type_name == "null",
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Object::Integer(i) => i.to_string(),
            Object::Float(f) => f.to_string(),
            Object::Boolean(b) => b.to_string(),
            Object::String(s) => s.clone(),
            Object::Char(c) => c.to_string(),
            Object::Array(arr) => {
                let elements: Vec<String> = arr.iter().map(|obj| obj.to_string()).collect();
                let str_elements: Vec<&str> = elements.iter().map(|s| s.as_str()).collect();
                format!("[{}]", str_elements.join(", "))
            }
            Object::HashTable(map) => {
                let entries: Vec<String> = map
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v.to_string()))
                    .collect();
                let str_entries: Vec<&str> = entries.iter().map(|s| s.as_str()).collect();
                format!("{{{}}}", str_entries.join(", "))
            }
            Object::CompiledFunction(func) => format!("fn<{}>", func.name.as_ref().unwrap_or(&"anonymous".to_string())),
            Object::Closure { function, free_vars: _ } => {
                format!("closure<{}>", function.name.as_ref().unwrap_or(&"anonymous".to_string()))
            }
            Object::Struct { name, fields } => {
                let field_strs: Vec<String> = fields
                    .iter()
                    .map(|(field_name, field_type)| format!("{}: {}", field_name, field_type))
                    .collect();
                let str_fields: Vec<&str> = field_strs.iter().map(|s| s.as_str()).collect();
                format!("struct {}{{ {} }}", name, str_fields.join(", "))
            }
            Object::Instance { struct_type, fields: _ } => {
                let str_type = match &**struct_type {
                    Object::Struct { name, .. } => name.clone(),
                    _ => "UnknownType".to_string(),
                };
                format!("{}::instance", str_type)
            }
            Object::Error {
                message,
                error_type,
                ..
            } => {
                if let Some(t) = error_type {
                    format!("error<{}>: {}", t, message)
                } else {
                    format!("error: {}", message)
                }
            }
            Object::Null => "null".to_string(),
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
            Object::CompiledFunction(function) => Some(function.clone()),
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
            _ => {}
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
        Object::CompiledFunction(val)
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
            Object::CompiledFunction(_) |
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