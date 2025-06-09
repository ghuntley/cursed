/// fr fr NoSQL driver interfaces - the contracts for NoSQL databases periodt

use async_trait::async_trait;

/// fr fr NoSQL driver trait
#[async_trait]
pub trait NoSqlDriver: Send + Sync {
    /// slay Connect to NoSQL database
    async fn connect(&self, connection_string: &str) -> Result<Box<dyn NoSqlConnection>, Box<dyn std::error::Error>>;
}

/// fr fr NoSQL connection trait
#[async_trait]
pub trait NoSqlConnection: Send + Sync {
    /// slay Insert document
    async fn insert(&mut self, collection: &str, document: serde_json::Value) -> Result<String, Box<dyn std::error::Error>>;
    
    /// slay Find documents
    async fn find(&mut self, collection: &str, query: serde_json::Value) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>>;
}
