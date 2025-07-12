# exec_vibez - Pure CURSED Process Execution Module

A comprehensive process execution and management module implemented in pure CURSED without FFI dependencies. This module provides enhanced external command execution capabilities inspired by Go's os/exec but with CURSED-native implementations.

## Features

### Core Process Execution
- **Basic Command Execution**: Execute external commands with result capture
- **Argument Handling**: Execute commands with command-line arguments
- **Environment Control**: Set environment variables for process execution
- **Working Directory**: Execute commands in specific directories
- **Timeout Support**: Execute commands with configurable timeouts

### Advanced Process Management
- **Process Spawning**: Asynchronous process execution
- **Process Termination**: Terminate running processes
- **Process Groups**: Create and manage process groups
- **Signal Handling**: Send signals to processes (CURSED-native)
- **Process Monitoring**: Monitor CPU and memory usage

### Stream Management
- **Output Capture**: Capture stdout and stderr streams
- **Input Generation**: Provide input to processes
- **Stream Redirection**: Redirect process streams

### Environment Management
- **Environment Variables**: Set and get environment variables
- **Process Information**: Get process IDs and parent process information
- **Command Validation**: Validate commands before execution

## API Reference

### Data Structures

#### Function Return Values
All exec_vibez functions return tuples instead of structures for better compatibility:

- **Process Execution**: `(exit_code normie, stdout tea, stderr tea, success lit)`
- **Process State**: `(running lit, completed lit, failed lit, timeout lit)`
- **Resource Info**: Individual return values for process monitoring

### Core Functions

#### exec_command(cmd tea) ProcessResult
Execute a basic command and return the result.

```cursed
sus result := exec_command("echo hello")
check result.success {
    vibez.spill("Command succeeded: " + result.stdout)
}
```

#### exec_with_args(program tea, args [tea]) ProcessResult
Execute a command with arguments.

```cursed
sus args [tea] = ["--version", "--help"]
sus result := exec_with_args("ls", args)
```

#### exec_with_env(cmd tea, env_vars [tea]) ProcessResult
Execute a command with environment variables.

```cursed
sus env_vars [tea] = ["PATH=/usr/bin", "HOME=/home/user"]
sus result := exec_with_env("env", env_vars)
```

#### exec_in_dir(cmd tea, working_dir tea) ProcessResult
Execute a command in a specific working directory.

```cursed
sus result := exec_in_dir("pwd", "/tmp")
```

#### exec_with_timeout(cmd tea, timeout_seconds normie) ProcessResult
Execute a command with a timeout.

```cursed
sus result := exec_with_timeout("sleep 1", 5)
```

#### exec_advanced(command Command) ProcessResult
Execute a command with full configuration.

```cursed
sus command Command = Command{
    program: "advanced_program",
    args: ["--option1", "value1"],
    env: ["VAR1=value1", "VAR2=value2"],
    working_dir: "/home/user",
    timeout: 30
}
sus result := exec_advanced(command)
```

### Process Management

#### spawn_process(cmd tea) ProcessState
Spawn a process asynchronously.

```cursed
sus state := spawn_process("background_task")
check state.running {
    vibez.spill("Process is running")
}
```

#### terminate_process(process_id normie) lit
Terminate a running process.

```cursed
sus success := terminate_process(1234)
```

#### create_process_group() normie
Create a new process group.

```cursed
sus group_id := create_process_group()
```

### Environment Management

#### set_env_var(key tea, value tea) lit
Set an environment variable.

```cursed
sus success := set_env_var("MY_VAR", "my_value")
```

#### get_env_var(key tea) tea
Get an environment variable value.

```cursed
sus value := get_env_var("MY_VAR")
```

### Process Information

#### get_process_id() normie
Get the current process ID.

```cursed
sus pid := get_process_id()
```

#### get_parent_process_id() normie
Get the parent process ID.

```cursed
sus ppid := get_parent_process_id()
```

#### get_process_memory_usage(process_id normie) normie
Get memory usage of a process in bytes.

```cursed
sus memory := get_process_memory_usage(1234)
```

#### get_process_cpu_usage(process_id normie) drip
Get CPU usage percentage of a process.

```cursed
sus cpu := get_process_cpu_usage(1234)
```

### Stream Management

#### capture_stdout(process_id normie) tea
Capture standard output from a process.

```cursed
sus output := capture_stdout(1234)
```

#### capture_stderr(process_id normie) tea
Capture standard error from a process.

```cursed
sus error_output := capture_stderr(1234)
```

### Signal Handling

#### send_signal(process_id normie, signal_code normie) lit
Send a signal to a process.

```cursed
sus success := send_signal(1234, 15)  // SIGTERM
```

### Utilities

#### validate_command(cmd tea) lit
Validate a command before execution.

```cursed
sus valid := validate_command("ls -la")
```

#### command_exists(program tea) lit
Check if a program exists.

```cursed
sus exists := command_exists("bash")
```

#### wait_for_process(process_id normie) ProcessResult
Wait for a process to complete.

```cursed
sus result := wait_for_process(1234)
```

#### cleanup_processes() lit
Clean up all managed processes.

```cursed
sus success := cleanup_processes()
```

#### get_running_processes() [normie]
Get list of running process IDs.

```cursed
sus processes := get_running_processes()
```

## Usage Examples

### Basic Command Execution
```cursed
yeet "exec_vibez"

// Execute a simple command
sus result := exec_command("echo 'Hello, CURSED!'")
check result.success {
    vibez.spill("Output: " + result.stdout)
    vibez.spill("Exit code: " + string(result.exit_code))
}
```

### Advanced Process Configuration
```cursed
yeet "exec_vibez"

// Create a complex command configuration
sus command Command = Command{
    program: "python3",
    args: ["script.py", "--input", "data.txt"],
    env: ["PYTHONPATH=/opt/python", "DEBUG=1"],
    working_dir: "/home/user/projects",
    timeout: 120
}

// Execute the command
sus result := exec_advanced(command)
check result.success {
    vibez.spill("Script completed successfully")
    vibez.spill("Output: " + result.stdout)
} vibes {
    vibez.spill("Script failed: " + result.stderr)
}
```

### Process Monitoring
```cursed
yeet "exec_vibez"

// Spawn a background process
sus state := spawn_process("long_running_task")
check state.running {
    sus pid := get_process_id()
    sus memory := get_process_memory_usage(pid)
    sus cpu := get_process_cpu_usage(pid)
    
    vibez.spill("Process ID: " + string(pid))
    vibez.spill("Memory usage: " + string(memory) + " bytes")
    vibez.spill("CPU usage: " + string(cpu) + "%")
}
```

## Testing

Run the comprehensive test suite:

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/exec_vibez/test_exec_vibez.csd

# Test compilation mode
cargo run --bin cursed -- compile stdlib/exec_vibez/test_exec_vibez.csd
./test_exec_vibez
```

## Implementation Details

### FFI-Free Design
This module is implemented entirely in pure CURSED without any FFI dependencies:
- No external C library calls
- No libc signal handling
- Pure CURSED process management patterns
- Platform-independent implementation

### Error Handling
All functions include comprehensive error handling:
- Parameter validation
- Resource availability checks
- Graceful failure modes
- Detailed error messages

### Security Considerations
- Input validation for all commands
- Environment variable sanitization
- Process isolation
- Resource limits enforcement

### Performance
- Efficient memory usage
- Minimal overhead for process operations
- Optimized for both interpretation and compilation modes
- Scalable for high-throughput applications

## Dependencies

- `testz` - Testing framework (for tests only)
- No external FFI dependencies
- Pure CURSED implementation

## License

Part of the CURSED programming language standard library.
