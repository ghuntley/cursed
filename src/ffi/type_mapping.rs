//! Type mapping system for FFI operations
//!
//! This module provides automatic type conversion between CURSED types and
//! foreign language types, including struct marshalling and complex type handling.

use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::sync::{Arc, RwLock};
use crate::error::CursedError;
use super::{FfiValue, FfiType, ForeignValue};

/// Type mapper for converting between CURSED and foreign types
pub struct TypeMapper {
    /// Type mappings for different languages
    mappings: HashMap<String, LanguageTypeMappings>,
    
    /// Custom type converters
    custom_converters: HashMap<String, Box<dyn TypeConverter>>,
    
    /// Struct marshallers
    struct_marshallers: HashMap<String, Box<dyn StructMarshaller>>,
    
    /// Type validation rules
    validation_rules: HashMap<String, Box<dyn TypeValidator>>,
}

/// Type mappings for a specific language
struct LanguageTypeMappings {
    /// Basic type mappings
    basic_types: HashMap<String, String>,
    
    /// Complex type mappings
    complex_types: HashMap<String, ComplexTypeMapping>,
    
    /// Default mappings for unknown types
    default_mappings: HashMap<String, String>,
}

/// Complex type mapping
struct ComplexTypeMapping {
    /// Source type
    source_type: String,
    
    /// Target type
    target_type: String,
    
    /// Conversion function
    converter: Box<dyn TypeConverter>,
    
    /// Validation function
    validator: Option<Box<dyn TypeValidator>>,
}

/// Type converter trait
pub trait TypeConverter: Send + Sync {
    /// Convert from CURSED to foreign type
    fn convert_to_foreign(&self, value: &FfiValue) -> Result<ForeignValue, CursedError>;
    
    /// Convert from foreign to CURSED type
    fn convert_from_foreign(&self, value: &ForeignValue) -> Result<FfiValue, CursedError>;
    
    /// Get converter name
    fn name(&self) -> &str;
}

/// Struct marshaller trait
pub trait StructMarshaller: Send + Sync {
    /// Marshal struct to foreign representation
    fn marshal_struct(&self, fields: &HashMap<String, FfiValue>) -> Result<ForeignValue, CursedError>;
    
    /// Unmarshal struct from foreign representation
    fn unmarshal_struct(&self, value: &ForeignValue) -> Result<HashMap<String, FfiValue>, CursedError>;
    
    /// Get struct name
    fn struct_name(&self) -> &str;
}

/// Type validator trait
pub trait TypeValidator: Send + Sync {
    /// Validate type conversion
    fn validate(&self, value: &FfiValue, target_type: &FfiType) -> Result<(), CursedError>;
    
    /// Get validation rules
    fn rules(&self) -> &[ValidationRule];
}

/// Validation rule
#[derive(Debug, Clone)]
pub struct ValidationRule {
    /// Rule name
    pub name: String,
    
    /// Rule description
    pub description: String,
    
    /// Validation function
    pub validator: fn(&FfiValue, &FfiType) -> Result<(), CursedError>,
}

impl TypeMapper {
    /// Create a new type mapper
    pub fn new() -> Self {
        let mut mapper = Self {
            mappings: HashMap::new(),
            custom_converters: HashMap::new(),
            struct_marshallers: HashMap::new(),
            validation_rules: HashMap::new(),
        };
        
        mapper.initialize_default_mappings();
        mapper.register_builtin_converters();
        mapper.register_builtin_validators();
        
        mapper
    }
    
