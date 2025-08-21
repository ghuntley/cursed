# ⚡ CURSED Concurrency Guide

CURSED provides powerful concurrency primitives inspired by Go, with goroutines, channels, and select operations for building scalable concurrent applications.

## 📚 Table of Contents

- [Goroutines](#goroutines)
- [Channels](#channels)
- [Select Operations](#select-operations)
- [Synchronization](#synchronization)
- [Error Handling](#error-handling)
- [Patterns](#concurrency-patterns)
- [Performance](#performance)
- [Best Practices](#best-practices)

## 🏃 Goroutines

Goroutines are lightweight threads managed by the CURSED runtime. They provide efficient concurrent execution with minimal overhead.

### Basic Goroutines

```cursed
yeet "concurrenz"
yeet "vibez"
yeet "timez"

# Simple goroutine
go {
    vibez.spill("Hello from goroutine!")
}

# Goroutine with parameters
slay worker(id drip, name tea) {
    vibez.spill("Worker", id, "named", name, "started")
    timez.sleep(1000)  # Simulate work
    vibez.spill("Worker", id, "finished")
}

go worker(1, "Alice")
go worker(2, "Bob")
```

### Goroutine Lifecycle

```cursed
# Goroutines with cleanup
slay background_task(id drip) {
    vibez.spill("Task", id, "starting...")
    
    defer {
        vibez.spill("Task", id, "cleanup completed")
    }
    
    # Simulate work that might fail
    ready (id % 2 == 0) {
        vibez.spill("Task", id, "completed successfully")
    } otherwise {
        vibez.spill("Task", id, "failed")
    }
}

# Launch multiple goroutines
sus i drip = 0
bestie (i < 5) {
    go background_task(i)
    i = i + 1
}

# Wait for completion
timez.sleep(2000)
```

### Goroutine Pools

```cursed
# Worker pool implementation
squad WorkerPool {
    workers drip
    jobs chan<slay()>
    quit chan<lit>
}

impl WorkerPool {
    slay new(worker_count drip) WorkerPool {
        sus pool WorkerPool = {
            workers: worker_count,
            jobs: make_channel<slay()>(),
            quit: make_channel<lit>()
        }
        
        # Start workers
        sus i drip = 0
        bestie (i < worker_count) {
            go pool.worker(i)
            i = i + 1
        }
        
        damn pool
    }
    
    slay worker(id drip) {
        vibez.spill("Worker", id, "started")
        
        bestie (based) {
            select {
                when job := <-self.jobs -> {
                    vibez.spill("Worker", id, "executing job")
                    job()  # Execute the job
                }
                when <-self.quit -> {
                    vibez.spill("Worker", id, "quitting")
                    damn
                }
            }
        }
    }
    
    slay submit(job slay()) {
        self.jobs <- job
    }
    
    slay shutdown() {
        # Signal all workers to quit
        sus i drip = 0
        bestie (i < self.workers) {
            self.quit <- based
            i = i + 1
        }
    }
}

# Usage example
sus pool WorkerPool = WorkerPool.new(3)

# Submit jobs
pool.submit(slay() { vibez.spill("Job 1 executed") })
pool.submit(slay() { vibez.spill("Job 2 executed") })
pool.submit(slay() { vibez.spill("Job 3 executed") })

timez.sleep(1000)
pool.shutdown()
```

## 📡 Channels

Channels are typed conduits for communication between goroutines, providing safe message passing.

### Channel Basics

```cursed
# Create channels
sus ch chan<drip> = make_channel<drip>()
sus messages chan<tea> = make_channel<tea>()
sus done chan<lit> = make_channel<lit>()

# Send and receive
go {
    ch <- 42
    messages <- "Hello"
    done <- based
}

sus number drip = <-ch
sus message tea = <-messages
sus finished lit = <-done

vibez.spill("Received:", number, message, finished)
```

### Buffered Channels

```cursed
# Buffered channels don't block until buffer is full
sus buffered chan<drip> = make_channel_with_buffer<drip>(5)

# These won't block
buffered <- 1
buffered <- 2
buffered <- 3

vibez.spill("Sent 3 values without blocking")

# Receive values
sus val1 drip = <-buffered
sus val2 drip = <-buffered
sus val3 drip = <-buffered
```

### Channel Directions

```cursed
# Send-only channel type
type SendOnlyChannel<T> = chan<- T

# Receive-only channel type  
type ReceiveOnlyChannel<T> = <-chan T

# Function that only sends
slay producer(ch SendOnlyChannel<drip>) {
    sus i drip = 0
    bestie (i < 10) {
        ch <- i
        i = i + 1
    }
    close(ch)
}

# Function that only receives
slay consumer(ch ReceiveOnlyChannel<drip>) {
    bestie (value := <-ch) {
        vibez.spill("Consumed:", value)
    }
}

# Usage with bidirectional channel
sus ch chan<drip> = make_channel<drip>()
go producer(ch)
go consumer(ch)
```

### Channel Patterns

```cursed
# Fan-out pattern: distribute work across multiple goroutines
slay fan_out_example() {
    sus input chan<drip> = make_channel<drip>()
    sus output1 chan<drip> = make_channel<drip>()
    sus output2 chan<drip> = make_channel<drip>()
    sus output3 chan<drip> = make_channel<drip>()
    
    # Distributor
    go {
        bestie (value := <-input) {
            output1 <- value
            output2 <- value
            output3 <- value
        }
    }
    
    # Workers
    go {
        bestie (value := <-output1) {
            vibez.spill("Worker 1 processing:", value)
        }
    }
    
    go {
        bestie (value := <-output2) {
            vibez.spill("Worker 2 processing:", value)
        }
    }
    
    go {
        bestie (value := <-output3) {
            vibez.spill("Worker 3 processing:", value)
        }
    }
    
    # Send work
    input <- 100
    input <- 200
    close(input)
}

# Fan-in pattern: merge multiple channels into one
slay fan_in<T>(ch1 ReceiveOnlyChannel<T>, ch2 ReceiveOnlyChannel<T>) ReceiveOnlyChannel<T> {
    sus merged chan<T> = make_channel<T>()
    
    go {
        bestie (value := <-ch1) {
            merged <- value
        }
    }
    
    go {
        bestie (value := <-ch2) {
            merged <- value
        }
    }
    
    damn merged
}
```

## ⚖️ Select Operations

Select operations provide non-blocking channel operations and multiplexing.

### Basic Select

```cursed
# Select between multiple channels
sus ch1 chan<tea> = make_channel<tea>()
sus ch2 chan<drip> = make_channel<drip>()
sus timeout chan<lit> = make_channel<lit>()

# Set timeout
go {
    timez.sleep(5000)  # 5 second timeout
    timeout <- based
}

# Select from multiple channels
select {
    when msg := <-ch1 -> {
        vibez.spill("Received string:", msg)
    }
    when num := <-ch2 -> {
        vibez.spill("Received number:", num)
    }
    when <-timeout -> {
        vibez.spill("Operation timed out")
    }
    when default -> {
        vibez.spill("No channels ready, continuing...")
    }
}
```

### Advanced Select Patterns

```cursed
# Request-response pattern with timeout
squad RequestResponse<T> {
    request T
    response chan<T>
}

slay make_request_with_timeout<T>(request T, handler slay(T) T, timeout_ms drip) yikes<T> {
    sus req_resp RequestResponse<T> = {
        request: request,
        response: make_channel<T>()
    }
    
    sus timeout chan<lit> = make_channel<lit>()
    
    # Start timeout
    go {
        timez.sleep(timeout_ms)
        timeout <- based
    }
    
    # Process request
    go {
        sus result T = handler(req_resp.request)
        req_resp.response <- result
    }
    
    # Wait for response or timeout
    select {
        when result := <-req_resp.response -> {
            damn result
        }
        when <-timeout -> {
            yikes "request timed out"
        }
    }
}

# Usage
sus result drip = make_request_with_timeout(42, slay(x drip) drip {
    timez.sleep(1000)  # Simulate processing
    damn x * 2
}, 2000) fam {
    when "request timed out" -> {
        vibez.spill("Request timed out")
        damn 0
    }
    when _ -> {
        vibez.spill("Request failed")
        damn -1
    }
}

vibez.spill("Request result:", result)
```

### Priority Select

```cursed
# Priority channel selection
slay priority_select() {
    sus high_priority chan<tea> = make_channel<tea>()
    sus medium_priority chan<tea> = make_channel<tea>()
    sus low_priority chan<tea> = make_channel<tea>()
    
    bestie (based) {
        # Check high priority first
        select {
            when msg := <-high_priority -> {
                vibez.spill("HIGH:", msg)
                skip
            }
            when default -> {
                # If no high priority, check medium
                select {
                    when msg := <-medium_priority -> {
                        vibez.spill("MEDIUM:", msg)
                        skip
                    }
                    when default -> {
                        # Finally check low priority
                        select {
                            when msg := <-low_priority -> {
                                vibez.spill("LOW:", msg)
                            }
                            when default -> {
                                # No messages, do other work
                                timez.sleep(100)
                            }
                        }
                    }
                }
            }
        }
    }
}
```

## 🔒 Synchronization

CURSED provides various synchronization primitives for coordinating goroutines.

### Mutexes

```cursed
yeet "concurrenz"

# Shared data protection
squad Counter {
    value drip
    mutex concurrenz.Mutex
}

impl Counter {
    slay new() Counter {
        damn Counter{
            value: 0,
            mutex: concurrenz.new_mutex()
        }
    }
    
    slay increment() {
        concurrenz.lock(&self.mutex)
        defer concurrenz.unlock(&self.mutex)
        
        self.value = self.value + 1
    }
    
    slay get() drip {
        concurrenz.lock(&self.mutex)
        defer concurrenz.unlock(&self.mutex)
        
        damn self.value
    }
}

# Usage
sus counter Counter = Counter.new()

# Multiple goroutines incrementing safely
sus i drip = 0
bestie (i < 10) {
    go {
        sus j drip = 0
        bestie (j < 100) {
            counter.increment()
            j = j + 1
        }
    }
    i = i + 1
}

timez.sleep(1000)
vibez.spill("Final counter value:", counter.get())
```

### WaitGroups

```cursed
# Coordinate completion of multiple goroutines
squad WaitGroup {
    count drip
    done chan<lit>
    mutex concurrenz.Mutex
}

impl WaitGroup {
    slay new() WaitGroup {
        damn WaitGroup{
            count: 0,
            done: make_channel<lit>(),
            mutex: concurrenz.new_mutex()
        }
    }
    
    slay add(delta drip) {
        concurrenz.lock(&self.mutex)
        defer concurrenz.unlock(&self.mutex)
        
        self.count = self.count + delta
    }
    
    slay done() {
        concurrenz.lock(&self.mutex)
        defer concurrenz.unlock(&self.mutex)
        
        self.count = self.count - 1
        ready (self.count == 0) {
            self.done <- based
        }
    }
    
    slay wait() {
        <-self.done
    }
}

# Usage
sus wg WaitGroup = WaitGroup.new()

# Launch workers
sus workers drip = 5
wg.add(workers)

sus i drip = 0
bestie (i < workers) {
    go {
        vibez.spill("Worker", i, "starting")
        timez.sleep(mathz.random(500, 1500))
        vibez.spill("Worker", i, "finished")
        wg.done()
    }
    i = i + 1
}

vibez.spill("Waiting for all workers to complete...")
wg.wait()
vibez.spill("All workers completed!")
```

### Condition Variables

```cursed
# Condition variables for complex synchronization
squad Condition {
    mutex concurrenz.Mutex
    waiters drip
    signal_ch chan<lit>
}

impl Condition {
    slay new() Condition {
        damn Condition{
            mutex: concurrenz.new_mutex(),
            waiters: 0,
            signal_ch: make_channel<lit>()
        }
    }
    
    slay wait(predicate slay() lit) {
        concurrenz.lock(&self.mutex)
        defer concurrenz.unlock(&self.mutex)
        
        bestie (!predicate()) {
            self.waiters = self.waiters + 1
            concurrenz.unlock(&self.mutex)
            
            <-self.signal_ch
            
            concurrenz.lock(&self.mutex)
            self.waiters = self.waiters - 1
        }
    }
    
    slay signal() {
        concurrenz.lock(&self.mutex)
        defer concurrenz.unlock(&self.mutex)
        
        ready (self.waiters > 0) {
            self.signal_ch <- based
        }
    }
    
    slay broadcast() {
        concurrenz.lock(&self.mutex)
        defer concurrenz.unlock(&self.mutex)
        
        bestie (self.waiters > 0) {
            self.signal_ch <- based
            self.waiters = self.waiters - 1
        }
    }
}
```

## 🚨 Error Handling

Proper error handling in concurrent code is crucial for reliability.

### Channel Error Patterns

```cursed
# Error handling with channels
squad Result<T> {
    value T
    error tea
    success lit
}

slay worker_with_errors(id drip, results chan<Result<drip>>) {
    # Simulate work that might fail
    ready (mathz.random(0, 10) < 3) {
        # 30% chance of failure
        results <- Result{
            value: 0,
            error: stringz.format("Worker %d failed", id),
            success: false
        }
    } otherwise {
        # Success case
        sus result drip = id * 100
        results <- Result{
            value: result,
            error: "",
            success: based
        }
    }
}

# Collect results with error handling
slay collect_results() {
    sus results chan<Result<drip>> = make_channel<Result<drip>>()
    sus worker_count drip = 10
    
    # Launch workers
    sus i drip = 0
    bestie (i < worker_count) {
        go worker_with_errors(i, results)
        i = i + 1
    }
    
    # Collect results
    sus successful drip = 0
    sus failed drip = 0
    
    bestie (i < worker_count) {
        sus result Result<drip> = <-results
        
        ready (result.success) {
            vibez.spill("Success:", result.value)
            successful = successful + 1
        } otherwise {
            vibez.spill("Error:", result.error)
            failed = failed + 1
        }
        i = i + 1
    }
    
    vibez.spill("Results: ", successful, "successful,", failed, "failed")
}
```

### Panic Recovery in Goroutines

```cursed
# Safe goroutine execution with panic recovery
slay safe_goroutine(task slay(), on_panic slay(tea)) {
    go {
        recover {
            task()
        } handle panic_msg -> {
            vibez.spill("Goroutine panicked:", panic_msg)
            on_panic(panic_msg)
        }
    }
}

# Usage
safe_goroutine(
    slay() {
        # This might panic
        sus x drip = 0
        sus y drip = 100 / x  # Division by zero
        vibez.spill("Result:", y)
    },
    slay(panic_msg tea) {
        vibez.spill("Handled panic in goroutine:", panic_msg)
    }
)
```

## 🏗️ Concurrency Patterns

### Pipeline Pattern

```cursed
# Pipeline processing with channels
slay pipeline_example() {
    # Stage 1: Generate numbers
    sus numbers chan<drip> = make_channel<drip>()
    go {
        bestie (i drip in range(1, 100)) {
            numbers <- i
        }
        close(numbers)
    }
    
    # Stage 2: Square numbers
    sus squares chan<drip> = make_channel<drip>()
    go {
        bestie (num := <-numbers) {
            squares <- num * num
        }
        close(squares)
    }
    
    # Stage 3: Filter even squares
    sus even_squares chan<drip> = make_channel<drip>()
    go {
        bestie (square := <-squares) {
            ready (square % 2 == 0) {
                even_squares <- square
            }
        }
        close(even_squares)
    }
    
    # Final consumer
    bestie (result := <-even_squares) {
        vibez.spill("Even square:", result)
    }
}
```

### Producer-Consumer Pattern

```cursed
# Producer-consumer with bounded buffer
squad ProducerConsumer<T> {
    buffer chan<T>
    producer_count drip
    consumer_count drip
    shutdown chan<lit>
}

impl<T> ProducerConsumer<T> {
    slay new(buffer_size drip, producers drip, consumers drip) ProducerConsumer<T> {
        damn ProducerConsumer{
            buffer: make_channel_with_buffer<T>(buffer_size),
            producer_count: producers,
            consumer_count: consumers,
            shutdown: make_channel<lit>()
        }
    }
    
    slay start_producers(producer slay() T) {
        sus i drip = 0
        bestie (i < self.producer_count) {
            go {
                bestie (based) {
                    select {
                        when <-self.shutdown -> damn
                        when default -> {
                            sus item T = producer()
                            self.buffer <- item
                            timez.sleep(100)  # Rate limiting
                        }
                    }
                }
            }
            i = i + 1
        }
    }
    
    slay start_consumers(consumer slay(T)) {
        sus i drip = 0
        bestie (i < self.consumer_count) {
            go {
                bestie (based) {
                    select {
                        when item := <-self.buffer -> {
                            consumer(item)
                        }
                        when <-self.shutdown -> damn
                    }
                }
            }
            i = i + 1
        }
    }
    
    slay shutdown() {
        # Signal all goroutines to shutdown
        sus total drip = self.producer_count + self.consumer_count
        sus i drip = 0
        bestie (i < total) {
            self.shutdown <- based
            i = i + 1
        }
    }
}
```

### Load Balancing Pattern

```cursed
# Round-robin load balancer
squad LoadBalancer<T> {
    workers []chan<T>
    current drip
    mutex concurrenz.Mutex
}

impl<T> LoadBalancer<T> {
    slay new(worker_count drip) LoadBalancer<T> {
        sus workers []chan<T> = []
        
        sus i drip = 0
        bestie (i < worker_count) {
            workers = arrayz.append(workers, make_channel<T>())
            i = i + 1
        }
        
        damn LoadBalancer{
            workers: workers,
            current: 0,
            mutex: concurrenz.new_mutex()
        }
    }
    
    slay distribute(item T) {
        concurrenz.lock(&self.mutex)
        defer concurrenz.unlock(&self.mutex)
        
        sus worker_ch chan<T> = self.workers[self.current]
        self.current = (self.current + 1) % len(self.workers)
        
        worker_ch <- item
    }
    
    slay start_workers(worker slay(drip, T)) {
        sus i drip = 0
        bestie (worker_ch in self.workers) {
            go {
                bestie (item := <-worker_ch) {
                    worker(i, item)
                }
            }
            i = i + 1
        }
    }
}
```

## 🚀 Performance

### Goroutine Performance

```cursed
# Benchmark goroutine creation and communication
slay benchmark_goroutines() {
    sus start_time drip = timez.now()
    sus goroutine_count drip = 10000
    sus done chan<lit> = make_channel<lit>()
    
    vibez.spill("Creating", goroutine_count, "goroutines...")
    
    sus i drip = 0
    bestie (i < goroutine_count) {
        go {
            # Minimal work
            sus x drip = i * 2
            ready (x > 0) {
                done <- based
            }
        }
        i = i + 1
    }
    
    # Wait for all goroutines
    i = 0
    bestie (i < goroutine_count) {
        <-done
        i = i + 1
    }
    
    sus elapsed drip = timez.now() - start_time
    sus per_goroutine meal = elapsed / goroutine_count
    
    vibez.spill("Created", goroutine_count, "goroutines in", elapsed, "ms")
    vibez.spill("Average:", per_goroutine, "ms per goroutine")
}
```

### Channel Performance

```cursed
# Benchmark channel operations
slay benchmark_channels() {
    sus iterations drip = 1000000
    sus ch chan<drip> = make_channel<drip>()
    
    # Benchmark unbuffered channel
    sus start_time drip = timez.now()
    
    go {
        sus i drip = 0
        bestie (i < iterations) {
            ch <- i
            i = i + 1
        }
    }
    
    sus i drip = 0
    bestie (i < iterations) {
        <-ch
        i = i + 1
    }
    
    sus elapsed drip = timez.now() - start_time
    sus ops_per_ms meal = iterations / elapsed
    
    vibez.spill("Channel operations:", iterations, "in", elapsed, "ms")
    vibez.spill("Throughput:", ops_per_ms, "ops/ms")
}
```

## ✅ Best Practices

### 1. Resource Management

```cursed
# Always clean up goroutines and channels
slay managed_goroutines() {
    sus done chan<lit> = make_channel<lit>()
    sus workers drip = 5
    
    # Start workers with cleanup
    sus i drip = 0
    bestie (i < workers) {
        go {
            defer {
                vibez.spill("Worker", i, "cleaned up")
            }
            
            # Work until done
            bestie (based) {
                select {
                    when <-done -> damn  # Exit cleanly
                    when default -> {
                        # Do work
                        timez.sleep(100)
                    }
                }
            }
        }
        i = i + 1
    }
    
    # Run for a while then cleanup
    timez.sleep(1000)
    
    # Signal all workers to stop
    bestie (i < workers) {
        done <- based
        i = i + 1
    }
    
    # Wait for cleanup
    timez.sleep(100)
}
```

### 2. Error Propagation

```cursed
# Proper error propagation in concurrent code
slay concurrent_with_errors() yikes<[]drip> {
    sus workers drip = 5
    sus results []drip = []
    sus errors chan<tea> = make_channel<tea>()
    sus success chan<drip> = make_channel<drip>()
    
    # Start workers
    sus i drip = 0
    bestie (i < workers) {
        go {
            # Simulate work that might fail
            ready (mathz.random(0, 10) < 2) {
                errors <- stringz.format("Worker %d failed", i)
            } otherwise {
                success <- i * 100
            }
        }
        i = i + 1
    }
    
    # Collect results
    sus completed drip = 0
    bestie (completed < workers) {
        select {
            when result := <-success -> {
                results = arrayz.append(results, result)
                completed = completed + 1
            }
            when error := <-errors -> {
                yikes error  # Propagate first error
            }
        }
    }
    
    damn results
}
```

### 3. Avoid Race Conditions

```cursed
# Thread-safe shared state
squad SafeCounter {
    value drip
    mutex concurrenz.Mutex
    operations drip
}

impl SafeCounter {
    slay new() SafeCounter {
        damn SafeCounter{
            value: 0,
            mutex: concurrenz.new_mutex(),
            operations: 0
        }
    }
    
    # Atomic operations
    slay increment_by(delta drip) drip {
        concurrenz.lock(&self.mutex)
        defer concurrenz.unlock(&self.mutex)
        
        self.value = self.value + delta
        self.operations = self.operations + 1
        damn self.value
    }
    
    slay get_stats() (drip, drip) {
        concurrenz.lock(&self.mutex)
        defer concurrenz.unlock(&self.mutex)
        
        damn (self.value, self.operations)
    }
}
```

### 4. Channel Closing

```cursed
# Proper channel closing patterns
slay channel_closing_example() {
    sus ch chan<drip> = make_channel<drip>()
    
    # Producer closes channel
    go {
        sus i drip = 0
        bestie (i < 10) {
            ch <- i
            i = i + 1
        }
        close(ch)  # Signal no more values
    }
    
    # Consumer detects channel closure
    bestie (based) {
        sus (value, ok) = <-ch
        ready (!ok) {
            vibez.spill("Channel closed")
            break
        }
        vibez.spill("Received:", value)
    }
}
```

### 5. Context and Cancellation

```cursed
# Context-aware goroutines (simplified context pattern)
squad Context {
    done chan<lit>
    cancelled lit
    mutex concurrenz.Mutex
}

impl Context {
    slay new() Context {
        damn Context{
            done: make_channel<lit>(),
            cancelled: false,
            mutex: concurrenz.new_mutex()
        }
    }
    
    slay cancel() {
        concurrenz.lock(&self.mutex)
        defer concurrenz.unlock(&self.mutex)
        
        ready (!self.cancelled) {
            self.cancelled = based
            close(self.done)
        }
    }
    
    slay is_cancelled() lit {
        concurrenz.lock(&self.mutex)
        defer concurrenz.unlock(&self.mutex)
        damn self.cancelled
    }
}

# Context-aware worker
slay context_aware_worker(ctx *Context, work slay()) {
    go {
        bestie (based) {
            select {
                when <-ctx.done -> {
                    vibez.spill("Worker cancelled")
                    damn
                }
                when default -> {
                    ready (!ctx.is_cancelled()) {
                        work()
                        timez.sleep(100)
                    }
                }
            }
        }
    }
}
```

---

**CURSED's concurrency model provides efficient, safe, and expressive concurrent programming with goroutines, channels, and comprehensive synchronization primitives! ⚡**
