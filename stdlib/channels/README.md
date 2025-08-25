# Channels Module

## Why This Module Exists

The `channels` module provides advanced channel operations and patterns beyond basic send/receive functionality found in the core `concurrenz` module. While basic channels enable simple message passing, real-world concurrent applications need sophisticated coordination patterns like select operations, buffered channels, priority queues, and timeout handling.

The module exists because:
- **Complex Coordination**: Production systems need patterns like fan-out, fan-in, worker pools, and pipeline processing
- **Performance Optimization**: Buffered channels and batching reduce context switching overhead in high-throughput systems
- **Deadlock Prevention**: Select operations and timeout mechanisms prevent common concurrency pitfalls
- **Enterprise Integration**: Production systems need monitoring, backpressure, and graceful degradation

## Why Testing Is Critical

Channel testing is absolutely essential because:
- **Race Conditions**: Channel operations are inherently concurrent and race conditions can cause data corruption or system deadlocks
- **Deadlock Detection**: Improper channel usage can create deadlocks that only appear under specific timing conditions
- **Memory Leaks**: Unbuffered channels can hold references to data indefinitely if receivers are blocked
- **Performance Under Load**: Channel implementations must maintain performance characteristics under high concurrency
- **Ordering Guarantees**: Some patterns require specific message ordering that must be verified under stress

## Implementation Rationale

### Key Design Decisions:

**1. Layered Architecture**
- Core channel primitives in `concurrenz` module provide basic send/receive
- This module builds advanced patterns on top of primitives
- Clear separation allows optimization of core operations while maintaining pattern flexibility

**2. Zero-Copy Message Passing**
- Channel operations transfer ownership rather than copying data
- Eliminates serialization overhead for complex data structures
- Maintains type safety through CURSED's ownership system

**3. Backpressure Management**
- Buffered channels provide explicit capacity limits
- Slow consumer detection prevents memory exhaustion
- Circuit breaker patterns for automatic system protection

**4. Select Operation Efficiency**
- Non-blocking channel operations prevent deadlocks
- Efficient polling implementation avoids busy-waiting
- Priority-based selection for quality-of-service guarantees

## API Reference

### Buffered Channel Operations

#### `buffered_channel<T>(capacity: drip) BufferedChannel<T>`
**Purpose**: Creates channel with internal buffer for asynchronous communication
**Buffer Behavior**: Senders block only when buffer is full
**Use Case**: Decoupling producer and consumer speeds

```cursed
sus buffer_chan = channels.buffered_channel<tea>(100)
channels.send(buffer_chan, "message1")  # Non-blocking if buffer has space
channels.send(buffer_chan, "message2")
sus msg tea = channels.receive(buffer_chan)
```

#### `send_timeout<T>(channel: Channel<T>, value: T, timeout_ms: drip) bool`
**Purpose**: Send with timeout to prevent indefinite blocking
**Returns**: `based` if sent successfully, `cap` if timeout occurred

#### `receive_timeout<T>(channel: Channel<T>, timeout_ms: drip) ?T`
**Purpose**: Receive with timeout to prevent indefinite blocking  
**Returns**: Message if received, `fam` if timeout occurred

### Select Operations

#### `select_channels<T>(operations: []SelectOperation<T>) SelectResult<T>`
**Purpose**: Wait on multiple channel operations simultaneously
**Behavior**: Returns as soon as any operation can proceed
**Use Case**: Implementing timeouts, multiplexing, coordination patterns

```cursed
sus chan1 = make_channel<drip>()
sus chan2 = make_channel<tea>()

sus operations = [
    channels.SelectSend{channel: chan1, value: 42},
    channels.SelectReceive{channel: chan2},
    channels.SelectTimeout{timeout_ms: 1000}
]

sus result = channels.select_channels(operations)
ready (result.type) {
    when SelectSend -> vibez.spill("Sent to chan1")
    when SelectReceive -> vibez.spill("Received:", result.value)
    when SelectTimeout -> vibez.spill("Operation timed out")
}
```

### Channel Patterns

#### `fan_out<T>(input: Channel<T>, outputs: []Channel<T>)`
**Purpose**: Distribute messages from one input to multiple outputs
**Behavior**: Round-robin distribution or broadcast mode
**Use Case**: Load balancing, parallel processing

#### `fan_in<T>(inputs: []Channel<T>, output: Channel<T>)`
**Purpose**: Merge messages from multiple inputs into single output
**Behavior**: Fair scheduling across inputs
**Use Case**: Aggregating results, multiplexing

#### `pipeline<T, U>(input: Channel<T>, transform: slay(T) U, parallelism: drip) Channel<U>`
**Purpose**: Process messages through transformation pipeline
**Parallelism**: Number of concurrent transform operations
**Use Case**: Stream processing, ETL operations

### Priority Channels

#### `priority_channel<T>(levels: drip) PriorityChannel<T>`
**Purpose**: Channel with multiple priority levels for message importance
**Behavior**: Higher priority messages delivered before lower priority
**Use Case**: QoS-aware systems, interrupt handling

```cursed
sus priority_chan = channels.priority_channel<tea>(3)
channels.send_priority(priority_chan, "low", 0)
channels.send_priority(priority_chan, "urgent", 2)
channels.send_priority(priority_chan, "normal", 1)

# Receives in order: "urgent", "normal", "low"
```

## Usage Examples

### Basic Buffered Channel
```cursed
yeet "channels"
yeet "concurrenz"

# Producer-consumer with buffering
sus work_queue = channels.buffered_channel<drip>(50)

# Producer goroutine
go {
    bestie (i drip = 0; i < 100; i++) {
        channels.send(work_queue, i)
    }
    channels.close(work_queue)
}

# Consumer goroutine
go {
    bestie (work_item := channels.receive(work_queue)) {
        vibez.spill("Processing:", work_item)
        # Process work...
    }
}
```