    /// Initialize default type mappings
    fn initialize_default_mappings(&mut self) {
        // C language mappings
        let mut c_mappings = LanguageTypeMappings {
            basic_types: HashMap::new(),
            complex_types: HashMap::new(),
            default_mappings: HashMap::new(),
        };
        
        // Basic type mappings for C
        c_mappings.basic_types.insert("normie".to_string(), "int".to_string());
        c_mappings.basic_types.insert("smol".to_string(), "short".to_string());
        c_mappings.basic_types.insert("thicc".to_string(), "long".to_string());
        c_mappings.basic_types.insert("byte".to_string(), "unsigned char".to_string());
        c_mappings.basic_types.insert("mid".to_string(), "unsigned short".to_string());
        c_mappings.basic_types.insert("drip".to_string(), "float".to_string());
        c_mappings.basic_types.insert("meal".to_string(), "double".to_string());
        c_mappings.basic_types.insert("sip".to_string(), "char".to_string());
        c_mappings.basic_types.insert("tea".to_string(), "char*".to_string());
        c_mappings.basic_types.insert("lit".to_string(), "bool".to_string());
        c_mappings.basic_types.insert("cringe".to_string(), "void".to_string());
        
        self.mappings.insert("c".to_string(), c_mappings);
        
        // Python language mappings
        let mut python_mappings = LanguageTypeMappings {
            basic_types: HashMap::new(),
            complex_types: HashMap::new(),
            default_mappings: HashMap::new(),
        };
        
        python_mappings.basic_types.insert("normie".to_string(), "int".to_string());
        python_mappings.basic_types.insert("drip".to_string(), "float".to_string());
        python_mappings.basic_types.insert("meal".to_string(), "float".to_string());
        python_mappings.basic_types.insert("tea".to_string(), "str".to_string());
        python_mappings.basic_types.insert("lit".to_string(), "bool".to_string());
        python_mappings.basic_types.insert("cringe".to_string(), "None".to_string());
        
        self.mappings.insert("python".to_string(), python_mappings);
        
        // Go language mappings
        let mut go_mappings = LanguageTypeMappings {
            basic_types: HashMap::new(),
            complex_types: HashMap::new(),
            default_mappings: HashMap::new(),
        };
        
        go_mappings.basic_types.insert("normie".to_string(), "int".to_string());
        go_mappings.basic_types.insert("smol".to_string(), "int16".to_string());
        go_mappings.basic_types.insert("thicc".to_string(), "int64".to_string());
        go_mappings.basic_types.insert("byte".to_string(), "byte".to_string());
        go_mappings.basic_types.insert("mid".to_string(), "uint16".to_string());
        go_mappings.basic_types.insert("drip".to_string(), "float32".to_string());
        go_mappings.basic_types.insert("meal".to_string(), "float64".to_string());
        go_mappings.basic_types.insert("sip".to_string(), "rune".to_string());
        go_mappings.basic_types.insert("tea".to_string(), "string".to_string());
        go_mappings.basic_types.insert("lit".to_string(), "bool".to_string());
        
        self.mappings.insert("go".to_string(), go_mappings);
        
        // JavaScript/WASM mappings
        let mut js_mappings = LanguageTypeMappings {
            basic_types: HashMap::new(),
            complex_types: HashMap::new(),
            default_mappings: HashMap::new(),
        };
        
        js_mappings.basic_types.insert("normie".to_string(), "number".to_string());
        js_mappings.basic_types.insert("drip".to_string(), "number".to_string());
        js_mappings.basic_types.insert("meal".to_string(), "number".to_string());
        js_mappings.basic_types.insert("tea".to_string(), "string".to_string());
        js_mappings.basic_types.insert("lit".to_string(), "boolean".to_string());
        js_mappings.basic_types.insert("cringe".to_string(), "undefined".to_string());
        
        self.mappings.insert("javascript".to_string(), js_mappings);
        self.mappings.insert("wasm".to_string(), js_mappings);
    }
    
    /// Register built-in type converters
    fn register_builtin_converters(&mut self) {
        // Integer converter
        self.custom_converters.insert(
            "integer".to_string(),
            Box::new(IntegerConverter::new()),
        );
        
        // Float converter
        self.custom_converters.insert(
            "float".to_string(),
            Box::new(FloatConverter::new()),
        );
        
        // String converter
        self.custom_converters.insert(
            "string".to_string(),
            Box::new(StringConverter::new()),
        );
        
        // Boolean converter
        self.custom_converters.insert(
            "boolean".to_string(),
            Box::new(BooleanConverter::new()),
        );
        
        // Array converter
        self.custom_converters.insert(
            "array".to_string(),
            Box::new(ArrayConverter::new()),
        );
        
        // Struct converter
        self.custom_converters.insert(
            "struct".to_string(),
            Box::new(StructConverter::new()),
        );
    }
    
