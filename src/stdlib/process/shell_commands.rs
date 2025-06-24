use crate::error::Error;
/// Shell command execution for CURSED process management
/// 
/// This module provides convenient shell command execution functionality
/// following the ExecSlay specification for shell operations.

use std::collections::HashMap;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Duration;

use crate::stdlib::process::error::{ProcessError, ProcessResult, execution_failed, invalid_arguments};
use crate::stdlib::process::exec_slay::{SlayCommand, SlayProcessState, ProcessStdout, ProcessStderr};

/// Shell command manager for executing shell commands
#[derive(Debug)]
pub struct ShellCommandManager {
    /// Default shell configuration
    pub config: ShellConfig,
}

impl ShellCommandManager {
    /// Create new shell command manager
    pub fn new(config: ShellConfig) -> Self {
        Self { config }
    }
    
    /// Create with default configuration
    pub fn default() -> Self {
        Self::new(ShellConfig::default())
    }
}

/// Shell options type alias
pub type ShellOptions = ShellConfig;

/// Shell configuration for command execution
#[derive(Debug, Clone)]
pub struct ShellConfig {
    /// Shell to use for execution
    pub shell_path: String,
    /// Shell arguments (e.g., ["-c"] for bash)
    pub shell_args: Vec<String>,
    /// Environment variables to set
    pub environment: HashMap<String, String>,
    /// Working directory
    pub working_dir: Option<String>,
    /// Timeout for command execution
    pub timeout: Option<Duration>,
    /// Whether to capture output
    pub capture_output: bool,
    /// Whether to merge stderr with stdout
    pub merge_stderr: bool,
}

impl Default for ShellConfig {
    fn default() -> Self {
        #[cfg(unix)]
        let (shell_path, shell_args) = ("/bin/sh".to_string(), vec!["-c".to_string()]);
        
        #[cfg(windows)]
        let (shell_path, shell_args) = ("cmd".to_string(), vec!["/C".to_string()]);
        
        Self {
            shell_path,
            shell_args,
            environment: HashMap::new(),
            working_dir: None,
            timeout: None,
            capture_output: true,
            merge_stderr: false,
        }
    }
}

/// Run a shell command directly
pub fn run_shell(cmd_string: &str) -> ProcessResult<()> {
    let config = ShellConfig::default();
    run_shell_with_config(cmd_string, &config)
}

/// Run a shell command and return output
pub fn shell_output(cmd_string: &str) -> ProcessResult<Vec<u8>> {
    let mut config = ShellConfig::default();
    config.capture_output = true;
    
    let (output, _) = shell_output_with_config(cmd_string, &config)?;
    Ok(output)
}

/// Run a shell command with environment variables
pub fn run_shell_with_env(cmd_string: &str, env: HashMap<String, String>) -> ProcessResult<()> {
    let mut config = ShellConfig::default();
    config.environment = env;
    
    run_shell_with_config(cmd_string, &config)
}

/// Run a shell command in a specific directory
pub fn run_shell_in_dir<P: AsRef<Path>>(cmd_string: &str, dir: P) -> ProcessResult<()> {
    let mut config = ShellConfig::default();
    config.working_dir = Some(dir.as_ref().to_string_lossy().to_string());
    
    run_shell_with_config(cmd_string, &config)
}

/// Run shell command with full configuration
pub fn run_shell_with_config(cmd_string: &str, config: &ShellConfig) -> ProcessResult<()> {
    if cmd_string.trim().is_empty() {
        return Err(invalid_arguments("run_shell", "cmd_string", "Command string cannot be empty"));
    }

    let mut command = create_shell_command(cmd_string, config)?;
    
    // Configure I/O
    if !config.capture_output {
        command.stdout(Stdio::inherit());
        command.stderr(Stdio::inherit());
    }
    
    // Execute command
    let mut child = command.spawn()
        .map_err(|e| execution_failed(&config.shell_path, &e.to_string()))?;
    
    // Handle timeout
    let status = if let Some(timeout) = config.timeout {
        wait_with_timeout(&mut child, timeout)?
    } else {
        child.wait()
            .map_err(|e| execution_failed("wait", &e.to_string()))?
    };
    
    if status.success() {
        Ok(())
    } else {
        let code = status.code().unwrap_or(-1);
        Err(ProcessError::ExecutionFailed {
            command: cmd_string.to_string(),
            exit_code: Some(code),
            message: format!("Shell command failed with exit code {}", code),
        })
    }
}

