/// Debug utilities and helper functions
use crate::debug::{DebugInfoManager, SourceLocation, debug_symbols::DebugSymbol};
use crate::error::Error;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tracing::{debug, info, instrument};

/// Debug utilities for enhanced debugging experience
pub struct DebugUtils;

impl DebugUtils {
    /// Create a formatted stack trace from debug information
    #[instrument(skip(debug_manager))]
    pub fn format_stack_trace(
        debug_manager: &DebugInfoManager,
        addresses: &[u64],
    ) -> Vec<String> {
        let mut trace = Vec::new();
        
        for (i, &address) in addresses.iter().enumerate() {
            if let Some(function_name) = Self::find_function_at_address(debug_manager, address) {
                let frame = format!(
                    "#{:<2} 0x{:016x} in {} at <unknown>",
                    i,
                    address,
                    function_name
                );
                trace.push(frame);
            } else {
                let frame = format!("#{:<2} 0x{:016x} in <unknown>", i, address);
                trace.push(frame);
            }
        }
        
        trace
    }

    /// Find function symbol at a specific address
    pub fn find_function_at_address(
        debug_manager: &DebugInfoManager,
        _address: u64,
    ) -> Option<String> {
        // Simplified implementation - return first function if available
        debug_manager.functions().first().cloned()
    }

    /// Create a source location from file path and line number
    pub fn create_location(file: &str, line: u32, column: u32) -> SourceLocation {
        SourceLocation::new(PathBuf::from(file), line, column)
    }

    /// Parse stack trace from debugger output
    pub fn parse_stack_trace(trace_output: &str) -> Vec<StackFrame> {
        let mut frames = Vec::new();
        
        for line in trace_output.split("\n") {
            if let Some(frame) = Self::parse_stack_frame(line) {
                frames.push(frame);
            }
        }
        
        frames
    }

    /// Parse a single stack frame from debugger output
    fn parse_stack_frame(line: &str) -> Option<StackFrame> {
        // Parse common stack frame formats:
        // #0  0x00007ffff7a05b25 in function_name (args) at file.c:42
        // #1  0x0000555555555169 in main () at main.c:10
        
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 6 {
            return None;
        }
        
        // Extract frame number
        let frame_num = parts[0].trim_start_matches('#');
        let frame_number: usize = frame_num.parse().ok()?;
        
        // Extract address
        let address_str = parts[1];
        let address = u64::from_str_radix(address_str.trim_start_matches("0x"), 16).ok()?;
        
        // Extract function name (between "in" and "(")
        let in_index = parts.iter().position(|&s| s == "in")?;
        let function_name = parts.get(in_index + 1)?.to_string();
        
        // Extract file and line (after "at")
        let at_index = parts.iter().position(|&s| s == "at");
        let (file, line) = if let Some(at_idx) = at_index {
            if let Some(file_line) = parts.get(at_idx + 1) {
                let parts: Vec<&str> = file_line.split(':').collect();
                if parts.len() >= 2 {
                    let file = parts[0].to_string();
                    let line: u32 = parts[1].parse().unwrap_or(0);
                    (Some(file), Some(line))
                } else {
                    (None, None)
                }
            } else {
                (None, None)
            }
        } else {
            (None, None)
        };
        
        Some(StackFrame {
            frame_number,
            address,
            function_name,
            file,
            line,
            column: None,
        })
    }

    /// Generate GDB commands for debugging a CURSED program
    pub fn generate_gdb_commands(
        executable_path: &Path,
        debug_manager: &DebugInfoManager,
    ) -> Vec<String> {
        let mut commands = Vec::new();
        
        // Load the executable
        commands.push(format!("file {}", executable_path.display()));
        
        // Set breakpoints on all functions
        for function in debug_manager.functions() {
            commands.push(format!("break {}", function));
        }
        
        // Enable pretty printing
        commands.push("set print pretty on".to_string());
        commands.push("set print array on".to_string());
        commands.push("set print array-indexes on".to_string());
        
        // Set up for CURSED-specific debugging
        commands.push("# CURSED-specific debugging setup".to_string());
        commands.push("set language c".to_string()); // Use C-like debugging until we have custom support
        
        commands
    }

