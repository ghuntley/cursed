# Enhanced Process Management and IPC System

## Overview

The CURSED programming language provides a comprehensive enhanced process management and Inter-Process Communication (IPC) system that builds upon the foundation of Go's `os/exec` package while adding significant enhancements for modern process orchestration, real-time monitoring, and advanced IPC capabilities.

## Architecture

The enhanced process management system consists of two main components:

### 1. ExecSlay - Enhanced Process Execution
**Location**: `src/stdlib/process/exec_slay.rs`

ExecSlay provides the core process execution functionality with enhanced features:

- **SlayCommand**: Enhanced command execution with fluent configuration
- **SlayCommandBuilder**: Fluent API for complex command construction  
- **SlayPipeline**: Process chaining and pipeline execution
- **SlayTask**: Background task management with monitoring
- **SlayProcess**: Advanced process control and monitoring
- **ProcessStats**: Real-time resource monitoring and statistics

### 2. ExecVibez Enhanced - Advanced Process Orchestration
**Location**: `src/stdlib/process/exec_vibez_enhanced.rs`

ExecVibez Enhanced provides advanced process orchestration capabilities:

- **ProcessGroup**: Multi-process coordination and management
- **OutputStreamer**: Real-time line-by-line output processing
- **InputGenerator**: Programmatic input generation with timing
- **EnhancedEnvironment**: Advanced environment variable management
- **ProcessContext**: Timeout and cancellation management
- **Cross-platform LookPath**: Enhanced executable discovery

## Key Features

### Core Process Management

#### SlayCommand Enhanced Features
```cursed
// Basic command execution
sus mut cmd = exec_slay.NewSlayCommand("ls", ["-la", "/tmp"]);
sus output = cmd.Output()?;

// Command with timeout
sus result = cmd.RunWithTimeout(time.Duration.from_seconds(30))?;

// Capture combined output
sus combined = cmd.CombinedOutput()?;
```

#### SlayCommandBuilder Fluent API
```cursed
// Fluent command construction
sus cmd = exec_slay.NewSlayCommandBuilder("find")
    .WithArgs(["/", "-name", "*.txt", "-type", "f"])
    .WithDir("/tmp")
    .AddEnv("LANG", "C")
    .WithTimeout(time.Duration.from_seconds(60))
    .UseShell(based)
    .Build();
```

#### SlayPipeline Process Chaining
```cursed
// Create process pipeline
sus cat_cmd = exec_slay.NewSlayCommand("cat", ["input.txt"]);
sus grep_cmd = exec_slay.NewSlayCommand("grep", ["pattern"]);
sus wc_cmd = exec_slay.NewSlayCommand("wc", ["-l"]);

sus pipeline = exec_slay.NewSlayPipeline([cat_cmd, grep_cmd, wc_cmd]);
sus result = pipeline.Output()?;
```

### Advanced Process Orchestration

#### ProcessGroup Coordination
```cursed
// Create and configure process group
sus mut group = exec_vibez_enhanced.new_process_group();

group.add_command(exec_vibez_enhanced.command("echo", ["Task 1"]));
group.add_command(exec_vibez_enhanced.command("echo", ["Task 2"]));
group.add_command(exec_vibez_enhanced.command("sleep", ["1"]));

sus options = exec_vibez_enhanced.ProcessGroupOptions {
    start_all: based,
    wait_all: based,
    continue_on_failure: based,
    group_timeout: time.Duration.from_seconds(30),
    max_concurrent: 5,
    kill_tree_on_failure: nah,
};
group.options(options);

group.start_all()?;
group.wait_all()?;
```

#### OutputStreamer Real-time Processing
```cursed
// Real-time output streaming
sus cmd = exec_vibez_enhanced.command("tail", ["-f", "logfile.txt"]);
sus mut streamer = exec_vibez_enhanced.new_output_streamer(cmd);

streamer.on_stdout_line(slay(line: tea) {
    vibez.spill("📝 Log: ");
    vibez.spill(line);
    vibez.spill("\n");
});

streamer.capture_output(based);
streamer.start()?;
```

#### InputGenerator Programmatic Input
```cursed
// Programmatic input generation
sus cmd = exec_vibez_enhanced.command("interactive_program", []);
sus mut generator = exec_vibez_enhanced.new_input_generator(cmd);

generator.write_line("command1")?;
generator.write_line_after("delayed_command", time.Duration.from_seconds(2))?;
generator.set_auto_close(based);

generator.start()?;
generator.wait()?;
```

### Enhanced Environment Management

#### Advanced Environment Operations
```cursed
// Enhanced environment management
sus mut env = exec_vibez_enhanced.new_environment();

env.set("APP_ENV", "production")
   .append("PATH", ":/opt/app/bin")
   .prepend("LD_LIBRARY_PATH", "/opt/app/lib")
   .remove("DEBUG_MODE");

sus mut cmd = exec_vibez_enhanced.command("app", ["--config", "prod.conf"]);
cmd.env(env);
```

