# CURSED Channel Tutorial

## Getting Started with Channels

Channels are CURSED's way of enabling safe communication between goroutines. Think of them as pipes that carry data between different parts of your concurrent program. Let's start with the basics and work our way up to advanced patterns.

## Part 1: Your First Channel

### Creating and Using Basic Channels

```cursed
vibe main

slay main() {
    // Create a simple channel that carries numbers
    sus ch = make(dm<normie>)
    
    // Start a goroutine that sends a value
    stan {
        ch <- 42
        puts("Sent 42 to channel")
    }
    
    // Receive the value in main
    sus value = <-ch
    puts(sprintf("Received: %d", value))
}
```

**What's happening here?**
1. We create an unbuffered channel with `make(dm<normie>)`
2. The `stan` goroutine sends `42` to the channel
3. The main function receives the value from the channel
4. Both operations synchronize - the send waits for the receive

### Understanding Unbuffered vs Buffered Channels

```cursed
slay unbuffered_example() {
    sus ch = make(dm<tea>)  // No buffer - synchronous
    
    stan {
        puts("About to send...")
        ch <- "Hello"
        puts("Sent! (This prints after receive)")
    }
    
    // Small delay to see the ordering
    sleep(100)
    puts("About to receive...")
    sus msg = <-ch
    puts(sprintf("Received: %s", msg))
}

slay buffered_example() {
    sus ch = make(dm<tea>, 2)  // Buffer of 2 - asynchronous
    
    stan {
        puts("About to send...")
        ch <- "Hello"
        puts("Sent first message")
        ch <- "World"  
        puts("Sent second message")
        // Both sends complete immediately due to buffer
    }
    
    sleep(100)
    sus msg1 = <-ch
    sus msg2 = <-ch
    puts(sprintf("Received: %s %s", msg1, msg2))
}
```

## Part 2: Communication Patterns

### The Producer-Consumer Pattern

One of the most common patterns - one goroutine produces data, another consumes it:

```cursed
slay producer(ch dm<normie>, count normie) {
    puts("Producer starting...")
    sus i = 0
    periodt i < count {
        puts(sprintf("Producing %d", i))
        ch <- i
        sleep(500)  // Simulate work
        i++
    }
    close(ch)  // Signal we're done
    puts("Producer finished")
}

slay consumer(ch dm<normie>, id normie) {
    puts(sprintf("Consumer %d starting...", id))
    periodt based {
        sus value, ok = <-ch
        lowkey !ok {
            puts(sprintf("Consumer %d finished - channel closed", id))
            yolo
        }
        puts(sprintf("Consumer %d consumed: %d", id, value))
        sleep(200)  // Simulate processing
    }
}

slay producer_consumer_demo() {
    sus ch = make(dm<normie>, 3)  // Small buffer
    
    // Start one producer
    stan producer(ch, 10)
    
    // Start multiple consumers
    stan consumer(ch, 1)
    stan consumer(ch, 2)
    
    // Wait for everything to finish
    sleep(8000)  // 8 seconds
}
```

### Pipeline Processing

Chain multiple processing stages together:

```cursed
// Stage 1: Generate numbers
slay generate_numbers(out dm<normie>) {
    sus i = 1
    periodt i <= 10 {
        out <- i
        i++
    }
    close(out)
}

// Stage 2: Square the numbers  
slay square_numbers(in dm<normie>, out dm<normie>) {
    periodt based {
        sus num, ok = <-in
        lowkey !ok {
            close(out)
            yolo
        }
        out <- num * num
    }
}

// Stage 3: Print results
slay print_results(in dm<normie>) {
    periodt based {
        sus result, ok = <-in
        lowkey !ok {
            puts("Pipeline finished")
            yolo
        }
        puts(sprintf("Result: %d", result))
    }
}

slay pipeline_demo() {
    sus numbers = make(dm<normie>)
    sus squares = make(dm<normie>)
    
    stan generate_numbers(numbers)
    stan square_numbers(numbers, squares)
    stan print_results(squares)
    
    sleep(3000)  // Wait for completion
}
```

