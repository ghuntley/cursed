# CURSED Process Management Module

Production-ready process management and system integration for CURSED programs.

## Features

- **Process Control**: Execute commands, spawn processes, process monitoring
- **Environment Variables**: Get/set environment variables
- **Directory Operations**: Change directory, get current directory
- **System Information**: Process IDs, user IDs, hostname, system info
- **Signal Handling**: Send signals, register signal handlers
- **Process Monitoring**: Check running status, memory/CPU usage
- **Command Line Args**: Access program arguments
- **Exit Code Management**: Set and retrieve exit codes

## Usage

### Import the Module

```cursed
yeet "process"
```

### Process Information

```cursed
// Get current process ID
sus pid normie = process.get_process_id()

// Get parent process ID
sus ppid normie = process.get_parent_process_id()

// Get user and group IDs
sus uid normie = process.get_user_id()
sus gid normie = process.get_group_id()

// Get hostname
sus hostname tea = process.get_hostname()

// Get comprehensive system info
sus info process.ProcessInfo = process.get_system_info()
vibez.spill("Hostname: " + info.hostname)
vibez.spill("PID: " + tea(info.pid))
```

### Environment Variables

```cursed
// Set environment variable
sus success lit = process.set_environment_variable("MY_VAR", "my_value")

// Get environment variable
sus value tea = process.get_environment_variable("MY_VAR")
vibez.spill("MY_VAR = " + value)

// Get PATH environment variable
sus path tea = process.get_environment_variable("PATH")
```

### Command Execution

```cursed
// Execute simple command
sus output tea = process.execute_command("ls -la")
vibez.spill(output)

// Execute command with arguments
sus args []tea = []tea{"-la", "/home"}
sus output tea = process.execute_with_args("ls", args)
vibez.spill(output)

// Check exit code
sus exit_code normie = process.get_exit_code()
vibez.spill("Exit code: " + tea(exit_code))
```

### Directory Operations

```cursed
// Get current directory
sus cwd tea = process.get_current_directory()
vibez.spill("Current directory: " + cwd)

// Change directory
sus success lit = process.change_directory("/tmp")
lowkey success {
    vibez.spill("Changed to /tmp")
}
```

### Process Spawning and Management

```cursed
// Spawn new process
sus args []tea = []tea{"hello", "world"}
sus child_pid normie = process.spawn_process("echo", args)
vibez.spill("Spawned process with PID: " + tea(child_pid))

// Wait for process completion
sus exit_code normie = process.wait_for_process(child_pid)
vibez.spill("Process exited with code: " + tea(exit_code))

// Check if process is running
sus is_running lit = process.is_process_running(child_pid)
lowkey is_running {
    vibez.spill("Process is still running")
}
```

### Process Monitoring

```cursed
// Get memory usage
sus memory_usage thicc = process.get_process_memory_usage(pid)
vibez.spill("Memory usage: " + tea(memory_usage) + " bytes")

// Get CPU usage
sus cpu_usage meal = process.get_process_cpu_usage(pid)
vibez.spill("CPU usage: " + tea(cpu_usage) + "%")
```

### Signal Handling

```cursed
// Register signal handler
sus handler_registered lit = process.register_signal_handler(15, slay() {
    vibez.spill("Received SIGTERM - shutting down gracefully")
    process.set_exit_code(0)
})

// Send signal to process
sus signal_sent lit = process.send_signal(child_pid, 15)  // SIGTERM
```

### Command Line Arguments

```cursed
// Get command line arguments
sus args []tea = process.get_command_line_args()
vibez.spill("Program name: " + args[0])
bestie i := 1; i < len(args); i++ {
    vibez.spill("Argument " + tea(i) + ": " + args[i])
}
```

## API Reference

### Process Information
- `get_process_id() -> normie` - Get current process ID
- `get_parent_process_id() -> normie` - Get parent process ID
- `get_user_id() -> normie` - Get current user ID
- `get_group_id() -> normie` - Get current group ID
- `get_hostname() -> tea` - Get system hostname
- `get_system_info() -> ProcessInfo` - Get comprehensive system information

### Environment Variables
- `get_environment_variable(key: tea) -> tea` - Get environment variable value
- `set_environment_variable(key: tea, value: tea) -> lit` - Set environment variable

### Directory Operations
- `get_current_directory() -> tea` - Get current working directory
- `change_directory(path: tea) -> lit` - Change current directory

### Command Execution
- `execute_command(command: tea) -> tea` - Execute command and return output
- `execute_with_args(command: tea, args: []tea) -> tea` - Execute command with arguments
- `get_exit_code() -> normie` - Get exit code of last command
- `set_exit_code(code: normie)` - Set exit code for current process

### Process Management
- `spawn_process(command: tea, args: []tea) -> normie` - Spawn new process
- `kill_process(pid: normie) -> lit` - Terminate process
- `wait_for_process(pid: normie) -> normie` - Wait for process completion
- `is_process_running(pid: normie) -> lit` - Check if process is running

### Process Monitoring
- `get_process_memory_usage(pid: normie) -> thicc` - Get memory usage in bytes
- `get_process_cpu_usage(pid: normie) -> meal` - Get CPU usage percentage

### Signal Handling
- `send_signal(pid: normie, signal: normie) -> lit` - Send signal to process
- `register_signal_handler(signal: normie, handler: slay()) -> lit` - Register signal handler

### Command Line Arguments
- `get_command_line_args() -> []tea` - Get command line arguments

## Types

```cursed
be_like ProcessInfo squad {
    hostname tea        // System hostname
    pid normie         // Process ID
    ppid normie        // Parent process ID
    uid normie         // User ID
    gid normie         // Group ID
    cwd tea           // Current working directory
}
```

## Common Signal Numbers

- `1` - SIGHUP (Hangup)
- `2` - SIGINT (Interrupt, Ctrl+C)
- `3` - SIGQUIT (Quit)
- `9` - SIGKILL (Kill, non-catchable)
- `15` - SIGTERM (Terminate)
- `30` - SIGUSR1 (User-defined signal 1)
- `31` - SIGUSR2 (User-defined signal 2)

## Error Handling

All functions handle errors gracefully:
- Command execution returns empty string on error
- Process operations return -1 or false on error
- Signal operations return false on error
- Environment variable operations return empty string on error

## Testing

Run the comprehensive test suite:

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/process/test_process.csd

# Test compilation mode
cargo run --bin cursed -- compile stdlib/process/test_process.csd
./test_process
```

## Implementation Notes

- Uses pure CURSED language features where possible
- Minimal FFI dependencies through runtime bridge
- Cross-platform compatible system calls
- Memory-safe string handling
- Proper error propagation
- Production-ready for self-hosting

## Self-Hosting Ready

This module is essential for self-hosting and provides all process management operations needed for:

- Build system integration
- Compiler process spawning
- Environment variable management
- Command execution for external tools
- Signal handling for graceful shutdown
- Process monitoring and resource management
