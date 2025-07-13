# signal_boost Module

**Pure CURSED Signal Management System**

The `signal_boost` module provides comprehensive signal handling capabilities without external FFI dependencies. It implements signal registration, graceful shutdown patterns, signal throttling, multiplexing, and process management entirely in native CURSED code.

## Features

- **Signal Handler Registration**: Register custom handlers for POSIX signals
- **Graceful Shutdown**: Coordinated shutdown with cleanup tasks
- **Signal Throttling**: Rate limiting for signal processing
- **Signal Multiplexing**: Multiple subscribers per signal
- **Pure CURSED Implementation**: Zero external dependencies

## Signal Constants

```cursed
SIGTERM = 15    # Termination signal
SIGINT = 2      # Interrupt signal (Ctrl+C)
SIGUSR1 = 10    # User-defined signal 1
SIGUSR2 = 12    # User-defined signal 2
SIGHUP = 1      # Hangup signal
SIGQUIT = 3     # Quit signal
SIGPIPE = 13    # Broken pipe signal
SIGALRM = 14    # Alarm signal
```

## Handler Types

```cursed
HANDLER_IGNORE = "ignore"     # Ignore the signal
HANDLER_DEFAULT = "default"   # Use default behavior
HANDLER_CUSTOM = "custom"     # Use custom action
```

## Core Functions

### Signal Handler Management

#### `register_signal_handler(signal normie, handler_type tea, action tea) lit`
Register a signal handler with specified type and action.

```cursed
# Register default SIGTERM handler
register_signal_handler(SIGTERM, HANDLER_DEFAULT, "")

# Register custom SIGUSR1 handler
register_signal_handler(SIGUSR1, HANDLER_CUSTOM, "reload_config")
```

#### `unregister_signal_handler(signal normie) lit`
Remove a signal handler.

```cursed
sus success lit = unregister_signal_handler(SIGTERM)
```

#### `has_signal_handler(signal normie) lit`
Check if a signal has a registered handler.

```cursed
if has_signal_handler(SIGINT) {
    vibez.spill("SIGINT handler is registered")
}
```

#### `get_signal_handler(signal normie) map`
Get handler information for a signal.

```cursed
sus handler map = get_signal_handler(SIGTERM)
sus handler_type tea = handler.get("type")
sus count normie = handler.get("count")
```

#### `list_signal_handlers() [normie]`
Get list of all signals with registered handlers.

```cursed
sus signals [normie] = list_signal_handlers()
```

### Signal Processing

#### `notify(signal normie) lit`
Simulate receiving a signal and execute its handler.

```cursed
# Simulate SIGTERM
sus result lit = notify(SIGTERM)
```

#### `get_signal_name(signal normie) tea`
Get the name of a signal from its number.

```cursed
sus name tea = get_signal_name(SIGTERM)  # Returns "SIGTERM"
```

### Graceful Shutdown

#### `initiate_graceful_shutdown()`
Start graceful shutdown process.

```cursed
initiate_graceful_shutdown()
```

#### `add_shutdown_task(task tea) lit`
Add a task to execute during shutdown.

```cursed
add_shutdown_task("save_user_data")
add_shutdown_task("close_database")
```

#### `remove_shutdown_task(task tea) lit`
Remove a shutdown task.

```cursed
remove_shutdown_task("save_user_data")
```

#### `is_shutdown_requested() lit`
Check if shutdown has been requested.

```cursed
if is_shutdown_requested() {
    vibez.spill("Shutdown in progress")
}
```

#### `cancel_shutdown() lit`
Cancel a shutdown request.

```cursed
sus cancelled lit = cancel_shutdown()
```

### Signal Throttling

#### `set_signal_throttle(signal normie, min_interval_ms normie) lit`
Set minimum interval between signal processing.

```cursed
# Throttle SIGTERM to once per second
set_signal_throttle(SIGTERM, 1000)
```

#### `is_signal_throttled(signal normie) lit`
Check if a signal is currently throttled.

```cursed
if !is_signal_throttled(SIGUSR1) {
    # Process the signal
}
```

### Signal Multiplexing

#### `subscribe_to_signal(signal normie, subscriber_id tea) lit`
Subscribe to signal notifications.

```cursed
subscribe_to_signal(SIGINT, "main_handler")
subscribe_to_signal(SIGINT, "logger")
```

#### `unsubscribe_from_signal(signal normie, subscriber_id tea) lit`
Unsubscribe from signal notifications.

```cursed
unsubscribe_from_signal(SIGINT, "logger")
```

#### `notify_subscribers(signal normie)`
Notify all subscribers of a signal.

```cursed
notify_subscribers(SIGUSR1)
```

### Utility Functions

#### `reset()`
Reset all signal boost state.

```cursed
reset()  # Clear all handlers and state
```

#### `init_signal_boost()`
Initialize with default signal handlers.

```cursed
init_signal_boost()  # Sets up common signal handlers
```