## Part 3: Advanced Select Operations

### Non-blocking Operations with `vibe_check`

The `vibe_check` statement lets you handle multiple channels at once:

```cursed
slay multi_channel_demo() {
    sus ch1 = make(dm<normie>)
    sus ch2 = make(dm<tea>)
    sus quit = make(dm<lit>)
    
    // Goroutine that sends random data
    stan {
        periodt based {
            vibe_check {
                mood ch1 <- 42:
                    puts("Sent number")
                mood ch2 <- "hello":
                    puts("Sent string")
                mood <-quit:
                    puts("Quitting sender")
                    yolo
            }
            sleep(1000)
        }
    }
    
    // Main loop handling multiple channels
    sus timeout = time.After(5000)  // 5 second timeout
    
    periodt based {
        vibe_check {
            mood num := <-ch1:
                puts(sprintf("Got number: %d", num))
            mood msg := <-ch2:
                puts(sprintf("Got message: %s", msg))
            mood <-timeout:
                puts("Timeout reached")
                quit <- based
                yolo
            basic:
                puts("No channels ready, doing other work...")
                sleep(100)
        }
    }
}
```

### Timeout Patterns

```cursed
slay receive_with_timeout(ch dm<normie>, timeout_ms normie) {
    sus timeout_ch = time.After(timeout_ms)
    
    vibe_check {
        mood value := <-ch:
            puts(sprintf("Received: %d", value))
        mood <-timeout_ch:
            puts("Operation timed out!")
    }
}

slay slow_operation_demo() {
    sus ch = make(dm<normie>)
    
    // Start a slow operation
    stan {
        sleep(3000)  // 3 seconds
        ch <- 99
    }
    
    // Try to receive with 2 second timeout
    receive_with_timeout(ch, 2000)  // Will timeout
    
    // Try again with 5 second timeout  
    receive_with_timeout(ch, 5000)  // Will succeed
}
```

## Part 4: Worker Pool Pattern

A powerful pattern for parallel processing:

```cursed
be_like Job squad {
    id normie
    data tea
}

be_like Result squad {
    job_id normie
    output tea
    error tea?
}

slay worker(id normie, jobs dm<Job>, results dm<Result>) {
    puts(sprintf("Worker %d started", id))
    
    periodt based {
        sus job, ok = <-jobs
        lowkey !ok {
            puts(sprintf("Worker %d stopping", id))
            yolo
        }
        
        puts(sprintf("Worker %d processing job %d", id, job.id))
        
        // Simulate work
        sleep(random(500, 2000))
        
        // Create result
        sus result = Result{
            job_id: job.id,
            output: sprintf("Processed: %s", job.data),
            error: cap  // No error
        }
        
        results <- result
    }
}

slay worker_pool_demo() {
    sus num_workers = 3
    sus num_jobs = 10
    
    sus jobs = make(dm<Job>, num_jobs)
    sus results = make(dm<Result>, num_jobs)
    
    // Start workers
    sus i = 0
    periodt i < num_workers {
        stan worker(i, jobs, results)
        i++
    }
    
    // Send jobs
    puts("Sending jobs...")
    i = 0
    periodt i < num_jobs {
        sus job = Job{
            id: i,
            data: sprintf("Job data %d", i)
        }
        jobs <- job
        i++
    }
    close(jobs)  // Signal no more jobs
    
    // Collect results
    puts("Collecting results...")
    i = 0
    periodt i < num_jobs {
        sus result = <-results
        puts(sprintf("Job %d result: %s", result.job_id, result.output))
        i++
    }
    
    puts("All jobs completed!")
}
```

## Part 5: Fan-in and Fan-out Patterns

### Fan-out: Distribute work to multiple workers

