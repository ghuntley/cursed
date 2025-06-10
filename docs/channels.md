# CURSED Channel System

## Overview

Channels in CURSED provide a powerful concurrency primitive for communication between goroutines. They follow Go-style channel semantics but use Gen Z slang syntax that's absolutely iconic for concurrent programming.

**Channel Type Declaration**: `dm<T>` where T is the element type

**Key Features**:
- Thread-safe communication between goroutines  
- Buffered and unbuffered channels
- Channel closing and error handling
- Integration with `stan` goroutines
- Select-style operations for advanced patterns

## Channel Types and Syntax

### Creating Channels

```cursed
// Unbuffered channel (synchronous)
sus ch dm<normie> = make(dm<normie>)

// Buffered channel (asynchronous)  
sus buffered_ch dm<tea> = make(dm<tea>, 10)

// Generic channel
sus data_ch dm<GenericType> = make(dm<GenericType>, 5)
```

### Channel Directions

Channels can be restricted to send-only or receive-only:

```cursed
// Send-only channel
slay sender(ch dm<- normie) {
    ch <- 42
}

// Receive-only channel  
slay receiver(ch <-dm<normie>) {
    sus value = <-ch
}
```

## Send and Receive Operations

### Basic Send/Receive

```cursed
sus ch = make(dm<normie>, 2)

// Send operation
ch <- 42
ch <- 100

// Receive operation
sus value1 = <-ch  // value1 = 42
sus value2 = <-ch  // value2 = 100
```

### Non-blocking Operations

```cursed
// Non-blocking send
vibe_check {
    mood ch <- value:
        puts("Sent successfully")
    basic:
        puts("Channel would block")
}

// Non-blocking receive
vibe_check {
    mood value := <-ch:
        puts(sprintf("Received: %d", value))
    basic:
        puts("No value available")
}
```

## Buffered vs Unbuffered Channels

### Unbuffered Channels (Synchronous)

Unbuffered channels provide synchronous communication where sends block until a receiver is ready:

```cursed
sus sync_ch = make(dm<tea>)  // No buffer

stan {
    sync_ch <- "Hello from goroutine!"
}

sus message = <-sync_ch  // Blocks until goroutine sends
puts(message)
```

**Characteristics**:
- Zero buffer capacity
- Sends block until receiver ready
- Receives block until sender ready
- Provides synchronization point

### Buffered Channels (Asynchronous)

Buffered channels allow asynchronous communication with a fixed-size buffer:

```cursed
sus async_ch = make(dm<normie>, 5)  // Buffer of 5

// Send without blocking (until buffer full)
async_ch <- 1
async_ch <- 2  
async_ch <- 3

// Receive when convenient
sus val1 = <-async_ch  // val1 = 1
sus val2 = <-async_ch  // val2 = 2
```

**Characteristics**:
- Fixed buffer capacity
- Sends block only when buffer full
- Receives block only when buffer empty
- Decouples sender and receiver timing

## Channel Closing and Error Handling

### Closing Channels

```cursed
sus ch = make(dm<normie>, 5)

// Send some values
ch <- 1
ch <- 2
ch <- 3

// Close the channel
close(ch)

// Reading from closed channel
periodt based {
    sus value, ok = <-ch
    lowkey !ok {
        puts("Channel closed")
        stan_it  // Break from loop
    }
    puts(sprintf("Received: %d", value))
}
```

### Error Handling Patterns

```cursed
slay safe_send(ch dm<normie>, value normie) lit {
    stan_it {
        ch <- value
        yolo based  // Success
    } no_cap err {
        puts(sprintf("Send failed: %s", err))
        yolo sus  // Failure
    }
}

slay safe_receive(ch dm<normie>) (normie, lit) {
    stan_it {
        sus value = <-ch
        yolo value, based
    } no_cap err {
        puts(sprintf("Receive failed: %s", err))
        yolo 0, sus
    }
}
```

## Integration with Goroutines

### Producer-Consumer Pattern

```cursed
slay producer(ch dm<normie>, count normie) {
    sus i = 0
    periodt i < count {
        ch <- i
        puts(sprintf("Produced: %d", i))
        i++
    }
    close(ch)
}

slay consumer(ch dm<normie>, id normie) {
    periodt based {
        sus value, ok = <-ch
        lowkey !ok {
            puts(sprintf("Consumer %d finished", id))
            yolo
        }
        puts(sprintf("Consumer %d consumed: %d", id, value))
    }
}

slay main() {
    sus ch = make(dm<normie>, 10)
    
    // Start producer
    stan producer(ch, 20)
    
    // Start multiple consumers
    stan consumer(ch, 1)
    stan consumer(ch, 2)
    stan consumer(ch, 3)
    
    // Wait for completion
    sleep(5000)  // 5 seconds
}
```

### Worker Pool Pattern

