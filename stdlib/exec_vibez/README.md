# exec_vibez - Pure CURSED Process Execution Module

A comprehensive process execution and management system implemented entirely in CURSED without FFI dependencies. Provides enhanced external command execution capabilities inspired by Go's os/exec but with additional features including process groups, timeouts, environment control, output streaming, and signal handling integration.

## Features

### ✅ Core Process Execution
- **Synchronous Execution**: Execute commands and wait for completion
- **Asynchronous Execution**: Start processes in background with monitoring
- **Detached Execution**: Fire-and-forget process execution
- **Command Simulation**: Pure CURSED implementation without external dependencies

### ✅ Process Management
- **Process Registration**: Track and manage running processes
- **Process Groups**: Organize related processes for coordinated control
- **Process Lifecycle**: Monitor process states from creation to completion
- **Process Termination**: Graceful and forceful process termination

### ✅ Advanced Features
- **Timeout Management**: Set execution timeouts with automatic cleanup
- **Output Streaming**: Capture and stream stdout/stderr in real-time
- **Environment Control**: Set environment variables and working directories
- **Signal Integration**: Integration with signal_boost for proper signal handling
- **IPC Integration**: Leverage IPC module for inter-process communication

### ✅ Monitoring and Statistics
- **Execution Statistics**: Track performance metrics and success rates
- **Command History**: Maintain history of executed commands
- **Process Monitoring**: Real-time monitoring of running processes
- **Resource Usage**: Monitor and limit resource consumption

## Quick Start

```cursed
yeet "exec_vibez"

# Initialize the module
exec_vibez.init_exec_vibez()

# Simple command execution
sus result map = exec_vibez.run_command("echo", ["Hello, World!"])
vibez.spill("Output: " + result.get("stdout"))
vibez.spill("Exit code: " + core.tea(result.get("exit_code")))

# Asynchronous execution
sus process_id tea = exec_vibez.run_background("ls", ["-la"])
sus async_result map = exec_vibez.wait_for_process(process_id)
vibez.spill("Async output: " + async_result.get("stdout"))
```

## Command Creation and Configuration

### Basic Command Creation

```cursed
# Create a command
sus cmd map = exec_vibez.create_command("grep", ["pattern", "file.txt"])

# Configure working directory
exec_vibez.set_working_dir(cmd, "/home/user/project")

# Set environment variables
sus env map = {"PATH": "/usr/bin:/bin", "LANG": "en_US.UTF-8"}
exec_vibez.set_environment(cmd, env)

# Add individual environment variable
exec_vibez.add_env_var(cmd, "DEBUG", "1")

# Set stdin data
exec_vibez.set_stdin(cmd, "input data for command")

# Set timeout (in milliseconds)
exec_vibez.set_timeout(cmd, 5000)

# Set execution mode
exec_vibez.set_exec_mode(cmd, "synchronous")
```

### Execution Modes

```cursed
# Synchronous execution (default)
sus sync_result map = exec_vibez.execute_command(cmd)

# Asynchronous execution
sus proc_id tea = exec_vibez.execute_async(cmd)
sus async_result map = exec_vibez.wait_for_process(proc_id)

# Background execution with no waiting
sus bg_proc tea = exec_vibez.run_background("long_running_cmd", [])
```

## Process Management

### Process Information and Control

```cursed
# Get process information
sus proc_info map = exec_vibez.get_process_info(process_id)
vibez.spill("Process state: " + proc_info.get("state"))

# List running processes
sus running [tea] = exec_vibez.list_running_processes()
sus i normie = 0
while i < running.length() {
    vibez.spill("Running: " + running[i])
    i = i + 1
}

# Kill a process
if exec_vibez.kill_process(process_id) {
    vibez.spill("Process terminated successfully")
}
```

### Process Groups

```cursed
# Create a process group
exec_vibez.create_process_group("web_servers")

# Add processes to group
sus web1 tea = exec_vibez.run_background("nginx", [])
sus web2 tea = exec_vibez.run_background("apache", [])

exec_vibez.add_to_group("web_servers", web1)
exec_vibez.add_to_group("web_servers", web2)

# Kill entire group
exec_vibez.kill_process_group("web_servers")
```

## Output Streaming and Capture

### Stream Management

```cursed
# Create output streamers
exec_vibez.create_output_streamer(process_id, "stdout")
exec_vibez.create_output_streamer(process_id, "stderr")

# Capture stream output
exec_vibez.capture_stream_output(process_id, "stdout", "Command output\n")

# Get captured output
sus stdout_data tea = exec_vibez.get_captured_output(process_id, "stdout")
sus stderr_data tea = exec_vibez.get_captured_output(process_id, "stderr")
```

