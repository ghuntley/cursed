//! I/O functionality for key_validation

use crate::error::CursedError;
use std::io::{self, Read, Write};
use crate::stdlib::packages::IOResult;
use crate::stdlib::packages::IOHandler;

/// Result type for I/O operations
/// I/O operations handler
/// Initialize I/O processing
pub fn init_key_validation() -> IOResult<()> {
    let handler = IOHandler::new();
    let test_data = b"test data";
    let mut cursor = std::io::Cursor::new(test_data);
    let result = handler.read_all(&mut cursor)?;
    if result != test_data {
        return Err(CursedError::runtime_error("I/O test failed"));
    }
    println!("📁 I/O processing (key_validation) initialized");
    Ok(())
}

/// Test I/O functionality
pub fn test_key_validation() -> IOResult<()> {
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



// Key Validation additional functions
pub fn validate_key(key: &[u8]) -> crate::error::Result<bool> {
    Ok(!key.is_empty() && key.len() >= 16)
}

pub fn validate_key_pair(private_key: &[u8], public_key: &[u8]) -> crate::error::Result<bool> {
    Ok(!private_key.is_empty() && !public_key.is_empty())
}

pub fn validate_key_strength(key: &[u8], min_bits: u32) -> crate::error::Result<bool> {
    let key_bits = key.len() * 8;
    Ok(key_bits >= min_bits as usize)
}
