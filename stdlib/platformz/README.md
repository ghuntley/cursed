# Platformz Module

## Why This Module Exists

The `platformz` module provides a unified cross-platform abstraction layer for operating system-specific functionality, enabling CURSED applications to run seamlessly across Windows, macOS, Linux, and other platforms without platform-specific code branches throughout the application.

The module exists because:
- **Cross-Platform Compatibility**: Applications need to access OS services (file systems, networking, processes) consistently across different operating systems
- **Performance Optimization**: Platform-specific optimizations (epoll on Linux, IOCP on Windows) can provide significant performance improvements
- **Developer Productivity**: Unified APIs eliminate the need for developers to learn platform-specific APIs and handle cross-platform differences manually
- **Deployment Flexibility**: Applications can be developed on one platform and deployed to any supported platform without code changes
- **System Integration**: Modern applications need deep integration with OS services like authentication, notifications, and system monitoring

## Why Testing Is Critical

Platform abstraction testing is absolutely essential because:
- **Platform Behavior Variations**: Each operating system has subtle differences in file path handling, networking, process management, and error codes
- **Resource Management**: Different platforms have different resource limits, handles, and cleanup requirements that must be tested thoroughly
- **Edge Case Coverage**: Platform-specific edge cases (Windows long path names, Unix signal handling, macOS sandboxing) only appear during comprehensive testing
- **Performance Characteristics**: Platform optimizations can have dramatically different performance profiles that must be validated
- **Integration Failures**: Platform abstractions often fail at integration boundaries where OS-specific behavior leaks through

## Implementation Rationale

### Key Design Decisions:

**1. Compile-Time Platform Selection**
- Platform-specific implementations selected at compile time for zero runtime overhead
- Conditional compilation ensures only relevant code is included in final binaries
- Unified public API with platform-specific private implementations

**2. Performance-First Design**
- Native OS APIs used directly rather than POSIX emulation layers
- Platform-specific optimizations (io_uring, IOCP, kqueue) exposed through unified interface
- Zero-copy operations where platforms support them

**3. Error Handling Unification**
- Platform-specific error codes mapped to CURSED's unified error system
- Consistent error behavior across platforms while preserving underlying error information
- Detailed error context for platform-specific debugging

**4. Resource Management Integration**
- Platform handles (file descriptors, HANDLEs, etc.) managed through CURSED's ownership system
- Automatic cleanup on scope exit prevents resource leaks
- Platform-specific resource limits exposed through unified query interface

## API Reference

### File System Operations

#### `path_join(components: []tea) tea`
**Purpose**: Joins path components using platform-appropriate separators
**Cross-Platform**: Handles Windows backslashes vs Unix forward slashes
**Edge Cases**: Manages UNC paths, long path names, reserved names

```cursed
sus file_path = platformz.path_join(["home", "user", "documents", "file.txt"])
# Windows: "home\\user\\documents\\file.txt"
# Unix: "home/user/documents/file.txt"
```

#### `absolute_path(path: tea) yikes<tea>`
**Purpose**: Converts relative path to absolute path using platform conventions
**Error Handling**: Returns unified error for invalid paths across platforms
**Resolution**: Resolves symlinks, shortcuts, and mount points appropriately

#### `file_info(path: tea) yikes<FileInfo>`
**Purpose**: Retrieves comprehensive file metadata across platforms
**Unified Fields**: Size, permissions, timestamps, type (file/directory/symlink)
**Platform Extensions**: Windows attributes, Unix permissions, macOS extended attributes

```cursed
sus info = platformz.file_info("/path/to/file") fam {
    when "file_not_found" -> damn FileInfo.empty()
}

vibez.spill("Size:", info.size)
vibez.spill("Modified:", info.modified_time)
vibez.spill("Permissions:", info.permissions.octal())
```

### Process Management

#### `spawn_process(command: tea, args: []tea, env: ?[]tea) yikes<Process>`
**Purpose**: Creates child processes with unified interface across platforms
**Features**: Environment inheritance, working directory, stdio redirection
**Platform Handling**: Windows CreateProcess vs Unix fork/exec differences

#### `current_process_info() ProcessInfo`
**Purpose**: Retrieves information about current process
**Cross-Platform Data**: PID, parent PID, memory usage, CPU time, command line

