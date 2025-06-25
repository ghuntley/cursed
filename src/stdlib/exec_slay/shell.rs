use crate::error::CursedError;
// Shell command execution utilities

use std::collections::HashMap;
use std::process::Command;
use super::{SlayResult, io_error_to_cursed, get_default_shell};

/// Run a shell command directly
pub fn run_shell(cmd_string: &str) -> SlayResult<()> {
    let output = execute_shell_command(cmd_string, None, None)?;
    
    if output.status.success() {
        Ok(())
    } else {
        Err(CursedError::RuntimeError(
            format!("Shell command failed with exit code: {}", 
                   output.status.code().unwrap_or(-1))
        ))
    }
}

/// Run a shell command and return its output
pub fn shell_output(cmd_string: &str) -> SlayResult<Vec<u8>> {
    let output = execute_shell_command(cmd_string, None, None)?;
    
    if output.status.success() {
        Ok(output.stdout)
    } else {
        Err(CursedError::RuntimeError(
            format!("Shell command failed: {}", String::from_utf8_lossy(&output.stderr))
        ))
    }
}

/// Run a shell command with environment variables
pub fn run_shell_with_env(cmd_string: &str, env: &HashMap<String, String>) -> SlayResult<()> {
    let output = execute_shell_command(cmd_string, Some(env), None)?;
    
    if output.status.success() {
        Ok(())
    } else {
        Err(CursedError::RuntimeError(
            format!("Shell command failed with exit code: {}", 
                   output.status.code().unwrap_or(-1))
        ))
    }
}

/// Run a shell command in a specific directory
pub fn run_shell_in_dir(cmd_string: &str, dir: &str) -> SlayResult<()> {
    let output = execute_shell_command(cmd_string, None, Some(dir))?;
    
    if output.status.success() {
        Ok(())
    } else {
        Err(CursedError::RuntimeError(
            format!("Shell command failed with exit code: {}", 
                   output.status.code().unwrap_or(-1))
        ))
    }
}

/// Run a shell command with both environment variables and working directory
pub fn run_shell_with_env_and_dir(
    cmd_string: &str,
    env: &HashMap<String, String>,
    dir: &str,
) -> SlayResult<()> {
    let output = execute_shell_command(cmd_string, Some(env), Some(dir))?;
    
    if output.status.success() {
        Ok(())
    } else {
        Err(CursedError::RuntimeError(
            format!("Shell command failed with exit code: {}", 
                   output.status.code().unwrap_or(-1))
        ))
    }
}

/// Get shell output with environment variables
pub fn shell_output_with_env(cmd_string: &str, env: &HashMap<String, String>) -> SlayResult<Vec<u8>> {
    let output = execute_shell_command(cmd_string, Some(env), None)?;
    
    if output.status.success() {
        Ok(output.stdout)
    } else {
        Err(CursedError::RuntimeError(
            format!("Shell command failed: {}", String::from_utf8_lossy(&output.stderr))
        ))
    }
}

/// Get shell output in a specific directory
pub fn shell_output_in_dir(cmd_string: &str, dir: &str) -> SlayResult<Vec<u8>> {
    let output = execute_shell_command(cmd_string, None, Some(dir))?;
    
    if output.status.success() {
        Ok(output.stdout)
    } else {
        Err(CursedError::RuntimeError(
            format!("Shell command failed: {}", String::from_utf8_lossy(&output.stderr))
        ))
    }
}

/// Get combined shell output (stdout + stderr)
pub fn shell_combined_output(cmd_string: &str) -> SlayResult<Vec<u8>> {
    let output = execute_shell_command(cmd_string, None, None)?;
    
    let mut combined = output.stdout;
    combined.extend_from_slice(&output.stderr);
    Ok(combined)
}

/// Execute a shell command with optional environment and directory
fn execute_shell_command(
    cmd_string: &str,
    env: Option<&HashMap<String, String>>,
    dir: Option<&str>,
) -> SlayResult<std::process::Output> {
    let mut command = if cfg!(target_os = "windows") {
        let mut cmd = Command::new("cmd");
        cmd.args(&["/C", cmd_string]);
        cmd
    } else {
        let mut cmd = Command::new(get_default_shell());
        cmd.args(&["-c", cmd_string]);
        cmd
    };

    // Set environment variables
    if let Some(env_vars) = env {
        for (key, value) in env_vars {
            command.env(key, value);
        }
    }

    // Set working directory
    if let Some(working_dir) = dir {
        command.current_dir(working_dir);
    }

    command.output().map_err(io_error_to_cursed)
}

/// Shell command builder for more complex operations
#[derive(Debug, Clone)]
pub struct ShellCommandBuilder {
    /// Command string
    command: String,
    /// Environment variables
    env: HashMap<String, String>,
    /// Working directory
    dir: Option<String>,
    /// Shell to use
    shell: Option<String>,
}

impl ShellCommandBuilder {
    /// Create a new shell command builder
    pub fn new(command: &str) -> Self {
        Self {
            command: command.to_string(),
            env: HashMap::new(),
            dir: None,
            shell: None,
        }
    }

    /// Set an environment variable
    pub fn env(mut self, key: &str, value: &str) -> Self {
        self.env.insert(key.to_string(), value.to_string());
        self
    }

    /// Set multiple environment variables
    pub fn envs(mut self, env_vars: &HashMap<String, String>) -> Self {
        self.env.extend(env_vars.clone());
        self
    }