    /// Register built-in type validators
    fn register_builtin_validators(&mut self) {
        // Range validator
        self.validation_rules.insert(
            "range".to_string(),
            Box::new(RangeValidator::new()),
        );
        
        // Null validator
        self.validation_rules.insert(
            "null".to_string(),
            Box::new(NullValidator::new()),
        );
        
        // Size validator
        self.validation_rules.insert(
            "size".to_string(),
            Box::new(SizeValidator::new()),
        );
        
        // UTF-8 validator
        self.validation_rules.insert(
            "utf8".to_string(),
            Box::new(Utf8Validator::new()),
        );
    }
    
    /// Register custom type mapping
    pub fn register_mapping(
        &mut self,
        cursed_type: &str,
        foreign_type: &str,
        language: &str,
    ) -> Result<(), CursedError> {
        let language_mappings = self.mappings.get_mut(language)
            .ok_or_else(|| CursedError::General(format!("Unknown language: {}", language)))?;
        
        language_mappings.basic_types.insert(cursed_type.to_string(), foreign_type.to_string());
        
        Ok(())
    }
    
    /// Register custom type converter
    pub fn register_converter(
        &mut self,
        name: &str,
        converter: Box<dyn TypeConverter>,
    ) -> Result<(), CursedError> {
        self.custom_converters.insert(name.to_string(), converter);
        Ok(())
    }
    
    /// Register struct marshaller
    pub fn register_struct_marshaller(
        &mut self,
        struct_name: &str,
        marshaller: Box<dyn StructMarshaller>,
    ) -> Result<(), CursedError> {
        self.struct_marshallers.insert(struct_name.to_string(), marshaller);
        Ok(())
    }
    
    /// Register type validator
    pub fn register_validator(
        &mut self,
        name: &str,
        validator: Box<dyn TypeValidator>,
    ) -> Result<(), CursedError> {
        self.validation_rules.insert(name.to_string(), validator);
        Ok(())
    }
    
    /// Get type mapping for a language
    pub fn get_type_mapping(&self, cursed_type: &str, language: &str) -> Option<String> {
        self.mappings.get(language)
            .and_then(|mappings| mappings.basic_types.get(cursed_type))
            .cloned()
    }
    
    /// Marshal value to foreign representation
    pub fn marshal_to_foreign(
        &self,
        value: &FfiValue,
        target_type: &FfiType,
        language: &str,
    ) -> Result<ForeignValue, CursedError> {
        // Validate the conversion
        self.validate_conversion(value, target_type)?;
        
        // Get the appropriate converter
        let converter = self.get_converter_for_type(target_type)?;
        
        // Perform the conversion
        converter.convert_to_foreign(value)
    }
    
    /// Unmarshal value from foreign representation
    pub fn unmarshal_from_foreign(
        &self,
        foreign_value: &ForeignValue,
        source_type: &FfiType,
        language: &str,
    ) -> Result<FfiValue, CursedError> {
        // Get the appropriate converter
        let converter = self.get_converter_for_type(source_type)?;
        
        // Perform the conversion
        let result = converter.convert_from_foreign(foreign_value)?;
        
        // Validate the result
        self.validate_conversion(&result, source_type)?;
        
        Ok(result)
    }
    
    /// Marshal struct to foreign representation
    pub fn marshal_struct(
        &self,
        struct_name: &str,
        fields: &HashMap<String, FfiValue>,
    ) -> Result<ForeignValue, CursedError> {
        if let Some(marshaller) = self.struct_marshallers.get(struct_name) {
            marshaller.marshal_struct(fields)
        } else {
            // Use default struct marshalling
            self.marshal_struct_default(fields)
        }
    }
    
    /// Unmarshal struct from foreign representation
    pub fn unmarshal_struct(
        &self,
        struct_name: &str,
        value: &ForeignValue,
    ) -> Result<HashMap<String, FfiValue>, CursedError> {
        if let Some(marshaller) = self.struct_marshallers.get(struct_name) {
            marshaller.unmarshal_struct(value)
        } else {
            // Use default struct unmarshalling
            self.unmarshal_struct_default(value)
        }
    }
    