/// Get shell command output with configuration
pub fn shell_output_with_config(cmd_string: &str, config: &ShellConfig) -> ProcessResult<(Vec<u8>, SlayProcessState)> {
    if cmd_string.trim().is_empty() {
        return Err(invalid_arguments("shell_output", "cmd_string", "Command string cannot be empty"));
    }

    let mut command = create_shell_command(cmd_string, config)?;
    
    // Configure output capture
    command.stdout(Stdio::piped());
    if config.merge_stderr {
        command.stderr(Stdio::piped());
    }
    
    // Execute and capture output
    let output = if let Some(timeout) = config.timeout {
        let mut child = command.spawn()
            .map_err(|e| execution_failed(&config.shell_path, &e.to_string()))?;
        
        let status = wait_with_timeout(&mut child, timeout)?;
        
        let mut stdout = Vec::new();
        if let Some(mut child_stdout) = child.stdout.take() {
            std::io::Read::read_to_end(&mut child_stdout, &mut stdout)
                .map_err(|e| ProcessError::IoError {
                    operation: "read_stdout".to_string(),
                    kind: format!("{:?}", e.kind()),
                    message: e.to_string(),
                })?;
        }
        
        let mut stderr = Vec::new();
        if config.merge_stderr {
            if let Some(mut child_stderr) = child.stderr.take() {
                std::io::Read::read_to_end(&mut child_stderr, &mut stderr)
                    .map_err(|e| ProcessError::IoError {
                        operation: "read_stderr".to_string(),
                        kind: format!("{:?}", e.kind()),
                        message: e.to_string(),
                    })?;
            }
        }
        
        let mut combined = stdout;
        if config.merge_stderr {
            combined.extend_from_slice(&stderr);
        }
        
        std::process::Output {
            status,
            stdout: combined,
            stderr,
        }
    } else {
        command.output()
            .map_err(|e| execution_failed(&config.shell_path, &e.to_string()))?
    };
    
    // Create process state
    let state = SlayProcessState {
        exit_status: output.status,
        pid: 0, // Not available for completed process
        user_time: Duration::from_millis(0),
        system_time: Duration::from_millis(0),
        max_rss: 0,
    };
    
    if output.status.success() {
        Ok((output.stdout, state))
    } else {
        let code = output.status.code().unwrap_or(-1);
        Err(ProcessError::ExecutionFailed {
            command: cmd_string.to_string(),
            exit_code: Some(code),
            message: format!("Shell command failed with exit code {}", code),
        })
    }
}

/// Enhanced shell execution with SlayCommand
pub fn run_shell_enhanced(cmd_string: &str, config: Option<ShellConfig>) -> ProcessResult<SlayProcessState> {
    let config = config.unwrap_or_default();
    
    let mut slay_cmd = create_slay_shell_command(cmd_string, &config)?;
    slay_cmd.run()?;
    slay_cmd.process_state()
}

/// Get shell output using SlayCommand
pub fn shell_output_enhanced(cmd_string: &str, config: Option<ShellConfig>) -> ProcessResult<(Vec<u8>, SlayProcessState)> {
    let config = config.unwrap_or_default();
    
    let mut slay_cmd = create_slay_shell_command(cmd_string, &config)?;
    slay_cmd.set_stdout(ProcessStdout::Pipe);
    
    if config.merge_stderr {
        slay_cmd.set_stderr(ProcessStderr::Pipe);
    }
    
    let output = if config.merge_stderr {
        slay_cmd.combined_output()?
    } else {
        slay_cmd.output()?
    };
    
    let state = slay_cmd.process_state()?;
    Ok((output, state))
}

/// Create shell command with proper argument handling
fn create_shell_command(cmd_string: &str, config: &ShellConfig) -> ProcessResult<Command> {
    let mut command = Command::new(&config.shell_path);
    
    // Add shell arguments and command string
    for arg in &config.shell_args {
        command.arg(arg);
    }
    command.arg(cmd_string);
    
    // Set environment variables
    if !config.environment.is_empty() {
        for (key, value) in &config.environment {
            command.env(key, value);
        }
    }
    
    // Set working directory
    if let Some(ref dir) = config.working_dir {
        command.current_dir(dir);
    }
    
    Ok(command)
}

