/// Core reflection functions for LookinGlass
// use crate::stdlib::lookin_glass::{Type, Value, Kind, error::*};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// TypeOf returns the reflection Type of the value
pub fn type_of_any<T: Any + 'static>(_value: &T) -> Type {
    let type_id = TypeId::of::<T>();
    
    // Map common Rust types to CURSED reflection types
    if type_id == TypeId::of::<bool>() {
        Type::basic(Kind::Bool)
    } else if type_id == TypeId::of::<i8>() {
        Type::basic(Kind::Int8)
    } else if type_id == TypeId::of::<i16>() {
        Type::basic(Kind::Int16)
    } else if type_id == TypeId::of::<i32>() {
        Type::basic(Kind::Int32)
    } else if type_id == TypeId::of::<i64>() {
        Type::basic(Kind::Int64)
    } else if type_id == TypeId::of::<isize>() {
        Type::basic(Kind::Int)
    } else if type_id == TypeId::of::<u8>() {
        Type::basic(Kind::Uint8)
    } else if type_id == TypeId::of::<u16>() {
        Type::basic(Kind::Uint16)
    } else if type_id == TypeId::of::<u32>() {
        Type::basic(Kind::Uint32)
    } else if type_id == TypeId::of::<u64>() {
        Type::basic(Kind::Uint64)
    } else if type_id == TypeId::of::<usize>() {
        Type::basic(Kind::Uintptr)
    } else if type_id == TypeId::of::<f32>() {
        Type::basic(Kind::Float32)
    } else if type_id == TypeId::of::<f64>() {
        Type::basic(Kind::Float64)
    } else if type_id == TypeId::of::<String>() {
        Type::basic(Kind::String)
    } else if type_id == TypeId::of::<&str>() {
        Type::basic(Kind::String)
    } else if type_id == TypeId::of::<Vec<u8>>() {
        Type::new(Kind::Slice, "[]byte".to_string(), "".to_string())
            .with_elem(Type::basic(Kind::Uint8))
    } else {
        // For unknown types, create a generic type
        Type::new(Kind::Interface, std::any::type_name::<T>().to_string(), "".to_string())
    }
}

/// TypeOf returns the reflection Type of the value (spec-compliant function)
pub fn type_of(value: &Value) -> Type {
    value.typ().clone()
}

/// ValueOf returns a new Value initialized to the concrete value
pub fn value_of<T: Any + Send + Sync + Clone + 'static>(value: T) -> Value {
    let typ = type_of_any(&value);
    
    // Convert the value to the appropriate ValueData
    if let Some(b) = (&value as &dyn Any).downcast_ref::<bool>() {
        Value::from_bool(*b)
    } else if let Some(i) = (&value as &dyn Any).downcast_ref::<i64>() {
        Value::from_int(*i)
    } else if let Some(i) = (&value as &dyn Any).downcast_ref::<i32>() {
        Value::from_int(*i as i64)
    } else if let Some(u) = (&value as &dyn Any).downcast_ref::<u64>() {
        Value::from_uint(*u)
    } else if let Some(f) = (&value as &dyn Any).downcast_ref::<f64>() {
        Value::from_float(*f)
    } else if let Some(s) = (&value as &dyn Any).downcast_ref::<String>() {
        Value::from_string(s.clone())
    } else if let Some(s) = (&value as &dyn Any).downcast_ref::<&str>() {
        Value::from_string(s.to_string())
    } else if let Some(b) = (&value as &dyn Any).downcast_ref::<Vec<u8>>() {
        Value::from_bytes(b.clone())
    } else {
        // For unknown types, create an interface value
//         Value::new(typ, crate::stdlib::lookin_glass::value::ValueData::Interface(None))
    }
}

/// New returns a Value representing a pointer to a new zero value for the specified type
pub fn new(typ: Type) -> LookinGlassResult<Value> {
    let zero_val = zero(typ.clone())?;
    let ptr_type = Type::new(Kind::Pointer, format!("*{}", typ.name()), "".to_string())
        .with_elem(typ);
    
//     Ok(Value::new(ptr_type, crate::stdlib::lookin_glass::value::ValueData::Pointer(Some(Box::new(zero_val)))))
}

