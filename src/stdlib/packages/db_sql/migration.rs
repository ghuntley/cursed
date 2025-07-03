//! I/O functionality for migration

use crate::error::CursedError;
use std::io::{self, Read, Write};
use crate::stdlib::packages::{IOResult, IOHandler, IOError};

/// Result type for I/O operations
/// I/O operations handler
/// Initialize I/O processing
pub fn init_migration() -> IOResult<()> {
    let handler = IOHandler::new();
    let test_data = b"test data";
    let mut cursor = std::io::Cursor::new(test_data);
    let result = handler.read_all(&mut cursor)?;
    if result != test_data {
        return Err(IOError::Other("I/O test failed".to_string()));
    }
    println!("📁 I/O processing (migration) initialized");
    Ok(())
}

/// Test I/O functionality
pub fn test_migration() -> IOResult<()> {
    let handler = IOHandler::new();
    let test_string = "Hello, CURSED I/O!";
    let mut buffer = Vec::new();
    handler.write_string(&mut buffer, test_string)?;
    let result = handler.read_string(std::io::Cursor::new(&buffer))?;
    if result != test_string {
        return Err(IOError::Other("I/O string test failed".to_string()));
    }
    Ok(())
}

// Migration types

use super::DbResult;

/// Database schema version
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SchemaVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl SchemaVersion {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        SchemaVersion { major, minor, patch }
    }
    
    pub fn from_string(version: &str) -> DbResult<Self> {
        let parts: Vec<&str> = version.split('.').collect();
        if parts.len() != 3 {
            return Err(crate::stdlib::database::DatabaseError::migration("Invalid version format"));
        }
        
        let major = parts[0].parse::<u32>()
            .map_err(|_| crate::stdlib::database::DatabaseError::migration("Invalid major version"))?;
        let minor = parts[1].parse::<u32>()
            .map_err(|_| crate::stdlib::database::DatabaseError::migration("Invalid minor version"))?;
        let patch = parts[2].parse::<u32>()
            .map_err(|_| crate::stdlib::database::DatabaseError::migration("Invalid patch version"))?;
            
        Ok(SchemaVersion::new(major, minor, patch))
    }
}

impl std::fmt::Display for SchemaVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Migration script
#[derive(Debug, Clone)]
pub struct MigrationScript {
    pub version: SchemaVersion,
    pub name: String,
    pub up_sql: String,
    pub down_sql: String,
}

impl MigrationScript {
    pub fn new(version: SchemaVersion, name: &str, up_sql: &str, down_sql: &str) -> Self {
        MigrationScript {
            version,
            name: name.to_string(),
            up_sql: up_sql.to_string(),
            down_sql: down_sql.to_string(),
        }
    }
    
    pub fn version(&self) -> &SchemaVersion {
        &self.version
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn up_sql(&self) -> &str {
        &self.up_sql
    }
    
    pub fn down_sql(&self) -> &str {
        &self.down_sql
    }
}
