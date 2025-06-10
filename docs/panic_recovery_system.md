# CURSED Panic/Recovery System

The CURSED programming language provides a comprehensive panic and recovery system for robust error handling, using Gen Z slang syntax for accessibility and memorability.

## Overview

The panic/recovery system in CURSED is inspired by Go's panic/recover mechanism but with enhanced features and Gen Z terminology:

- **`yeet_error`** - Triggers a panic (throws an error)
- **`catch`** - Establishes a recovery boundary 
- **`recover`** - Optional recovery block for handling panics

## Basic Syntax

### Panic (yeet_error)

```cursed
yeet_error "Something went wrong!"
```

Triggers a panic with the specified message. The panic will unwind the stack until it reaches a recovery point.

### Recovery (catch/recover)

```cursed
catch {
    // Protected code that might panic
    risky_operation()
} recover {
    // Optional recovery code
    println("Recovered from panic")
}
```

## Panic Severity Levels

The runtime system supports different panic severity levels:

- **Recoverable** - Can be caught and handled
- **Critical** - Should terminate the goroutine 
- **Fatal** - Should terminate the entire program

## Panic Categories

Panics are categorized for better error handling:

- **Memory** - Out of memory, null pointer errors
- **TypeAssertion** - Type assertion failures
- **BoundsCheck** - Array/slice bounds violations
- **Arithmetic** - Division by zero, overflow errors
- **Channel** - Channel operations on closed channels
- **Goroutine** - Goroutine-related errors
- **User** - Explicit panic calls
- **System** - OS-level errors
- **Generic** - Unknown category

## Examples

### Basic Error Handling

```cursed
slay safe_divide(normie a, normie b) normie {
    lowkey (b == 0) {
        yeet_error "Division by zero!"
    }
    yolo a / b
}

slay main() {
    catch {
        sus result = safe_divide(10, 0)
        println("Result: " + result)
    } recover {
        println("Division failed, using default value")
        sus result = 0
    }
}
```

### Nested Recovery

```cursed
catch {
    catch {
        yeet_error "Inner error"
    } recover {
        println("Inner recovery")
        yeet_error "Re-throwing error"
    }
} recover {
    println("Outer recovery caught re-thrown error")
}
```

### Conditional Panic

```cursed
slay validate_input(normie value) {
    lowkey (value < 0) {
        yeet_error "Value must be non-negative"
    }
    lowkey (value > 100) {
        yeet_error "Value must not exceed 100"
    }
}
```

## Runtime Features

### Thread Safety

The panic system is fully thread-safe and integrates with CURSED's goroutine system:

```cursed
stan {
    catch {
        risky_goroutine_operation()
    } recover {
        println("Goroutine recovered from panic")
    }
}
```

### Stack Traces

Automatic stack trace capture helps with debugging:

- Function names and source locations
- File and line number information
- Goroutine context when applicable

### Recovery Handlers

Register custom recovery handlers for specific panic types:

```cursed
// Runtime API (used internally)
runtime.register_recovery_handler(|panic_info| {
    lowkey (panic_info.category == PanicCategory::Memory) {
        // Handle memory errors specially
        RecoveryAction::Continue(default_value)
    } highkey {
        RecoveryAction::TerminateGoroutine
    }
})
```

## Configuration

The panic system can be configured at runtime:

```cursed
// Runtime configuration
PanicConfig {
    capture_backtraces: true,
    capture_stack_traces: true,
    max_stack_depth: 100,
    log_to_stderr: true,
    abort_on_fatal: true,
    recovery_timeout: Duration::from_secs(30),
}
```

## Integration with Error System

The panic system integrates seamlessly with CURSED's error handling:

### Error Propagation (`?` operator)

```cursed
slay might_fail() Result<normie, tea> {
    sus value = risky_operation()?  // Propagates errors
    yolo Ok(value)
}
```

### Converting Panics to Errors

```cursed
slay safe_wrapper() Result<normie, tea> {
    catch {
        sus result = panic_prone_function()
        yolo Ok(result)
    } recover {
        yolo Err("Operation failed")
    }
}
```

## Best Practices

### When to Use Panic

1. **Unrecoverable errors** - Program bugs, invariant violations
2. **Resource exhaustion** - Out of memory, stack overflow
3. **Critical failures** - System-level errors

### When to Use Regular Errors

1. **Expected failures** - Network timeouts, file not found
2. **User input errors** - Invalid data, parsing failures
3. **Business logic errors** - Validation failures

### Recovery Guidelines

1. **Keep recovery blocks simple** - Avoid complex logic
2. **Log panic information** - For debugging and monitoring
3. **Avoid silent recovery** - Always log or handle appropriately
4. **Test panic paths** - Ensure recovery works correctly

## Performance Considerations

- **Panic overhead** - Panics are expensive, use sparingly
- **Recovery boundaries** - Minimize nesting depth
- **Stack unwinding** - Large call stacks increase cleanup time

## FFI Integration

For integration with compiled code:

```c
// C FFI functions
void cursed_panic(const char* message, ...);
uint8_t cursed_recover(void);
uint8_t cursed_has_panic(void);
size_t cursed_get_panic_message(char* buffer, size_t len);
```

## Debugging

### Panic Information

Every panic includes:
- Unique panic ID
- Message and context
- Thread and goroutine information
- Timestamp and source location
- Stack trace and backtrace

### Debug Output

```
Panic #1234 [Critical] User: Something went wrong!
  at examples/panic_example.csd:15:5
  in goroutine #42
Stack trace:
  0: risky_operation at examples/panic_example.csd:15:5
  1: main at examples/panic_example.csd:8:12
```

## Testing

The panic system includes comprehensive test coverage:

- Unit tests for runtime components
- Integration tests for language features
- Performance tests for overhead measurement
- Stress tests for concurrent scenarios

## Future Enhancements

Planned improvements include:

- Advanced stack trace symbolication
- Panic analytics and monitoring
- Custom panic types and handlers
- Integration with external error reporting systems

This panic/recovery system provides CURSED with robust error handling capabilities while maintaining the language's accessibility through Gen Z slang syntax.