    /// Validate type conversion
    fn validate_conversion(&self, value: &FfiValue, target_type: &FfiType) -> Result<(), CursedError> {
        for validator in self.validation_rules.values() {
            validator.validate(value, target_type)?;
        }
        Ok(())
    }
    
    /// Get converter for a specific type
    fn get_converter_for_type(&self, ffi_type: &FfiType) -> Result<&dyn TypeConverter, CursedError> {
        match ffi_type {
            FfiType::SignedInteger(_) | FfiType::UnsignedInteger(_) => {
                self.custom_converters.get("integer")
                    .map(|c| c.as_ref())
                    .ok_or_else(|| CursedError::General("Integer converter not found".to_string()))
            }
            FfiType::Float(_) => {
                self.custom_converters.get("float")
                    .map(|c| c.as_ref())
                    .ok_or_else(|| CursedError::General("Float converter not found".to_string()))
            }
            FfiType::String | FfiType::CString => {
                self.custom_converters.get("string")
                    .map(|c| c.as_ref())
                    .ok_or_else(|| CursedError::General("String converter not found".to_string()))
            }
            FfiType::Boolean => {
                self.custom_converters.get("boolean")
                    .map(|c| c.as_ref())
                    .ok_or_else(|| CursedError::General("Boolean converter not found".to_string()))
            }
            FfiType::Array(_, _) => {
                self.custom_converters.get("array")
                    .map(|c| c.as_ref())
                    .ok_or_else(|| CursedError::General("Array converter not found".to_string()))
            }
            FfiType::Struct(_) => {
                self.custom_converters.get("struct")
                    .map(|c| c.as_ref())
                    .ok_or_else(|| CursedError::General("Struct converter not found".to_string()))
            }
            _ => Err(CursedError::General("Unsupported type for conversion".to_string())),
        }
    }
    
    /// Default struct marshalling
    fn marshal_struct_default(&self, fields: &HashMap<String, FfiValue>) -> Result<ForeignValue, CursedError> {
        // This would implement a default struct marshalling strategy
        // For now, return a placeholder
        Ok(ForeignValue {
            data: std::ptr::null_mut(),
            size: 0,
            type_info: FfiType::Struct(fields.clone()),
        })
    }
    
    /// Default struct unmarshalling
    fn unmarshal_struct_default(&self, value: &ForeignValue) -> Result<HashMap<String, FfiValue>, CursedError> {
        // This would implement a default struct unmarshalling strategy
        // For now, return an empty map
        Ok(HashMap::new())
    }
}

// Built-in type converters
struct IntegerConverter;

impl IntegerConverter {
    fn new() -> Self {
        Self
    }
}

impl TypeConverter for IntegerConverter {
    fn convert_to_foreign(&self, value: &FfiValue) -> Result<ForeignValue, CursedError> {
        match value {
            FfiValue::SignedInteger(val) => {
                let data = Box::into_raw(Box::new(*val as i32)) as *mut std::ffi::c_void;
                Ok(ForeignValue {
                    data,
                    size: std::mem::size_of::<i32>(),
                    type_info: FfiType::SignedInteger(32),
                })
            }
            FfiValue::UnsignedInteger(val) => {
                let data = Box::into_raw(Box::new(*val as u32)) as *mut std::ffi::c_void;
                Ok(ForeignValue {
                    data,
                    size: std::mem::size_of::<u32>(),
                    type_info: FfiType::UnsignedInteger(32),
                })
            }
            _ => Err(CursedError::General("Value is not an integer".to_string())),
        }
    }
    
    fn convert_from_foreign(&self, value: &ForeignValue) -> Result<FfiValue, CursedError> {
        match &value.type_info {
            FfiType::SignedInteger(32) => {
                let val = unsafe { *(value.data as *const i32) };
                Ok(FfiValue::SignedInteger(val as i64))
            }
            FfiType::UnsignedInteger(32) => {
                let val = unsafe { *(value.data as *const u32) };
                Ok(FfiValue::UnsignedInteger(val as u64))
            }
            _ => Err(CursedError::General("Foreign value is not an integer".to_string())),
        }
    }
    
    fn name(&self) -> &str {
        "integer"
    }
}

struct FloatConverter;

impl FloatConverter {
    fn new() -> Self {
        Self
    }
}

