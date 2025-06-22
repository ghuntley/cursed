/// Enhanced Pipeline Execution System for CURSED
/// 
/// This module provides comprehensive pipeline execution capabilities,
/// allowing multiple commands to be chained together with proper I/O
/// handling, error propagation, and resource management.

use std::collections::VecDeque;
use std::io::{self, Read, Write, BufReader, BufWriter};
use std::process::{Child, Stdio};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::{Duration, Instant};

use crate::stdlib::process::error::{
    ProcessError, ProcessResult, execution_failed, timeout_error, io_error
};
use crate::stdlib::process::enhanced_exec_slay::{SlayCommand, SlayOptions, SlayProcessState};

/// Process pipeline alias for consistency
pub type ProcessPipeline = SlayPipeline;

/// Pipeline stage information  
#[derive(Debug, Clone)]
pub struct PipelineStage {
    /// Stage index in pipeline
    pub index: usize,
    /// Command for this stage
    pub command: SlayCommand,
    /// Stage state
    pub state: PipelineState,
    /// Stage start time
    pub start_time: Option<Instant>,
    /// Stage completion time
    pub end_time: Option<Instant>,
}

/// SlayPipeline manages a series of commands connected via pipes
#[derive(Debug)]
pub struct SlayPipeline {
    /// Commands in the pipeline
    commands: Vec<SlayCommand>,
    /// Pipeline options
    options: SlayOptions,
    /// Active child processes
    children: Vec<Child>,
    /// Pipeline start time
    start_time: Option<Instant>,
    /// Output buffer for final command
    output_buffer: Arc<Mutex<Vec<u8>>>,
    /// Error buffer for pipeline
    error_buffer: Arc<Mutex<Vec<u8>>>,
    /// Pipeline state
    state: PipelineState,
}

/// Pipeline execution state
#[derive(Debug, Clone, PartialEq)]
pub enum PipelineState {
    Created,
    Running,
    Completed,
    Failed,
    Terminated,
}

impl SlayPipeline {
    /// Create a new pipeline with commands
    pub fn new(commands: Vec<SlayCommand>) -> Self {
        Self {
            commands,
            options: SlayOptions::default(),
            children: Vec::new(),
            start_time: None,
            output_buffer: Arc::new(Mutex::new(Vec::new())),
            error_buffer: Arc::new(Mutex::new(Vec::new())),
            state: PipelineState::Created,
        }
    }

    /// Create a pipeline from multiple commands
    pub fn pipe(commands: Vec<SlayCommand>) -> Self {
        Self::new(commands)
    }

    /// Run the entire pipeline and wait for completion
    pub fn run(&mut self) -> ProcessResult<()> {
        self.start()?;
        self.wait()
    }

    /// Start the pipeline without waiting
    pub fn start(&mut self) -> ProcessResult<()> {
        if self.commands.is_empty() {
            return Err(ProcessError::InvalidArguments {
                operation: "start_pipeline".to_string(),
                message: "Pipeline cannot be empty".to_string(),
            });
        }

        self.state = PipelineState::Running;
        self.start_time = Some(Instant::now());
        
        // Create pipes between commands
        let mut previous_stdout: Option<Stdio> = None;
        
        for (i, command) in self.commands.iter().enumerate() {
            let mut cmd = std::process::Command::new(&command.path);
            cmd.args(&command.args);
            
            // Set working directory if specified
            if let Some(dir) = &command.dir {
                cmd.current_dir(dir);
            }
            
            // Set environment variables
            for env_pair in &command.env {
                if let Some((key, value)) = env_pair.split_once('=') {
                    cmd.env(key, value);
                }
            }
            
            // Configure stdin
            if i == 0 {
                // First command uses pipeline input or null
                cmd.stdin(Stdio::null());
            } else {
                // Subsequent commands use output from previous command
                if let Some(stdin) = previous_stdout.take() {
                    cmd.stdin(stdin);
                }
            }
            
            // Configure stdout
            if i == self.commands.len() - 1 {
                // Last command outputs to pipe for capture
                cmd.stdout(Stdio::piped());
            } else {
                // Intermediate commands pipe to next command
                cmd.stdout(Stdio::piped());
            }
            
            // All commands pipe stderr for error capture
            cmd.stderr(Stdio::piped());
            
            // Spawn the process
            let mut child = cmd.spawn()
                .map_err(|e| execution_failed(&command.path, &e.to_string()))?;
            
            // Set up pipe for next command
            if i < self.commands.len() - 1 {
                if let Some(stdout) = child.stdout.take() {
                    previous_stdout = Some(Stdio::from(stdout));
                }
            }
            
            self.children.push(child);
        }
        
        // Start output and error capture for the last command
        self.start_output_capture()?;
        
        Ok(())
    }

