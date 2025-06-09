/// fr fr Document and collection types for NoSQL operations

/// fr fr Document representation
#[derive(Debug, Clone)]
pub struct Document {
    pub data: serde_json::Value,
}

/// fr fr Collection operations
#[derive(Debug, Clone)]
pub struct Collection {
    pub name: String,
}

impl Document {
    pub fn new(data: serde_json::Value) -> Self {
        Self { data }
    }
}

impl Collection {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}