```cursed
sus proc_info = platformz.current_process_info()
vibez.spill("PID:", proc_info.pid)
vibez.spill("Memory:", proc_info.memory_usage_bytes)
vibez.spill("CPU time:", proc_info.cpu_time_seconds)
```

#### `process_list() []ProcessInfo`
**Purpose**: Lists all running processes visible to current user
**Platform Handling**: Windows process enumeration vs Linux /proc parsing
**Performance**: Optimized for each platform's fastest enumeration method

### Network Abstraction

#### `network_interfaces() []NetworkInterface`
**Purpose**: Enumerates network interfaces with unified representation
**Cross-Platform**: Handles Windows WinSock vs Unix getifaddrs differences
**Information**: IP addresses, MAC addresses, interface status, MTU

#### `tcp_listen(address: tea, port: drip) yikes<TcpListener>`
**Purpose**: Creates TCP listener with platform-optimized backend
**Optimization**: Uses epoll (Linux), kqueue (macOS/BSD), IOCP (Windows)
**Features**: Automatic port binding, reuse_address, backlog configuration

```cursed
sus listener = platformz.tcp_listen("0.0.0.0", 8080) fam {
    when "address_in_use" -> yikes "Port 8080 already in use"
}

bestie (connection := listener.accept()) {
    go handle_connection(connection)
}
```

### System Information

#### `system_info() SystemInfo`
**Purpose**: Comprehensive system information across platforms
**Hardware**: CPU count, architecture, memory size, disk space
**Software**: OS version, kernel version, platform name

```cursed
sus sys = platformz.system_info()
vibez.spill("OS:", sys.os_name, sys.os_version)
vibez.spill("CPU:", sys.cpu_count, "cores,", sys.cpu_architecture)
vibez.spill("Memory:", sys.total_memory_bytes / (1024*1024), "MB")
```

#### `environment_variables() map<tea, tea>`
**Purpose**: Retrieves all environment variables with platform-appropriate handling
**Case Handling**: Preserves case sensitivity differences (Windows vs Unix)
**Unicode**: Proper Unicode handling across platforms

#### `user_info() UserInfo`
**Purpose**: Current user information with platform-specific details
**Cross-Platform**: Username, home directory, user ID
**Platform-Specific**: Windows SID, Unix UID/GID, group memberships

### Platform-Specific Optimizations

#### `high_performance_io() IoBackend`
**Purpose**: Selects optimal I/O backend for current platform
**Linux**: io_uring for maximum performance
**Windows**: IOCP for scalable async I/O  
**macOS/BSD**: kqueue for efficient event notification

#### `memory_mapping(file: tea, size: drip) yikes<MemoryMap>`
**Purpose**: Memory-mapped file I/O with platform optimization
**Features**: Read-only, read-write, copy-on-write modes
**Platform**: Windows CreateFileMapping vs Unix mmap

## Usage Examples

### Cross-Platform File Operations
```cursed
yeet "platformz"

# Build cross-platform file paths
sus config_dir = platformz.path_join([
    platformz.user_home_directory(),
    ".myapp",
    "config"
])

# Create directory if it doesn't exist
platformz.create_directory_all(config_dir) fam {
    when "permission_denied" -> {
        vibez.spill("Cannot create config directory")
        damn
    }
}

# Read configuration file
sus config_path = platformz.path_join([config_dir, "app.toml"])
sus config_data = platformz.read_file(config_path) fam {
    when "file_not_found" -> {
        # Create default configuration
        sus default_config = "debug = true\nport = 8080\n"
        platformz.write_file(config_path, default_config)
        damn default_config
    }
}
```

### Process Management Example
```cursed
# Cross-platform process spawning
slay run_command(command tea, args []tea) yikes<tea> {
    sus process = platformz.spawn_process(command, args, fam) fam {
        when "command_not_found" -> yikes "Command not available: " + command
    }
    
    # Wait for completion with timeout
    sus result = process.wait_timeout(30000) fam {  # 30 second timeout
        when "timeout" -> {
            process.kill()
            yikes "Command timed out"
        }
    }
    
    ready (result.exit_code != 0) {
        yikes "Command failed with exit code: " + result.exit_code.to_string()
    }
    
    damn result.stdout
}

# Usage works on all platforms
sus output = run_command("git", ["status", "--porcelain"])
```

