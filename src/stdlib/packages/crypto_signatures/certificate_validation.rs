//! Certificate validation functionality

use crate::error::CursedError;
use std::io::{self, Read, Write};
use std::collections::HashMap;
use crate::stdlib::packages::IOResult;
use crate::stdlib::packages::IOHandler;
use crate::stdlib::packages::IOError;

/// Result type for I/O operations
/// Certificate chain validation result
#[derive(Debug, Clone)]
pub struct CertificateChainValidationResult {
    pub is_valid: bool,
    pub chain_length: usize,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub revocation_status: RevocationStatus,
}

impl CertificateChainValidationResult {
    pub fn new(is_valid: bool, chain_length: usize) -> Self {
        Self {
            is_valid,
            chain_length,
            errors: Vec::new(),
            warnings: Vec::new(),
            revocation_status: RevocationStatus::Unknown,
        }
    }

    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
        self.is_valid = false;
    }

    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }
}

/// Certificate revocation status
#[derive(Debug, Clone, PartialEq)]
pub enum RevocationStatus {
    Valid,
    Revoked { reason: String, date: String },
    Unknown,
    CheckFailed { error: String },
}

/// I/O operations handler
/// Initialize I/O processing
pub fn init_certificate_validation() -> IOResult<()> {
    let handler = IOHandler::new();
    let test_data = b"test data";
    let mut cursor = std::io::Cursor::new(test_data);
    let result = handler.read_all(&mut cursor)?;
    if result != test_data {
        return Err(IOError::Other("I/O test failed".to_string()));
    }
    println!("📁 I/O processing (certificate_validation) initialized");
    Ok(())
}

/// Test I/O functionality
pub fn test_certificate_validation() -> IOResult<()> {
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
