# Signal Boost Module

A comprehensive signal handling and event processing system for CURSED that provides robust signal management, custom event handling, and graceful application lifecycle management.

## Features

- **Signal Management**: Complete signal handling system with registration and dispatch
- **Custom Handlers**: Register custom signal handlers with priorities and chaining
- **Signal Blocking**: Block and unblock signals with queuing support
- **Signal Forwarding**: Forward signals to other processes
- **Graceful Shutdown**: Elegant application shutdown and cleanup
- **Signal Monitoring**: Real-time signal monitoring and debugging
- **Event Processing**: Advanced event processing with metadata
- **Signal Patterns**: Common signal handling patterns and utilities

## Core Components

### Signal
Represents a system signal with properties like type, number, and handling options.

### SignalHandler
Handles specific signals with custom functions and priorities.

### SignalManager
Central signal management system that coordinates handlers and events.

### SignalEvent
Represents a signal event with timing and metadata information.

## Basic Usage

### Signal Manager Setup
```cursed
// Create signal manager
sus manager SignalManager = create_signal_manager()

// Register signal handler
manager = register_handler(manager, "SIGINT", "interrupt_handler", "handle_interrupt")
```

### Graceful Shutdown
```cursed
// Setup graceful shutdown
manager = setup_graceful_shutdown(manager)

// Handle shutdown signals
manager = register_handler(manager, "SIGTERM", "graceful_shutdown", "cleanup_and_exit")
manager = register_handler(manager, "SIGINT", "graceful_shutdown", "cleanup_and_exit")
```

### Signal Handling
```cursed
// Handle incoming signal
sus event_data map[tea]tea = {}
event_data["source"] = "web_server"
event_data["request_id"] = "12345"

manager = handle_signal(manager, "SIGUSR1", event_data)
```

## Signal Types

### Standard Signals
- **SIGINT**: Interrupt signal (Ctrl+C)
- **SIGTERM**: Termination signal
- **SIGKILL**: Force kill signal (cannot be caught)
- **SIGHUP**: Hangup signal (reload configuration)
- **SIGUSR1**: User-defined signal 1
- **SIGUSR2**: User-defined signal 2

### Custom Signals
```cursed
// Create custom signal
sus custom_signal Signal = create_custom_signal("CUSTOM_RELOAD", 100)
```

## Advanced Features

### Signal Blocking
```cursed
// Block specific signals
manager = block_signal(manager, "SIGINT")
manager = block_signal(manager, "SIGTERM")

// Unblock signals (processes pending signals)
manager = unblock_signal(manager, "SIGINT")
```

### Signal Masking
```cursed
// Create signal mask
sus mask [tea] = create_signal_mask(["SIGINT", "SIGTERM", "SIGUSR1"])

// Apply mask (blocks all signals in mask)
manager = apply_signal_mask(manager, mask)

// Remove mask (unblocks all signals in mask)
manager = remove_signal_mask(manager, mask)
```

### Signal Forwarding
```cursed
// Forward signal to another process
sus success lit = forward_signal(manager, "SIGUSR1", 12345)
```

### Handler Priorities
```cursed
// Chain handlers with priorities
manager = chain_handler(manager, "SIGTERM", "high_priority_handler")
```

## Signal Patterns

### Graceful Shutdown Pattern
```cursed
// Setup graceful shutdown handlers
manager = setup_graceful_shutdown(manager)

// Custom shutdown logic
manager = register_handler(manager, "SIGTERM", "cleanup", "cleanup_resources")
manager = register_handler(manager, "SIGINT", "cleanup", "cleanup_resources")
```

### Configuration Reload Pattern
```cursed
// Reload configuration on SIGHUP
manager = register_handler(manager, "SIGHUP", "reload_config", "reload_configuration")
```

### User Operations Pattern
```cursed
// User-defined operations
manager = register_handler(manager, "SIGUSR1", "log_stats", "print_statistics")
manager = register_handler(manager, "SIGUSR2", "debug_mode", "toggle_debug")
```

## Monitoring and Debugging

### Signal Statistics
```cursed
// Get signal statistics
sus stats map[tea]normie = get_signal_stats(manager)

vibez.spill("Total signals: " + string(stats["total_signals"]))
vibez.spill("Blocked signals: " + string(stats["blocked_signals"]))
vibez.spill("Pending signals: " + string(stats["pending_signals"]))
```

### Real-time Monitoring
```cursed
// Monitor signal activity
sus monitor_report tea = monitor_signals(manager)
vibez.spill(monitor_report)
```

### Signal Debugging
```cursed
// Debug specific signal
sus debug_info tea = debug_signal(manager, "SIGINT")
vibez.spill(debug_info)
```

## Testing and Simulation

