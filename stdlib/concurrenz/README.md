# concurrenz - Concurrency and Parallelism

## Overview

The `concurrenz` module provides CURSED's powerful concurrency primitives, including goroutines (green threads), channels, synchronization tools, and parallel execution patterns. Built on an efficient M:N threading model, it enables scalable concurrent programming with type safety and memory safety guarantees.

## Quick Start

```cursed
yeet "concurrenz"

// Simple goroutine
go {
    vibez.spill("Running concurrently!")
}

// Channel communication
sus ch chan<drip> = make_channel()
go {
    ch <- 42
}
sus value drip = <-ch
vibez.spill("Received:", value)
```

## Core Concepts

### Goroutines
Lightweight threads managed by the CURSED runtime. Goroutines use stack segmentation and efficient scheduling to support millions of concurrent operations.

### Channels
Type-safe communication primitives for passing data between goroutines. Channels enforce synchronization and prevent data races.

### Select Operations
Multi-channel operations that enable non-blocking communication and complex coordination patterns.

## API Reference

### Goroutines

#### `go { ... }`
Spawns a new goroutine to execute the given block concurrently.

```cursed
go {
    vibez.spill("Hello from goroutine!")
    sus result drip = expensive_computation()
    vibez.spill("Computation result:", result)
}

// Multiple goroutines
bestie (sus i drip = 0; i < 10; i++) {
    go {
        vibez.spill("Goroutine", i, "is running")
    }
}
```

#### `runtime_num_goroutines() drip`
Returns the current number of active goroutines.

```cursed
vibez.spill("Active goroutines:", concurrenz.runtime_num_goroutines())
```

#### `runtime_gosched()`
Yields execution to allow other goroutines to run.

```cursed
slay cpu_intensive_task() {
    bestie (sus i drip = 0; i < 1000000; i++) {
        // Periodically yield to be cooperative
        ready (i % 10000 == 0) {
            concurrenz.runtime_gosched()
        }
        // Do work...
    }
}
```

### Channels

#### `chan<T>` Type
Generic channel type for type-safe communication.

```cursed
sus int_chan chan<drip> = make_channel()
sus string_chan chan<tea> = make_channel()
sus bool_chan chan<lit> = make_channel()
```

#### `make_channel<T>() chan<T>`
Creates a new unbuffered channel.

```cursed
sus ch chan<drip> = make_channel()
```

#### `make_buffered_channel<T>(capacity drip) chan<T>`
Creates a buffered channel with the specified capacity.

```cursed
sus buffered chan<tea> = make_buffered_channel(10)

// Non-blocking sends until buffer is full
bestie (sus i drip = 0; i < 10; i++) {
    buffered <- "message " + i.(tea)
}
```

#### Channel Operations

**Send**: `channel <- value`
```cursed
sus ch chan<drip> = make_channel()
go {
    ch <- 42
    ch <- 24
    close(ch)
}
```

**Receive**: `value = <-channel`
```cursed
sus value drip = <-ch
vibez.spill("Received:", value)

// Check if channel is closed
sus value drip, sus ok lit = <-ch
ready (!ok) {
    vibez.spill("Channel is closed")
}
```

**Close**: `close(channel)`
```cursed
close(ch)  // No more sends allowed, receivers get zero values
```

#### Channel Iteration
```cursed
sus ch chan<drip> = make_channel()

// Send data in goroutine
go {
    bestie (sus i drip = 1; i <= 5; i++) {
        ch <- i
    }
    close(ch)
}

// Receive until closed
ready (lit) {
    sus value drip, sus ok lit = <-ch
    ready (!ok) {
        break
    }
    vibez.spill("Received:", value)
}
```

### Select Operations

#### `select { ... }`
Multi-channel operations for non-blocking communication.

```cursed
sus ch1 chan<drip> = make_channel()
sus ch2 chan<tea> = make_channel()

select {
    case value := <-ch1 {
        vibez.spill("Got number:", value)
    }
    case message := <-ch2 {
        vibez.spill("Got string:", message)
    }
    default {
        vibez.spill("No channels ready")
    }
}
```

#### Timeout Operations
```cursed
yeet "timez"

sus ch chan<drip> = make_channel()
sus timeout chan<lit> = timez.after(1000) // 1 second timeout

select {
    case value := <-ch {
        vibez.spill("Received:", value)
    }
    case <-timeout {
        vibez.spill("Operation timed out")
    }
}
```

### Synchronization Primitives

#### `Mutex`
Mutual exclusion lock for protecting shared resources.