impl TypeConverter for FloatConverter {
    fn convert_to_foreign(&self, value: &FfiValue) -> Result<ForeignValue, CursedError> {
        match value {
            FfiValue::Float(val) => {
                let data = Box::into_raw(Box::new(*val)) as *mut std::ffi::c_void;
                Ok(ForeignValue {
                    data,
                    size: std::mem::size_of::<f64>(),
                    type_info: FfiType::Float(64),
                })
            }
            _ => Err(CursedError::General("Value is not a float".to_string())),
        }
    }
    
    fn convert_from_foreign(&self, value: &ForeignValue) -> Result<FfiValue, CursedError> {
        match &value.type_info {
            FfiType::Float(64) => {
                let val = unsafe { *(value.data as *const f64) };
                Ok(FfiValue::Float(val))
            }
            FfiType::Float(32) => {
                let val = unsafe { *(value.data as *const f32) };
                Ok(FfiValue::Float(val as f64))
            }
            _ => Err(CursedError::General("Foreign value is not a float".to_string())),
        }
    }
    
    fn name(&self) -> &str {
        "float"
    }
}

struct StringConverter;

impl StringConverter {
    fn new() -> Self {
        Self
    }
}

impl TypeConverter for StringConverter {
    fn convert_to_foreign(&self, value: &FfiValue) -> Result<ForeignValue, CursedError> {
        match value {
            FfiValue::String(val) => {
                let c_string = CString::new(val.clone())
                    .map_err(|_| CursedError::General("Invalid string for C conversion".to_string()))?;
                let data = c_string.into_raw() as *mut std::ffi::c_void;
                Ok(ForeignValue {
                    data,
                    size: val.len() + 1,
                    type_info: FfiType::CString,
                })
            }
            FfiValue::CString(val) => {
                let data = val.as_ptr() as *mut std::ffi::c_void;
                Ok(ForeignValue {
                    data,
                    size: val.as_bytes().len() + 1,
                    type_info: FfiType::CString,
                })
            }
            _ => Err(CursedError::General("Value is not a string".to_string())),
        }
    }
    
    fn convert_from_foreign(&self, value: &ForeignValue) -> Result<FfiValue, CursedError> {
        match &value.type_info {
            FfiType::CString => {
                let c_str = unsafe { CStr::from_ptr(value.data as *const i8) };
                let string = c_str.to_string_lossy().into_owned();
                Ok(FfiValue::String(string))
            }
            FfiType::String => {
                let string = unsafe {
                    let slice = std::slice::from_raw_parts(value.data as *const u8, value.size);
                    String::from_utf8_lossy(slice).into_owned()
                };
                Ok(FfiValue::String(string))
            }
            _ => Err(CursedError::General("Foreign value is not a string".to_string())),
        }
    }
    
    fn name(&self) -> &str {
        "string"
    }
}

struct BooleanConverter;

impl BooleanConverter {
    fn new() -> Self {
        Self
    }
}

impl TypeConverter for BooleanConverter {
    fn convert_to_foreign(&self, value: &FfiValue) -> Result<ForeignValue, CursedError> {
        match value {
            FfiValue::Boolean(val) => {
                let data = Box::into_raw(Box::new(*val as i32)) as *mut std::ffi::c_void;
                Ok(ForeignValue {
                    data,
                    size: std::mem::size_of::<i32>(),
                    type_info: FfiType::Boolean,
                })
            }
            _ => Err(CursedError::General("Value is not a boolean".to_string())),
        }
    }
    
    fn convert_from_foreign(&self, value: &ForeignValue) -> Result<FfiValue, CursedError> {
        match &value.type_info {
            FfiType::Boolean => {
                let val = unsafe { *(value.data as *const i32) };
                Ok(FfiValue::Boolean(val != 0))
            }
            _ => Err(CursedError::General("Foreign value is not a boolean".to_string())),
        }
    }
    
    fn name(&self) -> &str {
        "boolean"
    }
}

struct ArrayConverter;

impl ArrayConverter {
    fn new() -> Self {
        Self
    }
}

