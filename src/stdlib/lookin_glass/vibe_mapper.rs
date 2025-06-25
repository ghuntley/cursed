/// VibeMapper - Enhanced mapping and conversion utilities for CURSED reflection
// use crate::stdlib::lookin_glass::{Value, Type, Kind, error::*, utilities::*};
use crate::error::CursedError;
use std::collections::HashMap;
use serde_json;

/// VibeMapper provides methods for manipulating structs/maps with reflection
#[derive(Debug, Clone)]
pub struct VibeMapper {
    /// Configuration options for the mapper
/// Configuration for VibeMapper behavior
#[derive(Debug, Clone)]
pub struct VibeMapperConfig {
    /// Whether to use JSON tags for field names
    /// Whether to omit empty fields
    /// Whether to include unexported fields
    /// Custom field name transformations
    /// Whether to handle nested structs recursively
    /// Maximum depth for recursive operations
impl Default for VibeMapperConfig {
    fn default() -> Self {
        Self {
        }
    }
impl VibeMapper {
    /// Create a new VibeMapper with default configuration
    pub fn new() -> Self {
        Self {
        }
    }

    /// Create a new VibeMapper with custom configuration
    pub fn with_config(config: VibeMapperConfig) -> Self {
        Self { config }
    }

    /// Convert a value to JSON bytes
    pub fn to_json(&self, v: &Value) -> LookinGlassResult<Vec<u8>> {
        let json_value = self.to_json_value(v, 0)?;
        serde_json::to_vec(&json_value)
            .map_err(|e| LookinGlassError::JsonError(e.to_string()))
    /// Convert JSON bytes to a value
    pub fn from_json(&self, data: &[u8], target_type: Option<&Type>) -> LookinGlassResult<Value> {
        let json_value: serde_json::Value = serde_json::from_slice(data)
            .map_err(|e| LookinGlassError::JsonError(e.to_string()))?;
        
        map_to_value(&json_value, target_type)
    /// Convert a value to a map
    pub fn to_map(&self, v: &Value) -> LookinGlassResult<HashMap<String, Value>> {
        self.to_map_internal(v, 0)
    /// Convert a map to a value of the specified type
    pub fn from_map(&self, m: &HashMap<String, Value>, target_type: &Type) -> LookinGlassResult<Value> {
        self.from_map_internal(m, target_type, 0)
    /// Create a deep clone of a value
    pub fn clone(&self, v: &Value) -> LookinGlassResult<Value> {
        deep_copy(v)
    /// Merge two values (dst = dst merged with src)
    pub fn merge(&self, dst: &Value, src: &Value) -> LookinGlassResult<Value> {
        self.merge_internal(dst, src, 0)
    /// Convert value to JSON value for serialization
    fn to_json_value(&self, v: &Value, depth: usize) -> LookinGlassResult<serde_json::Value> {
        if depth > self.config.max_depth {
            return Err(reflection_error("Maximum recursion depth exceeded"));
        match v.kind() {
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
            Kind::Slice | Kind::Array => {
                if !self.config.recursive {
                    return Ok(serde_json::Value::String(format!("<{} len={}>", v.kind(), v.len()?)));
                let len = v.len()?;
                let mut array = Vec::new();
                for i in 0..len {
                    let elem = v.index(i)?;
                    array.push(self.to_json_value(&elem, depth + 1)?);
                }
                Ok(serde_json::Value::Array(array))
            }
            Kind::Map => {
                if !self.config.recursive {
                    return Ok(serde_json::Value::String(format!("<map len={}>", v.len()?)));
                let keys = v.map_keys()?;
                let mut object = serde_json::Map::new();
                for key in keys {
                    let key_str = key.string().unwrap_or_else(|_| format!("{}", key));
                    let value = v.map_index(&key)?;
                    object.insert(key_str, self.to_json_value(&value, depth + 1)?);
                }
                Ok(serde_json::Value::Object(object))
            }
            Kind::Struct => {
                if !self.config.recursive {
                    return Ok(serde_json::Value::String(format!("<struct fields={}>", v.num_field())));
                self.struct_to_json_object(v, depth + 1)
            }
            Kind::Pointer | Kind::Interface => {
                if v.is_nil() {
                    Ok(serde_json::Value::Null)
                } else {
                    let elem = v.elem()?;
                    self.to_json_value(&elem, depth + 1)
                }
            }
        }
    }

    /// Convert struct to JSON object
    fn struct_to_json_object(&self, v: &Value, depth: usize) -> LookinGlassResult<serde_json::Value> {
        let mut object = serde_json::Map::new();
        let num_fields = v.num_field();
        
        for i in 0..num_fields {
            let field_info = v.typ().field(i)?;
            let field_value = v.field(i)?;
            
            // Skip unexported fields if configured
            if !self.config.include_unexported && !field_info.is_exported() {
                continue;
            // Skip JSON ignored fields
            if field_info.json_ignored() {
                continue;
            // Skip empty fields if omit_empty is enabled
            if self.config.omit_empty && field_info.omit_empty() && field_value.is_zero() {
                continue;
            // Determine field name
            let field_name = if self.config.use_json_tags {
                field_info.json_name().unwrap_or_else(|| field_info.name().to_string())
            } else {
                field_info.name().to_string()
            
            // Apply field name transformation if configured
            let final_name = if let Some(transformer) = self.config.field_name_transformer {
                transformer(&field_name)
            } else {
                field_name
            
            let json_value = self.to_json_value(&field_value, depth)?;
            object.insert(final_name, json_value);
        Ok(serde_json::Value::Object(object))
    /// Convert value to map (internal implementation with depth tracking)
    fn to_map_internal(&self, v: &Value, depth: usize) -> LookinGlassResult<HashMap<String, Value>> {
        if depth > self.config.max_depth {
            return Err(reflection_error("Maximum recursion depth exceeded"));
        match v.kind() {
            Kind::Struct => {
                let mut result = HashMap::new();
                let num_fields = v.num_field();
                
                for i in 0..num_fields {
                    let field_info = v.typ().field(i)?;
                    let field_value = v.field(i)?;
                    
                    // Skip unexported fields if configured
                    if !self.config.include_unexported && !field_info.is_exported() {
                        continue;
                    // Skip JSON ignored fields
                    if field_info.json_ignored() {
                        continue;
                    // Skip empty fields if omit_empty is enabled
                    if self.config.omit_empty && field_info.omit_empty() && field_value.is_zero() {
                        continue;
                    // Determine field name
                    let field_name = if self.config.use_json_tags {
                        field_info.json_name().unwrap_or_else(|| field_info.name().to_string())
                    } else {
                        field_info.name().to_string()
                    
                    // Apply field name transformation if configured
                    let final_name = if let Some(transformer) = self.config.field_name_transformer {
                        transformer(&field_name)
                    } else {
                        field_name
                    
                    // Recursively convert nested structs if enabled
                    let final_value = if self.config.recursive && field_value.kind() == Kind::Struct {
                        let nested_map = self.to_map_internal(&field_value, depth + 1)?;
                        // Convert nested map back to a map value
//                         let map_type = crate::stdlib::lookin_glass::core_functions::map_of(
                            Type::new(Kind::Interface, "interface{}".to_string(), "".to_string())
                        );
//                         Value::new(map_type, crate::stdlib::lookin_glass::value::ValueData::Map(
                            nested_map.into_iter().map(|(k, v)| (Value::from_string(k), v)).collect()
                        ))
                    } else {
                        field_value
                    
                    result.insert(final_name, final_value);
                Ok(result)
            }
            Kind::Map => {
                let keys = v.map_keys()?;
                let mut result = HashMap::new();
                for key in keys {
                    let key_str = key.string().unwrap_or_else(|_| format!("{}", key));
                    let value = v.map_index(&key)?;
                    result.insert(key_str, value);
                }
                Ok(result)
            }
        }
    }

    /// Convert map to value (internal implementation with depth tracking)
    fn from_map_internal(&self, m: &HashMap<String, Value>, target_type: &Type, depth: usize) -> LookinGlassResult<Value> {
        if depth > self.config.max_depth {
            return Err(reflection_error("Maximum recursion depth exceeded"));
        match target_type.kind() {
            Kind::Struct => {
                let num_fields = target_type.num_field();
                let mut field_values = Vec::with_capacity(num_fields);
                
                for i in 0..num_fields {
                    let field_info = target_type.field(i)?;
                    
                    // Try to find the field value in the map using various name strategies
                    let field_value = self.find_field_value(m, &field_info)?;
                    
                    let final_value = if let Some(value) = field_value {
                        // Try to convert the value to the field type
                        if self.config.recursive && field_info.field_type().kind() == Kind::Struct {
                            if let Ok(nested_map) = self.to_map_internal(&value, depth + 1) {
                                self.from_map_internal(&nested_map, field_info.field_type(), depth + 1)?
                            } else {
                                value.convert(field_info.field_type())?
                            }
                        } else {
                            value.convert(field_info.field_type())?
                        }
                    } else {
                        // Use zero value for missing fields
//                         crate::stdlib::lookin_glass::core_functions::zero(field_info.field_type().clone())?
                    
                    field_values.push(final_value);
//                     crate::stdlib::lookin_glass::value::ValueData::Struct(field_values)))
            }
            Kind::Map => {
                let mut map_data = HashMap::new();
                for (key, value) in m {
                    let key_val = Value::from_string(key.clone());
                    map_data.insert(key_val, value.clone());
                }
//                     crate::stdlib::lookin_glass::value::ValueData::Map(map_data)))
            }
        }
    }

    /// Find field value in map using various naming strategies
//     fn find_field_value(&self, m: &HashMap<String, Value>, field_info: &crate::stdlib::lookin_glass::StructField) -> LookinGlassResult<Option<Value>> {
        let field_name = field_info.name();
        
        // Strategy 1: Try JSON name if configured
        if self.config.use_json_tags {
            if let Some(json_name) = field_info.json_name() {
                if let Some(value) = m.get(&json_name) {
                    return Ok(Some(value.clone()));
                }
            }
        // Strategy 2: Try exact field name
        if let Some(value) = m.get(field_name) {
            return Ok(Some(value.clone()));
        // Strategy 3: Try field name transformation if configured
        if let Some(transformer) = self.config.field_name_transformer {
            let transformed_name = transformer(field_name);
            if let Some(value) = m.get(&transformed_name) {
                return Ok(Some(value.clone()));
            }
        }
        
        // Strategy 4: Try case-insensitive match
        for (key, value) in m {
            if key.to_lowercase() == field_name.to_lowercase() {
                return Ok(Some(value.clone()));
            }
        }
        
        Ok(None)
    /// Merge two values (internal implementation with depth tracking)
    fn merge_internal(&self, dst: &Value, src: &Value, depth: usize) -> LookinGlassResult<Value> {
        if depth > self.config.max_depth {
            return Err(reflection_error("Maximum recursion depth exceeded"));
        // If types don't match, return source
        if dst.typ() != src.typ() {
            return Ok(src.clone());
        match dst.kind() {
            Kind::Struct => {
                let dst_map = self.to_map_internal(dst, depth + 1)?;
                let src_map = self.to_map_internal(src, depth + 1)?;
                
                let mut merged_map = dst_map;
                for (key, value) in src_map {
                    if let Some(existing) = merged_map.get(&key) {
                        // Recursively merge if both are structs
                        if existing.kind() == Kind::Struct && value.kind() == Kind::Struct {
                            let merged_value = self.merge_internal(existing, &value, depth + 1)?;
                            merged_map.insert(key, merged_value);
                        } else {
                            // Source overwrites destination
                            merged_map.insert(key, value);
                        }
                    } else {
                        merged_map.insert(key, value);
                    }
                }
                
                self.from_map_internal(&merged_map, dst.typ(), depth + 1)
            }
            Kind::Map => {
                let mut merged_map = HashMap::new();
                
                // Add all dst entries
                let dst_keys = dst.map_keys()?;
                for key in dst_keys {
                    let value = dst.map_index(&key)?;
                    merged_map.insert(key, value);
                // Add/overwrite with src entries
                let src_keys = src.map_keys()?;
                for key in src_keys {
                    let value = src.map_index(&key)?;
                    merged_map.insert(key, value);
//                     crate::stdlib::lookin_glass::value::ValueData::Map(merged_map)))
            }
            Kind::Slice | Kind::Array => {
                // For arrays/slices, append src to dst
                let dst_len = dst.len()?;
                let src_len = src.len()?;
                let mut merged_elements = Vec::with_capacity(dst_len + src_len);
                
                for i in 0..dst_len {
                    merged_elements.push(dst.index(i)?);
                }
                for i in 0..src_len {
                    merged_elements.push(src.index(i)?);
                let data = if dst.kind() == Kind::Array {
//                     crate::stdlib::lookin_glass::value::ValueData::Array(merged_elements)
                } else {
//                     crate::stdlib::lookin_glass::value::ValueData::Slice(merged_elements)
                
                Ok(Value::new(dst.typ().clone(), data))
            }
            _ => {
                // For primitive types, source overwrites destination
                Ok(src.clone())
            }
        }
    /// Builder methods for configuration

    /// Set whether to use JSON tags for field names
    pub fn use_json_tags(mut self, use_tags: bool) -> Self {
        self.config.use_json_tags = use_tags;
        self
    /// Set whether to omit empty fields
    pub fn omit_empty(mut self, omit: bool) -> Self {
        self.config.omit_empty = omit;
        self
    /// Set whether to include unexported fields
    pub fn include_unexported(mut self, include: bool) -> Self {
        self.config.include_unexported = include;
        self
    /// Set field name transformer
    pub fn field_name_transformer(mut self, transformer: fn(&str) -> String) -> Self {
        self.config.field_name_transformer = Some(transformer);
        self
    /// Set whether to handle nested structs recursively
    pub fn recursive(mut self, recursive: bool) -> Self {
        self.config.recursive = recursive;
        self
    /// Set maximum recursion depth
    pub fn max_depth(mut self, depth: usize) -> Self {
        self.config.max_depth = depth;
        self
    }
}

impl Default for VibeMapper {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience functions for common transformations

/// Convert camelCase to snake_case
pub fn camel_to_snake(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars();
    
    if let Some(first) = chars.next() {
        result.push(first.to_lowercase().next().unwrap());
    for ch in chars {
        if ch.is_uppercase() {
            result.push('_');
            result.push(ch.to_lowercase().next().unwrap());
        } else {
            result.push(ch);
        }
    }
    
    result
/// Convert snake_case to camelCase
pub fn snake_to_camel(input: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;
    
    for ch in input.chars() {
        if ch == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(ch.to_uppercase().next().unwrap());
            capitalize_next = false;
        } else {
            result.push(ch);
        }
    }
    
    result
/// Convert to lowercase
pub fn to_lowercase(input: &str) -> String {
    input.to_lowercase()
/// Convert to uppercase
pub fn to_uppercase(input: &str) -> String {
    input.to_uppercase()
