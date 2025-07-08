# exec_slay - Process Execution Module

A comprehensive pure CURSED module for process execution, command running, and process management.

## Features

- **Process Execution**: Execute commands with various options and configurations
- **Command Management**: Run commands with arguments, timeouts, and custom environments
- **Process Control**: Background execution, process monitoring, and termination
- **Environment Variables**: Get, set, and manage environment variables
- **Directory Operations**: Working directory management and path utilities
- **Error Handling**: Comprehensive error reporting and status checking

## Core Functions

### Basic Command Execution

```cursed
# Execute a simple command
sus result ProcessResult = exec_command("echo hello")
vibez.spill(result.stdout)  # Output: "Command output: echo hello"

# Execute command with arguments
sus args []tea = []tea{"arg1", "arg2"}
sus result ProcessResult = exec_command_with_args("mycommand", args)

# Execute command with timeout
sus result ProcessResult = exec_command_timeout("long_command", 30)
```

### Background Process Management

```cursed
# Start background process
sus pid normie = exec_command_background("background_task")

# Check if process is running
vibe is_process_running(pid) {
    vibez.spill("Process is still running")
}

# Kill process
vibe kill_process(pid) {
    vibez.spill("Process terminated successfully")
}
```

### Environment Variable Management

```cursed
# Get environment variable
sus path_value tea = get_env_var("PATH")
vibez.spill("PATH: " + path_value)

# Set environment variable
vibe set_env_var("MY_VAR", "my_value") {
    vibez.spill("Environment variable set")
}

# Get all environment variables
sus all_env []EnvVar = get_all_env_vars()
```

### Advanced Command Execution

```cursed
# Execute shell command
sus result ProcessResult = exec_shell("ls -la | grep .txt")

# Execute command in specific directory
sus result ProcessResult = exec_command_with_dir("make", "/project/path")

# Execute command with custom environment
sus env_vars []EnvVar = []EnvVar{}
sus result ProcessResult = exec_command_with_env("env", env_vars)

# Execute command with input
sus result ProcessResult = exec_command_with_input("cat", "Hello, World!")
```

### Process Information and Control

```cursed
# Get process information
sus info ProcessResult = get_process_info(1234)
vibe info.success {
    vibez.spill("Process info: " + info.stdout)
}

# Check if command exists
vibe command_exists("git") {
    vibez.spill("Git is available")
}

# Get current directory
sus current_dir tea = get_current_dir()
vibez.spill("Current directory: " + current_dir)

# Change directory
vibe change_dir("/tmp") {
    vibez.spill("Changed to /tmp")
}
```

### Batch Command Execution

```cursed
# Execute commands sequentially
sus commands []tea = []tea{"cmd1", "cmd2", "cmd3"}
sus results []ProcessResult = exec_commands_sequential(commands)

# Execute commands in parallel
sus results []ProcessResult = exec_commands_parallel(commands)
```

## Data Structures

### ProcessResult

```cursed
struct ProcessResult {
    stdout tea      # Command output
    stderr tea      # Error output
    exit_code normie # Exit code (0 for success)
    success lit     # Boolean success flag
}
```

### EnvVar

```cursed
struct EnvVar {
    name tea        # Environment variable name
    value tea       # Environment variable value
}
```

## Usage Examples

### Basic Process Execution

```cursed
yeet "exec_slay"

# Execute a simple command
sus result ProcessResult = exec_command("date")
vibe result.success {
    vibez.spill("Date output: " + result.stdout)
} else {
    vibez.spill("Error: " + result.stderr)
}
```

### Command with Arguments

```cursed
yeet "exec_slay"

# Execute grep with arguments
sus args []tea = []tea{"-r", "pattern", "/path/to/search"}
sus result ProcessResult = exec_command_with_args("grep", args)
vibe result.success {
    vibez.spill("Search results: " + result.stdout)
}
```

### Environment Variable Management

```cursed
yeet "exec_slay"

# Check and set environment variables
sus home_dir tea = get_env_var("HOME")
vibez.spill("Home directory: " + home_dir)

# Set custom environment variable
vibe set_env_var("CURSED_MODE", "production") {
    vibez.spill("Environment variable set")
}
```

### Process Monitoring

```cursed
yeet "exec_slay"

# Start a background process
sus pid normie = exec_command_background("long_running_service")
vibez.spill("Started process with PID: " + pid)

# Monitor the process
vibe is_process_running(pid) {
    vibez.spill("Process is running")
    
    # Get process information
    sus info ProcessResult = get_process_info(pid)
    vibe info.success {
        vibez.spill("Process info: " + info.stdout)
    }
}
```

### Command Existence and Directory Operations

```cursed
yeet "exec_slay"

# Check if required commands exist
sus required_commands []tea = []tea{"git", "make", "gcc"}
bestie i := 0; i < len(required_commands); i++ {
    vibe command_exists(required_commands[i]) {
        vibez.spill(required_commands[i] + " is available")
    } else {
        vibez.spill("Warning: " + required_commands[i] + " not found")
    }
}

# Change to project directory and build
sus original_dir tea = get_current_dir()
vibe change_dir("/path/to/project") {
    sus build_result ProcessResult = exec_command("make")
    vibe build_result.success {
        vibez.spill("Build successful")
    }
    change_dir(original_dir)  # Return to original directory
}
```

## Error Handling

All functions return appropriate error indicators:

- `ProcessResult.success`: Boolean flag indicating success/failure
- `ProcessResult.exit_code`: Process exit code (0 for success)
- `ProcessResult.stderr`: Error messages and output
- Function return values: `based` for success, `cap` for failure

## Testing

Run the comprehensive test suite:

```bash
# Test in interpretation mode
cargo run --bin cursed stdlib/exec_slay/test_exec_slay.csd

# Test in compilation mode
cargo run --bin cursed -- compile stdlib/exec_slay/test_exec_slay.csd
./test_exec_slay
```

## Implementation Notes

This is a pure CURSED implementation with mock functionality for demonstration purposes. In a production environment, this module would interface with:

- System process management APIs
- Environment variable system calls
- File system operations
- Process monitoring utilities

The module follows CURSED conventions:
- Uses `tea` for string parameters
- Uses `lit` for boolean returns
- Uses `normie` for integer values
- Follows testz v2.0 testing framework
- Provides comprehensive error handling

## Security Considerations

- Command injection prevention through input validation
- Environment variable sanitization
- Process permission management
- Resource limitation and cleanup
- Secure temporary file handling

This module provides a foundation for process execution in CURSED applications while maintaining security and reliability standards.
