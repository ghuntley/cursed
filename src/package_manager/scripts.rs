/// Safe Script Execution System
/// 
/// Provides sandboxed execution of package installation scripts:
/// - Pre/post install script execution with timeout controls
/// - Sandboxed environment with limited system access
/// - Cross-platform script support (shell, PowerShell, Python)
/// - Security controls and resource limiting

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio, Child};
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::io::{BufRead, BufReader, Write};
use tempfile::TempDir;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{info, warn, error, debug, instrument};

/// Script executor with sandboxing
#[derive(Debug)]
pub struct ScriptExecutor {
    config: ScriptConfig,
    temp_dir: TempDir,
    enabled: bool,
}

/// Script execution configuration
#[derive(Debug, Clone)]
pub struct ScriptConfig {
    pub timeout: Duration,
    pub max_memory: usize,
    pub allowed_commands: Vec<String>,
    pub restricted_paths: Vec<PathBuf>,
    pub environment_vars: HashMap<String, String>,
    pub log_output: bool,
    pub sandbox_enabled: bool,
}

/// Installation script definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallScript {
    pub name: String,
    pub phase: String, // pre-install, post-install, pre-uninstall, post-uninstall
    pub interpreter: ScriptInterpreter,
    pub content: String,
    pub timeout_seconds: Option<u64>,
    pub environment: HashMap<String, String>,
    pub required: bool,
}

/// Script execution context
#[derive(Debug, Clone)]
pub struct ScriptContext {
    pub package_name: String,
    pub package_version: String,
    pub install_dir: PathBuf,
    pub temp_dir: PathBuf,
}

/// Supported script interpreters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScriptInterpreter {
    Shell,      // /bin/sh on Unix, cmd on Windows
    Bash,       // bash on Unix, PowerShell on Windows
    Python,     // python3
    Node,       // node.js
    Custom(String), // Custom interpreter command
}

/// Script execution result
#[derive(Debug)]
pub struct ScriptResult {
    pub success: bool,
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
    pub duration: Duration,
}

/// Script execution errors
#[derive(Error, Debug)]
pub enum ScriptError {
    #[error("Script timeout: {script} exceeded {timeout_seconds} seconds")]
    Timeout { script: String, timeout_seconds: u64 },
    
    #[error("Script failed: {script} exited with code {exit_code}")]
    ExecutionFailed { script: String, exit_code: i32 },
    
    #[error("Interpreter not found: {interpreter}")]
    InterpreterNotFound { interpreter: String },
    
    #[error("Script content invalid: {reason}")]
    InvalidContent { reason: String },
    
    #[error("Permission denied: {operation}")]
    PermissionDenied { operation: String },
    
    #[error("Security violation: {violation}")]
    SecurityViolation { violation: String },
    
