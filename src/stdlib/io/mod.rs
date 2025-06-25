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
/// ## Basic I/O
/// ```rust
/// use crate::stdlib::io::{println, read_line};
/// 
/// println("Hello, World!")?;
/// let name = read_line()?;
/// println(&format!("Hello, {}!", name))?;
/// ```
/// 
/// ## Interactive prompts
/// ```rust
/// use crate::stdlib::io::{prompt, confirm, select};
/// 
/// let name = prompt("Enter your name: ")?;
/// let proceed = confirm("Do you want to continue?")?;
/// 
/// let options = vec!["Option 1".to_string(), "Option 2".to_string()];
/// let choice = select("Choose an option:", &options)?;
/// ```
/// 
/// ## Buffered I/O
/// ```rust
/// use crate::stdlib::io::{buffered_stdin, buffered_stdout};
/// 
/// let mut reader = buffered_stdin();
/// let mut writer = buffered_stdout();
/// 
/// while let Some(line) = reader.read_line()? {
///     writer.write_line(&format!("Echo: {}", line))?;
/// }
/// writer.flush()?;
/// ```

pub mod error;
pub mod streams;
pub mod console;
pub mod interactive;
pub mod buffered;
pub mod async_io;

// Re-export main types and functions for easy access
pub use error::{IoError, IoResult, system_error, io_error, invalid_input};

// Stream handles
pub use streams::{Stdin, Stdout, Stderr, stdin, stdout, stderr, flush_all};

// Basic console operations
pub use console::{
    print, println, eprint, eprintln,
    printf, printfln, eprintf, eprintfln,
    read_line, read_char, read_until, read_all, flush
};

// Interactive utilities
pub use interactive::{
    prompt, confirm, select, multi_select, read_password, paginate, ProgressBar
};

// Buffered I/O
pub use buffered::{
    BufferedReader, BufferedWriter, SharedBufferedReader, SharedBufferedWriter,
    buffered_stdin, buffered_stdout, buffered_stderr,
    shared_buffered_stdin, shared_buffered_stdout, shared_buffered_stderr
};

/// Initialize the I/O subsystem
/// 
/// This function should be called once at program startup to initialize
/// global stream handles and set up proper Unicode handling.
pub fn initialize() -> IoResult<()> {
    // Initialize global stream handles
    let _ = stdin();
    let _ = stdout();
    let _ = stderr();
    
    // Ensure stdout/stderr are flushed
    flush_all()?;
    
    Ok(())
}

/// Shutdown the I/O subsystem
/// 
/// This function should be called at program shutdown to ensure all
/// buffers are flushed and resources are properly released.
pub fn shutdown() -> IoResult<()> {
    flush_all()?;
    Ok(())
}

