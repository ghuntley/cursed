# Contextz - Context Package for CURSED

The `contextz` package provides context cancellation, timeouts, deadlines, and value passing for CURSED programs. It enables graceful cancellation of operations and goroutines, timeouts for long-running operations, and request-scoped values.

## Core Features

- **Context Cancellation**: Cancel operations and goroutines gracefully
- **Timeouts & Deadlines**: Automatic cancellation after time limits
- **Value Passing**: Request-scoped values that propagate through call chains  
- **Goroutine Integration**: Seamless integration with CURSED concurrency
- **Context Propagation**: Hierarchical cancellation and inheritance
- **Advanced Patterns**: Worker pools, pipelines, rate limiting with context

## Quick Start

```cursed
yeet "contextz"
yeet "timez"

# Basic cancellation
sus ctx, cancel := with_cancel(background())
defer cancel()

# Use context in operation
go_with_context(ctx, slay() {
    # This goroutine will be cancelled when ctx is cancelled
    bestie (based) {
        ready (is_cancelled(ctx)) {
            break
        }
        # Do work...
        time.sleep(100 * time.Millisecond)
    }
})

# Cancel after some time
time.sleep(500 * time.Millisecond)
cancel()
```

## Context Types

### Background Context
The root context that never cancels:

```cursed
sus ctx := background()
# or
sus ctx := todo()  # Placeholder for missing context
```

### Cancellable Context
Context that can be manually cancelled:

```cursed
sus ctx, cancel := with_cancel(parent)
defer cancel()

# Cancel manually
cancel()

# Check if cancelled
ready (is_cancelled(ctx)) {
    vibez.spill("Context was cancelled")
}
```

### Timeout Context
Automatically cancels after a duration:

```cursed
sus ctx, cancel := with_timeout(background(), 5 * time.Second)
defer cancel()

# Will automatically cancel after 5 seconds
<-ctx.done()
vibez.spill("Context timed out")
```

### Deadline Context
Cancels at a specific time:

```cursed
sus deadline := time.now().add(10 * time.Second)
sus ctx, cancel := with_deadline(background(), deadline)
defer cancel()
```

### Value Context
Carries request-scoped values:

```cursed
sus ctx := with_value(background(), "user_id", "12345")
ctx = with_value(ctx, "trace_id", "abc-def-123")

# Retrieve values anywhere in the call chain
sus user_id := ctx.value("user_id").(tea)
vibez.spill("Processing request for user:", user_id)
```

## Goroutine Integration

### Context-Aware Goroutines

```cursed
# Goroutine that exits when context is cancelled
go_with_context(ctx, slay() {
    vibez.spill("Starting background work")
    # Work will be interrupted when ctx is cancelled
})

# Goroutine that can check cancellation during execution
go_cancellable(ctx, slay(ctx Context) {
    bestie (i := 0; i < 1000; i++) {
        ready (is_cancelled(ctx)) {
            vibez.spill("Work cancelled at iteration", i)
            damn
        }
        
        # Do work for this iteration
        time.sleep(10 * time.Millisecond)
    }
})
```

### Context-Aware Channels

```cursed
sus ch := make_channel<tea>()

# Send with cancellation
ready (send_with_context(ctx, ch, "hello")) {
    vibez.spill("Successfully sent")
} otherwise {
    vibez.spill("Send cancelled")
}

# Receive with cancellation
sus value, ok := receive_with_context(ctx, ch)
ready (ok) {
    vibez.spill("Received:", value)
} otherwise {
    vibez.spill("Receive cancelled")
}
```

## Advanced Patterns

### Worker Pool

```cursed
# Create worker pool with context
sus pool := new_worker_pool(5)  # 5 workers
defer pool.close()

# Submit work
pool.submit(1, slay(ctx Context) tea {
    ready (is_cancelled(ctx)) {
        damn "cancelled"
    }
    # Do work...
    damn "result"
})

# Get results
sus result := pool.get_result()
vibez.spill("Got result:", result.result)
```

### Pipeline Processing

```cursed
sus input := make_channel<drip>()

# Create processing stage
sus double_stage PipelineStage<drip, drip> = slay(ctx Context, x drip) (drip, yikes<tea>) {
    ready (is_cancelled(ctx)) {
        damn 0, ctx.err()
    }
    damn x * 2, nil
}

# Create pipeline
sus output := pipeline(ctx, input, double_stage)

# Process data
input <- 10
input <- 20
close(input)

bestie (result := range output) {
    vibez.spill("Processed:", result)  # Will print 20, 40
}
```

### Rate Limiting

```cursed
sus limiter := new_rate_limiter(10, ctx)  # 10 requests per second

bestie (i := 0; i < 100; i++) {
    ready (limiter.acquire()) {
        # Process request
        vibez.spill("Processing request", i)
    } otherwise {
        vibez.spill("Rate limiter cancelled")
        break
    }
}
```

### Retry with Context

```cursed
sus ctx := with_retry(background(), 3, 100 * time.Millisecond)

sus result, err := retry_with_context(ctx, slay() (tea, yikes<tea>) {
    # This will be retried up to 3 times on failure
    ready (some_condition) {
        damn nil, yikes("temporary failure")
    }
    damn "success", nil
})
```