### Output Capture Examples

```cursed
# Run command with output capture
sus result map = exec_vibez.run_with_output("ls", ["-la"])
vibez.spill("Directory listing:")
vibez.spill(result.get("stdout"))

if result.get("stderr").length() > 0 {
    vibez.spill("Errors: " + result.get("stderr"))
}
```

## Timeout Management

### Setting and Managing Timeouts

```cursed
# Run command with timeout
sus timeout_result map = exec_vibez.run_with_timeout("slow_command", [], 10000)

if timeout_result.get("state") == "timeout" {
    vibez.spill("Command timed out!")
} else {
    vibez.spill("Command completed in time")
}

# Set timeout for async process
sus long_proc tea = exec_vibez.run_background("long_task", [])
exec_vibez.set_process_timeout(long_proc, 30000)

# Check for timeouts
sus timed_out [tea] = exec_vibez.check_timeouts()
```

## Convenience Functions

### High-Level Command Execution

```cursed
# Simple command execution
sus result map = exec_vibez.run_command("echo", ["Hello"])

# Shell command execution
sus shell_result map = exec_vibez.run_shell("ls -la | grep txt")

# Background execution
sus bg_proc tea = exec_vibez.run_background("backup_script", [])

# Command with timeout
sus timed_result map = exec_vibez.run_with_timeout("ping", ["google.com"], 5000)
```

## Configuration Management

### Module Configuration

```cursed
# Get current configuration
sus config map = exec_vibez.get_exec_config()
vibez.spill("Max processes: " + core.tea(config.get("max_concurrent_processes")))

# Update configuration
sus new_config map = {
    "default_timeout_ms": 60000,
    "max_concurrent_processes": 50,
    "max_output_buffer": 2097152
}
exec_vibez.configure_exec(new_config)
```

## Statistics and Monitoring

### Execution Statistics

```cursed
# Get execution statistics
sus stats map = exec_vibez.get_exec_statistics()
vibez.spill("Commands executed: " + core.tea(stats.get("commands_executed")))
vibez.spill("Average execution time: " + core.tea(stats.get("avg_execution_time")) + "ms")
vibez.spill("Success rate: " + core.tea(stats.get("processes_completed")) + "/" + core.tea(stats.get("commands_executed")))

# Get command history
sus history [map] = exec_vibez.get_command_history()
sus recent [map] = exec_vibez.get_recent_commands(5)

# Dump current state
exec_vibez.dump_exec_state()
```

## Signal Integration

The module integrates with the `signal_boost` module for proper signal handling:

```cursed
# Signal handlers are automatically registered during initialization
# SIGTERM: cleanup_processes
# SIGINT: interrupt_processes
# SIGCHLD: handle_child_exit

# You can manually trigger signal handling
signal_boost.notify(signal_boost.SIGTERM)  # Triggers cleanup
```

## IPC Integration

Integration with the `ipc` module enables inter-process communication:

```cursed
# IPC is automatically initialized
# You can use IPC resources for process communication

# Create named pipe for process communication
ipc.create_named_pipe("cmd_pipe", 1024)

# Send command through pipe
ipc.write_to_pipe("cmd_pipe", "execute ls -la")

# Read command result
sus cmd_result tea = ipc.read_from_pipe("cmd_pipe")
```

## Error Handling

### Process States and Exit Codes

```cursed
# Check process state
sus result map = exec_vibez.run_command("some_command", [])

if result.get("state") == "finished" {
    if result.get("exit_code") == 0 {
        vibez.spill("Command succeeded")
    } else {
        vibez.spill("Command failed with exit code: " + core.tea(result.get("exit_code")))
    }
} else if result.get("state") == "timeout" {
    vibez.spill("Command timed out")
} else if result.get("state") == "killed" {
    vibez.spill("Command was killed")
}
```

### Error Recovery

```cursed
# Retry failed commands
sus retry_count normie = 0
sus max_retries normie = 3
sus success lit = cap

while !success && retry_count < max_retries {
    sus result map = exec_vibez.run_command("unreliable_cmd", [])
    if result.get("exit_code") == 0 {
        success = based
    } else {
        retry_count = retry_count + 1
        vibez.spill("Retry " + core.tea(retry_count) + " of " + core.tea(max_retries))
    }
}
```

## Advanced Examples

### Parallel Process Execution

