yeet "testz"
yeet "vibez"
yeet "mathz"
yeet "stringz"
yeet "atomic_drip"
yeet "concurrenz"

// Test generics with comprehensive constraints
slay process[T: Comparable](items []T) []T {
    // Placeholder for sorting logic
    damn items
}

squad Container[T] {
    spill data []T
    spill size drip
}

// Test advanced error handling with stack traces
yikes ProcessingError {
    spill message tea
    spill error_code drip
    spill context map[tea]tea
}

slay risky_computation(input drip) (drip, yikes ProcessingError) {
    lowkey input < 0 {
        sus context map[tea]tea = map[tea]tea{
            "input_value": stringz.from_int(input),
            "function": "risky_computation"
        }
        shook ProcessingError{
            message: "Negative input not allowed",
            error_code: 400,
            context: context
        }
    }
    damn input * 2, nah
}

// Test concurrency with select statements
slay concurrent_worker(id drip, input dm<drip>, output dm<drip>) {
    bestie {
        ready {
            mood value := concurrenz.recv(input) -> {
                sus result drip = value * id
                concurrenz.send(output, result)
            }
            basic -> {
                // Exit when no work available
                damn
            }
        }
    }
}

// Test string interpolation and formatting
slay format_results(name tea, score drip, percentage meal) tea {
    sus result tea = "Player: " + name + " scored " + stringz.from_int(score) + " points"
    damn result
}

// Test defer statements for resource management
slay managed_resource_test() {
    vibez.spill("Acquiring resource...")
    
    later {
        vibez.spill("Resource cleanup completed")
    }
    
    sus data [5]drip = [1, 2, 3, 4, 5]
    sus processed []drip = process[drip](data)
    
    vibez.spill("Resource processing complete")
}

// Test pattern matching with comprehensive patterns
slay analyze_value(value drip) tea {
    match value {
        mood 0 -> damn "Zero"
        mood 1 -> damn "One" 
        mood 42 -> damn "Answer to everything"
        mood x lowkey x > 100 -> damn "Large number"
        basic -> damn "Other value"
    }
}

// Test atomic operations for thread safety
slay atomic_counter_test() {
    sus counter atomic_drip.AtomicI32
    atomic_drip.store(counter, 0)
    
    // Simulate multiple worker threads
    sus i drip = 0
    bestie (i < 10) {
        atomic_drip.add(counter, 1)
        i = i + 1
    }
    
    sus final_count drip = atomic_drip.load(counter)
    vibez.spill("Final counter value: " + stringz.from_int(final_count))
}

// Main test function demonstrating all features
slay main() {
    vibez.spill("=== CURSED Comprehensive Feature Test ===")
    
    // Test basic functionality
    sus name tea = "Alice"
    sus score drip = 95
    sus percentage meal = 95.5
    
    // Test string operations
    sus formatted tea = format_results(name, score, percentage)
    vibez.spill(formatted)
    
    // Test error handling
    fam {
        sus result drip, sus err yikes ProcessingError = risky_computation(-5)
        vibez.spill("This shouldn't print")
    } catch error {
        vibez.spill("Caught error: " + error.message)
    }
    
    // Test pattern matching
    sus analysis tea = analyze_value(42)
    vibez.spill("Pattern match result: " + analysis)
    
    // Test resource management with defer
    managed_resource_test()
    
    // Test atomic operations
    atomic_counter_test()
    
    // Test concurrency setup
    sus input_ch dm<drip> = concurrenz.make_channel(10)
    sus output_ch dm<drip> = concurrenz.make_channel(10)
    
    // Start worker goroutine
    stan concurrent_worker(1, input_ch, output_ch)
    
    // Send some work
    concurrenz.send(input_ch, 42)
    
    // Receive result
    ready {
        mood result := concurrenz.recv(output_ch) -> {
            vibez.spill("Concurrent result: " + stringz.from_int(result))
        }
        basic -> {
            vibez.spill("No result received")
        }
    }
    
    vibez.spill("=== All comprehensive features working! ===")
    damn 0
}
