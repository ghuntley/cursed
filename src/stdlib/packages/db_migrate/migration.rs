use std::io::{Read, Write};
use crate::error::CursedError;
/// Database migration functionality
use std::collections::HashMap;
use std::fmt;
use crate::stdlib::packages::{IOHandler, IOError, IOResult};

/// Result type for migration operations
pub type MigrationResult<T> = Result<T, CursedError>;

/// Database migration structure
#[derive(Debug, Clone)]
pub struct Migration {
    pub id: String,
    pub name: String,
    pub version: u64,
    pub up_script: MigrationScript,
    pub down_script: MigrationScript,
    pub checksum: Option<String>,
}

impl Migration {
    pub fn new(id: String, name: String, version: u64) -> Self {
        Self {
            id,
            name,
            version,
            up_script: MigrationScript::new(),
            down_script: MigrationScript::new(),
            checksum: None,
        }
    }
    
    pub fn with_up_script(mut self, script: MigrationScript) -> Self {
        self.up_script = script;
        self
    }
    
    pub fn with_down_script(mut self, script: MigrationScript) -> Self {
        self.down_script = script;
        self
    }
    
    pub fn with_checksum(mut self, checksum: String) -> Self {
        self.checksum = Some(checksum);
        self
    }
}

/// Migration script containing SQL commands
#[derive(Debug, Clone)]
pub struct MigrationScript {
    pub statements: Vec<String>,
    pub parameters: HashMap<String, String>,
}

impl MigrationScript {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
            parameters: HashMap::new(),
        }
    }
    
    pub fn add_statement(&mut self, statement: String) {
        self.statements.push(statement);
    }
    
    pub fn add_parameter(&mut self, key: String, value: String) {
        self.parameters.insert(key, value);
    }
    
    pub fn from_sql(sql: String) -> Self {
        let statements = sql.split(';')
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.trim().to_string())
            .collect();
        
        Self {
            statements,
            parameters: HashMap::new(),
        }
    }
    
    pub fn to_sql(&self) -> String {
        self.statements.join(";\n") + ";"
    }
}

/// I/O operations handler
/// Initialize I/O processing
pub fn init_migration() -> IOResult<()> {
    let handler = IOHandler::new();
    let test_data = b"test data";
    let mut cursor = std::io::Cursor::new(test_data);
    let result = handler.read_all(&mut cursor)?;
    if result != test_data {
        return Err(CursedError::runtime_error(&"I/O test failed".to_string()));
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
        return Err(CursedError::runtime_error(&"I/O string test failed".to_string()));
    }
    Ok(())
}
