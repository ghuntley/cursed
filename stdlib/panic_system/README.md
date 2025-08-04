# Panic System (panic_system)

The `panic_system` module provides controlled program termination and panic recovery mechanisms for CURSED programs.

## Purpose

This module implements panic handling, stack unwinding, and recovery mechanisms for unrecoverable errors and exceptional conditions in CURSED applications.

## Main Functions

### Panic Operations
- `panic_system.panic(message)` - Trigger panic with message
- `panic_system.panic_with_code(code, message)` - Panic with error code
- `panic_system.is_panicking()` - Check if currently in panic state
- `panic_system.abort()` - Immediate program termination

### Panic Recovery
- `panic_system.recover(fn)` - Execute with panic recovery
- `panic_system.catch_panic(fn, handler)` - Catch and handle panics
- `panic_system.set_panic_handler(handler)` - Set custom panic handler
- `panic_system.reset_panic_handler()` - Reset to default handler

### Stack Management
- `panic_system.print_stack_trace()` - Print current stack trace
- `panic_system.get_stack_trace()` - Get stack trace as data
- `panic_system.unwind_stack()` - Manual stack unwinding
- `panic_system.stack_depth()` - Get current stack depth

### Panic Information
- `panic_system.last_panic_message()` - Get last panic message
- `panic_system.panic_count()` - Get total panic count
- `panic_system.panic_location()` - Get panic source location

## Usage Examples

### Basic Panic Handling

```cursed
yeet "panic_system"

slay risky_function(value normie) {
    if value == 0 {
        panic_system.panic("Division by zero not allowed")
    }
    vibez.spillf("Result: {}", 100 / value)
}

fr fr This will panic
risky_function(0)
```

### Panic Recovery

```cursed
yeet "panic_system"

slay safe_operation() {
    sus result = panic_system.recover(slay() {
        risky_function(0)  # This will panic
        damn "Success"
    })
    
    if result.panicked {
        vibez.spillf("Caught panic: {}", result.message)
        damn "Recovered safely"
    }
    damn result.value
}

sus outcome = safe_operation()
vibez.spillf("Final outcome: {}", outcome)
```

### Custom Panic Handler

```cursed
yeet "panic_system"
yeet "logging"

fr fr Set up custom panic handler
panic_system.set_panic_handler(slay(message, location) {
    logging.error("PANIC OCCURRED!")
    logging.errorf("Message: {}", message)
    logging.errorf("Location: {}:{}", location.file, location.line)
    
    fr fr Send to monitoring system
    monitoring.send_alert("PANIC", message)
    
    fr fr Graceful cleanup
    cleanup_resources()
    
    fr fr Allow default behavior (stack trace + exit)
    damn based  # Continue with default panic handling
})

fr fr Now all panics will use custom handler
panic_system.panic("Critical system failure")
```

### Panic with Recovery Strategy

```cursed
yeet "panic_system"
yeet "error_drip"

slay protected_operation(data []normie) Result<normie, tea> {
    sus result = panic_system.catch_panic(
        slay() {
            fr fr Potentially panicking operation
            if data.len() == 0 {
                panic_system.panic("Empty data array")
            }
            
            sus sum normie = 0
            bestie item in data {
                if item < 0 {
                    panic_system.panic("Negative values not allowed")  
                }
                sum = sum + item
            }
            damn sum
        },
        slay(panic_info) {
            vibez.spillf("Caught panic: {}", panic_info.message)
            panic_system.print_stack_trace()
            damn error_drip.new_error("Operation failed due to panic")
        }
    )
    
    if result.panicked {
        damn error_drip.Error(result.error)
    }
    damn error_drip.Ok(result.value)
}

sus data []normie = [1, 2, -3, 4]  # Contains negative value
sus result = protected_operation(data)
```

### Stack Trace Analysis

```cursed
yeet "panic_system"

slay debug_stack() {
    vibez.spillf("Current stack depth: {}", panic_system.stack_depth())
    
    sus trace = panic_system.get_stack_trace()
    vibez.spill("Stack trace:")
    bestie frame in trace.frames {
        vibez.spillf("  {}:{} in {}", frame.file, frame.line, frame.function)
    }
}

slay level3() { debug_stack() }
slay level2() { level3() }  
slay level1() { level2() }
level1()
```

## Compilation Examples

### Interpretation Mode
```bash
echo 'yeet "panic_system"
panic_system.catch_panic(
    slay() { panic_system.panic("Test panic") },
    slay(info) { vibez.spillf("Caught: {}", info.message) }
)' > panic_test.csd

./cursed-unified panic_test.csd
```

### Compilation Mode
```bash
./cursed-unified --compile panic_test.csd
./panic_test
```

## Panic Recovery Result

```cursed
squad PanicRecoveryResult<T> {
    spill panicked lit
    spill message tea
    spill value T
    spill error tea
}
```

## Panic Information

```cursed
squad PanicInfo {
    spill message tea
    spill location SourceLocation
    spill thread_id normie
    spill timestamp normie
}

squad SourceLocation {
    spill file tea
    spill line normie
    spill column normie
    spill function tea
}
```

## Implementation Notes

- Proper stack unwinding with cleanup
- Thread-local panic state management
- Integration with CURSED's defer statements
- Zero-cost when no panics occur
- Preserves stack traces for debugging

## Dependencies

- `memory` - For stack management
- `logging` - For panic logging
- `error_drip` - For error integration
- Core runtime system integration

## Performance Considerations

- Minimal overhead in normal execution
- Efficient stack unwinding mechanism
- Memory-safe panic recovery
- Optimized for rare panic conditions

## Best Practices

1. **Use panics for unrecoverable errors** only
2. **Prefer Result types** for expected failures  
3. **Set up panic handlers** for production systems
4. **Use recovery mechanisms** at appropriate boundaries
5. **Log panic information** for debugging
6. **Clean up resources** in panic handlers
7. **Avoid panics in hot paths** for performance

## Safety Considerations

- Panics bypass normal control flow
- Always clean up critical resources
- Be careful with panic recovery boundaries
- Consider using timeouts for panic-prone operations
- Test panic scenarios thoroughly
