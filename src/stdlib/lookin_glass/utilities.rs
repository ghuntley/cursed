/// Enhanced reflection utilities for LookinGlass
use crate::stdlib::lookin_glass::{Type, Value, Kind, StructField, error::*};
use std::collections::HashMap;
use std::any::Any;

/// DeepEqual reports whether x and y are deeply equal
pub fn deep_equal(x: &Value, y: &Value) -> bool {
    // If types don't match, they're not equal
    if x.typ() != y.typ() {
        return false;
    }

    // Handle different kinds
    match x.kind() {
        Kind::Invalid => y.kind() == Kind::Invalid,
        Kind::Bool => x.bool().unwrap_or(false) == y.bool().unwrap_or(false),
        Kind::Int | Kind::Int8 | Kind::Int16 | Kind::Int32 | Kind::Int64 => {
            x.int().unwrap_or(0) == y.int().unwrap_or(0)
        }
        Kind::Uint | Kind::Uint8 | Kind::Uint16 | Kind::Uint32 | Kind::Uint64 | Kind::Uintptr => {
            x.uint().unwrap_or(0) == y.uint().unwrap_or(0)
        }
        Kind::Float32 | Kind::Float64 => {
            let x_float = x.float().unwrap_or(0.0);
            let y_float = y.float().unwrap_or(0.0);
            (x_float - y_float).abs() < f64::EPSILON
        }
        Kind::Complex64 | Kind::Complex128 => {
            let (x_r, x_i) = x.complex().unwrap_or((0.0, 0.0));
            let (y_r, y_i) = y.complex().unwrap_or((0.0, 0.0));
            (x_r - y_r).abs() < f64::EPSILON && (x_i - y_i).abs() < f64::EPSILON
        }
        Kind::String => x.string().unwrap_or_default() == y.string().unwrap_or_default(),
        Kind::Slice | Kind::Array => {
            if let (Ok(x_len), Ok(y_len)) = (x.len(), y.len()) {
                if x_len != y_len {
                    return false;
                }
                for i in 0..x_len {
                    if let (Ok(x_elem), Ok(y_elem)) = (x.index(i), y.index(i)) {
                        if !deep_equal(&x_elem, &y_elem) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                true
            } else {
                false
            }
        }
        Kind::Map => {
            if let (Ok(x_keys), Ok(y_keys)) = (x.map_keys(), y.map_keys()) {
                if x_keys.len() != y_keys.len() {
                    return false;
                }
                for key in x_keys {
                    if let (Ok(x_val), Ok(y_val)) = (x.map_index(&key), y.map_index(&key)) {
                        if !deep_equal(&x_val, &y_val) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                true
            } else {
                false
            }
        }
        Kind::Struct => {
            let x_fields = x.num_field();
            let y_fields = y.num_field();
            if x_fields != y_fields {
                return false;
            }
            for i in 0..x_fields {
                if let (Ok(x_field), Ok(y_field)) = (x.field(i), y.field(i)) {
                    if !deep_equal(&x_field, &y_field) {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            true
        }
        Kind::Pointer | Kind::Interface => {
            match (x.is_nil(), y.is_nil()) {
                (true, true) => true,
                (false, false) => {
                    if let (Ok(x_elem), Ok(y_elem)) = (x.elem(), y.elem()) {
                        deep_equal(&x_elem, &y_elem)
                    } else {
                        false
                    }
                }
                _ => false,
            }
        }
        Kind::Chan | Kind::Func | Kind::UnsafePointer => {
            // These types are compared by identity, not value
            // For our implementation, we'll use pointer equality check
            std::ptr::eq(x, y)
        }
    }
}

/// DeepCopy creates a deep copy of a value
pub fn deep_copy(v: &Value) -> LookinGlassResult<Value> {
    match v.kind() {
        Kind::Invalid => Ok(Value::invalid()),
        
        // Basic types can be copied directly
        Kind::Bool => Ok(Value::from_bool(v.bool()?)),
        Kind::Int | Kind::Int8 | Kind::Int16 | Kind::Int32 | Kind::Int64 => {
            Ok(Value::from_int(v.int()?))
        }
        Kind::Uint | Kind::Uint8 | Kind::Uint16 | Kind::Uint32 | Kind::Uint64 | Kind::Uintptr => {
            Ok(Value::from_uint(v.uint()?))
        }
        Kind::Float32 | Kind::Float64 => Ok(Value::from_float(v.float()?)),
        Kind::Complex64 | Kind::Complex128 => {
            let (real, imag) = v.complex()?;
            Ok(Value::new(v.typ().clone(), 
                crate::stdlib::lookin_glass::value::ValueData::Complex(real, imag)))
        }
        Kind::String => Ok(Value::from_string(v.string()?)),
        
        // Composite types require recursive copying
        Kind::Slice => {
            let len = v.len()?;
            let mut copied_elements = Vec::with_capacity(len);
            for i in 0..len {
                let elem = v.index(i)?;
                copied_elements.push(deep_copy(&elem)?);
            }
            Ok(Value::new(v.typ().clone(), 
                crate::stdlib::lookin_glass::value::ValueData::Slice(copied_elements)))
        }
        
        Kind::Array => {
            let len = v.len()?;
            let mut copied_elements = Vec::with_capacity(len);
            for i in 0..len {
                let elem = v.index(i)?;
                copied_elements.push(deep_copy(&elem)?);
            }
            Ok(Value::new(v.typ().clone(), 
                crate::stdlib::lookin_glass::value::ValueData::Array(copied_elements)))
        }
        
        Kind::Map => {
            let keys = v.map_keys()?;
            let mut copied_map = HashMap::new();
            for key in keys {
                let value = v.map_index(&key)?;
                let copied_key = deep_copy(&key)?;
                let copied_value = deep_copy(&value)?;
                copied_map.insert(copied_key, copied_value);
            }
            Ok(Value::new(v.typ().clone(), 
                crate::stdlib::lookin_glass::value::ValueData::Map(copied_map)))
        }
        
        Kind::Struct => {
            let num_fields = v.num_field();
            let mut copied_fields = Vec::with_capacity(num_fields);
            for i in 0..num_fields {
                let field = v.field(i)?;
                copied_fields.push(deep_copy(&field)?);
            }
            Ok(Value::new(v.typ().clone(), 
                crate::stdlib::lookin_glass::value::ValueData::Struct(copied_fields)))
        }
        
        Kind::Pointer => {
            if v.is_nil() {
                Ok(Value::new(v.typ().clone(), 
                    crate::stdlib::lookin_glass::value::ValueData::Pointer(None)))
            } else {
                let elem = v.elem()?;
                let copied_elem = deep_copy(&elem)?;
                Ok(Value::new(v.typ().clone(), 
                    crate::stdlib::lookin_glass::value::ValueData::Pointer(Some(Box::new(copied_elem)))))
            }
        }
        
        Kind::Interface => {
            if v.is_nil() {
                Ok(Value::new(v.typ().clone(), 
                    crate::stdlib::lookin_glass::value::ValueData::Interface(None)))
            } else {
                let elem = v.elem()?;
                let copied_elem = deep_copy(&elem)?;
                Ok(Value::new(v.typ().clone(), 
                    crate::stdlib::lookin_glass::value::ValueData::Interface(Some(Box::new(copied_elem)))))
            }
        }
        
        // These types cannot be deeply copied
        Kind::Chan => Err(reflection_error("Cannot deep copy channel")),
        Kind::Func => Err(reflection_error("Cannot deep copy function")),
        Kind::UnsafePointer => Err(reflection_error("Cannot deep copy unsafe pointer")),
    }
}

/// StructToMap converts a struct to a map[string]interface{}
pub fn struct_to_map(v: &Value) -> LookinGlassResult<HashMap<String, Value>> {
    if v.kind() != Kind::Struct {
        return Err(type_error("StructToMap called on non-struct type"));
    }

    let mut result = HashMap::new();
    let num_fields = v.num_field();
    
    for i in 0..num_fields {
        let field_info = v.typ().field(i)?;
        let field_value = v.field(i)?;
        
        // Use the field name as the map key
        let field_name = field_info.name().to_string();
        
        // Check for JSON tag to use as map key
        let map_key = if let Some(json_name) = field_info.json_name() {
            json_name
        } else {
            field_name
        };
        
        // Skip fields that are JSON ignored
        if !field_info.json_ignored() {
            result.insert(map_key, field_value);
        }
    }
    
    Ok(result)
}

/// MapToStruct converts a map[string]interface{} to a struct
pub fn map_to_struct(m: &HashMap<String, Value>, struct_type: &Type) -> LookinGlassResult<Value> {
    if struct_type.kind() != Kind::Struct {
        return Err(type_error("MapToStruct called with non-struct type"));
    }

    let num_fields = struct_type.num_field();
    let mut field_values = Vec::with_capacity(num_fields);
    
    for i in 0..num_fields {
        let field_info = struct_type.field(i)?;
        let field_name = field_info.name();
        
        // Try to find the field value in the map
        let field_value = if let Some(json_name) = field_info.json_name() {
            // Try JSON name first
            m.get(&json_name).or_else(|| m.get(field_name))
        } else {
            // Use field name
            m.get(field_name)
        };
        
        let final_value = if let Some(value) = field_value {
            // Try to convert the value to the field type
            value.convert(field_info.field_type())?
        } else {
            // Use zero value for missing fields
            crate::stdlib::lookin_glass::core_functions::zero(field_info.field_type().clone())?
        };
        
        field_values.push(final_value);
    }
    
    Ok(Value::new(struct_type.clone(), 
        crate::stdlib::lookin_glass::value::ValueData::Struct(field_values)))
}

/// GetTags returns all struct tags as a map
pub fn get_tags(v: &Value) -> LookinGlassResult<HashMap<String, HashMap<String, String>>> {
    if v.kind() != Kind::Struct {
        return Err(type_error("GetTags called on non-struct type"));
    }

    let mut result = HashMap::new();
    let num_fields = v.num_field();
    
    for i in 0..num_fields {
        let field_info = v.typ().field(i)?;
        let field_name = field_info.name().to_string();
        let tag_map = field_info.tag().all().clone();
        
        if !tag_map.is_empty() {
            result.insert(field_name, tag_map);
        }
    }
    
    Ok(result)
}

/// SetField sets a field value by name
pub fn set_field(v: &mut Value, name: &str, value: Value) -> LookinGlassResult<()> {
    if v.kind() != Kind::Struct {
        return Err(type_error("SetField called on non-struct type"));
    }

    // Find the field by name
    let field_info = v.typ().field_by_name(name)?;
    let field_index = field_info.index()[0];
    
    // Check if the field can be set
    if !field_info.can_set() {
        return Err(cannot_set(&format!("Field '{}' cannot be set", name)));
    }
    
    // Type check
    if !value.typ().assignable_to(field_info.field_type()) {
        return Err(type_mismatch(&format!(
            "Cannot assign {} to field '{}' of type {}", 
            value.typ(), name, field_info.field_type()
        )));
    }
    
    // Note: In a real implementation, we would need access to the internal ValueData
    // For now, we'll return an error indicating this limitation
    Err(reflection_error("SetField not fully implemented - requires mutable access to ValueData"))
}

/// Get a field value by name (convenience function)
pub fn get_field(v: &Value, name: &str) -> LookinGlassResult<Value> {
    v.field_by_name(name)
}

/// Check if a struct has a field with the given name
pub fn has_field(v: &Value, name: &str) -> bool {
    if v.kind() != Kind::Struct {
        return false;
    }
    
    v.typ().field_by_name(name).is_ok()
}

/// Get all field names of a struct
pub fn field_names(v: &Value) -> LookinGlassResult<Vec<String>> {
    if v.kind() != Kind::Struct {
        return Err(type_error("FieldNames called on non-struct type"));
    }

    let mut names = Vec::new();
    let num_fields = v.num_field();
    
    for i in 0..num_fields {
        let field_info = v.typ().field(i)?;
        names.push(field_info.name().to_string());
    }
    
    Ok(names)
}

/// Get field names with their types
pub fn field_info(v: &Value) -> LookinGlassResult<Vec<(String, Type)>> {
    if v.kind() != Kind::Struct {
        return Err(type_error("FieldInfo called on non-struct type"));
    }

    let mut info = Vec::new();
    let num_fields = v.num_field();
    
    for i in 0..num_fields {
        let field_info = v.typ().field(i)?;
        info.push((field_info.name().to_string(), field_info.field_type().clone()));
    }
    
    Ok(info)
}

/// Convert any value to a generic map for JSON-like serialization
pub fn value_to_map(v: &Value) -> LookinGlassResult<serde_json::Value> {
    match v.kind() {
        Kind::Invalid => Ok(serde_json::Value::Null),
        Kind::Bool => Ok(serde_json::Value::Bool(v.bool()?)),
        Kind::Int | Kind::Int8 | Kind::Int16 | Kind::Int32 | Kind::Int64 => {
            Ok(serde_json::Value::Number(serde_json::Number::from(v.int()?)))
        }
        Kind::Uint | Kind::Uint8 | Kind::Uint16 | Kind::Uint32 | Kind::Uint64 | Kind::Uintptr => {
            Ok(serde_json::Value::Number(serde_json::Number::from(v.uint()?)))
        }
        Kind::Float32 | Kind::Float64 => {
            if let Some(num) = serde_json::Number::from_f64(v.float()?) {
                Ok(serde_json::Value::Number(num))
            } else {
                Ok(serde_json::Value::Null)
            }
        }
        Kind::String => Ok(serde_json::Value::String(v.string()?)),
        Kind::Slice | Kind::Array => {
            let len = v.len()?;
            let mut array = Vec::new();
            for i in 0..len {
                let elem = v.index(i)?;
                array.push(value_to_map(&elem)?);
            }
            Ok(serde_json::Value::Array(array))
        }
        Kind::Map => {
            let keys = v.map_keys()?;
            let mut object = serde_json::Map::new();
            for key in keys {
                let key_str = key.string().unwrap_or_else(|_| format!("{}", key));
                let value = v.map_index(&key)?;
                object.insert(key_str, value_to_map(&value)?);
            }
            Ok(serde_json::Value::Object(object))
        }
        Kind::Struct => {
            let map = struct_to_map(v)?;
            let mut object = serde_json::Map::new();
            for (key, value) in map {
                object.insert(key, value_to_map(&value)?);
            }
            Ok(serde_json::Value::Object(object))
        }
        Kind::Pointer | Kind::Interface => {
            if v.is_nil() {
                Ok(serde_json::Value::Null)
            } else {
                let elem = v.elem()?;
                value_to_map(&elem)
            }
        }
        _ => Err(reflection_error(&format!("Cannot convert {} to JSON value", v.kind()))),
    }
}

/// Convert a JSON value back to a CURSED Value
pub fn map_to_value(json: &serde_json::Value, target_type: Option<&Type>) -> LookinGlassResult<Value> {
    match json {
        serde_json::Value::Null => {
            if let Some(typ) = target_type {
                crate::stdlib::lookin_glass::core_functions::zero(typ.clone())
            } else {
                Ok(Value::invalid())
            }
        }
        serde_json::Value::Bool(b) => Ok(Value::from_bool(*b)),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(Value::from_int(i))
            } else if let Some(u) = n.as_u64() {
                Ok(Value::from_uint(u))
            } else if let Some(f) = n.as_f64() {
                Ok(Value::from_float(f))
            } else {
                Err(conversion_error("Invalid number format"))
            }
        }
        serde_json::Value::String(s) => Ok(Value::from_string(s.clone())),
        serde_json::Value::Array(arr) => {
            let mut values = Vec::new();
            for item in arr {
                values.push(map_to_value(item, None)?);
            }
            
            let elem_type = if let Some(first) = values.first() {
                first.typ().clone()
            } else {
                Type::new(Kind::Interface, "interface{}".to_string(), "".to_string())
            };
            
            let slice_type = crate::stdlib::lookin_glass::core_functions::slice_of(elem_type);
            Ok(Value::new(slice_type, crate::stdlib::lookin_glass::value::ValueData::Slice(values)))
        }
        serde_json::Value::Object(obj) => {
            if let Some(typ) = target_type {
                if typ.kind() == Kind::Struct {
                    // Convert to struct
                    let mut field_map = HashMap::new();
                    for (key, value) in obj {
                        field_map.insert(key.clone(), map_to_value(value, None)?);
                    }
                    return map_to_struct(&field_map, typ);
                }
            }
            
            // Convert to map
            let mut map = HashMap::new();
            for (key, value) in obj {
                let key_val = Value::from_string(key.clone());
                let value_val = map_to_value(value, None)?;
                map.insert(key_val, value_val);
            }
            
            let map_type = crate::stdlib::lookin_glass::core_functions::map_of(
                Type::basic(Kind::String),
                Type::new(Kind::Interface, "interface{}".to_string(), "".to_string())
            );
            Ok(Value::new(map_type, crate::stdlib::lookin_glass::value::ValueData::Map(map)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stdlib::lookin_glass::{StructField, StructTag};

    #[test]
    fn test_deep_equal() {
        let val1 = Value::from_int(42);
        let val2 = Value::from_int(42);
        let val3 = Value::from_int(24);

        assert!(deep_equal(&val1, &val2));
        assert!(!deep_equal(&val1, &val3));

        let str1 = Value::from_string("hello".to_string());
        let str2 = Value::from_string("hello".to_string());
        let str3 = Value::from_string("world".to_string());

        assert!(deep_equal(&str1, &str2));
        assert!(!deep_equal(&str1, &str3));
    }

    #[test]
    fn test_deep_copy() {
        let original = Value::from_string("hello".to_string());
        let copied = deep_copy(&original).unwrap();

        assert!(deep_equal(&original, &copied));
        assert_eq!(copied.string().unwrap(), "hello");

        let int_original = Value::from_int(42);
        let int_copied = deep_copy(&int_original).unwrap();
        assert_eq!(int_copied.int().unwrap(), 42);
    }

    #[test]
    fn test_struct_to_map() {
        // Create a simple struct type
        let name_field = StructField::builder("Name".to_string(), Type::basic(Kind::String))
            .tag_string("json:\"name\"".to_string())
            .build();
        let age_field = StructField::builder("Age".to_string(), Type::basic(Kind::Int32))
            .tag_string("json:\"age\"".to_string())
            .build();
        
        let struct_type = Type::new(Kind::Struct, "Person".to_string(), "".to_string())
            .with_fields(vec![name_field, age_field]);
        
        // Create struct value
        let fields = vec![
            Value::from_string("Alice".to_string()),
            Value::from_int(25)
        ];
        let struct_val = Value::new(struct_type, 
            crate::stdlib::lookin_glass::value::ValueData::Struct(fields));

        let map = struct_to_map(&struct_val).unwrap();
        
        assert_eq!(map.len(), 2);
        assert_eq!(map.get("name").unwrap().string().unwrap(), "Alice");
        assert_eq!(map.get("age").unwrap().int().unwrap(), 25);
    }

    #[test]
    fn test_map_to_struct() {
        // Create struct type
        let name_field = StructField::simple("Name".to_string(), Type::basic(Kind::String));
        let age_field = StructField::simple("Age".to_string(), Type::basic(Kind::Int32));
        
        let struct_type = Type::new(Kind::Struct, "Person".to_string(), "".to_string())
            .with_fields(vec![name_field, age_field]);
        
        // Create map
        let mut map = HashMap::new();
        map.insert("Name".to_string(), Value::from_string("Bob".to_string()));
        map.insert("Age".to_string(), Value::from_int(30));
        
        let struct_val = map_to_struct(&map, &struct_type).unwrap();
        
        assert_eq!(struct_val.field(0).unwrap().string().unwrap(), "Bob");
        assert_eq!(struct_val.field(1).unwrap().int().unwrap(), 30);
    }

    #[test]
    fn test_get_tags() {
        let field_with_tags = StructField::builder("TestField".to_string(), Type::basic(Kind::String))
            .tag_string("json:\"test_field\" db:\"test_column\" validate:\"required\"".to_string())
            .build();
        
        let struct_type = Type::new(Kind::Struct, "Test".to_string(), "".to_string())
            .with_field(field_with_tags);
        
        let fields = vec![Value::from_string("test".to_string())];
        let struct_val = Value::new(struct_type, 
            crate::stdlib::lookin_glass::value::ValueData::Struct(fields));

        let tags = get_tags(&struct_val).unwrap();
        
        assert!(tags.contains_key("TestField"));
        let field_tags = &tags["TestField"];
        assert_eq!(field_tags.get("json").unwrap(), "test_field");
        assert_eq!(field_tags.get("db").unwrap(), "test_column");
        assert_eq!(field_tags.get("validate").unwrap(), "required");
    }

    #[test]
    fn test_field_operations() {
        let name_field = StructField::simple("Name".to_string(), Type::basic(Kind::String));
        let age_field = StructField::simple("Age".to_string(), Type::basic(Kind::Int32));
        
        let struct_type = Type::new(Kind::Struct, "Person".to_string(), "".to_string())
            .with_fields(vec![name_field, age_field]);
        
        let fields = vec![
            Value::from_string("Charlie".to_string()),
            Value::from_int(35)
        ];
        let struct_val = Value::new(struct_type, 
            crate::stdlib::lookin_glass::value::ValueData::Struct(fields));

        // Test field_names
        let names = field_names(&struct_val).unwrap();
        assert_eq!(names, vec!["Name", "Age"]);

        // Test has_field
        assert!(has_field(&struct_val, "Name"));
        assert!(has_field(&struct_val, "Age"));
        assert!(!has_field(&struct_val, "NonExistent"));

        // Test get_field
        let name_val = get_field(&struct_val, "Name").unwrap();
        assert_eq!(name_val.string().unwrap(), "Charlie");
    }

    #[test]
    fn test_value_to_map_conversion() {
        let string_val = Value::from_string("hello".to_string());
        let json_val = value_to_map(&string_val).unwrap();
        assert_eq!(json_val, serde_json::Value::String("hello".to_string()));

        let int_val = Value::from_int(42);
        let json_int = value_to_map(&int_val).unwrap();
        assert_eq!(json_int, serde_json::Value::Number(serde_json::Number::from(42i64)));

        let bool_val = Value::from_bool(true);
        let json_bool = value_to_map(&bool_val).unwrap();
        assert_eq!(json_bool, serde_json::Value::Bool(true));
    }

    #[test]
    fn test_map_to_value_conversion() {
        let json_str = serde_json::Value::String("hello".to_string());
        let val = map_to_value(&json_str, None).unwrap();
        assert_eq!(val.string().unwrap(), "hello");

        let json_num = serde_json::Value::Number(serde_json::Number::from(42));
        let val_num = map_to_value(&json_num, None).unwrap();
        assert_eq!(val_num.int().unwrap(), 42);

        let json_bool = serde_json::Value::Bool(true);
        let val_bool = map_to_value(&json_bool, None).unwrap();
        assert_eq!(val_bool.bool().unwrap(), true);
    }
}
