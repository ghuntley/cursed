# Signal Handling Module

Pure CURSED implementation of cross-platform signal handling capabilities without FFI dependencies.

## Overview

The signal handling module provides comprehensive signal management functionality implemented entirely in CURSED. It offers safe signal handling without unsafe code blocks, cross-platform signal abstractions, and reliable signal delivery mechanisms.

## Features

### Core Signal Functionality
- **Signal Registration**: Register custom handlers for different signal types
- **Signal Masking**: Block and unblock signals as needed
- **Signal Queuing**: Reliable signal delivery with queue management
- **Process Management**: Send signals to processes (simulation for pure CURSED)
- **Cross-Platform**: Abstracted signal types that work across platforms

### Signal Types Supported
- `SIGNAL_INT` (SIGINT) - Interrupt signal
- `SIGNAL_TERM` (SIGTERM) - Termination signal
- `SIGNAL_KILL` (SIGKILL) - Kill signal
- `SIGNAL_USR1` (SIGUSR1) - User-defined signal 1
- `SIGNAL_USR2` (SIGUSR2) - User-defined signal 2
- `SIGNAL_HUP` (SIGHUP) - Hangup signal
- `SIGNAL_QUIT` (SIGQUIT) - Quit signal
- `SIGNAL_PIPE` (SIGPIPE) - Broken pipe signal
- `SIGNAL_ALRM` (SIGALRM) - Alarm signal
- `SIGNAL_CHLD` (SIGCHLD) - Child process signal

## Usage Examples

### Basic Signal Handling

```cursed
yeet "signal_handling"

# Initialize the signal system
signal_init()
signal_start()

# Register a signal handler
sus my_handler SignalHandler = slay(signal normie) lit {
    vibez.spill("Received signal: ", signal_name(signal))
    damn based
}

signal_register(SIGNAL_INT, my_handler)

# Install default handlers for common signals
signal_install_defaults()
```

### Signal Blocking and Masking

```cursed
# Block a single signal
signal_block(SIGNAL_USR1)

# Create a signal mask (block multiple signals)
sus signals_to_block []normie = [SIGNAL_USR1, SIGNAL_USR2, SIGNAL_HUP]
signal_create_mask(signals_to_block)

# Check if signal is blocked
lowkey signal_is_blocked(SIGNAL_USR1) {
    vibez.spill("SIGUSR1 is currently blocked")
}

# Unblock signals
signal_unblock(SIGNAL_USR1)
signal_clear_mask(signals_to_block)
```

### Signal Queuing and Processing

```cursed
# Queue a signal for processing
signal_queue_signal(SIGNAL_USR1)

# Process all pending signals
sus processed_count normie = signal_process_pending()
vibez.spill("Processed ", processed_count, " signals")

# Wait for a specific signal with timeout
lowkey signal_wait(SIGNAL_USR2, 5000) {
    vibez.spill("Received SIGUSR2 within timeout")
} yas {
    vibez.spill("Timeout waiting for SIGUSR2")
}
```

### Signal Statistics and Monitoring

```cursed
# Get signal handling statistics
sus stats map[tea]normie = signal_get_stats()
vibez.spill("Handlers registered: ", stats["handlers_registered"])
vibez.spill("Blocked signals: ", stats["blocked_signals"])
vibez.spill("Pending signals: ", stats["pending_signals"])
vibez.spill("Queue length: ", stats["queue_length"])
```

## API Reference

### Initialization Functions

- `signal_init() lit` - Initialize the signal handling system
- `signal_start() lit` - Start the signal handling background process
- `signal_stop() lit` - Stop the signal handling system
- `signal_reset() lit` - Reset the signal system to initial state

### Signal Registration

- `signal_register(signal normie, handler SignalHandler) lit` - Register a signal handler
- `signal_unregister(signal normie) lit` - Unregister a signal handler
- `signal_install_defaults() lit` - Install default handlers for common signals

### Signal Blocking

- `signal_block(signal normie) lit` - Block a specific signal
- `signal_unblock(signal normie) lit` - Unblock a specific signal
- `signal_is_blocked(signal normie) lit` - Check if signal is blocked
- `signal_create_mask(signals []normie) lit` - Block multiple signals
- `signal_clear_mask(signals []normie) lit` - Unblock multiple signals

### Signal Processing

- `signal_queue_signal(signal normie) lit` - Queue a signal for processing
- `signal_process_pending() normie` - Process all pending signals
- `signal_wait(signal normie, timeout_ms normie) lit` - Wait for specific signal

### Utility Functions

- `signal_name(signal normie) tea` - Get human-readable signal name
- `signal_get_stats() map[tea]normie` - Get signal handling statistics
- `signal_send_to_process(pid normie, signal normie) lit` - Send signal to process
- `get_current_pid() normie` - Get current process ID

## Type Definitions

```cursed
# Signal handler function type
be_like SignalHandler = slay(signal normie) lit

# Signal registry structure
collab SignalRegistry {
    sus handlers map[normie]SignalHandler
    sus blocked_signals []normie
    sus pending_signals []normie
    sus signal_queue chan normie
    sus is_active lit
}
```

## Testing

Run the comprehensive test suite:

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/signal_handling/test_signal_handling.csd

# Test compilation mode
cargo run --bin cursed -- compile stdlib/signal_handling/test_signal_handling.csd
./test_signal_handling

# Both-mode verification
test_both_modes() {
    cargo run --bin cursed stdlib/signal_handling/test_signal_handling.csd > interp_output.txt
    cargo run --bin cursed -- compile stdlib/signal_handling/test_signal_handling.csd
    ./test_signal_handling > comp_output.txt
    diff interp_output.txt comp_output.txt
}
```

## Implementation Details

### Pure CURSED Design
- **No FFI Dependencies**: Completely implemented in CURSED without external calls
- **Memory Safety**: No unsafe code blocks or manual memory management
- **Cross-Platform**: Abstract signal types work across different operating systems
- **Goroutine-Based**: Uses CURSED goroutines for background signal processing

### Signal Simulation
Since this is a pure CURSED implementation, actual OS signal handling is simulated:
- Signal handlers are stored in memory and called when signals are queued
- Signal blocking/unblocking affects processing, not actual OS signal masks
- Signal sending between processes is simulated for demonstration
- Real-world usage would require OS-specific integration

### Performance Characteristics
- **Signal Registration**: O(1) average case with map-based storage
- **Signal Processing**: O(n) where n is number of pending signals
- **Signal Blocking**: O(n) where n is number of blocked signals
- **Memory Usage**: Bounded by queue size and number of registered handlers

## Error Handling

The module provides robust error handling:
- Functions return `lit` (boolean) to indicate success/failure
- System must be initialized before use
- Blocked signals are queued but not processed
- Invalid signals are handled gracefully

## Thread Safety

The implementation is designed for concurrent use:
- Global signal registry protected by CURSED concurrency primitives
- Signal queue using channels for thread-safe communication
- Background goroutine for signal processing
- Atomic operations for state management

## Migration from FFI

This module replaces unsafe FFI-based signal handling with:
- Pure CURSED implementation
- Safe memory management
- Cross-platform abstractions
- Comprehensive testing
- Simplified API design

## Future Enhancements

Potential improvements:
- Integration with actual OS signal handling when available
- Advanced signal routing and filtering
- Signal handler priority system
- Signal debugging and tracing capabilities
- Performance optimizations for high-frequency signals