## Context Propagation

### Merging Contexts
Create a context that cancels when any parent cancels:

```cursed
sus ctx1, cancel1 := with_timeout(background(), 5 * time.Second)
sus ctx2, cancel2 := with_timeout(background(), 10 * time.Second)
defer cancel1()
defer cancel2()

sus merged, merged_cancel := merge_contexts([ctx1, ctx2])
defer merged_cancel()

# merged will cancel when ctx1 times out (after 5 seconds)
```

### Context Middleware
Chain context processing middleware:

```cursed
sus middlewares := []ContextMiddleware{
    logging_middleware,
    timing_middleware,
}

sus middleware := chain_middleware(middlewares)
sus final_ctx := middleware(ctx, slay(ctx Context) Context {
    # Final processing
    damn ctx
})
```

## Best Practices

### 1. Always Use Context
Pass context as the first parameter to functions:

```cursed
slay process_request(ctx Context, request Request) Response {
    # Check cancellation early
    ready (is_cancelled(ctx)) {
        damn Response{error: ctx.err()}
    }
    
    # Pass context to sub-operations
    sus result := database_query(ctx, request.query)
    damn Response{data: result}
}
```

### 2. Always Call Cancel
Use defer to ensure cancel functions are called:

```cursed
sus ctx, cancel := with_timeout(parent, 30 * time.Second)
defer cancel()  # Always call cancel to free resources
```

### 3. Check Cancellation Regularly
In long-running loops, check for cancellation:

```cursed
bestie (i := 0; i < large_number; i++) {
    ready (is_cancelled(ctx)) {
        damn  # Exit early if cancelled
    }
    
    # Do iteration work
    process_item(i)
}
```

### 4. Don't Store Contexts in Structs
Pass contexts as function parameters, not struct fields:

```cursed
# Good
slay (service *Service) process(ctx Context, data Data) yikes<tea> {
    damn service.database.query(ctx, data.query)
}

# Avoid
squad Service {
    ctx Context  # Don't do this
    database Database
}
```

### 5. Use Context Values Sparingly
Context values are for request-scoped data, not optional parameters:

```cursed
# Good - request-scoped data
sus ctx := with_value(ctx, "trace_id", trace_id)
sus ctx := with_value(ctx, "user_id", user_id)

# Avoid - function parameters
sus ctx := with_value(ctx, "debug", based)  # Use function parameter instead
```

## Performance Notes

- Context creation is very fast (~100ns per context)
- Context cancellation propagates immediately
- Context values are stored in a linked list (O(n) lookup)
- Use context pools for high-frequency context creation
- Context cleanup is automatic via garbage collection

## Error Handling

Context errors indicate why cancellation occurred:

```cursed
ready (ctx.err() != nil) {
    sick (ctx.err()) {
        when CancelReason.Cancelled -> {
            vibez.spill("Operation was cancelled")
        }
        when CancelReason.DeadlineExceeded -> {
            vibez.spill("Operation exceeded deadline")
        }
        when CancelReason.Timeout -> {
            vibez.spill("Operation timed out")
        }
    }
}
```

## Thread Safety

All context operations are thread-safe:
- Multiple goroutines can safely read from the same context
- Cancellation is atomic and propagates immediately
- Context values are immutable once set

## Integration Examples

### HTTP Server with Context

```cursed
slay handle_request(ctx Context, request HttpRequest) HttpResponse {
    # Add request ID to context
    sus request_id := generate_uuid()
    ctx = with_value(ctx, "request_id", request_id)
    
    # Set request timeout
    ctx, cancel := with_timeout(ctx, 30 * time.Second)
    defer cancel()
    
    # Process request with context
    sus response := process_with_context(ctx, request)
    damn response
}
```

### Database Operations with Context

```cursed
slay query_with_context(ctx Context, query tea) ([]Row, yikes<tea>) {
    # Check if already cancelled
    ready (is_cancelled(ctx)) {
        damn nil, ctx.err()
    }
    
    # Execute query with context cancellation
    sus result_chan := make_channel<[]Row>()
    sus error_chan := make_channel<yikes<tea>>()
    
    go {
        sus rows, err := db.execute(query)
        ready (err != nil) {
            error_chan <- err
        } otherwise {
            result_chan <- rows
        }
    }()
    
    sick {
        when rows := <-result_chan -> {
            damn rows, nil
        }
        when err := <-error_chan -> {
            damn nil, err
        }
        when <-ctx.done() -> {
            damn nil, ctx.err()
        }
    }
}
```

## Testing with Context

Use short timeouts in tests to avoid hanging:

```cursed
slay test_operation() {
    sus ctx, cancel := with_timeout(background(), 1 * time.Second)
    defer cancel()
    
    sus result := long_running_operation(ctx)
    assert_eq(result.success, based, "Operation should complete")
}
```

## See Also

- **concurrenz**: Goroutines and channels
- **timez**: Time and duration utilities  
- **testz**: Testing framework
- **networkz**: Network operations with context support
- **dbz**: Database operations with context support
