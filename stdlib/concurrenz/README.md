# CURSED Concurrency Module (concurrenz)

A comprehensive concurrency library for CURSED, equivalent to Go's `sync` package. Provides thread-safe synchronization primitives, goroutine coordination, and concurrent programming patterns.

## Features

### Core Synchronization Primitives
- **Mutex**: Mutual exclusion locks with try_lock support
- **RWMutex**: Reader-writer locks for concurrent read access
- **WaitGroup**: Coordinate multiple goroutines
- **Once**: Ensure single execution of initialization code
- **Atomic**: Atomic operations for lock-free programming
- **Condition Variables**: Wait and signal mechanisms
- **Semaphores**: Counting semaphores for resource management
- **Barriers**: Synchronization points for multiple goroutines

### Channel Patterns
- **Fan-out**: Distribute work across multiple goroutines
- **Fan-in**: Collect results from multiple goroutines
- **Pipeline**: Chain processing stages
- **Worker Pool**: Managed goroutine pools
- **Broadcast**: Send to multiple channels
- **Merge**: Combine multiple channels

### Advanced Features
- **Rate Limiting**: Token bucket rate limiter
- **Circuit Breaker**: Fault tolerance pattern
- **Timeout Channels**: Time-based operations
- **Select Patterns**: Non-blocking channel operations

## Usage Examples

### Basic Mutex Usage
```cursed
yeet "concurrenz"

sus m *Mutex = mutex_new()

# Lock and unlock
mutex_lock(m)
# Critical section
mutex_unlock(m)

# Try lock
sus m.locked = mutex_try_lock(m) {
    # Got lock
    mutex_unlock(m)
}
```

### WaitGroup for Goroutine Coordination
```cursed
yeet "concurrenz"

sus wg *WaitGroup = waitgroup_new()

# Spawn multiple goroutines
bestie i := 0; i < 5; i++ {
    waitgroup_add(wg, 1)
    yolo {
        defer waitgroup_done(wg)
        # Do work
        vibez.spill("Goroutine", i, "working")
    }()
}

# Wait for all goroutines to complete
waitgroup_wait(wg)
vibez.spill("All goroutines completed")
```

### Channel Communication
```cursed
yeet "concurrenz"

sus ch chan normie = make(chan normie, 5)

# Send data
ch <- 42
ch <- 100

# Receive data
sus val1 normie = <-ch
sus val2 normie = <-ch
```

### Atomic Operations
```cursed
yeet "concurrenz"

sus counter *Atomic = atomic_new(0)

# Atomic increment
sus new_value normie = atomic_add(counter, 1)

# Atomic compare and swap
sus swapped lit = atomic_compare_and_swap(counter, 1, 2)
```

### Worker Pool Pattern
```cursed
yeet "concurrenz"

sus jobs chan normie = make(chan normie, 100)
sus results chan normie = make(chan normie, 100)

# Create worker pool
worker_pool(jobs, results, slay(job normie) normie {
    damn job * 2  # Double the input
}, 5)  # 5 workers

# Send jobs
bestie i := 1; i <= 10; i++ {
    jobs <- i
}
close(jobs)

# Collect results
bestie result := <-results; result != cringe {
    vibez.spill("Result:", result)
}
```

### Rate Limiting
```cursed
yeet "concurrenz"

sus limiter *RateLimiter = rate_limiter_new(10, 1)  # 10 tokens, 1 per second

sus rate_limiter_allow(limiter) {
    # Request allowed
    perform_operation()
} else {
    # Request denied
    vibez.spill("Rate limited")
}
```

### Circuit Breaker
```cursed
yeet "concurrenz"

sus breaker *CircuitBreaker = circuit_breaker_new(5, 30000)  # 5 failures, 30s timeout

sus success lit = circuit_breaker_call(breaker, slay() lit {
    # Try operation
    damn perform_risky_operation()
})

sus success {
    vibez.spill("Operation succeeded")
} else {
    vibez.spill("Operation failed or circuit open")
}
```

### Pipeline Processing
```cursed
yeet "concurrenz"

sus input chan normie = make(chan normie, 10)
sus stage1 chan normie = make(chan normie, 10)
sus stage2 chan normie = make(chan normie, 10)
sus output chan normie = make(chan normie, 10)

# Create pipeline stages
pipeline_stage(input, stage1, slay(x normie) normie { damn x * 2 })
pipeline_stage(stage1, stage2, slay(x normie) normie { damn x + 1 })
pipeline_stage(stage2, output, slay(x normie) normie { damn x * 3 })

# Send data through pipeline
input <- 5
close(input)

# Get processed result
sus result normie = <-output  # ((5 * 2) + 1) * 3 = 33
```

