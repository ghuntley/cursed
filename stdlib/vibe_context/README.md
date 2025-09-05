# vibe_context Module

Request-scoped context management for CURSED applications.

## Overview

The `vibe_context` module implements request-scoped contexts for passing deadlines, cancellation signals, and other values across API boundaries. It's inspired by Go's context package but with a vibe-focused twist for CURSED applications.

## Core Features

### Context Interface
All contexts implement the `Context` interface with these methods:
- `Deadline() (normie, lit)` - Get context deadline
- `Done() chan squad{}` - Get cancellation channel
- `Err() tea` - Get context error
- `Value(key interface{}) interface{}` - Get context value
- `CheckVibe() tea` - Get context vibe (CURSED-specific)

### Context Types
- **Background**: Empty context that never cancels
- **TODO**: Empty context for incomplete code
- **Cancellable**: Context that can be manually cancelled
- **Deadline**: Context with expiration time
- **Timeout**: Context with relative timeout
- **Value**: Context with key-value pairs
- **Vibe**: Context with CURSED-specific vibe

## Usage Examples

### Basic Context Usage
```cursed
# Create background context
ctx := vibe_context.Background()

# Create cancellable context
ctx, cancel := vibe_context.WithCancel(ctx)
defer cancel()

# Create context with timeout
ctx, cancel := vibe_context.WithTimeout(ctx, 5000000000) # 5 seconds
defer cancel()

# Create context with deadline
deadline := getCurrentTime() + 10000000000 # 10 seconds from now
ctx, cancel := vibe_context.WithDeadline(ctx, deadline)
defer cancel()
```

### Context Values
```cursed
# Add values to context
ctx := vibe_context.WithValue(ctx, "user_id", 12345)
ctx = vibe_context.WithValue(ctx, "request_id", "req-abc-123")

# Retrieve values
userID := ctx.Value("user_id")
requestID := ctx.Value("request_id")

# Add multiple values at once
values := map[interface{}]interface{}{
    "key1": "value1",
    "key2": 42,
    "key3": based
}
ctx = vibe_context.WithValues(ctx, values)
```

### Vibe Context (CURSED-specific)
```cursed
# Add vibe to context
ctx := vibe_context.WithVibe(ctx, "chill")

# Check vibe
vibe := ctx.CheckVibe()
if vibe == "chill" {
    # Handle in a relaxed way
}

# Add multiple vibes
vibes := []tea{"focused", "energetic", "productive"}
ctx = vibe_context.WithVibes(ctx, vibes)
```

### Context Cancellation
```cursed
# Create cancellable context
ctx, cancel := vibe_context.WithCancel(parent)

# Check if context is cancelled
if vibe_context.IsCancelled(ctx) {
    vibez.spill("Context was cancelled")
    return
}

# Wait for cancellation
select {
case <-ctx.Done():
    vibez.spill("Context cancelled: %s", ctx.Err())
default:
    # Continue processing
}

# Cancel context
cancel()
```

### Context in Function Calls
```cursed
slay ProcessRequest(ctx vibe_context.Context, request *Request) {
    # Check if context is cancelled
    select {
    case <-ctx.Done():
        vibez.spill("Request cancelled: %s", ctx.Err())
        return
    default:
        # Continue processing
    }
    
    # Get values from context
    userID := ctx.Value("user_id")
    vibe := ctx.CheckVibe()
    
    # Process based on vibe
    if vibe == "urgent" {
        processUrgently(request)
    } else {
        processNormally(request)
    }
}
```

### Advanced Context Operations
```cursed
# Clone context
originalCtx := vibe_context.WithValue(parent, "key", "value")
clonedCtx := vibe_context.Clone(originalCtx)

# Merge contexts
ctx1 := vibe_context.WithValue(parent, "key1", "value1")
ctx2 := vibe_context.WithValue(parent, "key2", "value2")
merged := vibe_context.MergeContexts([]vibe_context.Context{ctx1, ctx2})

# Create detached context
detached := vibe_context.Detached()

# Context with expiration
ctx, cancel := vibe_context.WithExpiration(parent, 30000000000) # 30 seconds
defer cancel()
```