```cursed
struct Counter {
    value drip,
    mutex Mutex
}

slay (counter *Counter) increment() {
    counter.mutex.lock()
    shook counter.mutex.unlock()  // Ensure unlock on function exit
    
    counter.value++
}

slay (counter *Counter) get() drip {
    counter.mutex.lock()
    shook counter.mutex.unlock()
    
    damn counter.value
}
```

#### `RWMutex`
Reader-writer mutex allowing multiple readers or single writer.

```cursed
struct SafeMap {
    data map[tea]drip,
    rwmutex RWMutex
}

slay (sm *SafeMap) get(key tea) drip {
    sm.rwmutex.rlock()
    shook sm.rwmutex.runlock()
    
    damn sm.data[key]
}

slay (sm *SafeMap) set(key tea, value drip) {
    sm.rwmutex.lock()
    shook sm.rwmutex.unlock()
    
    sm.data[key] = value
}
```

#### `WaitGroup`
Waits for a collection of goroutines to finish.

```cursed
sus wg WaitGroup = WaitGroup{}

bestie (sus i drip = 0; i < 5; i++) {
    wg.add(1)
    go {
        shook wg.done()
        vibez.spill("Worker", i, "finished")
    }
}

wg.wait()
vibez.spill("All workers finished")
```

#### `Once`
Ensures a function is executed only once across multiple goroutines.

```cursed
sus once Once = Once{}

slay initialize() {
    vibez.spill("Initialization performed once")
}

// Multiple goroutines call this, but initialize() runs only once
bestie (sus i drip = 0; i < 10; i++) {
    go {
        once.do(initialize)
    }
}
```

### Atomic Operations

#### `atomic_load<T>(addr *T) T`
Atomically loads a value.

```cursed
sus counter drip = 0
sus value drip = concurrenz.atomic_load(&counter)
```

#### `atomic_store<T>(addr *T, value T)`
Atomically stores a value.

```cursed
concurrenz.atomic_store(&counter, 42)
```

#### `atomic_add(addr *drip, delta drip) drip`
Atomically adds and returns the new value.

```cursed
sus new_value drip = concurrenz.atomic_add(&counter, 1)
```

#### `atomic_compare_and_swap<T>(addr *T, old T, new T) lit`
Atomic compare-and-swap operation.

```cursed
sus success lit = concurrenz.atomic_compare_and_swap(&counter, 10, 20)
ready (success) {
    vibez.spill("Successfully updated counter")
}
```

## Advanced Usage Patterns

### Worker Pool Pattern
```cursed
struct WorkerPool {
    workers drip,
    jobs chan<slay()>,
    results chan<drip>,
    wg WaitGroup
}

slay create_worker_pool(num_workers drip) WorkerPool {
    sus pool WorkerPool = WorkerPool{
        workers: num_workers,
        jobs: make_channel(),
        results: make_buffered_channel(100),
        wg: WaitGroup{}
    }
    
    // Start workers
    bestie (sus i drip = 0; i < num_workers; i++) {
        pool.wg.add(1)
        go {
            shook pool.wg.done()
            
            ready (lit) {
                sus job slay(), sus ok lit = <-pool.jobs
                ready (!ok) {
                    break
                }
                
                sus result drip = job()
                pool.results <- result
            }
        }
    }
    
    damn pool
}

slay (pool *WorkerPool) submit(job slay() drip) {
    pool.jobs <- job
}

slay (pool *WorkerPool) shutdown() {
    close(pool.jobs)
    pool.wg.wait()
    close(pool.results)
}

// Usage
sus pool WorkerPool = create_worker_pool(4)

bestie (sus i drip = 0; i < 10; i++) {
    sus task_id drip = i
    pool.submit(slay() drip {
        // Simulate work
        timez.sleep(100)
        damn task_id * task_id
    })
}

pool.shutdown()
```

