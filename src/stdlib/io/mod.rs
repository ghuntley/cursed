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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_initialization() {
        // Test that the module can be initialized without errors
        assert!(initialize().is_ok());
        assert!(shutdown().is_ok());
    }

    #[test]
    fn test_basic_io_functions_exist() {
        // Test that all expected functions are exported
        // We can't easily test the actual I/O functions in unit tests
        // but we can verify they exist and have the right signatures
        
        // These would normally fail in a test environment, but we just want
        // to verify the functions exist and can be called
        let _ = print;
        let _ = println;
        let _ = eprint;
        let _ = eprintln;
        let _ = read_line;
        let _ = read_char;
        let _ = flush;
    }

    #[test]
    fn test_interactive_functions_exist() {
        let _ = prompt;
        let _ = confirm;
        let _ = select;
        let _ = multi_select;
        let _ = read_password;
        let _ = paginate;
    }

    #[test]
    fn test_buffered_functions_exist() {
        let _ = buffered_stdin;
        let _ = buffered_stdout;
        let _ = buffered_stderr;
        let _ = shared_buffered_stdin;
        let _ = shared_buffered_stdout;
        let _ = shared_buffered_stderr;
    }

    #[test]
    fn test_stream_functions_exist() {
        let _ = stdin;
        let _ = stdout;
        let _ = stderr;
        let _ = flush_all;
    }

    #[test]
    fn test_error_functions_exist() {
        let _ = system_error(1, "test");
        let _ = io_error("test");
        let _ = invalid_input("test");
    }
}
