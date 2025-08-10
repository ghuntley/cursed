//! I/O functionality for hardware_acceleration

use crate::error::CursedError;
use std::io::{self, Read, Write};
use crate::stdlib::packages::IOResult;
use crate::stdlib::packages::IOHandler;
use crate::stdlib::packages::IOError;

/// Result type for I/O operations
/// I/O operations handler
/// Initialize I/O processing
pub fn init_hardware_acceleration() -> IOResult<()> {
    let handler = IOHandler::new();
    let test_data = b"test data";
    let mut cursor = std::io::Cursor::new(test_data);
    let result = handler.read_all(&mut cursor)?;
    if result != test_data {
        return Err(IOError::Other("I/O test failed".to_string()));
    }
    println!("📁 I/O processing (hardware_acceleration) initialized");
    Ok(())
}

/// Test I/O functionality
pub fn test_hardware_acceleration() -> IOResult<()> {
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



// Hardware Acceleration additional functions
pub fn get_hardware_detector() -> crate::error::Result<String> {
    Ok("software".to_string()) // Default to software implementation
}
