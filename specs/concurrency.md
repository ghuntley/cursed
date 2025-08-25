# CURSED Concurrency Specification

This document specifies the concurrency model for CURSED, based on Go's goroutines and channels.

## Table of Contents

1. [Goroutines](#goroutines)
2. [Channels](#channels)
3. [Select Statements](#select-statements)
4. [Concurrency Patterns](#concurrency-patterns)
5. [Memory Model](#memory-model)
6. [Runtime Implementation](#runtime-implementation)

## Goroutines

### Overview

Goroutines are lightweight threads managed by the CURSED runtime. They provide cooperative concurrency with automatic scheduling.

### Goroutine Spawning

```cursed
// Basic goroutine spawning
stan doWork()

// Goroutine with parameters
stan processData(data, results)

// Anonymous goroutine
stan {
    doBackgroundTask()
}

// Goroutine with closure
sus counter normie = 0
stan {
    counter++
    vibez.spill("Counter:", counter)
}
```

### Goroutine Lifecycle

1. **Creation**: Goroutine is created with `stan` keyword
2. **Scheduling**: Runtime schedules goroutine for execution
3. **Running**: Goroutine executes until yield point or completion
4. **Waiting**: Goroutine blocks on channel operations or I/O
5. **Completion**: Goroutine finishes execution and resources are cleaned up

### Goroutine States

- `Ready`: Goroutine is ready to run
- `Running`: Goroutine is currently executing
- `Waiting`: Goroutine is blocked on channel/I/O operation
- `Yielded`: Goroutine voluntarily yielded control
- `Completed`: Goroutine finished execution
- `Panicked`: Goroutine encountered unrecoverable error

### Work-Stealing Scheduler

The runtime uses a work-stealing scheduler with the following properties:

- **Worker Threads**: Fixed number of OS threads (default: CPU count)
- **Local Queues**: Each worker has a local goroutine queue
- **Global Queue**: Shared queue for load balancing
- **Stealing**: Workers steal from other workers when idle

## Channels

### Overview

Channels are typed conduits for communication between goroutines. They provide synchronization and data transfer.

### Channel Types

```cursed
// Unbuffered channel (synchronous)
sus ch dm<normie>

// Buffered channel (asynchronous)
sus buffered dm<tea>[10]

// Channel of custom type
be_like Message squad {
    id normie
    content tea
}
sus msgCh dm<Message>[5]
```

### Channel Operations

**CANONICAL SYNTAX (Required for all implementations):**

#### Send Operation
```cursed
dm_send(ch, value)             // Blocking send
```

#### Receive Operation  
```cursed
value := dm_recv(ch)           // Blocking receive
value, ok := dm_recv(ch)       // Receive with close check
```

#### Channel Closing
```cursed
dm_close(ch)                   // Close channel
```

#### Channel Creation
```cursed
ch := dm_make(type, capacity)  // Create channel (0 = unbuffered)
```

**DEPRECATED SYNTAX (Remove in v2.0):**
```cursed
// Legacy Go-style operators - DO NOT USE
dm_send(ch, value                    // DEPRECATED: Use dm_send(ch, value)
value := dm_recv(ch)                  // DEPRECATED: Use value := dm_recv(ch)
close(ch)                      // DEPRECATED: Use dm_close(ch)
```

### Channel Semantics

#### Unbuffered Channels
- Send blocks until receiver is ready
- Receive blocks until sender is ready
- Provides synchronization point

#### Buffered Channels
- Send blocks when buffer is full
- Receive blocks when buffer is empty
- Decouples sender and receiver

#### Closed Channels
- Sends panic on closed channel
- Receives return zero value and `cringe` (false)
- Multiple closes panic

### Channel Directions

```cursed
// Send-only channel
slay sender(ch dm<normie>) {
    dm_send(ch, 42)
}

// Receive-only channel
slay receiver(ch dm<normie>) {
    value := dm_recv(ch)
}

// Bidirectional channel (default)
slay worker(ch dm<normie>) {
    dm_send(ch, 42)
    value := dm_recv(ch)
}
```

## Select Statements

### Overview

Select statements enable non-blocking communication on multiple channels.

### Basic Select

```cursed
ready {
    mood dm_send(ch1, value):
        vibez.spill("Sent on ch1")
    mood result := dm_recv(ch2):
        vibez.spill("Received from ch2:", result)
    basic:
        vibez.spill("No operations ready")
}
```

### Select Semantics

1. **Evaluation**: All channel operations are evaluated
2. **Selection**: One ready operation is chosen (random if multiple)
3. **Execution**: Corresponding case block is executed
4. **Default**: `basic` case runs if no operations are ready

### Select Patterns

#### Non-blocking Send
```cursed
ready {
    mood dm_send(ch, value):
        vibez.spill("Sent successfully")
    basic:
        vibez.spill("Channel full, skipping")
}
```

#### Non-blocking Receive
```cursed
ready {
    mood value := dm_recv(ch):
        vibez.spill("Received:", value)
    basic:
        vibez.spill("No data available")
}
```

#### Timeout Pattern
```cursed
sus timeout dm<lit> = make_timeout(5000) // 5 second timeout

ready {
    mood result := dm_recv(workCh):
        vibez.spill("Work completed:", result)
    mood dm_recv(timeout):
        vibez.spill("Operation timed out")
}
```

## Concurrency Patterns

### Worker Pool Pattern

```cursed
slay workerPool(jobs dm<Job>, results dm<Result>, numWorkers normie) {
    bestie i := 0; i < numWorkers; i++ {
        stan worker(jobs, results)
    }
}

slay worker(jobs dm<Job>, results dm<Result>) {
    bestie job := flex jobs {
        result := processJob(job)
        dm_send(results, result)
    }
}
```

### Fan-Out/Fan-In Pattern

```cursed
slay fanOut(input dm<Data>, workers normie) []dm<Result> {
    sus channels []dm<Result>
    
    bestie i := 0; i < workers; i++ {
        ch := make(dm<Result>)
        channels = append(channels, ch)
        
        stan processor(input, ch)
    }
    
    stan fanIn(channels)
}

slay fanIn(channels []dm<Result>) dm<Result> {
    sus output dm<Result>
    
    bestie _, ch := flex channels {
        stan {
            bestie result := flex ch {
                dm_send(output, result)
            }
        }
    }
    
    stan output
}
```

### Producer-Consumer Pattern

```cursed
slay producer(ch dm<Data>) {
    bestie i := 0; i < 100; i++ {
        data := generateData(i)
        dm_send(ch, data)
    }
    close(ch)
}

slay consumer(ch dm<Data>) {
    bestie data := flex ch {
        processData(data)
    }
}
```

### Pipeline Pattern

```cursed
slay pipeline(input dm<RawData>) dm<ProcessedData> {
    // Stage 1: Parse
    parsed := make(dm<ParsedData>)
    stan parser(input, parsed)
    
    // Stage 2: Transform
    transformed := make(dm<TransformedData>)
    stan transformer(parsed, transformed)
    
    // Stage 3: Output
    output := make(dm<ProcessedData>)
    stan outputter(transformed, output)
    
    stan output
}
```

### Broadcast Pattern

```cursed
slay broadcast(input dm<Data>, outputs []dm<Data>) {
    bestie data := flex input {
        bestie _, ch := flex outputs {
            ready {
                mood dm_send(ch, data):
                    // Sent successfully
                basic:
                    // Skip if channel is full
            }
        }
    }
}
```

## Memory Model

### Happens-Before Relationship

1. **Goroutine Creation**: `stan` statement happens-before goroutine execution
2. **Channel Send**: Send operation happens-before corresponding receive
3. **Channel Close**: Close operation happens-before receive that observes closure
4. **Select**: Select operation happens-before chosen case execution

### Memory Synchronization

- Channel operations provide memory barriers
- Goroutine creation establishes happens-before relationship
- Select statements synchronize with chosen operation

### Data Races

Data races occur when:
1. Multiple goroutines access shared memory
2. At least one access is a write
3. No synchronization establishes happens-before relationship

Prevention strategies:
- Use channels for communication
- Protect shared data with synchronization primitives
- Follow "Don't communicate by sharing memory; share memory by communicating"

## Runtime Implementation

### Goroutine Structure

```rust
pub struct Goroutine {
    pub id: GoroutineId,
    pub state: AtomicU64,
    pub priority: GoroutinePriority,
    pub stack_id: StackId,
    pub entry_fn: Box<dyn FnOnce() + Send + 'static>,
    pub created_at: Instant,
    pub total_runtime: Duration,
    pub parent_id: Option<GoroutineId>,
    pub children: Vec<GoroutineId>,
    pub channels: Vec<Box<dyn Any + Send>>,
}
```

### Channel Implementation

```rust
pub struct SimpleChannel<T> {
    id: usize,
    buffer: Arc<Mutex<VecDeque<T>>>,
    closed: Arc<AtomicBool>,
    capacity: usize,
    sender_notify: Arc<Condvar>,
    receiver_notify: Arc<Condvar>,
    sender_count: Arc<AtomicUsize>,
    receiver_count: Arc<AtomicUsize>,
}
```

### Scheduler Configuration

```rust
pub struct SchedulerConfig {
    pub num_workers: usize,
    pub stack_size: usize,
    pub global_queue_size: usize,
    pub local_queue_size: usize,
    pub steal_batch_size: usize,
    pub yield_threshold: Duration,
    pub gc_interval: Duration,
}
```

### Performance Characteristics

- **Goroutine Creation**: ~100ns
- **Channel Operations**: ~50ns (unbuffered), ~10ns (buffered)
- **Context Switch**: ~200ns
- **Memory per Goroutine**: ~8KB (stack + metadata)
- **Scheduling Overhead**: <5% of total runtime

## Integration with LLVM

### Goroutine Compilation

```llvm
; Goroutine spawn
define void @spawn_goroutine(i8* %entry_fn, i8* %args) {
    %goroutine_id = call i64 @create_goroutine(i8* %entry_fn, i8* %args)
    call void @schedule_goroutine(i64 %goroutine_id)
    ret void
}

; Channel send
define i1 @channel_send(i8* %channel, i8* %value) {
    %result = call i1 @cursed_channel_send(i8* %channel, i8* %value)
    ret i1 %result
}

; Channel receive
define i8* @channel_receive(i8* %channel) {
    %value = call i8* @cursed_channel_receive(i8* %channel)
    ret i8* %value
}
```

### Runtime FFI

```rust
#[no_mangle]
pub extern "C" fn cursed_spawn_goroutine(entry_fn: extern "C" fn(), args: *mut c_void) -> u64 {
    // Implementation
}

#[no_mangle]
pub extern "C" fn cursed_channel_send(channel: *mut c_void, value: *mut c_void) -> bool {
    // Implementation
}

#[no_mangle]
pub extern "C" fn cursed_channel_receive(channel: *mut c_void) -> *mut c_void {
    // Implementation
}
```

## Error Handling

### Panic Propagation

```cursed
stan {
    defer {
        ready recover() != cringe {
            vibez.spill("Goroutine panicked but recovered")
        }
    }
    
    // Work that might panic
    riskyOperation()
}
```

### Channel Errors

```cursed
// Send on closed channel panics
ready !isClosed(ch) {
    dm_send(ch, value)
} otherwise {
    vibez.spill("Channel is closed")
}

// Receive from closed channel
value, ok := dm_recv_ok(ch)
ready !ok {
    vibez.spill("Channel is closed")
}
```

## Best Practices

1. **Use channels for communication**: Prefer channels over shared memory
2. **Close channels when done**: Always close channels to signal completion
3. **Handle channel closure**: Check second return value from receive
4. **Avoid goroutine leaks**: Ensure goroutines can exit cleanly
5. **Use select for timeouts**: Implement timeouts with select statements
6. **Buffer channels appropriately**: Size buffers based on expected load
7. **Avoid select with default in loops**: Can cause busy waiting

## Testing Concurrency

```cursed
// Test goroutine execution
slay testGoroutine() {
    sus done dm<lit>
    sus result normie
    
    stan {
        result = 42
        dm_send(done, based)
    }
    
    dm_recv(done)
    assert_eq_int(result, 42)
}

// Test channel communication
slay testChannel() {
    sus ch dm<normie>
    
    stan {
        dm_send(ch, 42)
    }
    
    result := dm_recv(ch)
    assert_eq_int(result, 42)
}
```

This specification provides a complete foundation for CURSED's concurrency model, ensuring consistency with the existing runtime implementation while providing comprehensive documentation for developers.
