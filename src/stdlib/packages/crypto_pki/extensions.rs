//! I/O functionality for extensions

use crate::error::CursedError;
use std::io::{self, Read, Write};

/// Result type for I/O operations
pub type IOResult<T> = Result<T, CursedError>;

/// I/O operations handler
pub struct IOHandler {
    buffer_size: usize,
}

impl IOHandler {
    /// Create a new I/O handler
    pub fn new() -> Self {
        Self {
            buffer_size: 8192,
        }
    }
    
    /// Set buffer size
    pub fn buffer_size(mut self, size: usize) -> Self {
        self.buffer_size = size;
        self
    }
    
    /// Read from a reader
    pub fn read_all<R: Read>(&self, mut reader: R) -> IOResult<Vec<u8>> {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)
            .map_err(|e| CursedError::runtime_error(&format!("Read error: {}", e)))?;
        Ok(buffer)
    }
    
    /// Write to a writer
    pub fn write_all<W: Write>(&self, mut writer: W, data: &[u8]) -> IOResult<()> {
        writer.write_all(data)
            .map_err(|e| CursedError::runtime_error(&format!("Write error: {}", e)))?;
        Ok(())
    }
    
    /// Read string from reader
    pub fn read_string<R: Read>(&self, reader: R) -> IOResult<String> {
        let bytes = self.read_all(reader)?;
        String::from_utf8(bytes)
            .map_err(|e| CursedError::runtime_error(&format!("UTF-8 decode error: {}", e)))
    }
    
    /// Write string to writer
    pub fn write_string<W: Write>(&self, writer: W, text: &str) -> IOResult<()> {
        self.write_all(writer, text.as_bytes())
    }
}

impl Default for IOHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Initialize I/O processing
pub fn init_extensions() -> IOResult<()> {
    let handler = IOHandler::new();
    let test_data = b"test data";
    let mut cursor = std::io::Cursor::new(test_data);
    let result = handler.read_all(&mut cursor)?;
    if result != test_data {
        return Err(CursedError::runtime_error("I/O test failed"));
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
        return Err(CursedError::runtime_error("I/O string test failed"));
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