### Fan-out/Fan-in Pattern
```cursed
yeet "concurrenz"

sus input chan normie = make(chan normie, 10)
sus workers []chan normie = fan_out(input, 3)  # 3 workers

# Process with workers
bestie i := 0; i < 3; i++ {
    yolo {
        bestie job := <-workers[i]; job != cringe {
            # Process job
            vibez.spill("Worker", i, "processing", job)
        }
    }()
}

# Merge results
sus merged chan normie = fan_in(workers)
```

### Semaphore for Resource Management
```cursed
yeet "concurrenz"

sus sem *Semaphore = semaphore_new(3)  # Max 3 concurrent operations

yolo {
    semaphore_acquire(sem)
    defer semaphore_release(sem)
    
    # Use limited resource
    perform_database_operation()
}()
```

### Condition Variables
```cursed
yeet "concurrenz"

sus mutex *Mutex = mutex_new()
sus cond *Cond = cond_new(mutex)
sus ready lit = cap

# Producer
yolo {
    mutex_lock(mutex)
    # Prepare data
    ready = based
    cond_signal(cond)
    mutex_unlock(mutex)
}()

# Consumer
mutex_lock(mutex)
bestie !ready {
    cond_wait(cond)
}
# Process data
mutex_unlock(mutex)
```

### Barrier Synchronization
```cursed
yeet "concurrenz"

sus barrier *Barrier = barrier_new(3)  # 3 participants

bestie i := 0; i < 3; i++ {
    yolo {
        # Do phase 1 work
        vibez.spill("Goroutine", i, "phase 1 complete")
        
        # Wait at barrier
        sus position normie = barrier_wait(barrier)
        
        # Do phase 2 work
        vibez.spill("Goroutine", i, "phase 2 starting")
    }()
}
```

## Implementation Details

### Architecture
- **Pure CURSED Implementation**: No FFI dependencies
- **Goroutine Integration**: Full support for CURSED goroutines (`yolo`)
- **Channel Integration**: Native channel operations (`<-`, `ready`)
- **Memory Safety**: Proper heap allocation and cleanup
- **Thread Safety**: All primitives are thread-safe

### Performance Characteristics
- **Mutex**: O(1) lock/unlock operations
- **RWMutex**: O(1) read lock, O(1) write lock
- **WaitGroup**: O(1) add/done operations
- **Atomic**: Lock-free operations where possible
- **Channels**: Efficient goroutine communication
- **Rate Limiter**: O(1) token bucket operations

### Error Handling
- Graceful handling of closed channels
- Proper resource cleanup with `defer`
- Deadlock prevention mechanisms
- Timeout support for blocking operations

## Testing

Run the comprehensive test suite:

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/concurrenz/test_concurrenz.csd

# Test native compilation
cargo run --bin cursed -- compile stdlib/concurrenz/test_concurrenz.csd
./test_concurrenz
```

## Best Practices

### Deadlock Prevention
- Always acquire locks in the same order
- Use timeouts for blocking operations
- Avoid holding locks across goroutine boundaries
- Use defer for cleanup

### Resource Management
- Always pair lock/unlock operations
- Use defer for guaranteed cleanup
- Close channels when done sending
- Limit goroutine creation with worker pools

### Performance Optimization
- Use RWMutex for read-heavy workloads
- Prefer atomic operations for simple counters
- Use buffered channels to reduce blocking
- Implement backpressure with semaphores

### Error Handling
- Check channel operations for closure
- Handle circuit breaker states appropriately
- Implement graceful shutdown patterns
- Use context for cancellation

## Patterns and Idioms

### Producer-Consumer
```cursed
sus buffer chan normie = make(chan normie, 10)
sus wg *WaitGroup = waitgroup_new()

# Producer
waitgroup_add(wg, 1)
yolo {
    defer waitgroup_done(wg)
    bestie i := 0; i < 100; i++ {
        buffer <- i
    }
    close(buffer)
}()

# Consumer
waitgroup_add(wg, 1)
yolo {
    defer waitgroup_done(wg)
    bestie item := <-buffer; item != cringe {
        process(item)
    }
}()

waitgroup_wait(wg)
```

### Request-Response
```cursed
struct Request {
    data normie,
    response chan normie
}

sus requests chan Request = make(chan Request, 10)

# Server
yolo {
    bestie req := <-requests; req != cringe {
        # Process request
        sus result normie = process(req.data)
        req.response <- result
    }
}()

# Client
sus response chan normie = make(chan normie, 1)
requests <- Request{42, response}
sus result normie = <-response
```

### Graceful Shutdown
```cursed
sus shutdown chan lit = make(chan lit, 1)
sus done chan lit = make(chan lit, 1)

# Worker
yolo {
    bestie {
        ready {
            <-shutdown:
                done <- based
                damn
            work := <-work_queue:
                process(work)
        }
    }
}()

# Shutdown signal
shutdown <- based
<-done
```

## License

This module is part of the CURSED programming language and follows the same license terms.