### Process Monitoring and Statistics

#### Real-time Process Monitoring
```cursed
// Process monitoring with statistics
sus mut cmd = exec_slay.NewSlayCommand("long_running_task", []);
cmd.Start()?;

sus process = cmd.Process()?;

// Get process statistics
sus stats = process.Stats()?;
vibez.spill("CPU: ");
vibez.spill(stats.cpu.to_string());
vibez.spill("%, Memory: ");
vibez.spill((stats.memory / 1024 / 1024).to_string());
vibez.spill(" MB\n");

// Monitor with callback
process.Monitor(time.Duration.from_seconds(1), slay(stats: exec_slay.ProcessStats) {
    vibez.spill("Monitor: CPU ");
    vibez.spill(stats.cpu.to_string());
    vibez.spill("%, Memory ");
    vibez.spill((stats.memory / 1024).to_string());
    vibez.spill(" KB\n");
})?;
```

### Background Task Management

#### SlayTask Background Execution
```cursed
// Background task execution and monitoring
sus cmd = exec_slay.NewSlayCommand("background_job", ["--input", "data.txt"]);
sus mut task = exec_slay.RunBackground(cmd);

// Monitor task status
lowkey (task.IsRunning()) {
    sus elapsed = task.ElapsedTime();
    vibez.spill("Task running for ");
    vibez.spill(elapsed.as_seconds().to_string());
    vibez.spill(" seconds\n");
    time.sleep(time.Duration.from_seconds(1));
}

// Wait for completion
task.Wait()?;
sus output = task.GetOutput()?;
```

### Signal Handling and Process Control

#### Advanced Signal Management
```cursed
// Advanced signal handling
sus process = cmd.Process()?;

sus signal_opts = exec_slay.SignalOptions {
    grace_period: time.Duration.from_seconds(5),
    force: based,
    signal: 15, // SIGTERM
    recursive: based,
};

// Graceful termination with fallback
process.Terminate(signal_opts)?;

// Kill process tree if needed
process.KillTree()?;
```

### Cross-Platform Features

#### Enhanced Executable Discovery
```cursed
// Cross-platform executable lookup
sus shell_path = exec_vibez_enhanced.look_path("bash")?;
vibez.spill("Found bash at: ");
vibez.spill(shell_path);
vibez.spill("\n");

// Platform-specific handling
#[cfg(unix)]
{
    sus unix_cmd = exec_slay.NewSlayCommand("uname", ["-a"]);
}

#[cfg(windows)]
{
    sus win_cmd = exec_slay.NewSlayCommand("systeminfo", []);
}
```

### Shell Command Shortcuts

#### Convenient Shell Operations
```cursed
// Direct shell command execution
exec_slay.RunShell("echo 'Hello from shell' > output.txt")?;

// Shell command with output capture
sus output = exec_slay.ShellOutput("ls -la | grep .txt")?;

// Shell with environment variables
sus env = HashMap.new();
env.insert("VAR1", "value1");
exec_slay.RunShellWithEnv("echo $VAR1", env)?;

// Shell in specific directory
exec_slay.RunShellInDir("pwd", "/tmp")?;
```

### Timeout and Context Management

#### ProcessContext for Cancellation
```cursed
// Context-based timeout and cancellation
sus ctx = exec_vibez_enhanced.new_context_with_timeout(time.Duration.from_seconds(30));
sus mut cmd = exec_vibez_enhanced.command_context(ctx, "long_task", []);

// Cancel context from another thread
ctx.cancel();

// Command will be cancelled when context is cancelled
sus result = cmd.run(); // Will return cancellation error
```

## Error Handling and Recovery

### Comprehensive Error Types
The system provides detailed error information for better debugging and recovery:

```cursed
// Error handling with detailed information
sus result = cmd.run();
lowkey (result == cap) {
    sus error = result.unwrap_err();
    bestie (error.category()) {
        exec_vibez_enhanced.ErrorCategory.NotFound => {
            vibez.spill("Command not found: ");
            vibez.spill(error.message());
        },
        exec_vibez_enhanced.ErrorCategory.Timeout => {
            vibez.spill("Command timed out after ");
            vibez.spill(error.timeout().unwrap().as_seconds().to_string());
            vibez.spill(" seconds");
        },
        exec_vibez_enhanced.ErrorCategory.PermissionDenied => {
            vibez.spill("Permission denied: ");
            vibez.spill(error.message());
        },
        _ => {
            vibez.spill("Other error: ");
            vibez.spill(error.message());
        }
    }
}
```

