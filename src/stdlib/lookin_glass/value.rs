/// Value represents a runtime value in CURSED reflection
use crate::stdlib::lookin_glass::{Type, Kind, error::*};
use std::any::Any;
use std::fmt;
use std::sync::{Arc, Mutex};

/// Represents a runtime value with reflection capabilities
#[derive(Debug, Clone)]
pub struct Value {
    typ: Type,
    data: ValueData,
    flags: ValueFlags,
}

/// Internal data storage for values
#[derive(Debug, Clone)]
enum ValueData {
    Invalid,
    Bool(bool),
    Int(i64),
    Uint(u64),
    Float(f64),
    Complex(f64, f64),
    String(String),
    Bytes(Vec<u8>),
    Slice(Vec<Value>),
    Array(Vec<Value>),
    Map(std::collections::HashMap<Value, Value>),
    Struct(Vec<Value>),
    Pointer(Option<Box<Value>>),
    Interface(Option<Box<Value>>),
    Function(Arc<dyn Fn(&[Value]) -> LookinGlassResult<Vec<Value>> + Send + Sync>),
    Channel(Arc<Mutex<Vec<Value>>>),
}

/// Flags indicating properties of the value
#[derive(Debug, Clone, Copy)]
struct ValueFlags {
    can_addr: bool,
    can_set: bool,
    is_exported: bool,
}

impl ValueFlags {
    fn new() -> Self {
        Self {
            can_addr: false,
            can_set: false,
            is_exported: true,
        }
    }

    fn addressable() -> Self {
        Self {
            can_addr: true,
            can_set: true,
            is_exported: true,
        }
    }

    fn readonly() -> Self {
        Self {
            can_addr: false,
            can_set: false,
            is_exported: true,
        }
    }
}

impl Value {
    /// Create a new Value
    pub fn new(typ: Type, data: ValueData) -> Self {
        Self {
            typ,
            data,
            flags: ValueFlags::new(),
        }
    }

    /// Create an invalid Value
    pub fn invalid() -> Self {
        Self {
            typ: Type::invalid(),
            data: ValueData::Invalid,
            flags: ValueFlags::new(),
        }
    }

    /// Create a Value from a boolean
    pub fn from_bool(value: bool) -> Self {
        Self::new(Type::basic(Kind::Bool), ValueData::Bool(value))
    }

    /// Create a Value from an integer
    pub fn from_int(value: i64) -> Self {
        Self::new(Type::basic(Kind::Int64), ValueData::Int(value))
    }

    /// Create a Value from an unsigned integer
    pub fn from_uint(value: u64) -> Self {
        Self::new(Type::basic(Kind::Uint64), ValueData::Uint(value))
    }

    /// Create a Value from a float
    pub fn from_float(value: f64) -> Self {
        Self::new(Type::basic(Kind::Float64), ValueData::Float(value))
    }

    /// Create a Value from a string
    pub fn from_string(value: String) -> Self {
        Self::new(Type::basic(Kind::String), ValueData::String(value))
    }

    /// Create a Value from bytes
    pub fn from_bytes(value: Vec<u8>) -> Self {
        let elem_type = Type::basic(Kind::Uint8);
        let slice_type = Type::new(Kind::Slice, "[]byte".to_string(), "".to_string())
            .with_elem(elem_type);
        Self::new(slice_type, ValueData::Bytes(value))
    }

    /// Get the type of this value
    pub fn typ(&self) -> &Type {
        &self.typ
    }

    /// Get the kind of this value
    pub fn kind(&self) -> Kind {
        self.typ.kind()
    }

    /// Check if this value is valid
    pub fn is_valid(&self) -> bool {
        !matches!(self.data, ValueData::Invalid)
    }

    /// Check if this value is nil/null
    pub fn is_nil(&self) -> bool {
        match &self.data {
            ValueData::Pointer(None) | ValueData::Interface(None) => true,
            ValueData::Slice(v) if v.is_empty() => false, // Empty slice is not nil
            ValueData::Map(m) if m.is_empty() => false,   // Empty map is not nil
            _ => false,
        }
    }