/// Zero returns a Value representing the zero value for the specified type
pub fn zero(typ: Type) -> LookinGlassResult<Value> {
//     use crate::stdlib::lookin_glass::value::ValueData;
    
    let data = match typ.kind() {
        Kind::Invalid => return Err(type_error("Cannot create zero value for invalid type")),
        Kind::Bool => ValueData::Bool(false),
        Kind::Int | Kind::Int8 | Kind::Int16 | Kind::Int32 | Kind::Int64 => ValueData::Int(0),
        Kind::Uint | Kind::Uint8 | Kind::Uint16 | Kind::Uint32 | Kind::Uint64 | Kind::Uintptr => ValueData::Uint(0),
        Kind::Float32 | Kind::Float64 => ValueData::Float(0.0),
        Kind::Complex64 | Kind::Complex128 => ValueData::Complex(0.0, 0.0),
        Kind::String => ValueData::String(String::new()),
        Kind::Slice => ValueData::Slice(Vec::new()),
        Kind::Array => {
            let len = typ.len().unwrap_or(0);
            let elem_type = typ.elem().unwrap_or_else(|_| Type::invalid());
            let zero_elem = zero(elem_type)?;
            ValueData::Array(vec![zero_elem; len])
        }
        Kind::Map => ValueData::Map(HashMap::new()),
        Kind::Struct => {
            let mut fields = Vec::new();
            for i in 0..typ.num_field() {
                if let Ok(field_info) = typ.field(i) {
                    let field_zero = zero(field_info.field_type().clone())?;
                    fields.push(field_zero);
                }
            }
            ValueData::Struct(fields)
        }
        Kind::Pointer => ValueData::Pointer(None),
        Kind::Interface => ValueData::Interface(None),
        Kind::Chan => ValueData::Channel(Arc::new(Mutex::new(Vec::new()))),
        Kind::Func => {
            let func = Arc::new(|_args: &[Value]| -> LookinGlassResult<Vec<Value>> {
                Err(reflection_error("Zero function cannot be called"))
            });
            ValueData::Function(func)
        }
        Kind::UnsafePointer => ValueData::Pointer(None),
    };
    
    Ok(Value::new(typ, data))
}

/// Indirect returns the value that v points to
pub fn indirect(v: Value) -> LookinGlassResult<Value> {
    match v.kind() {
        Kind::Pointer | Kind::Interface => v.elem(),
        _ => Ok(v), // If not a pointer, return the value itself
    }
}

/// MakeSlice creates a new slice with specified type, length, and capacity
pub fn make_slice(typ: Type, len: usize, cap: usize) -> LookinGlassResult<Value> {
    if typ.kind() != Kind::Slice {
        return Err(type_error("MakeSlice called with non-slice type"));
    }
    
    if len > cap {
        return Err(invalid_operation("Slice length cannot exceed capacity"));
    }
    
    let elem_type = typ.elem()?;
    let zero_elem = zero(elem_type)?;
    
    let mut slice_data = Vec::with_capacity(cap);
    slice_data.resize(len, zero_elem);
    
//     Ok(Value::new(typ, crate::stdlib::lookin_glass::value::ValueData::Slice(slice_data)))
}

/// MakeMap creates a new map with the specified type
pub fn make_map(typ: Type) -> LookinGlassResult<Value> {
    if typ.kind() != Kind::Map {
        return Err(type_error("MakeMap called with non-map type"));
    }
    
//     Ok(Value::new(typ, crate::stdlib::lookin_glass::value::ValueData::Map(HashMap::new())))
}

/// MakeChan creates a new channel with the specified type and buffer size
pub fn make_chan(typ: Type, buffer: usize) -> LookinGlassResult<Value> {
    if typ.kind() != Kind::Chan {
        return Err(type_error("MakeChan called with non-channel type"));
    }
    
    let channel_data = Arc::new(Mutex::new(Vec::with_capacity(buffer)));
//     Ok(Value::new(typ, crate::stdlib::lookin_glass::value::ValueData::Channel(channel_data)))
}

/// MakeFunc creates a new function with the specified type and implementation
pub fn make_func<F>(typ: Type, func: F) -> LookinGlassResult<Value>
where
    F: Fn(&[Value]) -> LookinGlassResult<Vec<Value>> + Send + Sync + 'static,
{
    if typ.kind() != Kind::Func {
        return Err(type_error("MakeFunc called with non-function type"));
    }
    
    let func_data = Arc::new(func);
//     Ok(Value::new(typ, crate::stdlib::lookin_glass::value::ValueData::Function(func_data)))
}

/// Convenience functions for creating common types