### Pipeline Pattern
```cursed
// Pipeline stage
slay pipeline_stage(input chan<drip>, output chan<drip>, transform slay(drip) drip) {
    go {
        ready (lit) {
            sus value drip, sus ok lit = <-input
            ready (!ok) {
                close(output)
                break
            }
            
            sus result drip = transform(value)
            output <- result
        }
    }
}

// Create pipeline
sus input chan<drip> = make_channel()
sus stage1 chan<drip> = make_channel()
sus stage2 chan<drip> = make_channel()
sus output chan<drip> = make_channel()

// Stage 1: multiply by 2
pipeline_stage(input, stage1, slay(x drip) drip { damn x * 2 })

// Stage 2: add 10
pipeline_stage(stage1, stage2, slay(x drip) drip { damn x + 10 })

// Stage 3: convert to string and back (just for demo)
go {
    ready (lit) {
        sus value drip, sus ok lit = <-stage2
        ready (!ok) {
            close(output)
            break
        }
        output <- value
    }
}

// Send data through pipeline
go {
    bestie (sus i drip = 1; i <= 5; i++) {
        input <- i
    }
    close(input)
}

// Collect results
ready (lit) {
    sus result drip, sus ok lit = <-output
    ready (!ok) {
        break
    }
    vibez.spill("Pipeline result:", result)
}
```

### Producer-Consumer Pattern
```cursed
struct Buffer {
    data []drip,
    mutex Mutex,
    not_empty Condition,
    not_full Condition,
    head drip,
    tail drip,
    count drip,
    capacity drip
}

slay create_buffer(capacity drip) Buffer {
    damn Buffer{
        data: make_array(capacity),
        head: 0,
        tail: 0,
        count: 0,
        capacity: capacity
    }
}

slay (buf *Buffer) put(item drip) {
    buf.mutex.lock()
    shook buf.mutex.unlock()
    
    // Wait for space
    ready (buf.count == buf.capacity) {
        buf.not_full.wait(&buf.mutex)
    }
    
    buf.data[buf.tail] = item
    buf.tail = (buf.tail + 1) % buf.capacity
    buf.count++
    
    buf.not_empty.signal()
}

slay (buf *Buffer) get() drip {
    buf.mutex.lock()
    shook buf.mutex.unlock()
    
    // Wait for data
    ready (buf.count == 0) {
        buf.not_empty.wait(&buf.mutex)
    }
    
    sus item drip = buf.data[buf.head]
    buf.head = (buf.head + 1) % buf.capacity
    buf.count--
    
    buf.not_full.signal()
    damn item
}
```

### Fan-Out/Fan-In Pattern
```cursed
// Fan-out: distribute work to multiple workers
slay fan_out(input chan<drip>, workers drip) []chan<drip> {
    sus outputs []chan<drip> = []
    
    bestie (sus i drip = 0; i < workers; i++) {
        sus out chan<drip> = make_channel()
        outputs = append(outputs, out)
        
        go {
            ready (lit) {
                sus value drip, sus ok lit = <-input
                ready (!ok) {
                    close(out)
                    break
                }
                out <- value
            }
        }
    }
    
    damn outputs
}

// Fan-in: collect results from multiple workers
slay fan_in(inputs []chan<drip>) chan<drip> {
    sus output chan<drip> = make_channel()
    sus wg WaitGroup = WaitGroup{}
    
    bestie (sus input chan<drip> : inputs) {
        wg.add(1)
        go {
            shook wg.done()
            
            ready (lit) {
                sus value drip, sus ok lit = <-input
                ready (!ok) {
                    break
                }
                output <- value
            }
        }
    }
    
    // Close output when all inputs are done
    go {
        wg.wait()
        close(output)
    }
    
    damn output
}
```

## Error Handling and Recovery

### Panic Recovery in Goroutines
```cursed
slay safe_goroutine(work slay()) {
    go {
        // Recover from panics to prevent crashing other goroutines
        recover {
            work()
        } catch (error) {
            vibez.spill_error("Goroutine panic:", error)
        }
    }
}

// Usage
safe_goroutine(slay() {
    // This might panic
    sus result drip = risky_operation()
    vibez.spill("Result:", result)
})
```

### Channel Error Handling
```cursed
slay safe_channel_send<T>(ch chan<T>, value T, timeout drip) lit {
    sus timeout_ch chan<lit> = timez.after(timeout)
    
    select {
        case ch <- value {
            damn based  // Success
        }
        case <-timeout_ch {
            damn false  // Timeout
        }
    }
}

slay safe_channel_receive<T>(ch chan<T>, timeout drip) (T, lit) {
    sus timeout_ch chan<lit> = timez.after(timeout)
    
    select {
        case value := <-ch {
            damn (value, based)  // Success
        }
        case <-timeout_ch {
            damn (T{}, false)  // Timeout
        }
    }
}
```

## Performance Monitoring