### Select with Timeout
```cursed
# Request with timeout pattern
slay fetch_with_timeout(url tea, timeout_ms drip) tea {
    sus result_chan = make_channel<tea>()
    
    go {
        sus data = networkz.get(url)
        channels.send_timeout(result_chan, data, timeout_ms)
    }
    
    sus operations = [
        channels.SelectReceive{channel: result_chan},
        channels.SelectTimeout{timeout_ms: timeout_ms}
    ]
    
    sus select_result = channels.select_channels(operations)
    ready (select_result.type) {
        when SelectReceive -> damn select_result.value
        when SelectTimeout -> damn "Request timeout"
    }
}
```

### Worker Pool Pattern
```cursed
# Scalable worker pool with job distribution
slay worker_pool<Job, Result>(
    jobs: Channel<Job>, 
    results: Channel<Result>, 
    worker_count: drip,
    process_job: slay(Job) Result
) {
    # Start workers
    bestie (i drip = 0; i < worker_count; i++) {
        go {
            bestie (job := channels.receive(jobs)) {
                sus result = process_job(job)
                channels.send(results, result)
            }
        }
    }
}

# Usage
sus job_queue = channels.buffered_channel<tea>(100)
sus result_queue = channels.buffered_channel<drip>(100)

worker_pool(job_queue, result_queue, 5, slay(url tea) drip {
    damn networkz.get(url).len()
})
```

### Fan-Out Load Balancing
```cursed
# Distribute work across multiple processors
sus input_chan = make_channel<WorkItem>()
sus processor_chans = [
    make_channel<WorkItem>(),
    make_channel<WorkItem>(),
    make_channel<WorkItem>(),
]

# Start fan-out distribution
channels.fan_out(input_chan, processor_chans)

# Start processors
bestie (processor_chan in processor_chans) {
    go {
        bestie (item := channels.receive(processor_chan)) {
            # Process item...
        }
    }
}
```

## Performance Considerations

### Channel Buffer Sizing

**Small Buffers (1-10)**:
- Use for coordination and signaling
- Minimal memory overhead
- Tight coupling between producer/consumer

**Medium Buffers (10-1000)**:
- Use for typical producer/consumer scenarios  
- Balance between memory usage and throughput
- Handle temporary speed differences

**Large Buffers (1000+)**:
- Use for batch processing systems
- High throughput, loose coupling
- Monitor memory usage carefully

### Select Operation Optimization

1. **Order Operations by Likelihood**: Place most likely operations first in select arrays
2. **Avoid Timeout Churn**: Use reasonable timeout values to prevent excessive wake-ups
3. **Batch Operations**: Group related select operations when possible
4. **Profile Channel Contention**: Monitor channel usage patterns in production

### Memory Management

- **Channel Lifetime**: Explicitly close channels when producers are done
- **Buffer Monitoring**: Track buffer utilization to detect backpressure
- **Goroutine Cleanup**: Ensure receiving goroutines can exit gracefully
- **Arena Integration**: Use arena allocators for temporary channel data

## Security Considerations

### Channel Capacity Attacks

**Threat**: Malicious senders can exhaust memory by flooding unbounded channels
**Mitigation**: Always use bounded channels with appropriate capacity limits

```cursed
# Vulnerable - unbounded growth
sus dangerous_chan = make_channel<tea>()  # No buffer limit

# Secure - bounded capacity
sus safe_chan = channels.buffered_channel<tea>(1000)  # Clear limit
```

### Deadlock Prevention

**Threat**: Circular dependencies between channels can cause system deadlock
**Mitigation**: Use select operations with timeouts and establish clear channel dependency hierarchies

```cursed
# Potential deadlock
channels.send(chan_a, data)
channels.send(chan_b, data)  # Could block if chan_b is full

# Deadlock-safe alternative
sus sent_a = channels.send_timeout(chan_a, data, 100)
ready (sent_a) {
    channels.send_timeout(chan_b, data, 100)
}
```

### Information Leakage

**Threat**: Channel timing can leak information about system state
**Mitigation**: Use constant-time operations for security-critical applications

### Resource Exhaustion

**Threat**: Unbounded goroutine creation through channel operations
**Mitigation**: Use worker pools with fixed goroutine counts

## Error Handling Patterns

### Channel Error Recovery
```cursed
slay robust_send<T>(channel: Channel<T>, value: T) yikes<tea> {
    sus max_retries = 3
    bestie (attempt drip = 0; attempt < max_retries; attempt++) {
        sus success = channels.send_timeout(channel, value, 1000)
        ready (success) {
            damn fam
        }
        timez.sleep(100 * attempt)  # Exponential backoff
    }
    yikes "Failed to send after retries"
}
```

### Channel Health Monitoring
```cursed
slay monitor_channel<T>(channel: BufferedChannel<T>) {
    go {
        bestie (based) {
            sus stats = channels.get_stats(channel)
            ready (stats.buffer_utilization > 0.9) {
                vibez.spill("WARNING: Channel buffer 90% full")
            }
            timez.sleep(1000)  # Check every second
        }
    }
}
```

## Thread Safety and Concurrency

All channel operations are inherently thread-safe and designed for concurrent access. Multiple goroutines can safely send and receive on the same channel simultaneously. However:

1. **Channel Creation**: Not thread-safe, create channels before spawning goroutines
2. **Channel Closing**: Only the sender should close channels
3. **Select Operations**: Thread-safe but each select call is atomic
4. **Buffer Stats**: Stats collection is eventually consistent, not strongly consistent

The module integrates seamlessly with CURSED's goroutine scheduler and provides optimal performance for concurrent channel operations.