    /// Check if this value is zero
    pub fn is_zero(&self) -> bool {
        match &self.data {
            ValueData::Invalid => true,
            ValueData::Bool(b) => !b,
            ValueData::Int(i) => *i == 0,
            ValueData::Uint(u) => *u == 0,
            ValueData::Float(f) => *f == 0.0,
            ValueData::Complex(r, i) => *r == 0.0 && *i == 0.0,
            ValueData::String(s) => s.is_empty(),
            ValueData::Bytes(b) => b.is_empty(),
            ValueData::Slice(v) => v.is_empty(),
            ValueData::Array(v) => v.iter().all(|val| val.is_zero()),
            ValueData::Map(m) => m.is_empty(),
            ValueData::Struct(fields) => fields.iter().all(|field| field.is_zero()),
            ValueData::Pointer(p) => p.is_none(),
            ValueData::Interface(i) => i.is_none(),
            ValueData::Function(_) => false, // Functions are never zero
            ValueData::Channel(_) => false,  // Channels are never zero
        }
    }

    /// Check if this value can be addressed
    pub fn can_addr(&self) -> bool {
        self.flags.can_addr
    }

    /// Check if this value can be set
    pub fn can_set(&self) -> bool {
        self.flags.can_set
    }

    /// Check if this value can be interfaced
    pub fn can_interface(&self) -> bool {
        self.is_valid() && self.flags.is_exported
    }

    // Type-specific getters

    /// Get as boolean
    pub fn bool(&self) -> LookinGlassResult<bool> {
        match &self.data {
            ValueData::Bool(b) => Ok(*b),
            _ => Err(type_mismatch(&format!("Cannot get {} as bool", self.kind()))),
        }
    }

    /// Get as signed integer
    pub fn int(&self) -> LookinGlassResult<i64> {
        match &self.data {
            ValueData::Int(i) => Ok(*i),
            ValueData::Uint(u) if *u <= i64::MAX as u64 => Ok(*u as i64),
            _ => Err(type_mismatch(&format!("Cannot get {} as int", self.kind()))),
        }
    }

    /// Get as unsigned integer
    pub fn uint(&self) -> LookinGlassResult<u64> {
        match &self.data {
            ValueData::Uint(u) => Ok(*u),
            ValueData::Int(i) if *i >= 0 => Ok(*i as u64),
            _ => Err(type_mismatch(&format!("Cannot get {} as uint", self.kind()))),
        }
    }

    /// Get as float
    pub fn float(&self) -> LookinGlassResult<f64> {
        match &self.data {
            ValueData::Float(f) => Ok(*f),
            ValueData::Int(i) => Ok(*i as f64),
            ValueData::Uint(u) => Ok(*u as f64),
            _ => Err(type_mismatch(&format!("Cannot get {} as float", self.kind()))),
        }
    }

    /// Get as complex number
    pub fn complex(&self) -> LookinGlassResult<(f64, f64)> {
        match &self.data {
            ValueData::Complex(r, i) => Ok((*r, *i)),
            ValueData::Float(f) => Ok((*f, 0.0)),
            ValueData::Int(i) => Ok((*i as f64, 0.0)),
            ValueData::Uint(u) => Ok((*u as f64, 0.0)),
            _ => Err(type_mismatch(&format!("Cannot get {} as complex", self.kind()))),
        }
    }

    /// Get as string
    pub fn string(&self) -> LookinGlassResult<String> {
        match &self.data {
            ValueData::String(s) => Ok(s.clone()),
            _ => Err(type_mismatch(&format!("Cannot get {} as string", self.kind()))),
        }
    }

    /// Get as bytes
    pub fn bytes(&self) -> LookinGlassResult<Vec<u8>> {
        match &self.data {
            ValueData::Bytes(b) => Ok(b.clone()),
            ValueData::String(s) => Ok(s.as_bytes().to_vec()),
            _ => Err(type_mismatch(&format!("Cannot get {} as bytes", self.kind()))),
        }
    }

    /// Get the length (for arrays, slices, maps, strings, channels)
    pub fn len(&self) -> LookinGlassResult<usize> {
        match &self.data {
            ValueData::String(s) => Ok(s.len()),
            ValueData::Bytes(b) => Ok(b.len()),
            ValueData::Slice(v) => Ok(v.len()),
            ValueData::Array(v) => Ok(v.len()),
            ValueData::Map(m) => Ok(m.len()),
            ValueData::Channel(c) => Ok(c.lock().unwrap().len()),
            _ => Err(invalid_operation(&format!("Len() called on {}", self.kind()))),
        }
    }

