# VibeContext (context package)

## Overview
VibeContext implements request-scoped contexts for passing deadlines, cancellation signals, and other values across API boundaries. It's inspired by Go's context package but with a vibe-focused twist.

## Core Types

### `VibeCtx` Interface
The main interface that all context types must implement.

```go
interface VibeCtx {
    Deadline() (deadline time.Time, ok bool)
    Done() <-chan struct{}
    Err() error
    Value(key interface{}) interface{}
    CheckVibe() string // Additional method that returns the context "vibe"
}
```

### `BackgroundVibe`
Creates an empty context that never cancels (equivalent to context.Background).

```go
func BackgroundVibe() VibeCtx
```

### `EmptyVibe`
Creates an empty context that never cancels but differs from BackgroundVibe for testing (equivalent to context.TODO).

```go
func EmptyVibe() VibeCtx
```

## Context Modifiers

### `WithTimeout`
Creates a context with a timeout.

```go
func WithTimeout(parent VibeCtx, timeout time.Duration) (VibeCtx, CancelFunc)
```

### `WithDeadline`
Creates a context with a deadline.

```go
func WithDeadline(parent VibeCtx, deadline time.Time) (VibeCtx, CancelFunc)
```

### `WithCancel`
Creates a cancellable context.

```go
func WithCancel(parent VibeCtx) (VibeCtx, CancelFunc)
```

### `WithValue`
Creates a context with a key-value pair.

```go
func WithValue(parent VibeCtx, key, val interface{}) VibeCtx
```

### `WithVibe`
Adds a string "vibe" to the context (unique to Cursed).

```go
func WithVibe(parent VibeCtx, vibe string) VibeCtx
```

## Types

### `CancelFunc`
A function that cancels a context.

```go
type CancelFunc func()
```

## Usage Example

```go
func HandleRequest(ctx VibeCtx, req *Request) {
    // Create a context that cancels after 100ms
    ctx, cancel := WithTimeout(ctx, 100 * time.Millisecond)
    defer cancel() // Always call cancel to release resources
    
    // Add a vibe to the context
    ctx = WithVibe(ctx, "chill")
    
    // Process the request
    processRequest(ctx, req)
}

func processRequest(ctx VibeCtx, req *Request) {
    // Check if context is cancelled
    select {
    case <-ctx.Done():
        vibez.spill("Request cancelled, vibe: " + ctx.CheckVibe())
        return
    default:
        // Continue processing
    }
    
    // Use context values
    userID := ctx.Value("userID")
    
    // Check vibe
    if ctx.CheckVibe() == "chill" {
        // Handle in a relaxed way
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