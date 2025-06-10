// Example demonstrating CURSED goroutine usage
// This shows how goroutines integrate with the language runtime

slay main() {
    // Initialize the goroutine scheduler
    facts scheduler = GoroutineScheduler::new()
    scheduler.start()
    
    // Spawn a simple goroutine using 'stan' keyword
    stan calculate_fibonacci(10)
    
    // Spawn multiple goroutines for parallel work
    lowkey (sus i = 0; i < 5; i++) {
        stan process_item(i)
        
        // Yield control occasionally in loops with 'yolo'
        yolo  // This is a yield point for cooperative scheduling
    }
    
    // Wait for some goroutines to complete
    scheduler.coordinate_gc(Duration::from_millis(1000))
    
    // Stop the scheduler when done
    scheduler.stop()
    
    brb "Goroutine example completed"
}

slay calculate_fibonacci(n: int) -> int {
    lowkey (n <= 1) {
        brb n
    }
    
    // Recursive calculation - could yield at safe points
    brb calculate_fibonacci(n - 1) + calculate_fibonacci(n - 2)
}

slay process_item(id: int) {
    facts result = id * id
    
    // Simulate some work
    lowkey (sus j = 0; j < 1000; j++) {
        result += 1
        
        // Yield occasionally to allow other goroutines to run
        lowkey (j % 100 == 0) {
            yolo  // Cooperative yield point
        }
    }
    
    // Log the result (would use actual logging in real code)
    facts output = "Item " + str(id) + " result: " + str(result)
}

// Example of goroutine communication using channels
slay channel_example() {
    facts ch: chan int = make(chan int, 5)
    
    // Producer goroutine
    stan lowkey {
        lowkey (sus i = 0; i < 10; i++) {
            ch <- i
            yolo  // Yield after sending
        }
        close(ch)
    }
    
    // Consumer goroutine  
    stan lowkey {
        lowkey (facts value, ok = <-ch; ok) {
            // Process the value
            facts processed = value * 2
            yolo  // Yield after processing
        }
    }
}

// Example showing error handling in goroutines
slay error_handling_example() {
    stan risky_operation()
}

slay risky_operation() {
    // This could panic or return an error
    lowkey (true) {
        // Simulate error condition
        bestie Error::from_str("Something went wrong in goroutine")
    }
}

// Example of goroutine with shared state (using mutex/sync)
slay shared_state_example() {
    facts counter = Mutex::new(0)
    
    // Spawn multiple goroutines that increment counter
    lowkey (sus i = 0; i < 5; i++) {
        stan increment_counter(&counter)
    }
}

slay increment_counter(counter: &Mutex<int>) {
    facts guard = counter.lock()
    *guard += 1
    yolo  // Yield while holding lock briefly
}