/// Create an array type
pub fn array_of(elem_type: Type, length: usize) -> Type {
    Type::new(Kind::Array, format!("[{}]{}", length, elem_type.name()), "".to_string())
        .with_elem(elem_type)
        .with_len(length)
}

/// Create a slice type
pub fn slice_of(elem_type: Type) -> Type {
    Type::new(Kind::Slice, format!("[]{}", elem_type.name()), "".to_string())
        .with_elem(elem_type)
}

/// Create a map type
pub fn map_of(key_type: Type, elem_type: Type) -> Type {
    Type::new(Kind::Map, format!("map[{}]{}", key_type.name(), elem_type.name()), "".to_string())
        .with_key(key_type)
        .with_elem(elem_type)
}

/// Create a pointer type
pub fn ptr_to(elem_type: Type) -> Type {
    Type::new(Kind::Pointer, format!("*{}", elem_type.name()), "".to_string())
        .with_elem(elem_type)
}

/// Create a channel type
pub fn chan_of(elem_type: Type) -> Type {
    Type::new(Kind::Chan, format!("chan {}", elem_type.name()), "".to_string())
        .with_elem(elem_type)
}

/// Create a function type
pub fn func_of(in_types: Vec<Type>, out_types: Vec<Type>, variadic: bool) -> Type {
    let in_str = in_types.iter().map(|t| t.name()).collect::<Vec<_>>().join(", ");
    let out_str = if out_types.len() == 1 {
        out_types[0].name().to_string()
    } else {
        format!("({})", out_types.iter().map(|t| t.name()).collect::<Vec<_>>().join(", "))
    };
    
    let name = if out_types.is_empty() {
        format!("func({})", in_str)
    } else {
        format!("func({}) {}", in_str, out_str)
    };
    
    Type::new(Kind::Func, name, "".to_string())
        .with_in_types(in_types)
        .with_out_types(out_types)
        .with_variadic(variadic)
}

/// Type registry for complex types
static TYPE_REGISTRY: std::sync::LazyLock<Mutex<HashMap<String, Type>>> = 
    std::sync::LazyLock::new(|| Mutex::new(HashMap::new()));

/// Register a type in the global registry
pub fn register_type(name: String, typ: Type) {
    let mut registry = TYPE_REGISTRY.lock().unwrap();
    registry.insert(name, typ);
}

/// Look up a type by name in the registry
pub fn lookup_type(name: &str) -> Option<Type> {
    let registry = TYPE_REGISTRY.lock().unwrap();
    registry.get(name).cloned()
}

/// Get all registered types
pub fn registered_types() -> Vec<(String, Type)> {
    let registry = TYPE_REGISTRY.lock().unwrap();
    registry.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
}

/// Initialize common types in the registry
pub fn init_type_registry() {
        // TODO: implement
    }
    register_type("bool".to_string(), Type::basic(Kind::Bool));
    register_type("int".to_string(), Type::basic(Kind::Int));
    register_type("int8".to_string(), Type::basic(Kind::Int8));
    register_type("int16".to_string(), Type::basic(Kind::Int16));
    register_type("int32".to_string(), Type::basic(Kind::Int32));
    register_type("int64".to_string(), Type::basic(Kind::Int64));
    register_type("uint".to_string(), Type::basic(Kind::Uint));
    register_type("uint8".to_string(), Type::basic(Kind::Uint8));
    register_type("uint16".to_string(), Type::basic(Kind::Uint16));
    register_type("uint32".to_string(), Type::basic(Kind::Uint32));
    register_type("uint64".to_string(), Type::basic(Kind::Uint64));
    register_type("uintptr".to_string(), Type::basic(Kind::Uintptr));
    register_type("float32".to_string(), Type::basic(Kind::Float32));
    register_type("float64".to_string(), Type::basic(Kind::Float64));
    register_type("complex64".to_string(), Type::basic(Kind::Complex64));
    register_type("complex128".to_string(), Type::basic(Kind::Complex128));
    register_type("string".to_string(), Type::basic(Kind::String));
    
    // Common composite types
    register_type("[]byte".to_string(), slice_of(Type::basic(Kind::Uint8)));
    register_type("[]string".to_string(), slice_of(Type::basic(Kind::String)));
    register_type("map[string]interface{}".to_string(), 
        map_of(Type::basic(Kind::String), Type::new(Kind::Interface, "interface{}".to_string(), "".to_string())));
}