impl TypeConverter for ArrayConverter {
    fn convert_to_foreign(&self, value: &FfiValue) -> Result<ForeignValue, CursedError> {
        match value {
            FfiValue::Array(arr) => {
                // This would implement array marshalling
                // For now, return a placeholder
                Ok(ForeignValue {
                    data: std::ptr::null_mut(),
                    size: arr.len() * std::mem::size_of::<*mut std::ffi::c_void>(),
                    type_info: FfiType::Array(Box::new(FfiType::Void), Some(arr.len())),
                })
            }
            _ => Err(CursedError::General("Value is not an array".to_string())),
        }
    }
    
    fn convert_from_foreign(&self, value: &ForeignValue) -> Result<FfiValue, CursedError> {
        match &value.type_info {
            FfiType::Array(_, _) => {
                // This would implement array unmarshalling
                // For now, return an empty array
                Ok(FfiValue::Array(Vec::new()))
            }
            _ => Err(CursedError::General("Foreign value is not an array".to_string())),
        }
    }
    
    fn name(&self) -> &str {
        "array"
    }
}

struct StructConverter;

impl StructConverter {
    fn new() -> Self {
        Self
    }
}

impl TypeConverter for StructConverter {
    fn convert_to_foreign(&self, value: &FfiValue) -> Result<ForeignValue, CursedError> {
        match value {
            FfiValue::Struct(fields) => {
                // This would implement struct marshalling
                // For now, return a placeholder
                Ok(ForeignValue {
                    data: std::ptr::null_mut(),
                    size: 0,
                    type_info: FfiType::Struct(fields.clone()),
                })
            }
            _ => Err(CursedError::General("Value is not a struct".to_string())),
        }
    }
    
    fn convert_from_foreign(&self, value: &ForeignValue) -> Result<FfiValue, CursedError> {
        match &value.type_info {
            FfiType::Struct(fields) => {
                Ok(FfiValue::Struct(fields.clone()))
            }
            _ => Err(CursedError::General("Foreign value is not a struct".to_string())),
        }
    }
    
    fn name(&self) -> &str {
        "struct"
    }
}

// Built-in type validators
struct RangeValidator;

impl RangeValidator {
    fn new() -> Self {
        Self
    }
}

impl TypeValidator for RangeValidator {
    fn validate(&self, value: &FfiValue, target_type: &FfiType) -> Result<(), CursedError> {
        match (value, target_type) {
            (FfiValue::SignedInteger(val), FfiType::SignedInteger(bits)) => {
                let max_val = (1i64 << (bits - 1)) - 1;
                let min_val = -(1i64 << (bits - 1));
                
                if *val < min_val || *val > max_val {
                    return Err(CursedError::General(format!(
                        "Integer value {} out of range for {}-bit signed integer",
                        val, bits
                    )));
                }
            }
            (FfiValue::UnsignedInteger(val), FfiType::UnsignedInteger(bits)) => {
                let max_val = (1u64 << bits) - 1;
                
                if *val > max_val {
                    return Err(CursedError::General(format!(
                        "Integer value {} out of range for {}-bit unsigned integer",
                        val, bits
                    )));
                }
            }
            _ => {}
        }
        
        Ok(())
    }
    
    fn rules(&self) -> &[ValidationRule] {
        &[]
    }
}

struct NullValidator;

impl NullValidator {
    fn new() -> Self {
        Self
    }
}

impl TypeValidator for NullValidator {
    fn validate(&self, value: &FfiValue, target_type: &FfiType) -> Result<(), CursedError> {
        match (value, target_type) {
            (FfiValue::Pointer(ptr), _) => {
                if ptr.is_null() {
                    return Err(CursedError::General("Null pointer not allowed".to_string()));
                }
            }
            _ => {}
        }
        
        Ok(())
    }
    
    fn rules(&self) -> &[ValidationRule] {
        &[]
    }
}

struct SizeValidator;

impl SizeValidator {
    fn new() -> Self {
        Self
    }
}