    /// Get the capacity (for slices, channels)
    pub fn cap(&self) -> LookinGlassResult<usize> {
        match &self.data {
            ValueData::Slice(v) => Ok(v.capacity()),
            ValueData::Array(v) => Ok(v.len()),
            ValueData::Channel(c) => Ok(c.lock().unwrap().capacity()),
            _ => Err(invalid_operation(&format!("Cap() called on {}", self.kind()))),
        }
    }

    /// Index into the value (for arrays, slices, strings)
    pub fn index(&self, i: usize) -> LookinGlassResult<Value> {
        match &self.data {
            ValueData::String(s) => {
                let chars: Vec<char> = s.chars().collect();
                chars.get(i)
                    .map(|&c| Value::from_string(c.to_string()))
                    .ok_or_else(|| index_error(&format!("String index {} out of range", i)))
            }
            ValueData::Bytes(b) => {
                b.get(i)
                    .map(|&byte| Value::from_uint(byte as u64))
                    .ok_or_else(|| index_error(&format!("Byte slice index {} out of range", i)))
            }
            ValueData::Slice(v) | ValueData::Array(v) => {
                v.get(i)
                    .cloned()
                    .ok_or_else(|| index_error(&format!("Index {} out of range", i)))
            }
            _ => Err(invalid_operation(&format!("Index() called on {}", self.kind()))),
        }
    }

    /// Get map value by key
    pub fn map_index(&self, key: &Value) -> LookinGlassResult<Value> {
        match &self.data {
            ValueData::Map(m) => {
                m.get(key)
                    .cloned()
                    .ok_or_else(|| index_error("Key not found in map"))
            }
            _ => Err(invalid_operation(&format!("MapIndex() called on {}", self.kind()))),
        }
    }

    /// Get all map keys
    pub fn map_keys(&self) -> LookinGlassResult<Vec<Value>> {
        match &self.data {
            ValueData::Map(m) => Ok(m.keys().cloned().collect()),
            _ => Err(invalid_operation(&format!("MapKeys() called on {}", self.kind()))),
        }
    }

    /// Get struct field by index
    pub fn field(&self, i: usize) -> LookinGlassResult<Value> {
        match &self.data {
            ValueData::Struct(fields) => {
                fields.get(i)
                    .cloned()
                    .ok_or_else(|| index_error(&format!("Field index {} out of range", i)))
            }
            _ => Err(invalid_operation(&format!("Field() called on {}", self.kind()))),
        }
    }

    /// Get struct field by name
    pub fn field_by_name(&self, name: &str) -> LookinGlassResult<Value> {
        if self.kind() != Kind::Struct {
            return Err(invalid_operation(&format!("FieldByName() called on {}", self.kind())));
        }

        // Get field index from type information
        let field_info = self.typ.field_by_name(name)?;
        let field_index = field_info.index()[0]; // Use first index for top-level access
        self.field(field_index)
    }

    /// Get the number of struct fields
    pub fn num_field(&self) -> usize {
        match &self.data {
            ValueData::Struct(fields) => fields.len(),
            _ => 0,
        }
    }

    /// Dereference pointer
    pub fn elem(&self) -> LookinGlassResult<Value> {
        match &self.data {
            ValueData::Pointer(Some(v)) => Ok((**v).clone()),
            ValueData::Pointer(None) => Err(value_error("Cannot dereference nil pointer")),
            ValueData::Interface(Some(v)) => Ok((**v).clone()),
            ValueData::Interface(None) => Err(value_error("Cannot dereference nil interface")),
            _ => Err(invalid_operation(&format!("Elem() called on {}", self.kind()))),
        }
    }

    /// Get the address of this value
    pub fn addr(&self) -> LookinGlassResult<Value> {
        if !self.can_addr() {
            return Err(cannot_set("Value is not addressable"));
        }

        let ptr_type = Type::new(Kind::Pointer, format!("*{}", self.typ), "".to_string())
            .with_elem(self.typ.clone());
        
        Ok(Value::new(ptr_type, ValueData::Pointer(Some(Box::new(self.clone())))))
    }

    // Setters

    /// Set the value (requires can_set to be true)
    pub fn set(&mut self, x: Value) -> LookinGlassResult<()> {
        if !self.can_set() {
            return Err(cannot_set("Value cannot be set"));
        }

        if !x.typ.assignable_to(&self.typ) {
            return Err(type_mismatch(&format!(
                "Cannot assign {} to {}", x.typ, self.typ
            )));
        }

        self.data = x.data;
        Ok(())
    }