    /// Set the working directory
    pub fn dir(mut self, dir: &str) -> Self {
        self.dir = Some(dir.to_string());
        self
    }

    /// Set the shell to use
    pub fn shell(mut self, shell: &str) -> Self {
        self.shell = Some(shell.to_string());
        self
    }

    /// Execute the command
    pub fn run(self) -> SlayResult<()> {
        let output = self.execute()?;
        
        if output.status.success() {
            Ok(())
        } else {
            Err(CursedError::RuntimeError(
                format!("Shell command failed with exit code: {}", 
                       output.status.code().unwrap_or(-1))
            ))
        }
    }

    /// Get the command output
    pub fn output(self) -> SlayResult<Vec<u8>> {
        let output = self.execute()?;
        
        if output.status.success() {
            Ok(output.stdout)
        } else {
            Err(CursedError::RuntimeError(
                format!("Shell command failed: {}", String::from_utf8_lossy(&output.stderr))
            ))
        }
    }

    /// Get combined output
    pub fn combined_output(self) -> SlayResult<Vec<u8>> {
        let output = self.execute()?;
        
        let mut combined = output.stdout;
        combined.extend_from_slice(&output.stderr);
        Ok(combined)
    }

    /// Execute the command and return raw output
    fn execute(self) -> SlayResult<std::process::Output> {
        let shell = self.shell.as_deref().unwrap_or(get_default_shell());
        
        let mut command = if cfg!(target_os = "windows") {
            let mut cmd = Command::new(shell);
            cmd.args(&["/C", &self.command]);
            cmd
        } else {
            let mut cmd = Command::new(shell);
            cmd.args(&["-c", &self.command]);
            cmd
        };

        // Set environment variables
        for (key, value) in &self.env {
            command.env(key, value);
        }

        // Set working directory
        if let Some(ref dir) = self.dir {
            command.current_dir(dir);
        }

        command.output().map_err(io_error_to_cursed)
    }
}

/// Common shell utilities
pub mod utils {
    use super::*;

    /// Check if a command exists in PATH
    pub fn command_exists(command: &str) -> bool {
        let check_cmd = if cfg!(target_os = "windows") {
            format!("where {}", command)
        } else {
            format!("which {}", command)
        };

        shell_output(&check_cmd).is_ok()
    }

    /// Get the current working directory
    pub fn pwd() -> SlayResult<String> {
        let output = if cfg!(target_os = "windows") {
            shell_output("cd")?
        } else {
            shell_output("pwd")?
        };

        String::from_utf8(output)
            .map(|s| s.trim().to_string())
            .map_err(|e| CursedError::RuntimeError(format!("Invalid UTF-8: {}", e)))
    }

    /// List directory contents
    pub fn ls(path: Option<&str>) -> SlayResult<String> {
        let cmd = if cfg!(target_os = "windows") {
            format!("dir {}", path.unwrap_or("."))
        } else {
            format!("ls {}", path.unwrap_or("."))
        };

        let output = shell_output(&cmd)?;
        String::from_utf8(output)
            .map_err(|e| CursedError::RuntimeError(format!("Invalid UTF-8: {}", e)))
    }

    /// Change directory
    pub fn cd(path: &str) -> SlayResult<()> {
        std::env::set_current_dir(path)
            .map_err(|e| CursedError::RuntimeError(format!("Failed to change directory: {}", e)))
    }

    /// Create a directory
    pub fn mkdir(path: &str) -> SlayResult<()> {
        let cmd = if cfg!(target_os = "windows") {
            format!("mkdir \"{}\"", path)
        } else {
            format!("mkdir -p \"{}\"", path)
        };

        run_shell(&cmd)
    }

    /// Remove a file or directory
    pub fn rm(path: &str, recursive: bool) -> SlayResult<()> {
        let cmd = if cfg!(target_os = "windows") {
            if recursive {
                format!("rmdir /s /q \"{}\"", path)
            } else {
                format!("del \"{}\"", path)
            }
        } else {
            if recursive {
                format!("rm -rf \"{}\"", path)
            } else {
                format!("rm \"{}\"", path)
            }
        };

        run_shell(&cmd)
    }

    /// Copy a file or directory
    pub fn cp(src: &str, dst: &str, recursive: bool) -> SlayResult<()> {
        let cmd = if cfg!(target_os = "windows") {
            if recursive {
                format!("xcopy \"{}\" \"{}\" /e /i", src, dst)
            } else {
                format!("copy \"{}\" \"{}\"", src, dst)
            }
        } else {
            if recursive {
                format!("cp -r \"{}\" \"{}\"", src, dst)
            } else {
                format!("cp \"{}\" \"{}\"", src, dst)
            }
        };

        run_shell(&cmd)
    }

    /// Move/rename a file or directory
    pub fn mv(src: &str, dst: &str) -> SlayResult<()> {
        let cmd = if cfg!(target_os = "windows") {
            format!("move \"{}\" \"{}\"", src, dst)
        } else {
            format!("mv \"{}\" \"{}\"", src, dst)
        };

        run_shell(&cmd)
    }

    /// Get environment variable
    pub fn get_env(var: &str) -> SlayResult<String> {
        std::env::var(var)
            .map_err(|e| CursedError::RuntimeError(format!("Environment variable not found: {}", e)))
    }

    /// Set environment variable (for current process)
    pub fn set_env(var: &str, value: &str) {
        std::env::set_var(var, value);
    }
}

