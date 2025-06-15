//! Timeout wrapper functions for command execution

use std::time::Duration;
use crate::error::CursedError;
use super::{SlayCommand, SlayResult};

/// Run a command with a timeout
pub fn run_with_timeout(mut command: SlayCommand, timeout: Duration) -> SlayResult<()> {
    // Apply timeout to command options
    command.options.timeout = Some(timeout);
    command.run()
}

/// Run a command with a timeout and return output
pub fn output_with_timeout(mut command: SlayCommand, timeout: Duration) -> SlayResult<Vec<u8>> {
    // Apply timeout to command options
    command.options.timeout = Some(timeout);
    command.output()
}

/// Run a command with a timeout and return combined output
pub fn combined_output_with_timeout(mut command: SlayCommand, timeout: Duration) -> SlayResult<Vec<u8>> {
    // Apply timeout to command options
    command.options.timeout = Some(timeout);
    command.combined_output()
}

/// Run a command with a timeout using command name and args directly
pub fn run_command_with_timeout(name: &str, args: &[&str], timeout: Duration) -> SlayResult<()> {
    let command = SlayCommand::new(name, args);
    run_with_timeout(command, timeout)
}

/// Get output from a command with timeout using command name and args directly
pub fn get_output_with_timeout(name: &str, args: &[&str], timeout: Duration) -> SlayResult<Vec<u8>> {
    let command = SlayCommand::new(name, args);
    output_with_timeout(command, timeout)
}

/// Get combined output from a command with timeout using command name and args directly
pub fn get_combined_output_with_timeout(name: &str, args: &[&str], timeout: Duration) -> SlayResult<Vec<u8>> {
    let command = SlayCommand::new(name, args);
    combined_output_with_timeout(command, timeout)
}

/// Timeout configuration builder
#[derive(Debug, Clone)]
pub struct TimeoutConfig {
    /// Command execution timeout
    pub execution_timeout: Option<Duration>,
    /// Timeout for waiting before force kill
    pub kill_timeout: Option<Duration>,
    /// Timeout for I/O operations
    pub io_timeout: Option<Duration>,
}

impl TimeoutConfig {
    /// Create a new timeout configuration
    pub fn new() -> Self {
        Self {
            execution_timeout: None,
            kill_timeout: None,
            io_timeout: None,
        }
    }

    /// Set execution timeout
    pub fn with_execution_timeout(mut self, timeout: Duration) -> Self {
        self.execution_timeout = Some(timeout);
        self
    }

    /// Set kill timeout
    pub fn with_kill_timeout(mut self, timeout: Duration) -> Self {
        self.kill_timeout = Some(timeout);
        self
    }

    /// Set I/O timeout
    pub fn with_io_timeout(mut self, timeout: Duration) -> Self {
        self.io_timeout = Some(timeout);
        self
    }

    /// Apply this configuration to a command
    pub fn apply_to_command(self, mut command: SlayCommand) -> SlayCommand {
        if let Some(timeout) = self.execution_timeout {
            command.options.timeout = Some(timeout);
        }
        if let Some(wait_delay) = self.kill_timeout {
            command.options.wait_delay = Some(wait_delay);
        }
        command
    }
}

impl Default for TimeoutConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Timeout utilities
pub mod utils {
    use super::*;
    use std::time::Instant;

    /// Execute a function with a timeout
    pub fn with_timeout<F, T>(timeout: Duration, f: F) -> SlayResult<T>
    where
        F: FnOnce() -> SlayResult<T>,
    {
        let start = Instant::now();
        
        // For now, just execute the function directly
        // In a full implementation, we'd spawn it in a thread and timeout
        let result = f();
        
        if start.elapsed() > timeout {
            Err(CursedError::RuntimeError("Operation timed out".to_string()))
        } else {
            result
        }
    }

    /// Sleep for a duration (useful in timeout scenarios)
    pub fn sleep(duration: Duration) {
        std::thread::sleep(duration);
    }

    /// Get current time (useful for timeout calculations)
    pub fn now() -> Instant {
        Instant::now()
    }

    /// Check if duration has elapsed since start time
    pub fn has_elapsed(start: Instant, duration: Duration) -> bool {
        start.elapsed() >= duration
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stdlib::exec_slay::SlayCommand;

    #[test]
    fn test_timeout_config() {
        let config = TimeoutConfig::new()
            .with_execution_timeout(Duration::from_secs(10))
            .with_kill_timeout(Duration::from_secs(2))
            .with_io_timeout(Duration::from_millis(500));

        assert_eq!(config.execution_timeout, Some(Duration::from_secs(10)));
        assert_eq!(config.kill_timeout, Some(Duration::from_secs(2)));
        assert_eq!(config.io_timeout, Some(Duration::from_millis(500)));
    }

    #[test]
    fn test_timeout_config_apply() {
        let config = TimeoutConfig::new()
            .with_execution_timeout(Duration::from_secs(5));
        
        let command = SlayCommand::new("echo", &["test"]);
        let command_with_timeout = config.apply_to_command(command);
        
        assert_eq!(command_with_timeout.options.timeout, Some(Duration::from_secs(5)));
    }

    #[test]
    fn test_timeout_utils() {
        let start = utils::now();
        utils::sleep(Duration::from_millis(10));
        assert!(utils::has_elapsed(start, Duration::from_millis(5)));
        assert!(!utils::has_elapsed(start, Duration::from_secs(1)));
    }

    #[cfg(unix)]
    #[test]
    fn test_run_with_timeout() {
        let command = SlayCommand::new("echo", &["hello"]);
        let result = run_with_timeout(command, Duration::from_secs(5));
        assert!(result.is_ok());
    }

    #[cfg(unix)]
    #[test]
    fn test_output_with_timeout() {
        let command = SlayCommand::new("echo", &["hello"]);
        let output = output_with_timeout(command, Duration::from_secs(5)).unwrap();
        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("hello"));
    }

    #[cfg(unix)]
    #[test]
    fn test_run_command_with_timeout() {
        let result = run_command_with_timeout("echo", &["test"], Duration::from_secs(5));
        assert!(result.is_ok());
    }

    #[cfg(unix)]
    #[test]
    fn test_get_output_with_timeout() {
        let output = get_output_with_timeout("echo", &["hello"], Duration::from_secs(5)).unwrap();
        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("hello"));
    }

    #[test]
    fn test_with_timeout_util() {
        let result = utils::with_timeout(Duration::from_secs(1), || {
            Ok("test".to_string())
        });
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test");
    }
}
