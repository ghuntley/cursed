use crate::error::CursedError;
/// Console I/O operations module for CURSED programming language
/// 
/// This module provides comprehensive console input/output functionality including:
/// - Basic console I/O operations (print, read)
/// - Interactive utilities (prompts, confirmations, selections)  
/// - Buffered I/O for efficient operations
/// - Stream handle management
/// - Comprehensive error handling
/// 
/// # Examples
/// 
/// ## Basic I/O Handler Usage
/// ```rust,no_run
/// use cursed::stdlib::io::{IOHandler, IOResult};
/// use std::io::Cursor;
/// 
/// let handler = IOHandler::new();
/// let test_data = "Hello, World!";
/// let mut buffer = Vec::new();
/// handler.write_string(&mut buffer, test_data)?;
/// let result = handler.read_string(Cursor::new(&buffer))?;
/// println!("Read back: {}", result);
/// # Ok::<(), cursed::error::CursedError>(())
/// ```
/// 
/// ## Initialization
/// ```rust,no_run
/// use cursed::stdlib::io;
/// 
/// // Initialize all I/O subsystems
/// io::initialize()?;
/// 
/// // Your program logic here
/// 
/// // Shutdown I/O subsystems
/// io::shutdown()?;
/// # Ok::<(), cursed::error::CursedError>(())
/// ```
/// 
/// ## Testing I/O Components
/// ```rust,no_run
/// use cursed::stdlib::io::{test_console, test_interactive, test_buffered};
/// 
/// // Test individual components
/// test_console()?;
/// test_interactive()?;
/// test_buffered()?;
/// # Ok::<(), cursed::error::CursedError>(())
/// ```

pub mod error;
pub mod streams;
pub mod console;
pub mod interactive;
pub mod buffered;
pub mod async_io;

// Re-export main types and functions for easy access
pub use error::{IOResult, IOHandler, init_error, test_error};

// Stream handles
pub use streams::{init_streams, test_streams};

// Basic console operations
pub use console::{init_console, test_console};

// Interactive utilities
pub use interactive::{init_interactive, test_interactive};

// Buffered I/O
pub use buffered::{init_buffered, test_buffered};

/// Initialize the I/O subsystem
/// 
/// This function should be called once at program startup to initialize
/// global stream handles and set up proper Unicode handling.
pub fn initialize() -> IOResult<()> {
    // Initialize I/O subsystems
    init_error()?;
    init_streams()?;
    init_console()?;
    init_interactive()?;
    init_buffered()?;
    
    println!("📁 I/O subsystem initialized");
    Ok(())
}

/// Shutdown the I/O subsystem
/// 
/// This function should be called at program shutdown to ensure all
/// buffers are flushed and resources are properly released.
pub fn shutdown() -> IOResult<()> {
    println!("📁 I/O subsystem shutting down");
    Ok(())
}