    /// Wait for all processes in the pipeline to complete
    pub fn wait(&mut self) -> ProcessResult<()> {
        let mut exit_codes = Vec::new();
        
        for (i, child) in self.children.iter_mut().enumerate() {
            let status = child.wait()
                .map_err(|e| io_error("wait_pipeline", &format!("{:?}", e.kind()), &e.to_string()))?;
            
            exit_codes.push(status.code().unwrap_or(-1));
            
            if !status.success() {
                self.state = PipelineState::Failed;
                return Err(execution_failed(&format!("pipeline_command_{}", i), 
                    &format!("Command failed with exit code {}", status.code().unwrap_or(-1))));
            }
        }
        
        self.state = PipelineState::Completed;
        Ok(())
    }

    /// Get the output from the pipeline (stdout of last command)
    pub fn output(&mut self) -> ProcessResult<Vec<u8>> {
        if self.state == PipelineState::Created {
            self.run()?;
        } else if self.state == PipelineState::Running {
            self.wait()?;
        }
        
        if let Ok(buffer) = self.output_buffer.lock() {
            Ok(buffer.clone())
        } else {
            Err(io_error("get_output", "LockError", "Failed to lock output buffer"))
        }
    }

    /// Get the combined output (stdout + stderr from all commands)
    pub fn combined_output(&mut self) -> ProcessResult<Vec<u8>> {
        let stdout = self.output()?;
        let stderr = if let Ok(buffer) = self.error_buffer.lock() {
            buffer.clone()
        } else {
            Vec::new()
        };
        
        let mut combined = stdout;
        combined.extend(stderr);
        Ok(combined)
    }

    /// Configure pipeline with options
    pub fn with_options(mut self, opts: SlayOptions) -> Self {
        self.options = opts;
        self
    }

    /// Add a command to the pipeline
    pub fn add_command(mut self, cmd: SlayCommand) -> Self {
        self.commands.push(cmd);
        self
    }

    /// Set all commands in the pipeline
    pub fn set_commands(mut self, cmds: Vec<SlayCommand>) -> Self {
        self.commands = cmds;
        self
    }

    /// Get string representation of the pipeline
    pub fn string(&self) -> String {
        self.commands.iter()
            .map(|cmd| format!("{} {}", cmd.path, cmd.args.join(" ")))
            .collect::<Vec<_>>()
            .join(" | ")
    }

    /// Get pipeline state
    pub fn state(&self) -> PipelineState {
        self.state.clone()
    }

    /// Get pipeline execution time
    pub fn execution_time(&self) -> Option<Duration> {
        self.start_time.map(|start| start.elapsed())
    }

    /// Kill all processes in the pipeline
    pub fn kill(&mut self) -> ProcessResult<()> {
        for child in &mut self.children {
            let _ = child.kill(); // Ignore errors, process might already be dead
        }
        self.state = PipelineState::Terminated;
        Ok(())
    }

    /// Check if pipeline is still running
    pub fn is_running(&mut self) -> ProcessResult<bool> {
        if self.state != PipelineState::Running {
            return Ok(false);
        }
        
        for child in &mut self.children {
            match child.try_wait() {
                Ok(Some(_)) => continue, // This process finished
                Ok(None) => return Ok(true), // At least one process still running
                Err(e) => return Err(io_error("is_running", &format!("{:?}", e.kind()), &e.to_string())),
            }
        }
        
        // All processes finished
        self.state = PipelineState::Completed;
        Ok(false)
    }