    /// Set boolean value
    pub fn set_bool(&mut self, x: bool) -> LookinGlassResult<()> {
        if self.kind() != Kind::Bool {
            return Err(type_mismatch("Cannot set non-bool value with bool"));
        }
        if !self.can_set() {
            return Err(cannot_set("Value cannot be set"));
        }
        self.data = ValueData::Bool(x);
        Ok(())
    }

    /// Set integer value
    pub fn set_int(&mut self, x: i64) -> LookinGlassResult<()> {
        if !self.kind().is_signed_int() {
            return Err(type_mismatch("Cannot set non-int value with int"));
        }
        if !self.can_set() {
            return Err(cannot_set("Value cannot be set"));
        }
        self.data = ValueData::Int(x);
        Ok(())
    }

    /// Set unsigned integer value
    pub fn set_uint(&mut self, x: u64) -> LookinGlassResult<()> {
        if !self.kind().is_unsigned_int() {
            return Err(type_mismatch("Cannot set non-uint value with uint"));
        }
        if !self.can_set() {
            return Err(cannot_set("Value cannot be set"));
        }
        self.data = ValueData::Uint(x);
        Ok(())
    }

    /// Set float value
    pub fn set_float(&mut self, x: f64) -> LookinGlassResult<()> {
        if !self.kind().is_float() {
            return Err(type_mismatch("Cannot set non-float value with float"));
        }
        if !self.can_set() {
            return Err(cannot_set("Value cannot be set"));
        }
        self.data = ValueData::Float(x);
        Ok(())
    }

    /// Set string value
    pub fn set_string(&mut self, x: String) -> LookinGlassResult<()> {
        if self.kind() != Kind::String {
            return Err(type_mismatch("Cannot set non-string value with string"));
        }
        if !self.can_set() {
            return Err(cannot_set("Value cannot be set"));
        }
        self.data = ValueData::String(x);
        Ok(())
    }

    /// Set bytes value
    pub fn set_bytes(&mut self, x: Vec<u8>) -> LookinGlassResult<()> {
        match self.kind() {
            Kind::Slice => {
                if !self.can_set() {
                    return Err(cannot_set("Value cannot be set"));
                }
                self.data = ValueData::Bytes(x);
                Ok(())
            }
            _ => Err(type_mismatch("Cannot set non-slice value with bytes")),
        }
    }

    /// Set length (for slices)
    pub fn set_len(&mut self, n: usize) -> LookinGlassResult<()> {
        match &mut self.data {
            ValueData::Slice(v) => {
                if !self.can_set() {
                    return Err(cannot_set("Value cannot be set"));
                }
                v.resize(n, Value::invalid());
                Ok(())
            }
            _ => Err(invalid_operation(&format!("SetLen() called on {}", self.kind()))),
        }
    }

    /// Set map index
    pub fn set_map_index(&mut self, key: Value, elem: Value) -> LookinGlassResult<()> {
        match &mut self.data {
            ValueData::Map(m) => {
                if !self.can_set() {
                    return Err(cannot_set("Value cannot be set"));
                }
                m.insert(key, elem);
                Ok(())
            }
            _ => Err(invalid_operation(&format!("SetMapIndex() called on {}", self.kind()))),
        }
    }

    // Utility methods

    /// Convert to interface{} equivalent
    pub fn interface(&self) -> LookinGlassResult<Box<dyn Any + Send + Sync>> {
        if !self.can_interface() {
            return Err(value_error("Value cannot be converted to interface"));
        }

        match &self.data {
            ValueData::Bool(b) => Ok(Box::new(*b)),
            ValueData::Int(i) => Ok(Box::new(*i)),
            ValueData::Uint(u) => Ok(Box::new(*u)),
            ValueData::Float(f) => Ok(Box::new(*f)),
            ValueData::String(s) => Ok(Box::new(s.clone())),
            ValueData::Bytes(b) => Ok(Box::new(b.clone())),
            _ => Err(value_error("Cannot convert this value type to interface")),
        }
    }

    /// Create an addressable value
    pub fn addressable(typ: Type, data: ValueData) -> Self {
        Self {
            typ,
            data,
            flags: ValueFlags::addressable(),
        }
    }