impl TypeValidator for SizeValidator {
    fn validate(&self, value: &FfiValue, target_type: &FfiType) -> Result<(), CursedError> {
        match (value, target_type) {
            (FfiValue::Array(arr), FfiType::Array(_, Some(size))) => {
                if arr.len() != *size {
                    return Err(CursedError::General(format!(
                        "Array size mismatch: expected {}, got {}",
                        size, arr.len()
                    )));
                }
            }
            (FfiValue::String(s), FfiType::CString) => {
                if s.len() > 65536 {
                    return Err(CursedError::General("String too long for C string".to_string()));
                }
            }
            _ => {}
        }
        
        Ok(())
    }
    
    fn rules(&self) -> &[ValidationRule] {
        &[]
    }
}

struct Utf8Validator;

impl Utf8Validator {
    fn new() -> Self {
        Self
    }
}

impl TypeValidator for Utf8Validator {
    fn validate(&self, value: &FfiValue, target_type: &FfiType) -> Result<(), CursedError> {
        match (value, target_type) {
            (FfiValue::String(s), FfiType::String) => {
                if !s.is_ascii() {
                    // Check if it's valid UTF-8
                    if s.chars().any(|c| c.is_control() && c != '\n' && c != '\r' && c != '\t') {
                        return Err(CursedError::General("Invalid UTF-8 string".to_string()));
                    }
                }
            }
            _ => {}
        }
        
        Ok(())
    }
    
    fn rules(&self) -> &[ValidationRule] {
        &[]
    }
}

impl Default for TypeMapper {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_type_mapper_creation() {
        let mapper = TypeMapper::new();
        
        // Test basic C type mappings
        assert_eq!(mapper.get_type_mapping("normie", "c"), Some("int".to_string()));
        assert_eq!(mapper.get_type_mapping("tea", "c"), Some("char*".to_string()));
        
        // Test Python type mappings
        assert_eq!(mapper.get_type_mapping("normie", "python"), Some("int".to_string()));
        assert_eq!(mapper.get_type_mapping("tea", "python"), Some("str".to_string()));
    }
    
    #[test]
    fn test_integer_converter() {
        let converter = IntegerConverter::new();
        
        let value = FfiValue::SignedInteger(42);
        let foreign = converter.convert_to_foreign(&value).unwrap();
        
        assert_eq!(foreign.size, std::mem::size_of::<i32>());
        assert!(matches!(foreign.type_info, FfiType::SignedInteger(32)));
        
        let back = converter.convert_from_foreign(&foreign).unwrap();
        assert!(matches!(back, FfiValue::SignedInteger(42)));
    }
    
    #[test]
    fn test_string_converter() {
        let converter = StringConverter::new();
        
        let value = FfiValue::String("hello".to_string());
        let foreign = converter.convert_to_foreign(&value).unwrap();
        
        assert_eq!(foreign.size, 6); // 5 chars + null terminator
        assert!(matches!(foreign.type_info, FfiType::CString));
        
        let back = converter.convert_from_foreign(&foreign).unwrap();
        assert!(matches!(back, FfiValue::String(s) if s == "hello"));
    }
    
    #[test]
    fn test_boolean_converter() {
        let converter = BooleanConverter::new();
        
        let value = FfiValue::Boolean(true);
        let foreign = converter.convert_to_foreign(&value).unwrap();
        
        assert_eq!(foreign.size, std::mem::size_of::<i32>());
        assert!(matches!(foreign.type_info, FfiType::Boolean));
        
        let back = converter.convert_from_foreign(&foreign).unwrap();
        assert!(matches!(back, FfiValue::Boolean(true)));
    }
    
    #[test]
    fn test_range_validator() {
        let validator = RangeValidator::new();
        
        // Test valid range
        let value = FfiValue::SignedInteger(42);
        let target_type = FfiType::SignedInteger(32);
        assert!(validator.validate(&value, &target_type).is_ok());
        
        // Test invalid range
        let value = FfiValue::SignedInteger(i64::MAX);
        let target_type = FfiType::SignedInteger(32);
        assert!(validator.validate(&value, &target_type).is_err());
    }
    
    #[test]
    fn test_custom_type_mapping() {
        let mut mapper = TypeMapper::new();
        
        mapper.register_mapping("custom_type", "custom_foreign_type", "c").unwrap();
        
        assert_eq!(
            mapper.get_type_mapping("custom_type", "c"),
            Some("custom_foreign_type".to_string())
        );
    }
}