### High-Performance Network Server
```cursed
# Platform-optimized TCP server
slay start_server(port drip) yikes<fam> {
    # Get optimal I/O backend for platform
    sus io_backend = platformz.high_performance_io()
    
    sus listener = platformz.tcp_listen("0.0.0.0", port) fam {
        when "permission_denied" -> yikes "Cannot bind to port " + port.to_string()
    }
    
    vibez.spill("Server listening on port", port)
    vibez.spill("Using I/O backend:", io_backend.name())
    
    # Accept connections with platform optimization
    bestie (connection := listener.accept()) {
        go handle_connection_optimized(connection, io_backend)
    }
}

slay handle_connection_optimized(conn Connection, io_backend IoBackend) {
    # Use platform-specific optimizations
    ready (io_backend.supports_zero_copy()) {
        # Use zero-copy operations where available
        conn.enable_zero_copy()
    }
    
    # Handle connection...
    defer conn.close()
}
```

### System Resource Monitoring
```cursed
# Cross-platform system monitoring
slay system_monitor() {
    bestie (based) {
        sus sys_info = platformz.system_info()
        sus proc_info = platformz.current_process_info()
        
        vibez.spill("System Load:")
        vibez.spill("  Total Memory:", sys_info.total_memory_bytes / (1024*1024), "MB")
        vibez.spill("  Available Memory:", sys_info.available_memory_bytes / (1024*1024), "MB")
        vibez.spill("  CPU Usage:", sys_info.cpu_usage_percent, "%")
        
        vibez.spill("Process Info:")
        vibez.spill("  PID:", proc_info.pid)
        vibez.spill("  Memory Usage:", proc_info.memory_usage_bytes / (1024*1024), "MB")
        vibez.spill("  CPU Time:", proc_info.cpu_time_seconds, "seconds")
        
        timez.sleep(5000)  # Monitor every 5 seconds
    }
}
```

### Environment Configuration
```cursed
# Cross-platform environment handling
slay setup_environment() {
    sus env_vars = platformz.environment_variables()
    sus user = platformz.user_info()
    
    # Set platform-appropriate paths
    sus app_data_dir = ready (platformz.is_windows()) {
        when based -> platformz.path_join([env_vars.get("APPDATA"), "MyApp"])
        when cap -> platformz.path_join([user.home_directory, ".myapp"])
    }
    
    # Create application directories
    platformz.create_directory_all(app_data_dir) fam {
        when _ -> vibez.spill("Warning: Could not create app data directory")
    }
    
    # Set up logging with platform-appropriate paths
    sus log_dir = platformz.path_join([app_data_dir, "logs"])
    platformz.create_directory_all(log_dir)
    
    vibez.spill("App data directory:", app_data_dir)
    vibez.spill("Log directory:", log_dir)
}
```

## Performance Considerations

### Platform-Specific Optimizations

**I/O Operations**: Different platforms excel with different I/O patterns
- **Linux**: Use io_uring for batch operations, epoll for event-driven I/O
- **Windows**: Use IOCP for high-concurrency scenarios, overlapped I/O for throughput
- **macOS**: Use kqueue for event handling, Grand Central Dispatch for parallelism

**Memory Management**: Platform memory characteristics vary significantly
- **Linux**: Overcommit behavior affects memory allocation strategies
- **Windows**: Virtual memory system behaves differently from Unix
- **macOS**: Memory pressure system requires different monitoring approaches

### Performance Best Practices

1. **Use Platform Detection**: Check `platformz.current_platform()` to enable platform-specific optimizations
2. **Batch Operations**: Group file system operations to reduce syscall overhead
3. **Memory Mapping**: Use memory-mapped files for large file I/O operations
4. **Async I/O**: Leverage platform-optimized async I/O for network operations
5. **Resource Pooling**: Pool expensive resources like file handles and network connections

### Resource Management

```cursed
# Efficient resource management across platforms
slay optimized_file_processing(files []tea) {
    # Use platform-appropriate batch size
    sus batch_size = ready (platformz.is_windows()) {
        when based -> 64   # Windows handles batching well
        when cap -> 256    # Unix systems prefer larger batches
    }
    
    # Process files in platform-optimized batches
    bestie (i drip = 0; i < files.len(); i += batch_size) {
        sus batch = files[i:mathz.min(i + batch_size, files.len())]
        process_file_batch(batch)
    }
}
```

## Security Considerations

### Path Traversal Prevention

**Threat**: Malicious paths can access files outside intended directories
**Mitigation**: Platform-aware path validation and canonicalization

