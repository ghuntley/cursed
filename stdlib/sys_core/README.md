# sys_core Module

The `sys_core` module provides low-level system operations for CURSED programs, essential for self-hosting and system programming capabilities.

## Overview

This module implements core system functionality using pure CURSED code, making it suitable for self-hosting scenarios and ensuring compatibility across interpretation and compilation modes.

## Features

### System Information
- `get_system_info()` - Comprehensive system information
- `get_platform()` - Platform identifier (e.g., "linux-x64")
- `get_architecture()` - CPU architecture
- `get_os_version()` - Operating system version

### Memory Management
- `alloc(size)` - Allocate memory blocks
- `free(ptr)` - Free allocated memory
- `memory_usage()` - Current memory usage
- `get_memory_limit()` / `set_memory_limit()` - Memory limits
- `get_heap_size()` - Current heap size

### Process Management
- `spawn_process(command)` - Create new processes
- `kill_process(pid)` - Terminate processes
- `get_process_id()` / `get_parent_process_id()` - Process IDs
- `process_exists(pid)` - Check process existence

### Signal Handling
- `register_signal_handler(signal)` - Register signal handlers
- `send_signal(pid, signal)` - Send signals to processes
- `ignore_signal(signal)` - Ignore specific signals

### Resource Management
- `set_resource_limit(resource, limit)` - Set resource limits
- `get_resource_limit(resource)` - Get resource limits
- `get_cpu_usage()` - CPU usage percentage
- `get_open_files_count()` - Open file descriptors

### Environment Operations
- `get_environment_variable(name)` / `set_environment_variable(name, value)` - Environment variables
- `get_working_directory()` / `set_working_directory(path)` - Working directory

### Time and Scheduling
- `get_system_time()` - System time (seconds since epoch)
- `sleep_milliseconds(ms)` - Sleep for specified duration
- `get_process_priority()` / `set_process_priority()` - Process priority

### Hardware Information
- `get_cpu_count()` - Number of CPU cores
- `get_total_memory()` / `get_available_memory()` - Memory information
- `get_stack_size()` / `set_stack_size()` - Stack size management

### Network Information
- `get_hostname()` - System hostname
- `get_network_interfaces()` - Network interface details

### Security Functions
- `get_user_id()` / `get_group_id()` - User and group IDs
- `has_root_privileges()` - Check for elevated privileges

### Performance Monitoring
- `get_load_average()` - System load average
- `get_uptime()` - System uptime

## Usage Examples

### Basic System Information
```cursed
yeet "sys_core"

sus platform tea = sys_core.get_platform()
vibez.spill("Platform: " + platform)

sus cpu_count normie = sys_core.get_cpu_count()
vibez.spill("CPU Cores: " + tea(cpu_count))
```

### Memory Management
```cursed
yeet "sys_core"

# Allocate memory block
sus memory_addr normie = sys_core.alloc(1024)
vibez.spill("Allocated at: " + tea(memory_addr))

# Check memory usage
sus usage normie = sys_core.memory_usage()
vibez.spill("Memory usage: " + tea(usage) + " bytes")

# Free memory
sys_core.free(memory_addr)
```

### Process Management
```cursed
yeet "sys_core"

# Get current process info
sus pid normie = sys_core.get_process_id()
vibez.spill("Current PID: " + tea(pid))

# Spawn new process
sus child_pid normie = sys_core.spawn_process("echo Hello")
vibez.spill("Spawned PID: " + tea(child_pid))
```

### Resource Monitoring
```cursed
yeet "sys_core"

# Monitor system resources
sus cpu_usage normie = sys_core.get_cpu_usage()
sus memory_total normie = sys_core.get_total_memory()
sus memory_available normie = sys_core.get_available_memory()

vibez.spill("CPU Usage: " + tea(cpu_usage) + "%")
vibez.spill("Memory: " + tea(memory_available) + "/" + tea(memory_total))
```

### Environment Variables
```cursed
yeet "sys_core"

# Get environment variable
sus home_dir tea = sys_core.get_environment_variable("HOME")
vibez.spill("Home directory: " + home_dir)

# Set environment variable
sys_core.set_environment_variable("CURSED_ENV", "production")
```

## Self-Hosting Features

The `sys_core` module is designed with self-hosting in mind:

- **Pure CURSED Implementation**: No external FFI dependencies
- **Cross-Mode Compatibility**: Works in both interpretation and compilation modes
- **Memory Management**: Essential for runtime memory allocation
- **Process Control**: Required for compiler process management
- **Resource Monitoring**: Critical for performance optimization

## Security Considerations

- Memory operations use simulated addresses for safety
- Signal handling provides controlled access to system signals
- Resource limits prevent resource exhaustion
- Privilege checking ensures secure operation

## Testing

Comprehensive test suite included in `test_sys_core.csd`:

```bash
# Run sys_core tests
cargo run --bin cursed stdlib/sys_core/test_sys_core.csd

# Test compilation mode
cargo run --bin cursed -- compile stdlib/sys_core/test_sys_core.csd
./test_sys_core
```

## Performance

The module provides efficient operations for:
- Fast memory allocation/deallocation
- Lightweight process management
- Minimal overhead system calls
- Optimized resource monitoring

## Implementation Notes

- All functions return safe, simulated values for portability
- Memory addresses are calculated rather than actual system pointers
- Process IDs use safe simulation for testing purposes
- Resource limits are tracked internally for consistency

## Dependencies

- `testz` - Testing framework (for tests only)
- No external FFI dependencies
- Pure CURSED implementation

## Compatibility

- **Interpretation Mode**: Full compatibility
- **Compilation Mode**: Full compatibility
- **Self-Hosting**: Essential module for compiler self-hosting
- **Cross-Platform**: Designed for portability across platforms

## Future Enhancements

Potential future additions:
- Extended signal handling
- Advanced memory profiling
- Enhanced process monitoring
- System call interception
- Performance counters
- Real-time resource tracking

This module forms a critical foundation for CURSED's self-hosting capabilities and provides the essential system interface required for advanced system programming.