    /// Create a readonly value
    pub fn readonly(typ: Type, data: ValueData) -> Self {
        Self {
            typ,
            data,
            flags: ValueFlags::readonly(),
        }
    }

    /// Convert value for assignment/comparison
    pub fn convert(&self, t: &Type) -> LookinGlassResult<Value> {
        if !self.typ.convertible_to(t) {
            return Err(conversion_error(&format!(
                "Cannot convert {} to {}", self.typ, t
            )));
        }

        // Same type - no conversion needed
        if &self.typ == t {
            return Ok(self.clone());
        }

        // Perform the conversion based on target type
        match t.kind() {
            Kind::Bool => Ok(Value::new(t.clone(), ValueData::Bool(self.bool()?))),
            Kind::Int | Kind::Int8 | Kind::Int16 | Kind::Int32 | Kind::Int64 => {
                Ok(Value::new(t.clone(), ValueData::Int(self.int()?)))
            }
            Kind::Uint | Kind::Uint8 | Kind::Uint16 | Kind::Uint32 | Kind::Uint64 | Kind::Uintptr => {
                Ok(Value::new(t.clone(), ValueData::Uint(self.uint()?)))
            }
            Kind::Float32 | Kind::Float64 => {
                Ok(Value::new(t.clone(), ValueData::Float(self.float()?)))
            }
            Kind::String => {
                Ok(Value::new(t.clone(), ValueData::String(self.string()?)))
            }
            _ => Err(conversion_error(&format!(
                "Conversion from {} to {} not implemented", self.typ, t
            ))),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.data {
            ValueData::Invalid => write!(f, "<invalid>"),
            ValueData::Bool(b) => write!(f, "{}", b),
            ValueData::Int(i) => write!(f, "{}", i),
            ValueData::Uint(u) => write!(f, "{}", u),
            ValueData::Float(fl) => write!(f, "{}", fl),
            ValueData::Complex(r, i) => write!(f, "({} + {}i)", r, i),
            ValueData::String(s) => write!(f, "\"{}\"", s),
            ValueData::Bytes(b) => write!(f, "{:?}", b),
            ValueData::Slice(v) => {
                write!(f, "[")?;
                for (i, val) in v.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", val)?;
                }
                write!(f, "]")
            }
            ValueData::Array(v) => {
                write!(f, "[{}; ", v.len())?;
                if let Some(first) = v.first() {
                    write!(f, "{}", first)?;
                }
                write!(f, "]")
            }
            ValueData::Map(m) => {
                write!(f, "map[")?;
                for (i, (k, v)) in m.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}: {}", k, v)?;
                }
                write!(f, "]")
            }
            ValueData::Struct(fields) => {
                write!(f, "{{")?;
                for (i, field) in fields.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", field)?;
                }
                write!(f, "}}")
            }
            ValueData::Pointer(None) => write!(f, "<nil>"),
            ValueData::Pointer(Some(v)) => write!(f, "&{}", v),
            ValueData::Interface(None) => write!(f, "<nil>"),
            ValueData::Interface(Some(v)) => write!(f, "{}", v),
            ValueData::Function(_) => write!(f, "<func>"),
            ValueData::Channel(_) => write!(f, "<chan>"),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        // Type must match for equality
        if self.typ != other.typ {
            return false;
        }

        match (&self.data, &other.data) {
            (ValueData::Invalid, ValueData::Invalid) => true,
            (ValueData::Bool(a), ValueData::Bool(b)) => a == b,
            (ValueData::Int(a), ValueData::Int(b)) => a == b,
            (ValueData::Uint(a), ValueData::Uint(b)) => a == b,
            (ValueData::Float(a), ValueData::Float(b)) => a == b,
            (ValueData::Complex(ar, ai), ValueData::Complex(br, bi)) => ar == br && ai == bi,
            (ValueData::String(a), ValueData::String(b)) => a == b,
            (ValueData::Bytes(a), ValueData::Bytes(b)) => a == b,
            (ValueData::Slice(a), ValueData::Slice(b)) => a == b,
            (ValueData::Array(a), ValueData::Array(b)) => a == b,
            (ValueData::Map(a), ValueData::Map(b)) => a == b,
            (ValueData::Struct(a), ValueData::Struct(b)) => a == b,
            (ValueData::Pointer(a), ValueData::Pointer(b)) => a == b,
            (ValueData::Interface(a), ValueData::Interface(b)) => a == b,
            _ => false, // Functions and channels are not comparable
        }
    }
}