```cursed
slay fan_out_demo() {
    sus input = make(dm<normie>)
    sus output1 = make(dm<normie>)
    sus output2 = make(dm<normie>)
    sus output3 = make(dm<normie>)
    
    // Fan-out function
    stan {
        periodt based {
            sus value, ok = <-input
            lowkey !ok {
                close(output1)
                close(output2)
                close(output3)
                yolo
            }
            
            // Send to all outputs
            output1 <- value
            output2 <- value
            output3 <- value
        }
    }
    
    // Workers
    stan process_channel(output1, "Worker A")
    stan process_channel(output2, "Worker B")
    stan process_channel(output3, "Worker C")
    
    // Send data
    sus i = 1
    periodt i <= 5 {
        input <- i
        i++
    }
    close(input)
    
    sleep(3000)
}

slay process_channel(ch dm<normie>, name tea) {
    periodt based {
        sus value, ok = <-ch
        lowkey !ok {
            puts(sprintf("%s finished", name))
            yolo
        }
        puts(sprintf("%s processed: %d", name, value))
        sleep(500)
    }
}
```

### Fan-in: Merge multiple sources

```cursed
slay fan_in[T](inputs []dm<T>) dm<T> {
    sus output = make(dm<T>)
    sus count = len(inputs)
    
    // Start a goroutine for each input
    sus i = 0
    periodt i < count {
        sus input_ch = inputs[i]
        stan {
            periodt based {
                sus value, ok = <-input_ch
                lowkey !ok {
                    yolo
                }
                output <- value
            }
        }
        i++
    }
    
    yolo output
}

slay fan_in_demo() {
    sus ch1 = make(dm<tea>)
    sus ch2 = make(dm<tea>)
    sus ch3 = make(dm<tea>)
    
    sus inputs = []dm<tea>{ch1, ch2, ch3}
    sus merged = fan_in(inputs)
    
    // Producers
    stan produce_messages(ch1, "Source A", 3)
    stan produce_messages(ch2, "Source B", 3) 
    stan produce_messages(ch3, "Source C", 3)
    
    // Consumer
    stan {
        sus received = 0
        periodt received < 9 {  // 3 sources × 3 messages each
            sus msg = <-merged
            puts(sprintf("Merged: %s", msg))
            received++
        }
    }
    
    sleep(4000)
}

slay produce_messages(ch dm<tea>, source tea, count normie) {
    sus i = 1
    periodt i <= count {
        sus msg = sprintf("%s message %d", source, i)
        ch <- msg
        sleep(random(200, 800))
        i++
    }
    close(ch)
}
```

## Part 6: Error Handling and Best Practices

### Graceful Shutdown Pattern

```cursed
slay graceful_shutdown_demo() {
    sus work = make(dm<normie>)
    sus shutdown = make(dm<lit>)
    sus done = make(dm<lit>)
    
    // Worker with graceful shutdown
    stan {
        puts("Worker started")
        periodt based {
            vibe_check {
                mood job := <-work:
                    puts(sprintf("Processing job: %d", job))
                    sleep(1000)  // Simulate work
                    puts(sprintf("Completed job: %d", job))
                mood <-shutdown:
                    puts("Shutdown signal received")
                    done <- based
                    yolo
            }
        }
    }
    
    // Send some work
    sus i = 1
    periodt i <= 3 {
        work <- i
        i++
    }
    
    // Wait a bit, then shutdown
    sleep(2500)
    puts("Initiating shutdown...")
    shutdown <- based
    
    // Wait for graceful shutdown
    <-done
    puts("Worker shut down gracefully")
}
```

### Error Handling in Channels

```cursed
be_like WorkItem squad {
    id normie
    data tea
}

be_like WorkResult squad {
    id normie
    result tea
    error tea?
}

slay safe_worker(work dm<WorkItem>, results dm<WorkResult>) {
    periodt based {
        sus item, ok = <-work
        lowkey !ok {
            yolo
        }
        
        // Simulate work that might fail
        stan_it {
            sus processed = process_data(item.data)
            results <- WorkResult{
                id: item.id,
                result: processed,
                error: cap
            }
        } no_cap err {
            results <- WorkResult{
                id: item.id,
                result: "",
                error: err
            }
        }
    }
}

slay process_data(data tea) tea? {
    // Simulate processing that might fail
    lowkey data == "bad_data" {
        yolo "", Error.new("Invalid data")
    }
    yolo sprintf("Processed: %s", data), cap
}
```