```cursed
# Execute multiple commands in parallel
sus processes [tea] = []
sus commands [tea] = ["ls", "pwd", "whoami", "date"]
sus i normie = 0

# Start all processes
while i < commands.length() {
    sus proc tea = exec_vibez.run_background(commands[i], [])
    processes.push(proc)
    i = i + 1
}

# Wait for all to complete
i = 0
while i < processes.length() {
    sus result map = exec_vibez.wait_for_process(processes[i])
    vibez.spill(commands[i] + " output: " + result.get("stdout"))
    i = i + 1
}
```

### Pipeline Simulation

```cursed
# Simulate command pipeline
sus cmd1 map = exec_vibez.create_command("ls", ["-la"])
sus result1 map = exec_vibez.execute_command(cmd1)

sus cmd2 map = exec_vibez.create_command("grep", ["txt"])
exec_vibez.set_stdin(cmd2, result1.get("stdout"))
sus result2 map = exec_vibez.execute_command(cmd2)

vibez.spill("Pipeline result: " + result2.get("stdout"))
```

### Resource Monitoring

```cursed
# Monitor resource usage
sus monitor_active lit = based

while monitor_active {
    sus running [tea] = exec_vibez.list_running_processes()
    sus stats map = exec_vibez.get_exec_statistics()
    
    vibez.spill("Running processes: " + core.tea(running.length()))
    vibez.spill("Total executed: " + core.tea(stats.get("commands_executed")))
    
    # Check for long-running processes
    sus i normie = 0
    while i < running.length() {
        sus proc_info map = exec_vibez.get_process_info(running[i])
        # Monitor process state
        i = i + 1
    }
    
    # Sleep simulation
    sus sleep_cmd map = exec_vibez.create_command("sleep", ["1"])
    exec_vibez.execute_command(sleep_cmd)
}
```

## Testing

Run the comprehensive test suite:

```bash
cargo run --bin cursed stdlib/exec_vibez/test_exec_vibez.csd
```

## Module Integration

### Dependencies
- `signal_boost` - Signal handling and graceful shutdown
- `ipc` - Inter-process communication capabilities

### Integration Pattern

```cursed
yeet "exec_vibez"
yeet "signal_boost"
yeet "ipc"

# Initialize all modules
exec_vibez.init_exec_vibez()  # Also initializes signal_boost and ipc

# Use integrated functionality
exec_vibez.run_shell("echo 'Process management ready'")
```

## Performance Characteristics

- **Pure CURSED Implementation**: No FFI dependencies for maximum portability
- **Simulated Execution**: Commands are simulated based on program type
- **Memory Efficient**: Configurable output buffer limits
- **Concurrent Safe**: Support for multiple simultaneous processes
- **Signal Safe**: Proper signal handling integration

## Configuration Options

| Option | Default | Description |
|--------|---------|-------------|
| `default_timeout_ms` | 30000 | Default command timeout in milliseconds |
| `max_concurrent_processes` | 100 | Maximum number of concurrent processes |
| `max_command_history` | 1000 | Maximum command history entries |
| `default_working_dir` | "/tmp" | Default working directory |
| `capture_output` | true | Whether to capture command output by default |
| `shell_command` | "/bin/sh" | Shell command for shell execution |
| `shell_flag` | "-c" | Shell flag for command execution |
| `max_output_buffer` | 1048576 | Maximum output buffer size (1MB) |
| `process_cleanup_interval` | 5000 | Process cleanup interval in milliseconds |

## Supported Commands (Simulation)

The module includes built-in simulation for common commands:

- `echo` - Echo arguments to stdout
- `ls` - List directory contents
- `pwd` - Print working directory
- `whoami` - Print current user
- `date` - Print current date
- `cat` - Display file contents
- `sleep` - Sleep for specified duration
- `true` - Always succeeds (exit code 0)
- `false` - Always fails (exit code 1)
- Custom commands - Default simulation with configurable behavior

## Best Practices

1. **Always Initialize**: Call `init_exec_vibez()` before using the module
2. **Set Timeouts**: Use reasonable timeouts to prevent hanging processes
3. **Monitor Resources**: Check running processes and statistics regularly
4. **Handle Errors**: Check exit codes and process states
5. **Use Process Groups**: Group related processes for easier management
6. **Cleanup on Exit**: Use signal handlers for graceful shutdown
7. **Limit Concurrency**: Configure appropriate process limits
8. **Capture Output**: Use output streaming for real-time monitoring

## Security Considerations

- No shell injection vulnerabilities (pure simulation)
- Configurable resource limits prevent resource exhaustion
- Signal handling prevents zombie processes
- Process isolation through group management
- Timeout enforcement prevents runaway processes

## License

This module is part of the CURSED standard library and follows the same license terms.