    /// Generate LLDB commands for debugging a CURSED program
    pub fn generate_lldb_commands(
        executable_path: &Path,
        debug_manager: &DebugInfoManager,
    ) -> Vec<String> {
        let mut commands = Vec::new();
        
        // Load the executable
        commands.push(format!("target create {}", executable_path.display()));
        
        // Set breakpoints on all functions
        for function in debug_manager.functions() {
            commands.push(format!("breakpoint set --name {}", function));
        }
        
        // Enable better output formatting
        commands.push("settings set target.prefer-dynamic-value run-target".to_string());
        commands.push("type summary add --summary-string \"${var%V}\" -x \".*\"".to_string());
        
        commands
    }

    /// Create a debug session configuration file
    pub fn create_debug_session_config(
        executable_path: &Path,
        source_files: &[PathBuf],
        debug_manager: &DebugInfoManager,
    ) -> DebugSessionConfig {
        DebugSessionConfig {
            executable: executable_path.to_path_buf(),
            source_directories: source_files
                .iter()
                .filter_map(|f| f.parent().map(|p| p.to_path_buf()))
                .collect::<std::collections::HashSet<_>>()
                .into_iter()
                .collect(),
            breakpoints: debug_manager
                .functions()
                .iter()
                .map(|f| Breakpoint {
                    function_name: Some(f.clone()),
                    file: None,
                    line: None,
                    column: None,
                    condition: None,
                })
                .collect(),
            watch_expressions: Vec::new(),
            environment_variables: HashMap::new(),
            arguments: Vec::new(),
        }
    }

    /// Generate VS Code launch configuration for debugging
    pub fn generate_vscode_launch_config(
        executable_path: &Path,
        source_root: &Path,
    ) -> serde_json::Value {
        serde_json::json!({
            "version": "0.2.0",
            "configurations": [
                {
                    "name": "Debug CURSED Program",
                    "type": "cppdbg",
                    "request": "launch",
                    "program": executable_path.display().to_string(),
                    "args": [],
                    "stopAtEntry": false,
                    "cwd": source_root.display().to_string(),
                    "environment": [],
                    "externalConsole": false,
                    "MIMode": "gdb",
                    "setupCommands": [
                        {
                            "description": "Enable pretty-printing for gdb",
                            "text": "-enable-pretty-printing",
                            "ignoreFailures": true
                        }
                    ],
                    "preLaunchTask": "",
                    "miDebuggerPath": "/usr/bin/gdb",
                    "debugServerPath": "",
                    "debugServerArgs": "",
                    "serverStarted": "",
                    "logging": {
                        "trace": true,
                        "traceResponse": true,
                        "engineLogging": true
                    }
                }
            ]
        })
    }

    /// Extract variable values from debugger output
    pub fn extract_variable_values(debugger_output: &str) -> HashMap<String, String> {
        let mut variables = HashMap::new();
        
        for line in debugger_output.split("\n") {
            // Parse various debugger output formats
            // GDB: variable_name = value
            // LLDB: (type) variable_name = value
            
            if let Some(eq_pos) = line.find(" = ") {
                let (left, right) = line.split_at(eq_pos);
                let variable_name = Self::extract_variable_name(left);
                let value = right[3..].trim().to_string(); // Skip " = "
                
                variables.insert(variable_name, value);
            }
        }
        
        variables
    }

    /// Extract variable name from debugger output
    fn extract_variable_name(output: &str) -> String {
        // Remove type information and extract just the variable name
        if let Some(paren_pos) = output.rfind(')') {
            // LLDB format: (type) variable_name
            output[paren_pos + 1..].trim().to_string()
        } else {
            // GDB format: variable_name
            output.trim().to_string()
        }
    }

    /// Validate debug information for a function
    pub fn validate_function_debug_info(
        function: &DebugSymbol,
        source_file: &Path,
    ) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        // Check if source file exists
        if !source_file.exists() {
            errors.push(format!(
                "Source file {} for function {} does not exist",
                source_file.display(),
                function.name
            ));
        }
        
        // Check if location is valid
        if !function.location.is_valid() {
            errors.push(format!(
                "Invalid source location for function {}",
                function.name
            ));
        }
        
        // Check if address information is present
        if function.address.is_none() {
            errors.push(format!(
                "No address information for function {}",
                function.name
            ));
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

/// Stack frame information parsed from debugger output
#[derive(Debug, Clone)]
pub struct StackFrame {
    pub frame_number: usize,
    pub address: u64,
    pub function_name: String,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub column: Option<u32>,
}

impl std::fmt::Display for StackFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let (Some(file), Some(line)) = (&self.file, self.line) {
            write!(
                f,
                "#{} 0x{:016x} in {} at {}:{}",
                self.frame_number, self.address, self.function_name, file, line
            )
        } else {
            write!(
                f,
                "#{} 0x{:016x} in {}",
                self.frame_number, self.address, self.function_name
            )
        }
    }
}