/// Create SlayCommand for shell execution
fn create_slay_shell_command(cmd_string: &str, config: &ShellConfig) -> ProcessResult<SlayCommand> {
    let mut args = config.shell_args.clone();
    args.push(cmd_string.to_string());
    
    let mut slay_cmd = SlayCommand::new(&config.shell_path, &args.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    
    // Set working directory
    if let Some(ref dir) = config.working_dir {
        slay_cmd.set_dir(dir);
    }
    
    // Set environment variables
    if !config.environment.is_empty() {
        let env_pairs: Vec<String> = config.environment
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect();
        slay_cmd.set_env(env_pairs);
    }
    
    Ok(slay_cmd)
}

/// Wait for child process with timeout
fn wait_with_timeout(child: &mut std::process::Child, timeout: Duration) -> ProcessResult<std::process::ExitStatus> {
    let start = std::time::Instant::now();
    
    loop {
        match child.try_wait() {
            Ok(Some(status)) => return Ok(status),
            Ok(None) => {
                if start.elapsed() >= timeout {
                    // Kill the process and wait for it to exit
                    let _ = child.kill();
                    let _ = child.wait();
                    return Err(ProcessError::TimeoutError {
                        operation: "shell_command".to_string(),
                        timeout,
                        message: "Shell command execution timed out".to_string(),
                    });
                }
                std::thread::sleep(Duration::from_millis(10));
            }
            Err(e) => {
                return Err(ProcessError::IoError {
                    operation: "try_wait".to_string(),
                    kind: format!("{:?}", e.kind()),
                    message: e.to_string(),
                });
            }
        }
    }
}

/// Cross-platform command validation
pub fn validate_shell_command(cmd_string: &str) -> ProcessResult<()> {
    if cmd_string.trim().is_empty() {
        return Err(invalid_arguments("validate_shell_command", "cmd_string", "Command string cannot be empty"));
    }
    
    // Check for dangerous patterns (basic security)
    let dangerous_patterns = [
        "rm -rf /",
        "del /",
        "format c:",
        ":(){ :|:& };:",  // Fork bomb
    ];
    
    let cmd_lower = cmd_string.to_lowercase();
    for pattern in &dangerous_patterns {
        if cmd_lower.contains(pattern) {
            return Err(ProcessError::SecurityError {
                operation: "validate_shell_command".to_string(),
                message: format!("Potentially dangerous command pattern detected: {}", pattern),
            });
        }
    }
    
    Ok(())
}

/// Parse shell command into components
pub fn parse_shell_command(cmd_string: &str) -> ProcessResult<(String, Vec<String>)> {
    validate_shell_command(cmd_string)?;
    
    let trimmed = cmd_string.trim();
    if trimmed.is_empty() {
        return Err(invalid_arguments("parse_shell_command", "cmd_string", "Command string cannot be empty"));
    }
    
    // Simple shell parsing (doesn't handle complex quoting)
    let parts: Vec<&str> = trimmed.split_whitespace().collect();
    if parts.is_empty() {
        return Err(invalid_arguments("parse_shell_command", "cmd_string", "No command found"));
    }
    
    let command = parts[0].to_string();
    let args = parts[1..].iter().map(|s| s.to_string()).collect();
    
    Ok((command, args))
}

/// Execute shell script from file
pub fn run_shell_script<P: AsRef<Path>>(script_path: P, config: Option<ShellConfig>) -> ProcessResult<SlayProcessState> {
    let script_path = script_path.as_ref();
    
    if !script_path.exists() {
        return Err(ProcessError::NotFound {
            resource_type: "file".to_string(),
            name: script_path.to_string_lossy().to_string(),
            message: "Script file not found".to_string(),
        });
    }
    
    let script_content = std::fs::read_to_string(script_path)
        .map_err(|e| ProcessError::IoError {
            operation: "read_script".to_string(),
            kind: format!("{:?}", e.kind()),
            message: e.to_string(),
        })?;
    
    let mut config = config.unwrap_or_default();
    config.working_dir = Some(
        script_path.parent()
            .unwrap_or_else(|| Path::new("."))
            .to_string_lossy()
            .to_string()
    );
    
    run_shell_enhanced(&script_content, Some(config))
}

/// Execute shell script and capture output
pub fn shell_script_output<P: AsRef<Path>>(script_path: P, config: Option<ShellConfig>) -> ProcessResult<(Vec<u8>, SlayProcessState)> {
    let script_path = script_path.as_ref();
    
    if !script_path.exists() {
        return Err(ProcessError::NotFound {
            resource_type: "file".to_string(),
            name: script_path.to_string_lossy().to_string(),
            message: "Script file not found".to_string(),
        });
    }
    
    let script_content = std::fs::read_to_string(script_path)
        .map_err(|e| ProcessError::IoError {
            operation: "read_script".to_string(),
            kind: format!("{:?}", e.kind()),
            message: e.to_string(),
        })?;
    
    let mut config = config.unwrap_or_default();
    config.working_dir = Some(
        script_path.parent()
            .unwrap_or_else(|| Path::new("."))
            .to_string_lossy()
            .to_string()
    );
    
    shell_output_enhanced(&script_content, Some(config))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
use crate::stdlib::process::info::ProcessState;
use crate::stdlib::process::error::ProcessResult;
use crate::stdlib::process::error::ProcessError;

    #[test]
    fn test_shell_config_default() {
        let config = ShellConfig::default();
        
        #[cfg(unix)]
        assert_eq!(config.shell_path, "/bin/sh");
        
        #[cfg(windows)]
        assert_eq!(config.shell_path, "cmd");
        
        assert!(config.capture_output);
        assert!(!config.merge_stderr);
    }

    #[test]
    fn test_validate_shell_command() {
        assert!(validate_shell_command("echo hello").is_ok());
        assert!(validate_shell_command("ls -la").is_ok());
        assert!(validate_shell_command("").is_err());
        assert!(validate_shell_command("   ").is_err());
    }

    #[test]
    fn test_parse_shell_command() {
        let result = parse_shell_command("echo hello world").unwrap();
        assert_eq!(result.0, "echo");
        assert_eq!(result.1, vec!["hello", "world"]);
        
        let result = parse_shell_command("ls").unwrap();
        assert_eq!(result.0, "ls");
        assert!(result.1.is_empty());
        
        assert!(parse_shell_command("").is_err());
    }

    #[test]
    fn test_shell_config_with_environment() {
        let mut config = ShellConfig::default();
        config.environment.insert("TEST_VAR".to_string(), "test_value".to_string());
        config.timeout = Some(Duration::from_secs(5));
        
        assert_eq!(config.environment.get("TEST_VAR"), Some(&"test_value".to_string()));
        assert_eq!(config.timeout, Some(Duration::from_secs(5)));
    }

    #[test]
    fn test_create_shell_command() {
        let mut config = ShellConfig::default();
        config.environment.insert("PATH".to_string(), "/usr/bin".to_string());
        config.working_dir = Some("/tmp".to_string());
        
        let command = create_shell_command("echo test", &config).unwrap();
        // We can't easily test the internal state of Command, but we can verify it was created
        assert!(format!("{:?}", command).contains("echo test"));
    }

    #[test]
    #[cfg(unix)]
    fn test_run_shell_basic() {
        // Test basic shell execution
        let result = run_shell("echo hello");
        assert!(result.is_ok());
    }

    #[test]
    #[cfg(unix)]
    fn test_shell_output_basic() {
        // Test shell output capture
        let result = shell_output("echo hello");
        assert!(result.is_ok());
        
        if let Ok(output) = result {
            let output_str = String::from_utf8_lossy(&output);
            assert!(output_str.contains("hello"));
        }
    }

    #[test]
    fn test_shell_config_builder() {
        let mut config = ShellConfig::default();
        config.timeout = Some(Duration::from_secs(10));
        config.capture_output = false;
        config.merge_stderr = true;
        config.environment.insert("TEST".to_string(), "value".to_string());
        
        assert_eq!(config.timeout, Some(Duration::from_secs(10)));
        assert!(!config.capture_output);
        assert!(config.merge_stderr);
        assert_eq!(config.environment.len(), 1);
    }
}