    /// Start background output capture
    fn start_output_capture(&mut self) -> ProcessResult<()> {
        if let Some(last_child) = self.children.last_mut() {
            // Capture stdout from last command
            if let Some(stdout) = last_child.stdout.take() {
                let output_buffer = Arc::clone(&self.output_buffer);
                thread::spawn(move || {
                    let mut reader = BufReader::new(stdout);
                    let mut buffer = Vec::new();
                    if reader.read_to_end(&mut buffer).is_ok() {
                        if let Ok(mut output) = output_buffer.lock() {
                            output.extend(buffer);
                        }
                    }
                });
            }
            
            // Capture stderr from last command
            if let Some(stderr) = last_child.stderr.take() {
                let error_buffer = Arc::clone(&self.error_buffer);
                thread::spawn(move || {
                    let mut reader = BufReader::new(stderr);
                    let mut buffer = Vec::new();
                    if reader.read_to_end(&mut buffer).is_ok() {
                        if let Ok(mut error) = error_buffer.lock() {
                            error.extend(buffer);
                        }
                    }
                });
            }
        }
        
        // Also capture stderr from all other commands
        for (i, child) in self.children.iter_mut().enumerate() {
            if i == self.children.len() - 1 {
                continue; // Skip last command, already handled above
            }
            
            if let Some(stderr) = child.stderr.take() {
                let error_buffer = Arc::clone(&self.error_buffer);
                thread::spawn(move || {
                    let mut reader = BufReader::new(stderr);
                    let mut buffer = Vec::new();
                    if reader.read_to_end(&mut buffer).is_ok() {
                        if let Ok(mut error) = error_buffer.lock() {
                            error.extend(buffer);
                        }
                    }
                });
            }
        }
        
        Ok(())
    }
}

/// Advanced pipeline builder for complex scenarios
pub struct PipelineBuilder {
    commands: Vec<SlayCommand>,
    options: SlayOptions,
    timeout: Option<Duration>,
    buffer_size: usize,
    parallel_stages: Vec<Vec<SlayCommand>>,
}

impl PipelineBuilder {
    /// Create a new pipeline builder
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            options: SlayOptions::default(),
            timeout: None,
            buffer_size: 8192,
            parallel_stages: Vec::new(),
        }
    }

    /// Add a command to the pipeline
    pub fn add_command(mut self, cmd: SlayCommand) -> Self {
        self.commands.push(cmd);
        self
    }

    /// Add multiple commands as a single stage
    pub fn add_stage(mut self, commands: Vec<SlayCommand>) -> Self {
        for cmd in commands {
            self.commands.push(cmd);
        }
        self
    }

    /// Add a parallel stage (commands that run concurrently)
    pub fn add_parallel_stage(mut self, commands: Vec<SlayCommand>) -> Self {
        self.parallel_stages.push(commands);
        self
    }

    /// Set pipeline timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Set buffer size for I/O operations
    pub fn buffer_size(mut self, size: usize) -> Self {
        self.buffer_size = size;
        self
    }

    /// Set pipeline options
    pub fn options(mut self, opts: SlayOptions) -> Self {
        self.options = opts;
        self
    }

    /// Build the pipeline
    pub fn build(self) -> SlayPipeline {
        let mut pipeline = SlayPipeline::new(self.commands);
        pipeline.options = self.options;
        
        if let Some(timeout) = self.timeout {
            pipeline.options.timeout = Some(timeout);
        }
        
        pipeline.options.buffer_size = self.buffer_size;
        pipeline
    }

    /// Build and run the pipeline
    pub fn run(self) -> ProcessResult<Vec<u8>> {
        let mut pipeline = self.build();
        pipeline.run()?;
        pipeline.output()
    }
}

impl Default for PipelineBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Parallel pipeline executor for concurrent command execution
pub struct ParallelPipeline {
    stages: Vec<Vec<SlayCommand>>,
    options: SlayOptions,
    results: Vec<Arc<Mutex<Option<ProcessResult<Vec<u8>>>>>>,
}

impl ParallelPipeline {
    /// Create a new parallel pipeline
    pub fn new() -> Self {
        Self {
            stages: Vec::new(),
            options: SlayOptions::default(),
            results: Vec::new(),
        }
    }

    /// Add a stage of commands that run in parallel
    pub fn add_stage(mut self, commands: Vec<SlayCommand>) -> Self {
        self.stages.push(commands);
        self
    }

