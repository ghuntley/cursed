//! I/O functionality for chain_validation

use crate::error::CursedError;
use std::io::{self, Read, Write};
use crate::stdlib::packages::IOResult;
use crate::stdlib::packages::IOHandler;

/// Result type for I/O operations
/// I/O operations handler
/// Initialize I/O processing
pub fn init_chain_validation() -> IOResult<()> {
    let handler = IOHandler::new();
    let test_data = b"test data";
    let mut cursor = std::io::Cursor::new(test_data);
    let result = handler.read_all(&mut cursor)?;
    if result != test_data {
        return Err(CursedError::runtime_error("I/O test failed"));
    }
    println!("📁 I/O processing (chain_validation) initialized");
    Ok(())
}

/// Test I/O functionality
pub fn test_chain_validation() -> IOResult<()> {
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

// Chain validation specific types
#[derive(Debug, Clone)]
pub enum ChainError {
    InvalidChain,
    TrustAnchorNotFound,
    ValidationFailed,
    ExpiredCertificate,
}

pub type ChainResult<T> = Result<T, CursedError>;

#[derive(Debug, Clone)]
pub struct ChainValidationPolicy {
    pub max_depth: usize,
    pub allow_self_signed: bool,
    pub check_revocation: bool,
    pub require_basic_constraints: bool,
}

impl Default for ChainValidationPolicy {
    fn default() -> Self {
        Self {
            max_depth: 10,
            allow_self_signed: false,
            check_revocation: true,
            require_basic_constraints: true,
        }
    }
}