## API Reference

### Core Functions
- `Background() Context` - Create background context
- `TODO() Context` - Create empty context for incomplete code
- `WithCancel(Context) (Context, CancelFunc)` - Create cancellable context
- `WithTimeout(Context, normie) (Context, CancelFunc)` - Create timeout context
- `WithDeadline(Context, normie) (Context, CancelFunc)` - Create deadline context
- `WithValue(Context, interface{}, interface{}) Context` - Add value to context
- `WithVibe(Context, tea) Context` - Add vibe to context

### Helper Functions
- `IsCancelled(Context) lit` - Check if context is cancelled
- `WaitForDone(Context) tea` - Wait for context to be done
- `GetValue(Context, interface{}) interface{}` - Get context value
- `HasValue(Context, interface{}) lit` - Check if context has value
- `GetDepth(Context) normie` - Get context chain depth
- `GetAllValues(Context) map[interface{}]interface{}` - Get all values

### Extended Functions
- `WithValues(Context, map[interface{}]interface{}) Context` - Add multiple values
- `WithVibes(Context, []tea) Context` - Add multiple vibes
- `WithExpiration(Context, normie) (Context, CancelFunc)` - Create expiring context
- `WithoutCancel(Context) Context` - Create non-cancellable context
- `MergeContexts([]Context) Context` - Merge multiple contexts
- `Detached() Context` - Create detached context
- `Clone(Context) Context` - Clone context

## Error Constants

- `ErrCanceled` - "context canceled"
- `ErrDeadlineExceeded` - "context deadline exceeded"

## Context Patterns

### Request Processing
```cursed
slay HandleHTTPRequest(w http.ResponseWriter, r *http.Request) {
    # Create request context with timeout
    ctx, cancel := vibe_context.WithTimeout(r.Context(), 30000000000) # 30s
    defer cancel()
    
    # Add request metadata
    ctx = vibe_context.WithValue(ctx, "request_id", generateRequestID())
    ctx = vibe_context.WithValue(ctx, "user_id", getUserID(r))
    
    # Set vibe based on request priority
    if isPriorityRequest(r) {
        ctx = vibe_context.WithVibe(ctx, "urgent")
    } else {
        ctx = vibe_context.WithVibe(ctx, "normal")
    }
    
    # Process request
    processRequest(ctx, r)
}
```

### Service Chain
```cursed
slay ServiceA(ctx vibe_context.Context, data interface{}) {
    # Check cancellation
    select {
    case <-ctx.Done():
        return
    default:
    }
    
    # Add service metadata
    ctx = vibe_context.WithValue(ctx, "service", "A")
    
    # Call next service
    ServiceB(ctx, data)
}

slay ServiceB(ctx vibe_context.Context, data interface{}) {
    # Check vibe and adapt behavior
    vibe := ctx.CheckVibe()
    if vibe == "urgent" {
        # Use fast path
        fastProcessData(data)
    } else {
        # Use normal path
        normalProcessData(data)
    }
}
```

## Testing

Run the test suite:
```bash
cargo run --bin cursed stdlib/vibe_context/test_vibe_context.💀
```

Test both interpretation and compilation modes:
```bash
cargo run --bin cursed stdlib/vibe_context/test_vibe_context.💀
cargo run --bin cursed -- compile stdlib/vibe_context/test_vibe_context.💀
./test_vibe_context
```

## Integration

The `vibe_context` module is designed to integrate with:
- `trace_tea` - For tracing context propagation
- `concurrenz` - For concurrent operations
- `web_vibez` - For web request contexts
- `async` - For asynchronous operations

## Implementation Notes

- All context operations are thread-safe
- Context values should only be used for request-scoped data
- Contexts form an immutable tree structure
- Cancellation propagates to all derived contexts
- Context lookup uses efficient parent chain traversal
- Vibe functionality is CURSED-specific extension

## Best Practices

1. Always call cancel functions to avoid resource leaks
2. Use context values for request-scoped data, not optional parameters
3. Check for cancellation in long-running operations
4. Use vibes to adapt behavior based on request characteristics
5. Keep context chains shallow for better performance
6. Use appropriate timeout values for different operations
