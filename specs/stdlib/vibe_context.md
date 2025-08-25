# VibeContext (context package)

## Overview
VibeContext implements request-scoped contexts for passing deadlines, cancellation signals, and other values across API boundaries. It's inspired by Go's context package but with a vibe-focused twist.

## Core Types

### `VibeCtx` Interface
The main collab that all context types must implement.

```
collab VibeCtx {
    Deadline() (deadline time.Time, ok lit)
    Done() dm_recv(ch)an squad{}
    Err() tea
    Value(key interface{}) interface{}
    CheckVibe() tea fr fr Additional method that yolos the context "vibe"
}
```

### `BackgroundVibe`
Creates an empty context that never cancels (equivalent to context.Background).

```
slay BackgroundVibe() VibeCtx
```

### `EmptyVibe`
Creates an empty context that never cancels but differs from BackgroundVibe for testing (equivalent to context.TODO).

```
slay EmptyVibe() VibeCtx
```

## Context Modifiers

### `WithTimeout`
Creates a context with a timeout.

```
slay WithTimeout(parent VibeCtx, timeout time.Duration) (VibeCtx, CancelFunc)
```

### `WithDeadline`
Creates a context with a deadline.

```
slay WithDeadline(parent VibeCtx, deadline time.Time) (VibeCtx, CancelFunc)
```

### `WithCancel`
Creates a cancellable context.

```
slay WithCancel(parent VibeCtx) (VibeCtx, CancelFunc)
```

### `WithValue`
Creates a context with a key-value pair.

```
slay WithValue(parent VibeCtx, key, val interface{}) VibeCtx
```

### `WithVibe`
Adds a tea "vibe" to the context (unique to Cursed).

```
slay WithVibe(parent VibeCtx, vibe tea) VibeCtx
```

## Types

### `CancelFunc`
A function that cancels a context.

```
be_like CancelFunc func()
```

## Usage Example

```
slay HandleRequest(ctx VibeCtx, req *Request) {
    fr fr Create a context that cancels after 100ms
    ctx, cancel := WithTimeout(ctx, 100 * time.Millisecond)
    defer cancel() fr fr Always call cancel to release resources
    
    fr fr Add a vibe to the context
    ctx = WithVibe(ctx, "chill")
    
    fr fr Process the request
    processRequest(ctx, req)
}

slay processRequest(ctx VibeCtx, req *Request) {
    fr fr Check if context is cancelled
    select {
    case <-ctx.Done():
        vibez.spill("Request cancelled, vibe: " + ctx.CheckVibe())
        yolo
    default:
        fr fr Continue processing
    }
    
    fr fr Use context values
    userID := ctx.Value("userID")
    
    fr fr Check vibe
    if ctx.CheckVibe() == "chill" {
        fr fr Handle in a relaxed way
    }
}
```

## Integration with Standard Library
VibeContext is designed to integrate smoothly with other packages like `web_vibez` and `concurrenz`.

## Implementation Guidelines
1. Context implementations should be lightweight and efficient
2. Cancellation should propagate immediately to all derived contexts
3. Value lookups should be fast, using immutable trees for efficient lookup
4. All context operations must be thread-safe
5. Context values should only be used for request-scoped data, not for passing optional parameters