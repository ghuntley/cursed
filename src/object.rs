// Object system for CURSED language
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Object {
    pub fields: HashMap<String, ObjectValue>,
}

#[derive(Debug, Clone)]
pub enum ObjectValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
    Object(Object),
}

impl Object {
    pub fn new() -> Self {
        Object {
            fields: HashMap::new(),
        }
    }

    pub fn get(&self, field: &str) -> Option<&ObjectValue> {
        self.fields.get(field)
    }

    pub fn set(&mut self, field: String, value: ObjectValue) {
        self.fields.insert(field, value);
    }
}

impl fmt::Display for ObjectValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ObjectValue::Number(n) => write!(f, "{}", n),
            ObjectValue::String(s) => write!(f, "{}", s),
            ObjectValue::Boolean(b) => write!(f, "{}", b),
            ObjectValue::Null => write!(f, "null"),
            ObjectValue::Object(_) => write!(f, "[object Object]"),
        }
    }
}
