/// fr fr Database drivers module - all the database connection vibes
pub mod sqlite;
pub mod postgres;
pub mod mysql;
pub mod mock;

// Re-export driver implementations
pub use sqlite::SqliteDriver;
pub use postgres::PostgresDriver;
pub use mysql::MySqlDriver;
pub use mock::MockDriver;

use crate::error::CursedError;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Basic database driver trait
pub trait DatabaseDriver {
    fn connect(&self, connection_string: &str) -> Result<(), CursedError>;
    fn execute(&self, query: &str) -> Result<Vec<HashMap<String, String>>, CursedError>;
    fn close(&self) -> Result<(), CursedError>;
}

/// Result type for SQL operations
pub type SqlResult<T> = Result<T, CursedError>;

/// SQL Error type (alias for now)
pub type SqlError = CursedError;

/// fr fr Driver registry for managing all available database drivers
pub struct DriverRegistry {
    drivers: Arc<Mutex<HashMap<String, Box<dyn DatabaseDriver + Send + Sync>>>>,
}

impl DriverRegistry {
    /// sus Create new driver registry with built-in drivers
    pub fn new() -> Self {
        let mut drivers: HashMap<String, Box<dyn DatabaseDriver + Send + Sync>> = HashMap::new();
        
        // Register built-in drivers
        drivers.insert("sqlite".to_string(), Box::new(SqliteDriver::new()));
        drivers.insert("postgres".to_string(), Box::new(PostgresDriver::new()));
        drivers.insert("postgresql".to_string(), Box::new(PostgresDriver::new()));
        drivers.insert("mysql".to_string(), Box::new(MySqlDriver::new()));
        drivers.insert("mock".to_string(), Box::new(MockDriver::new()));
        
        Self {
            drivers: Arc::new(Mutex::new(drivers))
        }
    }
    
    /// facts Get a driver by name
    pub fn get_driver(&self, name: &str) -> SqlResult<String> {
        let drivers = self.drivers.lock()
            .map_err(|_| CursedError::runtime_error("Failed to lock driver registry"))?;
        
        match drivers.get(name) {
            Some(_) => {
                // Return driver name for now since we can't clone the trait object easily
                Ok(name.to_string())
            }
            None => Err(CursedError::runtime_error(&format!("Driver '{}' not found", name)))
        }
    }
    
    /// lowkey Register a new driver
    pub fn register_driver(&self, name: String, driver: Box<dyn DatabaseDriver + Send + Sync>) -> SqlResult<()> {
        let mut drivers = self.drivers.lock()
            .map_err(|_| CursedError::runtime_error("Failed to lock driver registry"))?;
        
        drivers.insert(name, driver);
        Ok(())
    }
    
    /// highkey List all available drivers
    pub fn list_drivers(&self) -> SqlResult<Vec<String>> {
        let drivers = self.drivers.lock()
            .map_err(|_| CursedError::runtime_error("Failed to lock driver registry"))?;
        
        Ok(drivers.keys().cloned().collect())
    }
    
    /// periodt Check if a driver is available
    pub fn has_driver(&self, name: &str) -> bool {
        if let Ok(drivers) = self.drivers.lock() {
            drivers.contains_key(name)
        } else {
            false
        }
    }
}

impl Default for DriverRegistry {
    fn default() -> Self {
        Self::new()
    }
}

