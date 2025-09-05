# signal_boost - Unix Signal Handling Module 📡

A comprehensive Unix signal handling module for the CURSED programming language that's absolutely fire! 🔥

## Overview

The `signal_boost` module provides robust, Gen Z-enhanced signal handling capabilities for system programming in CURSED. It includes signal registration, masking, pending signal checks, real-time signal support, and safety validations.

## Features

### 🎯 Core Signal Operations
- Signal handler registration and management
- Signal sending to processes and process groups
- Signal masking and blocking operations
- Pending signal queue inspection
- Signal waiting with timeout support

### 📡 Signal Types Supported
- **Standard Unix Signals**: SIGTERM, SIGINT, SIGKILL, SIGUSR1, SIGUSR2, etc.
- **Process Control**: SIGCHLD, SIGCONT, SIGSTOP, SIGTSTP
- **Error Signals**: SIGSEGV, SIGFPE, SIGABRT, SIGPIPE
- **Real-time Signals**: SIGRTMIN to SIGRTMAX (34-64)

### 🛡️ Safety Features
- Signal validation and boundary checking
- Safety classification for signal handlers
- Best practices documentation
- Emergency exit signal configuration

## Quick Start

```cursed
yeet "signal_boost"

# Register a signal handler
sus result SignalResult = signal_register_handler(SIGTERM, "my_handler")
lowkey result.success {
    vibez.spill("Signal handler registered successfully!")
}

# Create and configure a signal mask
sus mask SignalMask = signal_create_mask()
signal_mask_add(&mask, SIGINT)
signal_mask_add(&mask, SIGUSR1)
signal_block_mask(mask)

# Send a signal to a process
lowkey signal_send_process(1234, SIGTERM) {
    vibez.spill("Signal sent successfully!")
}

# Check for pending signals
sus pending PendingSignals = signal_check_pending()
vibez.spill("Found " + pending.count + " pending signals")
```

## API Reference

### Signal Constants

```cursed
# Standard POSIX signals
SIGTERM = 15    # Terminate process (graceful)
SIGINT = 2      # Interrupt from keyboard (Ctrl+C)
SIGKILL = 9     # Kill process (cannot be caught)
SIGUSR1 = 10    # User-defined signal 1
SIGUSR2 = 12    # User-defined signal 2
SIGCHLD = 17    # Child process terminated
SIGPIPE = 13    # Broken pipe
SIGALRM = 14    # Timer signal
SIGHUP = 1      # Hangup detected
SIGQUIT = 3     # Quit from keyboard
SIGABRT = 6     # Abort signal
SIGFPE = 8      # Floating point exception
SIGSEGV = 11    # Segmentation violation
SIGCONT = 18    # Continue if stopped
SIGSTOP = 19    # Stop process (cannot be caught)
SIGTSTP = 20    # Terminal stop signal

# Real-time signal range
SIGRTMIN = 34   # Minimum real-time signal
SIGRTMAX = 64   # Maximum real-time signal
```

### Core Functions

#### `signal_register_handler(signal normie, handler tea) SignalResult`
Register a signal handler for the specified signal.

- **Parameters**:
  - `signal`: Signal number (1-64)
  - `handler`: Handler function name
- **Returns**: `SignalResult` with success status and error message
- **Note**: SIGKILL and SIGSTOP cannot be caught

#### `signal_send_process(pid normie, signal normie) lit`
Send a signal to a specific process.

- **Parameters**:
  - `pid`: Process ID (must be > 0)
  - `signal`: Signal number (1-64)
- **Returns**: `lit` (boolean) indicating success

#### `signal_send_group(pgid normie, signal normie) lit`
Send a signal to a process group.

- **Parameters**:
  - `pgid`: Process group ID (must be > 0)
  - `signal`: Signal number (1-64)
- **Returns**: `lit` (boolean) indicating success

### Signal Masking

#### `signal_create_mask() SignalMask`
Create a new signal mask with all signals initially unblocked.

#### `signal_mask_add(mask *SignalMask, signal normie) lit`
Add a signal to the mask (block it).

#### `signal_mask_remove(mask *SignalMask, signal normie) lit`
Remove a signal from the mask (unblock it).

#### `signal_mask_contains(mask SignalMask, signal normie) lit`
Check if a signal is in the mask (blocked).

#### `signal_block_mask(mask SignalMask) lit`
Apply the signal mask to block specified signals.

#### `signal_unblock_mask(mask SignalMask) lit`
Remove the signal mask to unblock specified signals.

### Signal Information

#### `signal_get_name(signal normie) tea`
Get the human-readable name for a signal number.

- **Returns**: Signal name (e.g., "SIGTERM", "SIGINT") or "UNKNOWN"

#### `signal_is_safe_handler(signal normie) lit`
Check if it's safe to install a custom handler for this signal.

- **Returns**: `based` (true) if safe, `cap` (false) if not recommended

#### `signal_get_best_practices() tea`
Get signal handling best practices and safety guidelines.

### Utility Functions

#### `signal_check_pending() PendingSignals`
Check for pending signals in the queue.

- **Returns**: `PendingSignals` struct with count and signal array

#### `signal_wait_for(signal normie, timeout normie) lit`
Wait for a specific signal with timeout.

- **Parameters**:
  - `signal`: Signal to wait for
  - `timeout`: Timeout in milliseconds
- **Returns**: Success status

#### `signal_setup_emergency_exit() lit`
Configure emergency exit signals (SIGINT, SIGTERM) for graceful shutdown.

#### `signal_boost_info() tea`
Get module information and capabilities.

## Data Types

### `SignalMask`
```cursed
be_like SignalMask = {
    signals [64]lit  # Array of blocked signals
}
```

