/// fr fr Document and collection types for NoSQL operations

/// fr fr Document representation
#[derive(Debug, Clone)]
pub struct Document {
/// fr fr Collection operations
#[derive(Debug, Clone)]
pub struct Collection {
impl Document {
    pub fn new(data: serde_json::Value) -> Self {
        Self { data }
    }
impl Collection {
    pub fn new(name: &str) -> Self {
        Self {
        }
    }
}