### Error Recovery Strategies
```cursed
// Process group with failure recovery
sus options = exec_vibez_enhanced.ProcessGroupOptions {
    continue_on_failure: based,  // Continue despite individual failures
    kill_tree_on_failure: nah,   // Don't kill other processes on failure
    group_timeout: time.Duration.from_seconds(60),
};

// Retry mechanism for failed commands
sus max_retries = 3;
lowkey (sus attempt = 0; attempt < max_retries; attempt++) {
    sus result = cmd.run();
    lowkey (result != cap) {
        break; // Success
    }
    vibez.spill("Attempt ");
    vibez.spill((attempt + 1).to_string());
    vibez.spill(" failed, retrying...\n");
    time.sleep(time.Duration.from_seconds(1));
}
```

## Performance Characteristics

### Optimization Features
- **Concurrent Execution**: Process groups support concurrent execution with configurable limits
- **Memory Efficiency**: Configurable buffer sizes and streaming to minimize memory usage
- **Resource Monitoring**: Real-time statistics collection with minimal overhead
- **Efficient I/O**: Asynchronous I/O handling for better performance

### Scalability
- **Process Groups**: Support for managing hundreds of concurrent processes
- **Background Tasks**: Efficient background task coordination
- **Resource Limits**: Configurable memory and CPU limits per process
- **Connection Pooling**: Efficient resource management for long-running operations

## Integration with IPC

### IPC Integration Points
The enhanced process management system integrates seamlessly with the IPC infrastructure:

1. **Named Pipes**: Process output can be redirected to named pipes for IPC
2. **Shared Memory**: Processes can share memory regions for high-performance data exchange
3. **Message Queues**: Process coordination through message queues
4. **Signals**: Advanced signal handling for process communication

### Real-World Integration Example
```cursed
// Complete log processing pipeline with IPC
slay process_logs() -> tea {
    // 1. Create shared memory for data exchange
    sus shared_mem = ipc.create_shared_memory("log_buffer", 1024 * 1024)?;
    
    // 2. Set up process group for parallel analysis
    sus mut log_group = exec_vibez_enhanced.new_process_group();
    
    // Add log processing commands
    log_group.add_command(exec_vibez_enhanced.command("log_parser", ["--input", "stdin"]));
    log_group.add_command(exec_vibez_enhanced.command("log_analyzer", ["--shared-mem", "log_buffer"]));
    log_group.add_command(exec_vibez_enhanced.command("log_reporter", ["--output", "report.json"]));
    
    // 3. Configure real-time streaming
    sus log_cmd = exec_vibez_enhanced.command("tail", ["-f", "/var/log/app.log"]);
    sus mut streamer = exec_vibez_enhanced.new_output_streamer(log_cmd);
    
    streamer.on_stdout_line(slay(line: tea) {
        // Write to shared memory for processing
        shared_mem.write(line.as_bytes())?;
    });
    
    // 4. Start coordinated processing
    log_group.start_all()?;
    streamer.start()?;
    
    // 5. Monitor and manage the pipeline
    // ... monitoring logic ...
    
    yolo cap;
}
```

## Testing and Validation

### Comprehensive Test Suite
The enhanced process management system includes extensive testing:

- **Unit Tests**: Individual component testing
- **Integration Tests**: End-to-end workflow testing
- **Performance Tests**: Scalability and performance validation
- **Error Handling Tests**: Comprehensive error scenario coverage
- **Cross-Platform Tests**: Platform-specific functionality validation

### Running Tests
```bash
# Quick validation
make enhanced-process-test-quick

# Complete test suite
make enhanced-process-test

# All tests including stress tests
make enhanced-process-test-all

# Performance tests
make enhanced-process-test-performance

# Coverage report
make enhanced-process-test-coverage
```

### Demo and Examples
```bash
# Run the comprehensive demo
make enhanced-process-demo

# Build example programs
make enhanced-process-build-examples
```

## Best Practices

### 1. Resource Management
- Always set appropriate timeouts for long-running processes
- Use process groups to manage related processes together
- Monitor resource usage to prevent system overload
- Clean up processes and resources properly

### 2. Error Handling
- Always handle errors appropriately for your use case
- Use context cancellation for graceful shutdown
- Implement retry mechanisms for transient failures
- Log detailed error information for debugging

### 3. Security
- Validate all input parameters before execution
- Use appropriate environment isolation
- Limit process privileges where possible
- Sanitize shell commands to prevent injection attacks

### 4. Performance
- Use streaming for large data processing
- Configure appropriate buffer sizes
- Leverage concurrent execution where beneficial
- Monitor and optimize resource usage

## Future Enhancements

### Planned Features
1. **Container Integration**: Direct container orchestration support
2. **Service Discovery**: Automatic service discovery and health checking
3. **Load Balancing**: Built-in load balancing for process groups
4. **Metrics Collection**: Enhanced metrics and observability
5. **Plugin System**: Extensible plugin architecture for custom processing

### Extension Points
The system is designed to be extensible:
- Custom output processors
- Custom input generators
- Custom environment providers
- Custom monitoring backends
- Custom error handlers

This enhanced process management system provides a solid foundation for building robust, scalable, and maintainable process orchestration solutions in the CURSED programming language.