### Goroutine Profiling
```cursed
struct GoroutineStats {
    total drip,
    running drip,
    waiting drip,
    sleeping drip
}

slay get_goroutine_stats() GoroutineStats {
    damn GoroutineStats{
        total: concurrenz.runtime_num_goroutines(),
        running: concurrenz.runtime_num_running(),
        waiting: concurrenz.runtime_num_waiting(),
        sleeping: concurrenz.runtime_num_sleeping()
    }
}

slay monitor_goroutines() {
    go {
        ready (lit) {
            sus stats GoroutineStats = get_goroutine_stats()
            vibez.spill("Goroutines - Total:", stats.total, 
                       "Running:", stats.running,
                       "Waiting:", stats.waiting)
            timez.sleep(5000)  // 5 second intervals
        }
    }
}
```

### Channel Monitoring
```cursed
struct ChannelStats {
    buffer_size drip,
    buffer_used drip,
    senders_waiting drip,
    receivers_waiting drip
}

slay get_channel_stats<T>(ch chan<T>) ChannelStats {
    damn ChannelStats{
        buffer_size: ch.capacity(),
        buffer_used: ch.len(),
        senders_waiting: ch.senders_waiting(),
        receivers_waiting: ch.receivers_waiting()
    }
}
```

## Testing Concurrent Code

### Race Detection
```cursed
// Enable race detection in tests
#[test]
slay test_concurrent_counter() {
    sus counter drip = 0
    sus iterations drip = 1000
    sus wg WaitGroup = WaitGroup{}
    
    // Start multiple goroutines that increment counter
    bestie (sus i drip = 0; i < 10; i++) {
        wg.add(1)
        go {
            shook wg.done()
            
            bestie (sus j drip = 0; j < iterations; j++) {
                // This will trigger race detection if not synchronized
                counter++
            }
        }
    }
    
    wg.wait()
    
    // This test will fail with race detection enabled
    testz.assert_eq_int(counter, 10 * iterations)
}
```

### Deadlock Detection
```cursed
#[test]
slay test_no_deadlock() {
    sus ch1 chan<drip> = make_channel()
    sus ch2 chan<drip> = make_channel()
    sus timeout chan<lit> = timez.after(1000)  // 1 second timeout
    
    go {
        ch1 <- 1
        <-ch2
    }
    
    go {
        ch2 <- 2
        <-ch1
    }
    
    select {
        case <-timez.after(100) {
            // Test passes if operations complete quickly
        }
        case <-timeout {
            testz.fail("Potential deadlock detected")
        }
    }
}
```

### Load Testing
```cursed
#[test]
slay test_channel_throughput() {
    sus ch chan<drip> = make_buffered_channel(1000)
    sus messages drip = 100000
    sus start drip = timez.get_time_microseconds()
    
    // Producer
    go {
        bestie (sus i drip = 0; i < messages; i++) {
            ch <- i
        }
        close(ch)
    }
    
    // Consumer
    sus received drip = 0
    ready (lit) {
        sus value drip, sus ok lit = <-ch
        ready (!ok) {
            break
        }
        received++
    }
    
    sus elapsed drip = timez.get_time_microseconds() - start
    sus throughput drip = (messages * 1000000) / elapsed
    
    vibez.spill("Throughput:", throughput, "messages/second")
    testz.assert_eq_int(received, messages)
}
```

## Best Practices

### Goroutine Lifecycle
1. **Always have a way to stop goroutines** - use context or channels for cancellation
2. **Avoid goroutine leaks** - ensure all goroutines can exit
3. **Use WaitGroup for coordination** - wait for goroutines to complete
4. **Handle panics** - recover from panics to prevent crashes

### Channel Usage
1. **Close channels only from sender side** - receivers should check if closed
2. **Use buffered channels judiciously** - avoid hiding synchronization issues
3. **Prefer channels over shared memory** - "Don't communicate by sharing memory; share memory by communicating"
4. **Use select with default for non-blocking operations**

### Synchronization
1. **Keep critical sections small** - minimize time holding locks
2. **Use RWMutex for read-heavy workloads** - allow concurrent readers
3. **Prefer atomic operations for simple operations** - faster than mutexes
4. **Avoid nested locking** - can cause deadlocks

### Performance
1. **Pool goroutines for short-lived tasks** - reduce creation overhead
2. **Use buffered channels to reduce blocking** - but don't hide synchronization issues
3. **Profile concurrent code** - identify bottlenecks and contention
4. **Consider GOMAXPROCS** - tune for your hardware and workload

---

The `concurrenz` module enables powerful concurrent programming with safety guarantees and excellent performance. Its design follows proven patterns from languages like Go while adding CURSED's type safety and memory safety features.