#### `get_module_info() tea`
Get module version and information.

```cursed
sus info tea = get_module_info()
```

## Usage Examples

### Basic Signal Handling

```cursed
yeet "signal_boost"

# Initialize the module
init_signal_boost()

# Register custom handlers
register_signal_handler(SIGUSR1, HANDLER_CUSTOM, "reload_config")
register_signal_handler(SIGUSR2, HANDLER_CUSTOM, "dump_stats")

# Add shutdown tasks
add_shutdown_task("save_application_state")
add_shutdown_task("cleanup_temp_files")

# Simulate receiving signals
notify(SIGUSR1)  # Triggers config reload
notify(SIGTERM)  # Triggers graceful shutdown
```

### Signal Throttling

```cursed
yeet "signal_boost"

# Set up throttling
set_signal_throttle(SIGUSR1, 5000)  # Max once per 5 seconds

# Register handler
register_signal_handler(SIGUSR1, HANDLER_CUSTOM, "heavy_operation")

# Multiple rapid signals - only first will be processed
notify(SIGUSR1)  # Processed
notify(SIGUSR1)  # Throttled
notify(SIGUSR1)  # Throttled
```

### Signal Multiplexing

```cursed
yeet "signal_boost"

# Multiple subscribers for same signal
subscribe_to_signal(SIGINT, "main_handler")
subscribe_to_signal(SIGINT, "cleanup_handler")
subscribe_to_signal(SIGINT, "logger")

# Register handler
register_signal_handler(SIGINT, HANDLER_DEFAULT, "")

# All subscribers will be notified
notify(SIGINT)
```

### Graceful Shutdown Pattern

```cursed
yeet "signal_boost"

# Set up graceful shutdown
init_signal_boost()

# Add application-specific shutdown tasks
add_shutdown_task("save_user_session")
add_shutdown_task("close_database_connections")
add_shutdown_task("flush_log_buffers")
add_shutdown_task("notify_monitoring_system")

# Main application loop
while !is_shutdown_requested() {
    # Do application work
    vibez.spill("Application running...")
    
    # Check for signals periodically
    # In real implementation, this would be event-driven
}

vibez.spill("Application shutting down gracefully")
```

### Custom Signal Actions

```cursed
yeet "signal_boost"

# Register handlers with custom actions
register_signal_handler(SIGUSR1, HANDLER_CUSTOM, "reload_config")
register_signal_handler(SIGUSR2, HANDLER_CUSTOM, "dump_stats")
register_signal_handler(SIGHUP, HANDLER_CUSTOM, "rotate_logs")

# Simulate configuration reload
notify(SIGUSR1)

# Simulate statistics dump
notify(SIGUSR2)

# Simulate log rotation
notify(SIGHUP)
```

## Testing

Run the comprehensive test suite:

```bash
cargo run --bin cursed stdlib/signal_boost/test_signal_boost.csd
```

Test both interpretation and compilation modes:

```bash
# Interpretation mode
cargo run --bin cursed stdlib/signal_boost/test_signal_boost.csd

# Compilation mode
cargo run --bin cursed -- compile stdlib/signal_boost/test_signal_boost.csd
./test_signal_boost
```

## Architecture

The `signal_boost` module implements signal handling using pure CURSED data structures:

- **Signal Handlers**: Stored in global map with signal number as key
- **Shutdown Tasks**: Array of task names executed during shutdown
- **Throttling**: Time-based rate limiting using timestamps
- **Multiplexing**: Subscriber lists per signal for notifications
- **State Management**: Global variables track module state

## Signal Processing Flow

1. **Registration**: `register_signal_handler()` adds handler to global map
2. **Notification**: `notify()` looks up handler and executes action
3. **Action Execution**: Handler type determines processing behavior
4. **Subscriber Notification**: All subscribers are notified of signal
5. **Throttling Check**: Rate limiting prevents excessive signal processing

## Default Signal Behaviors

- **SIGTERM/SIGINT**: Initiate graceful shutdown
- **SIGUSR1**: Reload configuration
- **SIGUSR2**: Dump statistics
- **SIGHUP**: Reload configuration
- **Other signals**: Log and continue

## Integration

The `signal_boost` module integrates with other CURSED stdlib modules:

- **core**: Type conversions and utilities
- **vibez**: Logging and output
- **testz**: Comprehensive testing framework

## Production Considerations

- **Error Handling**: All functions return success/failure indicators
- **State Consistency**: Thread-safe operations through atomic updates
- **Resource Management**: Automatic cleanup during shutdown
- **Monitoring**: Built-in statistics and logging
- **Extensibility**: Custom actions and handler types supported

## Version History

- **v1.0**: Initial pure CURSED implementation
  - Core signal handling functionality
  - Graceful shutdown with task management
  - Signal throttling and multiplexing
  - Comprehensive test coverage
  - Zero FFI dependencies
