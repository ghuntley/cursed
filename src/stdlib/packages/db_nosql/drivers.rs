use crate::error::Error;
/// fr fr NoSQL driver interfaces - the contracts for NoSQL databases periodt

use async_trait::async_trait;
use std::any::Any;

/// fr fr NoSQL driver trait
#[async_trait]
pub trait NoSqlDriver: Send + Sync {
    /// slay Connect to NoSQL database
    async fn connect(&self, connection_string: &str) -> Result<(), Error>;
}

/// fr fr NoSQL connection trait
#[async_trait]
pub trait NoSqlConnection: Send + Sync {
    /// slay Insert document
    async fn insert(&mut self, collection: &str, document: serde_json::Value) -> Result<(), Error>;
    
    /// slay Find documents
    async fn find(&mut self, collection: &str, query: serde_json::Value) -> Result<(), Error>;
    
    /// slay Get underlying type for downcasting
    fn as_any(&self) -> &dyn Any;
    
    /// slay Get mutable underlying type for downcasting
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
