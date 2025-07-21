//! Mock database driver implementation for testing

use crate::error::CursedError;
use super::DatabaseDriver;
use std::collections::HashMap;

/// Mock database driver for testing
pub struct MockDriver {
    enabled: bool,
}

impl MockDriver {
    pub fn new() -> Self {
        Self { enabled: true }
    }
}

impl DatabaseDriver for MockDriver {
    fn connect(&self, connection_string: &str) -> Result<(), CursedError> {
        println!("🧪 Mock connecting to database: {}", connection_string);
        Ok(())
    }
    
    fn execute(&self, query: &str) -> Result<Vec<HashMap<String, String>>, CursedError> {
        println!("🧪 Mock executing query: {}", query);
        let mut result = HashMap::new();
        result.insert("mock_column".to_string(), "mock_value".to_string());
        Ok(vec![result])
    }
    
    fn close(&self) -> Result<(), CursedError> {
        println!("🧪 Mock closing connection");
        Ok(())
    }
}

impl Default for MockDriver {
    fn default() -> Self {
        Self::new()
    }
}