### Signal Testing
```cursed
// Test signal delivery
sus test_success lit = test_signal_delivery(manager, "SIGTERM")

// Simulate signal with delay
sus sim_success lit = simulate_signal(manager, "SIGUSR1", 1000)
```

### Handler Testing
```cursed
// Test specific handler
sus handlers [SignalHandler] = get_handlers(manager, "SIGINT")

bestie i := 0; i < len(handlers); i++ {
    vibez.spill("Handler: " + handlers[i].handler_name)
    vibez.spill("Calls: " + string(handlers[i].call_count))
}
```

## Integration Examples

### Web Server Signal Handling
```cursed
// Setup web server signal handling
sus web_manager SignalManager = create_signal_manager()

// Graceful shutdown for web server
web_manager = register_handler(web_manager, "SIGTERM", "shutdown_server", "stop_accepting_connections")
web_manager = register_handler(web_manager, "SIGINT", "shutdown_server", "stop_accepting_connections")

// Reload configuration
web_manager = register_handler(web_manager, "SIGHUP", "reload_config", "reload_server_config")

// Health check trigger
web_manager = register_handler(web_manager, "SIGUSR1", "health_check", "run_health_check")
```

### Database Service Signals
```cursed
// Database service signal handling
sus db_manager SignalManager = create_signal_manager()

// Graceful database shutdown
db_manager = register_handler(db_manager, "SIGTERM", "db_shutdown", "close_connections")

// Backup trigger
db_manager = register_handler(db_manager, "SIGUSR1", "backup", "create_backup")

// Statistics dump
db_manager = register_handler(db_manager, "SIGUSR2", "stats", "dump_statistics")
```

### Background Process Signals
```cursed
// Background process signal handling
sus bg_manager SignalManager = create_signal_manager()

// Pause/resume processing
bg_manager = register_handler(bg_manager, "SIGSTOP", "pause", "pause_processing")
bg_manager = register_handler(bg_manager, "SIGCONT", "resume", "resume_processing")

// Process queue status
bg_manager = register_handler(bg_manager, "SIGUSR1", "queue_status", "report_queue_status")
```

## Error Handling

### Signal Handler Errors
```cursed
// Handle signal handler errors
manager = register_handler(manager, "SIGTERM", "error_handler", "handle_signal_error")
```

### Recovery Strategies
```cursed
// Implement recovery for failed handlers
manager = register_handler(manager, "SIGINT", "recovery", "recover_from_error")
```

## Configuration

### Signal Manager Configuration
```cursed
// Configure signal manager
manager.enabled = based        // Enable/disable signal handling
manager.debug_mode = based     // Enable debug output
manager.max_pending = 100      // Maximum pending signals
```

### Handler Configuration
```cursed
// Configure signal handlers
sus handler SignalHandler = SignalHandler{
    signal_type: "SIGTERM",
    handler_name: "graceful_shutdown",
    handler_func: "cleanup_and_exit",
    priority: 1,
    enabled: based,
    call_count: 0
}
```

## Best Practices

1. **Signal Safety**: Always handle signals in a signal-safe manner
2. **Graceful Shutdown**: Implement proper cleanup in signal handlers
3. **Handler Priorities**: Use priorities to ensure critical handlers run first
4. **Error Handling**: Handle errors in signal handlers gracefully
5. **Testing**: Test signal handlers in isolation and integration
6. **Monitoring**: Monitor signal activity for debugging and optimization
7. **Documentation**: Document signal handling behavior clearly

## Performance Considerations

### Signal Overhead
- **Handler Execution**: Keep signal handlers lightweight
- **Memory Usage**: Minimize memory allocation in handlers
- **Blocking Time**: Minimize time signals are blocked
- **Queue Management**: Manage pending signal queues efficiently

### Optimization Strategies
- **Batch Processing**: Process multiple signals together when possible
- **Async Handlers**: Use asynchronous processing for complex operations
- **Handler Pooling**: Reuse handler resources where possible
- **Selective Handling**: Only handle necessary signals

## Security Considerations

### Signal Injection
- **Validation**: Validate signal sources and data
- **Sanitization**: Sanitize signal event data
- **Access Control**: Restrict signal handling to authorized processes

### Signal Masking
- **Critical Sections**: Mask signals during critical operations
- **Atomic Operations**: Use atomic operations where necessary
- **Race Conditions**: Prevent race conditions in signal handling

## Testing

Test the signal boost module:
```bash
cargo run --bin cursed stdlib/signal_boost/simple_test.csd
```

## Integration with System

### Operating System Integration
The signal boost module integrates with the underlying operating system's signal handling mechanisms while providing a higher-level, safer interface for CURSED applications.

### Process Management
Works seamlessly with process management systems to provide clean shutdown, configuration reloading, and operational control.

This signal handling system provides a robust foundation for building reliable, maintainable CURSED applications that respond appropriately to system signals and operational events.
