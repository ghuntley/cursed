//! I/O functionality for extensions

use crate::error::CursedError;
use std::io::{self, Read, Write};
use crate::stdlib::packages::IOResult;
use crate::stdlib::packages::IOHandler;
use crate::stdlib::packages::IOError;

/// Result type for I/O operations
/// I/O operations handler
/// Initialize I/O processing
pub fn init_extensions() -> IOResult<()> {
    let handler = IOHandler::new();
    let test_data = b"test data";
    let mut cursor = std::io::Cursor::new(test_data);
    let result = handler.read_all(&mut cursor)?;
    if result != test_data {
        return Err(CursedError::runtime_error(&"I/O test failed".to_string()));
    }
    println!("📁 I/O processing (extensions) initialized");
    Ok(())
}

/// Test I/O functionality
pub fn test_extensions() -> IOResult<()> {
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

// Extension specific types
#[derive(Debug, Clone)]
pub enum ExtensionError {
    InvalidOid,
    ParseError,
    UnsupportedExtension,
    CriticalExtensionNotSupported,
}

pub type ExtensionResult<T> = Result<T, CursedError>;

#[derive(Debug, Clone)]
pub struct ExtensionBuilder {
    pub critical: bool,
    pub oid: String,
    pub value: Vec<u8>,
}

impl ExtensionBuilder {
    pub fn new(oid: &str) -> Self {
        Self {
            critical: false,
            oid: oid.to_string(),
            value: vec![],
        }
    }
    
    pub fn critical(mut self, critical: bool) -> Self {
        self.critical = critical;
        self
    }
    
    pub fn value(mut self, value: Vec<u8>) -> Self {
        self.value = value;
        self
    }
    
    pub fn build(&self) -> ExtensionResult<Vec<u8>> {
        Ok(self.value.clone())
    }
}

#[derive(Debug, Clone)]
pub struct ExtensionValidator {
    pub strict_mode: bool,
}

impl ExtensionValidator {
    pub fn new() -> Self {
        Self { strict_mode: false }
    }
    
    pub fn validate(&self, extension_data: &[u8]) -> ExtensionResult<bool> {
        Ok(true)
    }
    
    pub fn is_critical(&self, extension_data: &[u8]) -> ExtensionResult<bool> {
        Ok(false)
    }
}
