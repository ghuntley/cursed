use crate::error::CursedError;
// SlayPipeline implementation for command pipeline execution

use std::io::{self, Read, Write};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use super::{SlayCommand, SlayOptions, SlayResult, io_error_to_cursed};

/// Execution pipeline for multiple commands
#[derive(Debug)]
pub struct SlayPipeline {
    /// Commands to execute in sequence
    pub commands: Vec<SlayCommand>,
    /// Pipeline execution options
    pub options: SlayOptions,
}

impl SlayPipeline {
    /// Create a new pipeline with the given commands
    pub fn new(commands: Vec<SlayCommand>) -> Self {
        Self {
            commands,
            options: SlayOptions::default(),
        }
    }

    /// Create a pipeline from commands (convenience function)
    pub fn pipe(commands: Vec<SlayCommand>) -> Self {
        Self::new(commands)
    }

    /// Run the entire pipeline and wait for completion
    pub fn run(&mut self) -> SlayResult<()> {
        self.start()?;
        self.wait()
    }

    /// Start the pipeline without waiting
    pub fn start(&mut self) -> SlayResult<()> {
        if self.commands.is_empty() {
            return Err(CursedError::RuntimeError("Pipeline has no commands".to_string()));
        }

        // Start all commands in the pipeline
        for (i, command) in self.commands.iter_mut().enumerate() {
            // Configure stdio for pipeline
            if i == 0 {
                // First command - can read from stdin
            } else {
                // Middle/last commands - read from previous command's stdout
            }

            command.start()?;
        }

        Ok(())
    }

    /// Wait for all commands in the pipeline to complete
    pub fn wait(&mut self) -> SlayResult<()> {
        for command in &mut self.commands {
            command.wait()?;
        }
        Ok(())
    }

    /// Get the output from the last command in the pipeline
    pub fn output(&mut self) -> SlayResult<Vec<u8>> {
        self.execute_pipeline_with_output(false)
    }

    /// Get combined output from the last command in the pipeline
    pub fn combined_output(&mut self) -> SlayResult<Vec<u8>> {
        self.execute_pipeline_with_output(true)
    }

    /// Configure the pipeline with options
    pub fn with_options(mut self, options: SlayOptions) -> Self {
        self.options = options;
        // Apply options to all commands
        for command in &mut self.commands {
            command.options = options.clone();
        }
        self
    }

    /// Add a command to the pipeline
    pub fn add_command(&mut self, command: SlayCommand) -> &mut Self {
        self.commands.push(command);
        self
    }

    /// Set the commands for the pipeline
    pub fn set_commands(&mut self, commands: Vec<SlayCommand>) -> &mut Self {
        self.commands = commands;
        self
    }

    /// Get string representation of the pipeline
    pub fn to_string(&self) -> String {
        self.commands
            .iter()
            .map(|cmd| cmd.to_string())
            .collect::<Vec<_>>()
            .join(" | ")
    }

    /// Execute the pipeline and collect output
    fn execute_pipeline_with_output(&mut self, combined: bool) -> SlayResult<Vec<u8>> {
        if self.commands.is_empty() {
            return Err(CursedError::RuntimeError("Pipeline has no commands".to_string()));
        }

        if self.commands.len() == 1 {
            // Single command - execute directly
            return if combined {
                self.commands[0].combined_output()
            } else {
                self.commands[0].output()
            };
        }

        // For multiple commands, use shell pipeline for simplicity and reliability
        // This is more portable and handles complex pipelines better
        self.execute_shell_pipeline_with_output(combined)
    }

    /// Execute pipeline using shell for better reliability
    fn execute_shell_pipeline_with_output(&self, combined: bool) -> SlayResult<Vec<u8>> {
        let pipeline_cmd = self.to_string();
        
        let mut cmd = if cfg!(target_os = "windows") {
            let mut command = Command::new("cmd");
            command.args(&["/C", &pipeline_cmd]);
            command
        } else {
            let mut command = Command::new("sh");
            command.args(&["-c", &pipeline_cmd]);
            command
        };

        // Apply pipeline options
        if let Some(ref dir) = self.options.dir {
            cmd.current_dir(dir);
        }

        for env_var in &self.options.env {
            if let Some(eq_pos) = env_var.find('=') {
                let key = &env_var[..eq_pos];
                let value = &env_var[eq_pos + 1..];
                cmd.env(key, value);
            }
        }

        let output = cmd.output().map_err(io_error_to_cursed)?;

        if output.status.success() {
            if combined {
                let mut result = output.stdout;
                result.extend_from_slice(&output.stderr);
                Ok(result)
            } else {
                Ok(output.stdout)
            }
        } else {
            Err(CursedError::RuntimeError(
                format!("Pipeline failed: {}", String::from_utf8_lossy(&output.stderr))
            ))
        }
    }


}

impl SlayCommand {
    /// Helper method to build the underlying Command (made public for pipeline use)
    pub(crate) fn build_command(&self) -> SlayResult<Command> {
        let mut cmd = if self.options.use_shell {
            let shell_args = super::get_shell_args(true, self.options.shell_path.as_deref());
            let mut shell_cmd = Command::new(&shell_args[0]);
            
            if shell_args.len() > 1 {
                shell_cmd.args(&shell_args[1..]);
            }
            
            // Build the full command string
            let full_cmd = format!("{} {}", self.name, self.args.join(" "));
            shell_cmd.arg(full_cmd);
            shell_cmd
        } else {
            let mut direct_cmd = Command::new(&self.name);
            direct_cmd.args(&self.args);
            direct_cmd
        };

        // Set working directory
        if let Some(ref dir) = self.options.dir {
            cmd.current_dir(dir);
        }

        // Set environment variables
        for env_var in &self.options.env {
            if let Some(eq_pos) = env_var.find('=') {
                let key = &env_var[..eq_pos];
                let value = &env_var[eq_pos + 1..];
                cmd.env(key, value);
            }
        }

        Ok(cmd)
    }
}