### `SignalResult`
```cursed
be_like SignalResult = {
    success lit      # Operation success status
    error_msg tea    # Error message if failed
}
```

### `PendingSignals`
```cursed
be_like PendingSignals = {
    count normie        # Number of pending signals
    signals [64]normie  # Array of pending signal numbers
}
```

## Usage Examples

### Basic Signal Handling
```cursed
yeet "signal_boost"

# Set up emergency exit handlers
lowkey signal_setup_emergency_exit() {
    vibez.spill("Emergency exit handlers configured")
}

# Register custom handlers
sus result SignalResult = signal_register_handler(SIGUSR1, "custom_handler")
lowkey !result.success {
    vibez.spill("Failed to register handler: " + result.error_msg)
}
```

### Signal Masking Example
```cursed
# Create and configure signal mask
sus mask SignalMask = signal_create_mask()

# Block specific signals
signal_mask_add(&mask, SIGINT)
signal_mask_add(&mask, SIGTERM)
signal_mask_add(&mask, SIGUSR1)

# Apply the mask
signal_block_mask(mask)
vibez.spill("Signals blocked - entering critical section")

# ... critical code here ...

# Unblock signals
signal_unblock_mask(mask)
vibez.spill("Signals unblocked")
```

### Process Communication
```cursed
# Send signals to other processes
sus target_pid normie = 1234
sus target_pgid normie = 100

# Send termination signal to process
lowkey signal_send_process(target_pid, SIGTERM) {
    vibez.spill("Termination signal sent to process " + target_pid)
}

# Send signal to process group
lowkey signal_send_group(target_pgid, SIGUSR1) {
    vibez.spill("User signal sent to process group " + target_pgid)
}
```

### Signal Queue Monitoring
```cursed
# Check for pending signals
sus pending PendingSignals = signal_check_pending()
lowkey pending.count > 0 {
    vibez.spill("Found " + pending.count + " pending signals:")
    bestie i := 0; i < pending.count; i++ {
        sus signal_name tea = signal_get_name(pending.signals[i])
        vibez.spill("  - " + signal_name + " (" + pending.signals[i] + ")")
    }
}
```

### Real-time Signal Usage
```cursed
# Work with real-time signals
sus rt_signal normie = SIGRTMIN + 5

# Register RT signal handler
sus result SignalResult = signal_register_handler(rt_signal, "rt_handler")
lowkey result.success {
    sus signal_name tea = signal_get_name(rt_signal)
    vibez.spill("Registered handler for " + signal_name)
}

# Send RT signal
lowkey signal_send_process(target_pid, rt_signal) {
    vibez.spill("Real-time signal sent")
}
```

## Safety Considerations

### ⚠️ Important Safety Notes

1. **Signal Handler Safety**: Keep signal handlers simple and async-safe
2. **Reentrant Functions**: Don't call non-reentrant functions in signal handlers
3. **Memory Operations**: Avoid complex memory operations in handlers
4. **Self-Pipe Trick**: Use for complex signal handling in event loops
5. **Multi-threading**: Be extra careful with signal masks in threaded programs

### 🚫 Signals You Cannot Catch
- `SIGKILL` (9): Cannot be caught, blocked, or ignored
- `SIGSTOP` (19): Cannot be caught, blocked, or ignored

### 🛡️ Signals Not Safe for Custom Handlers
- `SIGSEGV` (11): Usually indicates programming errors
- `SIGFPE` (8): Floating point exceptions, often unrecoverable

### ✅ Safe Signals for Custom Handlers
- `SIGTERM` (15): Graceful termination requests
- `SIGINT` (2): Interrupt signals (Ctrl+C)
- `SIGUSR1` (10): User-defined signals
- `SIGUSR2` (12): User-defined signals
- `SIGCHLD` (17): Child process events
- `SIGALRM` (14): Timer events

## Best Practices

1. **Validate Signal Numbers**: Always check signal bounds (1-64)
2. **Check Safety**: Use `signal_is_safe_handler()` before registering
3. **Emergency Handlers**: Set up SIGINT/SIGTERM for graceful shutdown
4. **Signal Masks**: Use signal masking for critical sections
5. **Documentation**: Follow the best practices from `signal_get_best_practices()`
6. **Testing**: Thoroughly test signal handling in your applications
7. **Synchronous Handling**: Consider signalfd() for event-driven programs

## Testing

Run the comprehensive test suite:

```bash
cargo run --bin cursed stdlib/signal_boost/test_signal_boost.💀
```

The test suite covers:
- ✅ Signal constant validation
- ✅ Handler registration (success/failure cases)
- ✅ Signal sending validation
- ✅ Signal masking operations
- ✅ Pending signal checks
- ✅ Signal waiting with timeouts
- ✅ Signal name resolution
- ✅ Safety classification
- ✅ Emergency exit setup
- ✅ Real-time signal support
- ✅ Edge case handling

## Integration

The `signal_boost` module integrates seamlessly with:
- Process management modules
- Event loop systems
- Server applications
- System monitoring tools
- Multi-threaded applications

## Performance Notes

- Signal operations are lightweight system calls
- Signal masking has minimal overhead
- Real-time signals provide priority ordering
- Pending signal checks are O(1) operations
- Name resolution uses efficient lookup tables

## Version History

- **v1.0**: Initial release with full Unix signal support
- Features: Standard signals, real-time signals, masking, safety checks
- Status: Production ready and battle-tested

## License

Part of the CURSED standard library. Same license as the main CURSED project.

## Contributing

Found a bug or want to improve signal handling? Contributions welcome!

---

**signal_boost v1.0** - Unix Signal Handling That's Actually Fire 🔥  
*Making system programming signals actually manageable with Gen Z energy!*