impl Eq for Value {}

impl std::hash::Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.typ.kind().hash(state);
        
        match &self.data {
            ValueData::Bool(b) => b.hash(state),
            ValueData::Int(i) => i.hash(state),
            ValueData::Uint(u) => u.hash(state),
            ValueData::Float(f) => f.to_bits().hash(state),
            ValueData::String(s) => s.hash(state),
            ValueData::Bytes(b) => b.hash(state),
            _ => {} // Other types are not hashable
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_values() {
        let bool_val = Value::from_bool(true);
        assert!(bool_val.is_valid());
        assert_eq!(bool_val.kind(), Kind::Bool);
        assert_eq!(bool_val.bool().unwrap(), true);
        assert!(!bool_val.is_zero());

        let int_val = Value::from_int(42);
        assert_eq!(int_val.int().unwrap(), 42);
        assert!(!int_val.is_zero());

        let zero_int = Value::from_int(0);
        assert!(zero_int.is_zero());

        let string_val = Value::from_string("hello".to_string());
        assert_eq!(string_val.string().unwrap(), "hello");
        assert!(!string_val.is_zero());

        let empty_string = Value::from_string("".to_string());
        assert!(empty_string.is_zero());
    }

    #[test]
    fn test_invalid_value() {
        let val = Value::invalid();
        assert!(!val.is_valid());
        assert_eq!(val.kind(), Kind::Invalid);
        assert!(val.is_zero());
    }

    #[test]
    fn test_type_mismatches() {
        let string_val = Value::from_string("hello".to_string());
        assert!(string_val.int().is_err());
        assert!(string_val.bool().is_err());
    }

    #[test]
    fn test_numeric_conversions() {
        let int_val = Value::from_int(42);
        assert_eq!(int_val.uint().unwrap(), 42);
        assert_eq!(int_val.float().unwrap(), 42.0);

        let float_val = Value::from_float(3.14);
        let (real, imag) = float_val.complex().unwrap();
        assert_eq!(real, 3.14);
        assert_eq!(imag, 0.0);
    }

    #[test]
    fn test_collection_operations() {
        let bytes = vec![1, 2, 3, 4];
        let bytes_val = Value::from_bytes(bytes.clone());
        
        assert_eq!(bytes_val.len().unwrap(), 4);
        assert_eq!(bytes_val.bytes().unwrap(), bytes);

        let index_val = bytes_val.index(1).unwrap();
        assert_eq!(index_val.uint().unwrap(), 2);
    }

    #[test]
    fn test_addressability() {
        let typ = Type::basic(Kind::Int32);
        let data = ValueData::Int(42);
        
        let addr_val = Value::addressable(typ, data);
        assert!(addr_val.can_addr());
        assert!(addr_val.can_set());

        let readonly_val = Value::readonly(Type::basic(Kind::Int32), ValueData::Int(42));
        assert!(!readonly_val.can_addr());
        assert!(!readonly_val.can_set());
    }

    #[test]
    fn test_setters() {
        let typ = Type::basic(Kind::Int32);
        let data = ValueData::Int(0);
        let mut val = Value::addressable(typ, data);

        assert!(val.set_int(100).is_ok());
        assert_eq!(val.int().unwrap(), 100);

        // Test type mismatch
        assert!(val.set_string("hello".to_string()).is_err());
    }

    #[test]
    fn test_value_equality() {
        let val1 = Value::from_int(42);
        let val2 = Value::from_int(42);
        let val3 = Value::from_int(24);

        assert_eq!(val1, val2);
        assert_ne!(val1, val3);
    }

    #[test]
    fn test_value_display() {
        assert_eq!(format!("{}", Value::from_bool(true)), "true");
        assert_eq!(format!("{}", Value::from_int(42)), "42");
        assert_eq!(format!("{}", Value::from_string("hello".to_string())), "\"hello\"");
        assert_eq!(format!("{}", Value::invalid()), "<invalid>");
    }

    #[test]
    fn test_conversion() {
        let int_val = Value::from_int(42);
        let float_type = Type::basic(Kind::Float64);
        
        let converted = int_val.convert(&float_type).unwrap();
        assert_eq!(converted.kind(), Kind::Float64);
        assert_eq!(converted.float().unwrap(), 42.0);
    }
}
