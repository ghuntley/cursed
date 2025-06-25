use crate::error::CursedError;
// SlayCommandBuilder implementation for fluent command building

use std::time::Duration;
use super::{SlayCommand, SlayOptions, SlayResult};

/// Fluent builder for creating SlayCommands
#[derive(Debug, Clone)]
pub struct SlayCommandBuilder {
    /// Command name/path
    /// Command arguments
    /// Working directory
    /// Environment variables
    /// Execution timeout
    /// Use shell for execution
    /// Shell path
    /// Buffer size for I/O
    /// Collect output flag
    /// Resource limits
impl SlayCommandBuilder {
    /// Create a new command builder with the given command name
    pub fn new(command: &str) -> Self {
        Self {
        }
    }

    /// Add arguments to the command
    pub fn with_args(mut self, args: &[&str]) -> Self {
        self.args.extend(args.iter().map(|s| s.to_string()));
        self
    /// Add a single argument to the command
    pub fn with_arg(mut self, arg: &str) -> Self {
        self.args.push(arg.to_string());
        self
    /// Set the working directory
    pub fn with_dir(mut self, dir: &str) -> Self {
        self.dir = Some(dir.to_string());
        self
    /// Set environment variables from a slice of key=value strings
    pub fn with_env(mut self, env: &[&str]) -> Self {
        self.env = env.iter().map(|s| s.to_string()).collect();
        self
    /// Add a single environment variable
    pub fn add_env(mut self, key: &str, value: &str) -> Self {
        self.env.push(format!("{}={}", key, value));
        self
    /// Remove an environment variable by key
    pub fn remove_env(mut self, key: &str) -> Self {
        self.env.retain(|env_var| {
            !env_var.starts_with(&format!("{}=", key))
        });
        self
    /// Clear all environment variables
    pub fn clear_env(mut self) -> Self {
        self.env.clear();
        self
    /// Set execution timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    /// Enable or disable shell execution
    pub fn use_shell(mut self, use_shell: bool) -> Self {
        self.use_shell = use_shell;
        self
    /// Set the shell path for shell execution
    pub fn with_shell_path(mut self, shell_path: &str) -> Self {
        self.shell_path = Some(shell_path.to_string());
        self.use_shell = true; // Automatically enable shell when path is set
        self
    /// Set the buffer size for I/O operations
    pub fn with_buffer_size(mut self, buffer_size: usize) -> Self {
        self.buffer_size = buffer_size;
        self
    /// Enable or disable output collection
    pub fn collect_output(mut self, collect: bool) -> Self {
        self.collect_output = collect;
        self
    /// Set memory limit in megabytes
    pub fn with_memory_limit(mut self, memory_mb: i64) -> Self {
        self.memory_limit = Some(memory_mb * 1024 * 1024); // Convert to bytes
        self
    /// Set CPU usage limit as percentage
    pub fn with_cpu_limit(mut self, cpu_percent: f64) -> Self {
        self.cpu_limit = Some(cpu_percent);
        self
    /// Validate the builder configuration
    pub fn validate(&self) -> SlayResult<()> {
        if self.command.is_empty() {
            return Err(CursedError::RuntimeError(
                "Command name cannot be empty".to_string()
            ));
        if let Some(cpu_limit) = self.cpu_limit {
            if cpu_limit <= 0.0 || cpu_limit > 100.0 {
                return Err(CursedError::RuntimeError(
                    "CPU limit must be between 0 and 100".to_string()
                ));
            }
        }

        if let Some(memory_limit) = self.memory_limit {
            if memory_limit <= 0 {
                return Err(CursedError::RuntimeError(
                    "Memory limit must be positive".to_string()
                ));
            }
        }

        if self.buffer_size == 0 {
            return Err(CursedError::RuntimeError(
                "Buffer size must be greater than 0".to_string()
            ));
        Ok(())
    /// Build the SlayCommand with the configured options
    pub fn build(self) -> SlayResult<SlayCommand> {
        self.validate()?;

        let options = SlayOptions {

        let command = SlayCommand::new(&self.command, &self.args.iter().map(|s| s.as_str()).collect::<Vec<_>>())
            .with_options(options);

        Ok(command)
    /// Build and run the command immediately
    pub fn run(self) -> SlayResult<()> {
        let mut command = self.build()?;
        command.run()
    /// Build and get the command output
    pub fn output(self) -> SlayResult<Vec<u8>> {
        let mut command = self.build()?;
        command.output()
    /// Build and get the combined command output
    pub fn combined_output(self) -> SlayResult<Vec<u8>> {
        let mut command = self.build()?;
        command.combined_output()
    /// Get the command string representation
    pub fn to_string(&self) -> String {
        let mut cmd_str = self.command.clone();
        for arg in &self.args {
            cmd_str.push(' ');
            if arg.contains(' ') {
                cmd_str.push_str(&format!("\"{}\"", arg));
            } else {
                cmd_str.push_str(arg);
            }
        }
        cmd_str
    /// Create a builder for common Unix commands
    pub fn ls() -> Self {
        Self::new("ls")
    /// Create a builder for grep command
    pub fn grep(pattern: &str) -> Self {
        Self::new("grep").with_arg(pattern)
    /// Create a builder for find command
    pub fn find(path: &str) -> Self {
        Self::new("find").with_arg(path)
    /// Create a builder for cat command
    pub fn cat(file: &str) -> Self {
        Self::new("cat").with_arg(file)
    /// Create a builder for echo command
    pub fn echo(message: &str) -> Self {
        Self::new("echo").with_arg(message)
    /// Create a builder for curl command
    pub fn curl(url: &str) -> Self {
        Self::new("curl").with_arg(url)
    /// Create a builder for git command
    pub fn git() -> Self {
        Self::new("git")
    /// Create a builder for docker command
    pub fn docker() -> Self {
        Self::new("docker")
    /// Create a builder for python command
    pub fn python(script: Option<&str>) -> Self {
        let mut builder = Self::new("python");
        if let Some(script) = script {
            builder = builder.with_arg(script);
        }
        builder
    /// Create a builder for npm command
    pub fn npm() -> Self {
        Self::new("npm")
    /// Create a builder for cargo command
    pub fn cargo() -> Self {
        Self::new("cargo")
    }
}

/// Convenience function to create a new command builder
pub fn slay_command(command: &str) -> SlayCommandBuilder {
    SlayCommandBuilder::new(command)