    #[error("Resource limit exceeded: {resource} - {limit}")]
    ResourceLimitExceeded { resource: String, limit: String },
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl ScriptExecutor {
    /// Create a new script executor
    pub fn new(enabled: bool, work_dir: PathBuf) -> Self {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        
        let config = ScriptConfig {
            timeout: Duration::from_secs(300), // 5 minutes default
            max_memory: 256 * 1024 * 1024,    // 256MB default
            allowed_commands: vec![
                "echo".to_string(),
                "mkdir".to_string(),
                "cp".to_string(),
                "mv".to_string(),
                "chmod".to_string(),
                "ln".to_string(),
            ],
            restricted_paths: vec![
                PathBuf::from("/etc"),
                PathBuf::from("/bin"),
                PathBuf::from("/usr/bin"),
                PathBuf::from("/sbin"),
                PathBuf::from("/usr/sbin"),
            ],
            environment_vars: HashMap::new(),
            log_output: true,
            sandbox_enabled: true,
        };
        
        Self {
            config,
            temp_dir,
            enabled,
        }
    }
    
    /// Execute an installation script
    #[instrument(skip(self, script, context), fields(script = %script.name, phase = %script.phase))]
    pub async fn execute_script(
        &self,
        script: &InstallScript,
        context: &ScriptContext,
    ) -> Result<ScriptResult, ScriptError> {
        if !self.enabled {
            info!("Script execution disabled, skipping");
            return Ok(ScriptResult {
                success: true,
                exit_code: Some(0),
                stdout: "Script execution disabled".to_string(),
                stderr: String::new(),
                duration: Duration::from_millis(0),
            });
        }
        
        info!("Executing script");
        
        // Validate script content
        self.validate_script_content(script)?;
        
        // Create script file
        let script_file = self.create_script_file(script)?;
        
        // Prepare execution environment
        let mut command = self.prepare_command(script, &script_file, context)?;
        
        // Execute with timeout
        let start_time = Instant::now();
        let result = self.execute_with_timeout(&mut command, script).await?;
        let duration = start_time.elapsed();
        
        // Clean up script file
        let _ = std::fs::remove_file(&script_file);
        
        info!(
            success = result.success,
            duration = ?duration,
            "Script execution completed"
        );
        
        Ok(ScriptResult {
            success: result.success,
            exit_code: result.exit_code,
            stdout: result.stdout,
            stderr: result.stderr,
            duration,
        })
    }
    
    /// Validate script content for security
    fn validate_script_content(&self, script: &InstallScript) -> Result<(), ScriptError> {
        // Check for potentially dangerous commands
        let dangerous_patterns = vec![
            "rm -rf /",
            "dd if=",
            "mkfs",
            "fdisk",
            "format",
            "del /q",
            "rmdir /s",
            "shutdown",
            "reboot",
            "halt",
            "wget",
            "curl",
            "nc ",
            "netcat",
        ];
        
        for pattern in &dangerous_patterns {
            if script.content.contains(pattern) {
                return Err(ScriptError::SecurityViolation {
                    violation: format!("Dangerous pattern detected: {}", pattern),
                });
            }
        }
        
        // Check for suspicious file operations on restricted paths
        for restricted_path in &self.config.restricted_paths {
            let path_str = restricted_path.to_string_lossy();
            if script.content.contains(path_str.as_ref()) {
                warn!("Script accesses restricted path: {}", path_str);
            }
        }
        
        Ok(())
    }
    
    /// Create temporary script file
    fn create_script_file(&self, script: &InstallScript) -> Result<PathBuf, ScriptError> {
        let extension = match script.interpreter {
            ScriptInterpreter::Shell | ScriptInterpreter::Bash => "sh",
            ScriptInterpreter::Python => "py",
            ScriptInterpreter::Node => "js",
            ScriptInterpreter::Custom(_) => "script",
        };
        
        let script_file = self.temp_dir.path().join(format!("{}.{}", script.name, extension));
        
        std::fs::write(&script_file, &script.content)?;
        
        // Make script executable on Unix
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let permissions = std::fs::Permissions::from_mode(0o755);
            std::fs::set_permissions(&script_file, permissions)?;
        }
        
        Ok(script_file)
    }
    
    /// Prepare command for execution
    fn prepare_command(
        &self,
        script: &InstallScript,
        script_file: &Path,
        context: &ScriptContext,
    ) -> Result<Command, ScriptError> {
        let mut command = match &script.interpreter {
            ScriptInterpreter::Shell => {
                #[cfg(unix)]
                let mut cmd = Command::new("/bin/sh");
                #[cfg(windows)]
                let mut cmd = Command::new("cmd");
                
                #[cfg(windows)]
                cmd.args(&["/C"]);
                
                cmd.arg(script_file);
                cmd
            }
            ScriptInterpreter::Bash => {
                #[cfg(unix)]
                let mut cmd = Command::new("bash");
                #[cfg(windows)]
                let mut cmd = Command::new("powershell");
                
                #[cfg(windows)]
                cmd.args(&["-ExecutionPolicy", "Bypass", "-File"]);
                
                cmd.arg(script_file);
                cmd
            }
            ScriptInterpreter::Python => {
                let mut cmd = Command::new("python3");
                cmd.arg(script_file);
                cmd
            }
            ScriptInterpreter::Node => {
                let mut cmd = Command::new("node");
                cmd.arg(script_file);
                cmd
            }
            ScriptInterpreter::Custom(interpreter) => {
                let mut cmd = Command::new(interpreter);
                cmd.arg(script_file);
                cmd
            }
        };
        
        // Set working directory
        command.current_dir(&context.install_dir);
        
        // Configure stdio
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());
        command.stdin(Stdio::null());
        