```cursed
be_like Job squad {
    id normie
    data tea
}

slay worker(id normie, jobs dm<Job>, results dm<tea>) {
    periodt based {
        sus job, ok = <-jobs
        lowkey !ok {
            puts(sprintf("Worker %d stopping", id))
            yolo
        }
        
        // Process job
        sus result = sprintf("Worker %d processed job %d: %s", 
                           id, job.id, job.data)
        results <- result
    }
}

slay main() {
    sus jobs = make(dm<Job>, 100)
    sus results = make(dm<tea>, 100)
    
    // Start workers
    sus num_workers = 5
    sus i = 0
    periodt i < num_workers {
        stan worker(i, jobs, results)
        i++
    }
    
    // Send jobs
    sus job_count = 20
    i = 0
    periodt i < job_count {
        jobs <- Job{id: i, data: sprintf("Job data %d", i)}
        i++
    }
    close(jobs)
    
    // Collect results
    i = 0
    periodt i < job_count {
        sus result = <-results
        puts(result)
        i++
    }
}
```

## Advanced Channel Patterns

### Fan-in Pattern

```cursed
slay fan_in(input1 dm<normie>, input2 dm<normie>) dm<normie> {
    sus output = make(dm<normie>)
    
    stan {
        periodt based {
            sus value, ok = <-input1
            lowkey !ok {
                yolo
            }
            output <- value
        }
    }
    
    stan {
        periodt based {
            sus value, ok = <-input2
            lowkey !ok {
                yolo
            }
            output <- value
        }
    }
    
    yolo output
}
```

### Fan-out Pattern

```cursed
slay fan_out(input dm<normie>, count normie) []dm<normie> {
    sus outputs = make([]dm<normie>, count)
    
    sus i = 0
    periodt i < count {
        outputs[i] = make(dm<normie>)
        i++
    }
    
    stan {
        periodt based {
            sus value, ok = <-input
            lowkey !ok {
                // Close all output channels
                i = 0
                periodt i < count {
                    close(outputs[i])
                    i++
                }
                yolo
            }
            
            // Send to all outputs
            i = 0
            periodt i < count {
                outputs[i] <- value
                i++
            }
        }
    }
    
    yolo outputs
}
```

## Best Practices

### 1. Channel Ownership

```cursed
// Good: Clear ownership
slay process_data() {
    sus ch = make(dm<normie>, 10)
    defer close(ch)  // Owner closes channel
    
    stan producer(ch)
    consumer(ch)
}
```

### 2. Avoiding Deadlocks

```cursed
// Bad: Potential deadlock
sus ch = make(dm<normie>)  // Unbuffered
ch <- 42  // Blocks forever - no receiver

// Good: Use buffered channel or goroutine
sus ch = make(dm<normie>, 1)  // Buffered
ch <- 42  // Doesn't block

// Or use goroutine
sus ch2 = make(dm<normie>)
stan {
    ch2 <- 42
}
sus value = <-ch2
```

### 3. Channel Direction Restrictions

```cursed
// Good: Restrict channel directions for clarity
slay send_only(ch dm<- normie) {
    ch <- 42
}

slay receive_only(ch <-dm<normie>) {
    sus value = <-ch
}
```

### 4. Graceful Shutdown

```cursed
slay worker_with_shutdown(work dm<Job>, shutdown dm<lit>) {
    periodt based {
        vibe_check {
            mood job := <-work:
                // Process job
                process_job(job)
            mood <-shutdown:
                puts("Worker shutting down gracefully")
                yolo
        }
    }
}
```

## Performance Considerations

### Channel Buffer Sizing

```cursed
// Small buffer for low latency
sus low_latency = make(dm<normie>, 1)

// Large buffer for high throughput  
sus high_throughput = make(dm<normie>, 1000)

// Unbuffered for synchronization
sus sync_point = make(dm<lit>)
```

### Memory Management

Channels are garbage collected when no references remain:

```cursed
slay create_temp_channel() {
    sus ch = make(dm<normie>, 100)
    // Use channel
    // Channel automatically cleaned up when function exits
}
```

## Common Pitfalls

### 1. Forgetting to Close Channels

```cursed
// Bad: Channel never closed
sus ch = make(dm<normie>)
periodt based {
    sus value, ok = <-ch  // Hangs when no more senders
    // ...
}

// Good: Close channel when done sending
close(ch)
```

### 2. Sending on Closed Channels

```cursed
sus ch = make(dm<normie>)
close(ch)
ch <- 42  // Runtime panic!

// Better: Check if channel is closed
sus is_closed = cap  // Track closure state
```

### 3. Range over Channels

```cursed
// Wait for channel to be closed
periodt value := range ch {
    puts(sprintf("Received: %d", value))
}
// Loop exits when channel is closed
```

This channel system provides powerful, thread-safe communication primitives that integrate seamlessly with CURSED's goroutine system while maintaining the language's distinctive Gen Z aesthetic.
