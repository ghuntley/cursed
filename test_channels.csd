// Test channel operations and select statements
yeet "concurrenz"

slay test_basic_channels() {
    vibez.spill("=== Testing Basic Channel Operations ===")
    
    // Create buffered channel
    sus ch dm[drip] = dm[drip](3)
    
    // Send some values
    ch <- 10
    ch <- 20
    ch <- 30
    
    vibez.spill("Sent three values to buffered channel")
    
    // Receive values
    sus val1 drip = <-ch
    sus val2 drip = <-ch
    sus val3 drip = <-ch
    
    vibez.spill("Received:", val1, val2, val3)
}

slay test_unbuffered_channels() {
    vibez.spill("\n=== Testing Unbuffered Channels ===")
    
    // Create unbuffered channel
    sus ch dm[tea] = dm[tea]()
    
    // Goroutine to send message
    stan {
        ch <- "Hello from goroutine!"
        vibez.spill("Message sent from goroutine")
    }
    
    // Receive message in main thread
    sus message tea = <-ch
    vibez.spill("Received:", message)
}

slay test_select_statements() {
    vibez.spill("\n=== Testing Select Statements ===")
    
    sus ch1 dm[drip] = dm[drip](1)
    sus ch2 dm[tea] = dm[tea](1)
    sus done dm[lit] = dm[lit]()
    
    // Send values to channels
    stan {
        sleep(100) // Wait 100ms
        ch1 <- 42
    }
    
    stan {
        sleep(200) // Wait 200ms
        ch2 <- "delayed message"
    }
    
    stan {
        sleep(500) // Wait 500ms
        done <- based
    }
    
    // Select statement to handle multiple channels
    ready {
        case num := <-ch1: {
            vibez.spill("Received number:", num)
        }
        case msg := <-ch2: {
            vibez.spill("Received message:", msg)
        }
        case <-done: {
            vibez.spill("Done signal received")
        }
        default: {
            vibez.spill("No channels ready")
        }
    }
    
    // Another select with timeout
    ready {
        case num := <-ch1: {
            vibez.spill("Got number:", num)
        }
        case msg := <-ch2: {
            vibez.spill("Got message:", msg)
        }
        timeout 300ms: {
            vibez.spill("Select timed out after 300ms")
        }
    }
}

slay test_channel_closing() {
    vibez.spill("\n=== Testing Channel Closing ===")
    
    sus ch dm[drip] = dm[drip](2)
    
    // Send values and close
    ch <- 1
    ch <- 2
    close(ch)
    
    // Receive until channel is closed
    bestie (based) {
        sus (value, ok) = <-ch
        ready (!ok) {
            vibez.spill("Channel closed, breaking")
            break
        }
        vibez.spill("Received:", value)
    }
}

slay producer(ch dm[drip], start drip, count drip) {
    bestie (i < count) {
        ch <- start + i
        vibez.spill("Produced:", start + i)
        sleep(50) // Small delay
        i = i + 1
    }
    close(ch)
    vibez.spill("Producer finished")
}

slay consumer(ch dm[drip], id drip) {
    bestie (based) {
        sus (value, ok) = <-ch
        ready (!ok) {
            vibez.spill("Consumer", id, "done - channel closed")
            break
        }
        vibez.spill("Consumer", id, "consumed:", value)
        sleep(75) // Processing time
    }
}

slay test_producer_consumer() {
    vibez.spill("\n=== Testing Producer-Consumer Pattern ===")
    
    sus ch dm[drip] = dm[drip](5)
    
    // Start producer
    stan producer(ch, 100, 10)
    
    // Start multiple consumers
    stan consumer(ch, 1)
    stan consumer(ch, 2)
    
    // Wait for completion
    sleep(2000) // 2 seconds
    vibez.spill("Producer-consumer test completed")
}

slay test_fan_out_pattern() {
    vibez.spill("\n=== Testing Fan-out Pattern ===")
    
    sus input dm[drip] = dm[drip](3)
    sus output1 dm[drip] = dm[drip](3)
    sus output2 dm[drip] = dm[drip](3)
    
    // Fan-out goroutine
    stan {
        bestie (based) {
            sus (value, ok) = <-input
            ready (!ok) {
                close(output1)
                close(output2)
                break
            }
            
            // Send to both outputs
            output1 <- value * 2
            output2 <- value * 3
        }
        vibez.spill("Fan-out completed")
    }
    
    // Send input values
    input <- 1
    input <- 2
    input <- 3
    close(input)
    
    // Collect results
    vibez.spill("Output1 (x2):")
    bestie (based) {
        sus (value, ok) = <-output1
        ready (!ok) break
        vibez.spill("  ", value)
    }
    
    vibez.spill("Output2 (x3):")
    bestie (based) {
        sus (value, ok) = <-output2
        ready (!ok) break
        vibez.spill("  ", value)
    }
}

slay main() {
    // Initialize scheduler
    init_scheduler(4) // 4 worker threads
    
    test_basic_channels()
    test_unbuffered_channels()
    test_select_statements()
    test_channel_closing()
    test_producer_consumer()
    test_fan_out_pattern()
    
    vibez.spill("\n=== All Channel Tests Complete ===")
    
    // Cleanup
    shutdown_scheduler()
}