## Part 7: Common Pitfalls and How to Avoid Them

### Pitfall 1: Deadlock with Unbuffered Channels

```cursed
// ❌ DON'T DO THIS - Deadlock!
slay deadlock_example() {
    sus ch = make(dm<normie>)
    ch <- 42  // Blocks forever - no receiver!
    sus value = <-ch  // Never reached
}

// ✅ DO THIS - Use goroutine or buffer
slay fixed_example() {
    sus ch = make(dm<normie>)
    
    stan {
        ch <- 42
    }
    
    sus value = <-ch
    puts(sprintf("Received: %d", value))
}
```

### Pitfall 2: Forgetting to Close Channels

```cursed
// ❌ DON'T DO THIS - Goroutine hangs
slay bad_range_example() {
    sus ch = make(dm<normie>)
    
    stan {
        sus i = 0
        periodt i < 5 {
            ch <- i
            i++
        }
        // Forgot to close!
    }
    
    // This hangs after receiving 5 values
    periodt value := range ch {
        puts(sprintf("Value: %d", value))
    }
}

// ✅ DO THIS - Always close when done sending
slay good_range_example() {
    sus ch = make(dm<normie>)
    
    stan {
        defer close(ch)  // Always close
        sus i = 0
        periodt i < 5 {
            ch <- i
            i++
        }
    }
    
    periodt value := range ch {
        puts(sprintf("Value: %d", value))
    }
}
```

### Pitfall 3: Race Conditions with Channel Length

```cursed
// ❌ DON'T DO THIS - Race condition
slay bad_length_check() {
    sus ch = make(dm<normie>, 5)
    
    lowkey len(ch) < cap(ch) {
        // Another goroutine might fill the channel here!
        ch <- 42  // Might block unexpectedly
    }
}

// ✅ DO THIS - Use select for non-blocking
slay good_non_blocking_send() {
    sus ch = make(dm<normie>, 5)
    
    vibe_check {
        mood ch <- 42:
            puts("Sent successfully")
        basic:
            puts("Channel full")
    }
}
```

## Part 8: Performance Tips

### Choosing Buffer Size

```cursed
// Small buffer for synchronization
sus sync_ch = make(dm<normie>, 1)

// Medium buffer for moderate decoupling
sus medium_ch = make(dm<normie>, 10)

// Large buffer for high throughput
sus throughput_ch = make(dm<normie>, 1000)

// Unbuffered for strict synchronization
sus strict_ch = make(dm<normie>)
```

### Batch Processing

```cursed
slay batch_processor(input dm<normie>, batch_size normie) {
    sus batch = make([]normie, 0, batch_size)
    
    periodt based {
        sus value, ok = <-input
        lowkey !ok {
            // Process final batch
            lowkey len(batch) > 0 {
                process_batch(batch)
            }
            yolo
        }
        
        batch = append(batch, value)
        
        lowkey len(batch) >= batch_size {
            process_batch(batch)
            batch = batch[:0]  // Reset slice
        }
    }
}

slay process_batch(batch []normie) {
    puts(sprintf("Processing batch of %d items", len(batch)))
    // Process all items together for efficiency
}
```

## Summary

Channels in CURSED provide a powerful way to build concurrent programs. Key takeaways:

1. **Start simple** - Use basic send/receive before advanced patterns
2. **Choose the right buffer size** - Unbuffered for sync, buffered for throughput
3. **Always close channels** when done sending
4. **Use `vibe_check`** for non-blocking and multi-channel operations
5. **Handle errors gracefully** with proper shutdown patterns
6. **Avoid common pitfalls** like deadlocks and race conditions

With these patterns, you can build robust concurrent applications that are both performant and maintainable. The key is to start with simple patterns and gradually build up to more complex communication flows as needed.

Practice with these examples and experiment with your own patterns. Channels make concurrent programming much safer and more intuitive than traditional synchronization primitives!