```cursed
slay safe_path_join(base tea, user_input tea) yikes<tea> {
    # Canonicalize paths to prevent traversal attacks
    sus canonical_base = platformz.canonical_path(base) fam {
        when _ -> yikes "Invalid base path"
    }
    
    sus proposed_path = platformz.path_join([base, user_input])
    sus canonical_proposed = platformz.canonical_path(proposed_path) fam {
        when _ -> yikes "Invalid path"
    }
    
    # Ensure proposed path is within base directory
    ready (!canonical_proposed.starts_with(canonical_base)) {
        yikes "Path traversal detected"
    }
    
    damn canonical_proposed
}
```

### Process Security

**Threat**: Process spawning can be exploited for command injection
**Mitigation**: Proper argument escaping and validation

```cursed
slay safe_process_spawn(command tea, args []tea) yikes<Process> {
    # Validate command exists and is executable
    sus command_path = platformz.find_executable(command) fam {
        when _ -> yikes "Command not found or not executable"
    }
    
    # Sanitize arguments
    sus safe_args []tea = []
    bestie (arg in args) {
        sus sanitized = platformz.sanitize_argument(arg)
        safe_args.append(sanitized)
    }
    
    damn platformz.spawn_process(command_path, safe_args, fam)
}
```

### Permission Management

**Threat**: Incorrect permissions can lead to privilege escalation or data exposure
**Mitigation**: Platform-appropriate permission handling

```cursed
slay create_secure_file(path tea) yikes<File> {
    # Create file with restrictive permissions
    sus permissions = ready (platformz.is_unix()) {
        when based -> platformz.unix_permissions(0o600)  # Owner read/write only
        when cap -> platformz.windows_permissions("OWNER_FULL_CONTROL")
    }
    
    damn platformz.create_file_with_permissions(path, permissions)
}
```

## Error Handling Patterns

### Platform-Aware Error Handling
```cursed
slay handle_file_error(err tea) tea {
    # Map platform-specific errors to user-friendly messages
    damn ready (err) {
        when "file_not_found" -> "The requested file could not be found"
        when "access_denied" -> ready (platformz.is_windows()) {
            when based -> "Access denied. Check file permissions or UAC settings"
            when cap -> "Permission denied. Check file ownership and permissions"
        }
        when "disk_full" -> "Not enough disk space available"
        when "path_too_long" -> ready (platformz.is_windows()) {
            when based -> "File path too long. Windows paths must be < 260 characters"
            when cap -> "File path too long for filesystem"
        }
        when _ -> "An unexpected file system error occurred: " + err
    }
}
```

### Cross-Platform Resource Cleanup
```cursed
slay with_platform_resource<T>(create_resource: slay() yikes<T>, use_resource: slay(T)) yikes<fam> {
    sus resource = create_resource() fam {
        when _ -> yikes "Failed to create platform resource"
    }
    
    defer {
        # Platform-specific cleanup
        ready (platformz.is_windows()) {
            when based -> platformz.windows_close_handle(resource)
            when cap -> platformz.unix_close_fd(resource)
        }
    }
    
    use_resource(resource)
    damn fam
}
```

## Integration with CURSED Ecosystem

### Memory Management Integration
```cursed
# Platform allocators integrate with CURSED's arena system
sus platform_arena = platformz.create_platform_arena()

# Automatically use platform-optimized allocation strategies
# Linux: Uses madvise() for memory hints
# Windows: Uses VirtualAlloc() with appropriate flags  
# macOS: Uses mach_vm_allocate() for optimal performance
```

### Concurrency Integration
```cursed
# Platform-optimized goroutine scheduling
ready (platformz.supports_cpu_affinity()) {
    # Pin goroutines to CPU cores on supporting platforms
    concurrenz.set_goroutine_affinity(goroutine_id, cpu_core)
}
```

### Error System Integration
```cursed
# Platform errors automatically integrate with CURSED's error system
sus file_result = platformz.read_file("config.txt") fam {
    when "file_not_found" -> handle_missing_config()
    when "permission_denied" -> handle_permission_error()
    when platform_error -> handle_platform_specific_error(platform_error)
}
```

The platformz module provides seamless cross-platform functionality while enabling platform-specific optimizations when needed. It abstracts away OS differences without sacrificing performance or capabilities, making CURSED applications truly portable across all supported platforms.