    /// Run all stages of the parallel pipeline
    pub fn run(&mut self) -> ProcessResult<Vec<Vec<u8>>> {
        let mut all_results = Vec::new();
        
        for stage in &self.stages {
            let mut stage_results = Vec::new();
            let mut handles = Vec::new();
            let mut result_holders = Vec::new();
            
            // Start all commands in this stage
            for command in stage {
                let result_holder = Arc::new(Mutex::new(None));
                result_holders.push(result_holder.clone());
                
                let mut cmd = command.clone();
                let handle = thread::spawn(move || {
                    let result = cmd.output();
                    if let Ok(mut holder) = result_holder.lock() {
                        *holder = Some(result);
                    }
                });
                
                handles.push(handle);
            }
            
            // Wait for all commands in this stage to complete
            for handle in handles {
                handle.join().map_err(|_| 
                    ProcessError::System {
                        operation: "parallel_stage".to_string(),
                        message: "Thread join failed".to_string(),
                    }
                )?;
            }
            
            // Collect results from this stage
            for result_holder in result_holders {
                if let Ok(holder) = result_holder.lock() {
                    if let Some(result) = holder.as_ref() {
                        match result {
                            Ok((output, _)) => stage_results.push(output.clone()),
                            Err(e) => return Err(e.clone()),
                        }
                    }
                }
            }
            
            all_results.push(stage_results);
        }
        
        // Flatten results
        Ok(all_results.into_iter().flatten().collect())
    }
}

impl Default for ParallelPipeline {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience functions for pipeline operations
pub fn pipe(commands: Vec<SlayCommand>) -> SlayPipeline {
    SlayPipeline::pipe(commands)
}

pub fn run_pipeline(commands: Vec<SlayCommand>) -> ProcessResult<Vec<u8>> {
    let mut pipeline = pipe(commands);
    pipeline.run()?;
    pipeline.output()
}

pub fn run_pipeline_with_timeout(commands: Vec<SlayCommand>, timeout: Duration) -> ProcessResult<Vec<u8>> {
    let mut pipeline = pipe(commands);
    pipeline.options.timeout = Some(timeout);
    
    let start = Instant::now();
    pipeline.start()?;
    
    while start.elapsed() < timeout {
        if !pipeline.is_running()? {
            return pipeline.output();
        }
        thread::sleep(Duration::from_millis(10));
    }
    
    pipeline.kill()?;
    Err(timeout_error("run_pipeline_with_timeout", timeout))
}

/// Shell-style pipeline parsing and execution
pub fn parse_and_run_shell_pipeline(pipeline_str: &str) -> ProcessResult<Vec<u8>> {
    let commands: Vec<SlayCommand> = pipeline_str
        .split(" | ")
        .map(|cmd_str| {
            let parts: Vec<&str> = cmd_str.trim().split_whitespace().collect();
            if parts.is_empty() {
                return SlayCommand::new("true", &[]);
            }
            
            let command = parts[0];
            let args = &parts[1..];
            SlayCommand::new(command, args)
        })
        .collect();
    
    run_pipeline(commands)
}

#[cfg(test)]
mod tests {
    use super::*;
use crate::stdlib::process::info::ProcessState;
use crate::stdlib::process::error::ProcessResult;
use crate::stdlib::process::error::ProcessError;

    #[test]
    fn test_pipeline_creation() {
        let cmd1 = SlayCommand::new("echo", &["hello"]);
        let cmd2 = SlayCommand::new("grep", &["hello"]);
        
        let pipeline = pipe(vec![cmd1, cmd2]);
        assert_eq!(pipeline.state(), PipelineState::Created);
        assert_eq!(pipeline.commands.len(), 2);
    }

    #[test]
    fn test_pipeline_builder() {
        let builder = PipelineBuilder::new()
            .add_command(SlayCommand::new("echo", &["test"]))
            .add_command(SlayCommand::new("wc", &["-l"]))
            .timeout(Duration::from_secs(10))
            .buffer_size(4096);
        
        let pipeline = builder.build();
        assert_eq!(pipeline.commands.len(), 2);
        assert_eq!(pipeline.options.timeout, Some(Duration::from_secs(10)));
        assert_eq!(pipeline.options.buffer_size, 4096);
    }

    #[test]
    #[cfg(unix)]
    fn test_simple_pipeline_execution() {
        let cmd1 = SlayCommand::new("echo", &["hello\nworld\nhello"]);
        let cmd2 = SlayCommand::new("grep", &["hello"]);
        
        let mut pipeline = pipe(vec![cmd1, cmd2]);
        let result = pipeline.run();
        assert!(result.is_ok());
        
        let output = pipeline.output().unwrap();
        let output_str = String::from_utf8_lossy(&output);
        assert!(output_str.contains("hello"));
    }
}