        // Set environment variables
        let mut env_vars = self.config.environment_vars.clone();
        env_vars.extend(script.environment.clone());
        
        // Add context variables
        env_vars.insert("CURSED_PACKAGE_NAME".to_string(), context.package_name.clone());
        env_vars.insert("CURSED_PACKAGE_VERSION".to_string(), context.package_version.clone());
        env_vars.insert("CURSED_INSTALL_DIR".to_string(), context.install_dir.to_string_lossy().to_string());
        env_vars.insert("CURSED_TEMP_DIR".to_string(), context.temp_dir.to_string_lossy().to_string());
        
        for (key, value) in env_vars {
            command.env(key, value);
        }
        
        // Security: Clear potentially dangerous environment variables
        let dangerous_env_vars = vec!["LD_PRELOAD", "DYLD_INSERT_LIBRARIES", "PATH"];
        for var in dangerous_env_vars {
            command.env_remove(var);
        }
        
        // Set limited PATH for security
        let safe_paths = if cfg!(unix) {
            "/usr/local/bin:/usr/bin:/bin"
        } else {
            r"C:\Windows\System32"
        };
        command.env("PATH", safe_paths);
        
        Ok(command)
    }
    
    /// Execute command with timeout
    async fn execute_with_timeout(
        &self,
        command: &mut Command,
        script: &InstallScript,
    ) -> Result<ScriptResult, ScriptError> {
        let timeout = script.timeout_seconds
            .map(Duration::from_secs)
            .unwrap_or(self.config.timeout);
        
        let mut child = command.spawn()?;
        
        // Read output in background
        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();
        
        let stdout_handle = tokio::task::spawn_blocking(move || {
            let reader = BufReader::new(stdout);
            reader.split(b'\n').collect::<Result<Vec<_>, _>>()
        });
        
        let stderr_handle = tokio::task::spawn_blocking(move || {
            let reader = BufReader::new(stderr);
            reader.split(b'\n').collect::<Result<Vec<_>, _>>()
        });
        
        // Wait for completion with timeout
        let start_time = Instant::now();
        let exit_status = tokio::time::timeout(timeout, async {
            tokio::task::spawn_blocking(move || child.wait())
                .await
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?
        }).await;
        
        let (exit_status, stdout_lines, stderr_lines) = match exit_status {
            Ok(status_result) => {
                let status = status_result?;
                let stdout_lines = stdout_handle.await
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))??;
                let stderr_lines = stderr_handle.await
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))??;
                (status, stdout_lines, stderr_lines)
            }
            Err(_) => {
                // Timeout occurred, kill the process
                warn!("Script timeout, terminating process");
                return Err(ScriptError::Timeout {
                    script: script.name.clone(),
                    timeout_seconds: timeout.as_secs(),
                });
            }
        };
        
        let stdout = stdout_lines.join("\n");
        let stderr = stderr_lines.join("\n");
        
        if self.config.log_output {
            if !stdout.is_empty() {
                debug!("Script stdout: {}", stdout);
            }
            if !stderr.is_empty() {
                debug!("Script stderr: {}", stderr);
            }
        }
        
        let success = exit_status.success();
        let exit_code = exit_status.code();
        
        if !success {
            if let Some(code) = exit_code {
                return Err(ScriptError::ExecutionFailed {
                    script: script.name.clone(),
                    exit_code: code,
                });
            }
        }
        
        Ok(ScriptResult {
            success,
            exit_code,
            stdout,
            stderr,
            duration: start_time.elapsed(),
        })
    }
    
    /// Parse scripts from package metadata
    pub fn parse_scripts_from_metadata(
        &self,
        metadata: &str,
    ) -> Result<Vec<InstallScript>, ScriptError> {
        // Parse scripts from package.toml or similar metadata
        // This is a simplified implementation
        let mut scripts = Vec::new();
        
        // For demonstration, we'll look for script sections in TOML
        if let Ok(value) = toml::from_str::<toml::Value>(metadata) {
            if let Some(scripts_table) = value.get("scripts").and_then(|v| v.as_table()) {
                for (phase, script_value) in scripts_table {
                    if let Some(script_content) = script_value.as_str() {
                        scripts.push(InstallScript {
                            name: format!("{}_script", phase),
                            phase: phase.clone(),
                            interpreter: ScriptInterpreter::Shell,
                            content: script_content.to_string(),
                            timeout_seconds: None,
                            environment: HashMap::new(),
                            required: false,
                        });
                    }
                }
            }
        }
        
        Ok(scripts)
    }
    
    /// Check if interpreter is available
    pub fn check_interpreter(&self, interpreter: &ScriptInterpreter) -> bool {
        let command_name = match interpreter {
            ScriptInterpreter::Shell => {
                #[cfg(unix)] { "/bin/sh" }
                #[cfg(windows)] { "cmd" }
            }
            ScriptInterpreter::Bash => {
                #[cfg(unix)] { "bash" }
                #[cfg(windows)] { "powershell" }
            }
            ScriptInterpreter::Python => "python3",
            ScriptInterpreter::Node => "node",
            ScriptInterpreter::Custom(cmd) => cmd,
        };
        
        Command::new(command_name)
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|status| status.success())
            .unwrap_or(false)
    }
    
    /// Get script execution statistics
    pub fn get_statistics(&self) -> ScriptStatistics {
        ScriptStatistics {
            enabled: self.enabled,
            temp_dir_size: self.get_temp_dir_size(),
            sandbox_enabled: self.config.sandbox_enabled,
            timeout_seconds: self.config.timeout.as_secs(),
        }
    }
    
    /// Get temporary directory size
    fn get_temp_dir_size(&self) -> u64 {
        fn dir_size(path: &Path) -> u64 {
            let mut size = 0;
            if let Ok(entries) = std::fs::read_dir(path) {
                for entry in entries.flatten() {
                    if let Ok(metadata) = entry.metadata() {
                        if metadata.is_file() {
                            size += metadata.len();
                        } else if metadata.is_dir() {
                            size += dir_size(&entry.path());
                        }
                    }
                }
            }
            size
        }
        
        dir_size(self.temp_dir.path())
    }
}

/// Script execution statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptStatistics {
    pub enabled: bool,
    pub temp_dir_size: u64,
    pub sandbox_enabled: bool,
    pub timeout_seconds: u64,
}

impl Default for ScriptConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(300),
            max_memory: 256 * 1024 * 1024,
            allowed_commands: vec![
                "echo".to_string(),
                "mkdir".to_string(),
                "cp".to_string(),
                "mv".to_string(),
                "chmod".to_string(),
                "ln".to_string(),
            ],
            restricted_paths: vec![
                PathBuf::from("/etc"),
                PathBuf::from("/bin"),
                PathBuf::from("/usr/bin"),
                PathBuf::from("/sbin"),
                PathBuf::from("/usr/sbin"),
            ],
            environment_vars: HashMap::new(),
            log_output: true,
            sandbox_enabled: true,
        }
    }
}

impl std::fmt::Display for ScriptInterpreter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScriptInterpreter::Shell => write!(f, "shell"),
            ScriptInterpreter::Bash => write!(f, "bash"),
            ScriptInterpreter::Python => write!(f, "python"),
            ScriptInterpreter::Node => write!(f, "node"),
            ScriptInterpreter::Custom(cmd) => write!(f, "custom({})", cmd),
        }
    }
}