/// Breakpoint configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Breakpoint {
    pub function_name: Option<String>,
    pub file: Option<PathBuf>,
    pub line: Option<u32>,
    pub column: Option<u32>,
    pub condition: Option<String>,
}

/// Debug session configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DebugSessionConfig {
    pub executable: PathBuf,
    pub source_directories: Vec<PathBuf>,
    pub breakpoints: Vec<Breakpoint>,
    pub watch_expressions: Vec<String>,
    pub environment_variables: HashMap<String, String>,
    pub arguments: Vec<String>,
}

impl DebugSessionConfig {
    /// Save configuration to a file
    pub fn save_to_file(&self, path: &Path) -> Result<(), Error> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| Error::Compile(format!("Failed to serialize debug config: {}", e)))?;
        
        std::fs::write(path, json)
            .map_err(|e| Error::Compile(format!("Failed to write debug config: {}", e)))?;
        
        Ok(())
    }
    
    /// Load configuration from a file
    pub fn load_from_file(path: &Path) -> Result<(), Error> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| Error::Compile(format!("Failed to read debug config: {}", e)))?;
        
        serde_json::from_str(&content)
            .map_err(|e| Error::Compile(format!("Failed to parse debug config: {}", e)))
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::debug::DebugConfig;
    use std::path::PathBuf;

    #[test]
    fn test_stack_frame_parsing() {
        let trace_line = "#0  0x00007ffff7a05b25 in test_function () at main.c:42";
        let frame = DebugUtils::parse_stack_frame(trace_line).unwrap();
        
        assert_eq!(frame.frame_number, 0);
        assert_eq!(frame.address, 0x00007ffff7a05b25);
        assert_eq!(frame.function_name, "test_function");
        assert_eq!(frame.file, Some("main.c".to_string()));
        assert_eq!(frame.line, Some(42));
    }

    #[test]
    fn test_variable_value_extraction() {
        let debugger_output = "x = 42\ny = \"hello\"\n(int) z = 100";
        let variables = DebugUtils::extract_variable_values(debugger_output);
        
        assert_eq!(variables.get("x"), Some(&"42".to_string()));
        assert_eq!(variables.get("y"), Some(&"\"hello\"".to_string()));
        assert_eq!(variables.get("z"), Some(&"100".to_string()));
    }

    #[test]
    fn test_gdb_command_generation() {
        let debug_manager = DebugInfoManager::new();
        let executable = PathBuf::from("/tmp/test_program");
        
        let commands = DebugUtils::generate_gdb_commands(&executable, &debug_manager);
        
        assert!(!commands.is_empty());
        assert!(commands.iter().any(|cmd| cmd.starts_with("file")));
        assert!(commands.iter().any(|cmd| cmd.contains("set print pretty")));
    }

    #[test]
    fn test_vscode_launch_config() {
        let executable = PathBuf::from("/tmp/test_program");
        let source_root = PathBuf::from("/tmp/source");
        
        let config = DebugUtils::generate_vscode_launch_config(&executable, &source_root);
        
        assert!(config.is_object());
        assert!(config["configurations"].is_array());
        assert_eq!(config["configurations"].as_array().unwrap().len(), 1);
    }

    #[test]
    fn test_debug_session_config_serialization() {
        let config = DebugSessionConfig {
            executable: PathBuf::from("/tmp/test"),
            source_directories: Vec::from([PathBuf::from("/tmp/src")]),
            breakpoints: vec![Breakpoint {
                function_name: Some("main".to_string()),
                file: Some(PathBuf::from("main.c")),
                line: Some(10),
                column: Some(1),
                condition: None,
            }],
            watch_expressions: Vec::from(["x".to_string()]),
            environment_variables: HashMap::new(),
            arguments: Vec::from(["arg1".to_string()]),
        };
        
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: DebugSessionConfig = serde_json::from_str(&json).unwrap();
        
        assert_eq!(config.executable, deserialized.executable);
        assert_eq!(config.breakpoints.len(), deserialized.breakpoints.len());
    }
}
