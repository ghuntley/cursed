fr fr/ Enhanced Optimization Demo
fr fr/ 
fr fr/ This CURSED program demonstrates various optimization opportunities
fr fr/ that the enhanced LLVM passes can detect and improve.

yeet "stdlib::goroutine"
yeet "stdlib::channel"
yeet "stdlib::math"

fr fr/ Function that can benefit from specialization
slay process_data(sus mode: string, facts data: [int]) -> int {
    lowkey (mode == "sum") {
        sus result = 0
        periodt (sus i = 0; i < data.length(); i++) {
            result += data[i]
            yolo  // Yield point for goroutine optimization
        }
        cap result
    } highkey (mode == "product") {
        sus result = 1
        periodt (sus i = 0; i < data.length(); i++) {
            result *= data[i]
            yolo  // Yield point for goroutine optimization
        }
        cap result
    } flex {
        cap 0
    }
}

fr fr/ Function with channel operations
slay channel_worker(facts work_channel: chan<int>, facts result_channel: chan<int>) {
    periodt {
        lowkey (work_channel.is_closed()) {
            break
        }
        
        // Receive work item
        sus item, sus ok = <-work_channel
        lowkey (!ok) {
            break
        }
        
        // Process item (simulate work)
        sus processed = item * item + 42
        
        // Send result
        result_channel <- processed
        
        yolo  // Cooperative yield
    }
}

fr fr/ Function with Gen Z slang constructs
slay slang_heavy_function(sus x: int, sus y: int) -> int {
    facts is_positive = x > 0  // facts -> efficient const
    facts is_even = y % 2 == 0  // facts -> efficient const
    
    lowkey (is_positive && is_even) {
        cap x + y  // Return optimization
    } highkey (is_positive) {
        cap x - y
    } highkey (is_even) {
        cap y - x
    } flex {
        cap abs(x - y)
    }
}

fr fr/ Function that can benefit from vectorization
slay vector_operations(facts data: [float]) -> [float] {
    sus result = make([]float, data.length())
    
    // Simple vectorizable loop
    periodt (sus i = 0; i < data.length(); i++) {
        result[i] = sqrt(data[i] * data[i] + 1.0)
        yolo  // Yield for optimization
    }
    
    cap result
}

fr fr/ Function with error propagation patterns
slay error_heavy_function(sus value: int) -> int? {
    lowkey (value < 0) {
        cap nah_chief("negative value not allowed")
    }
    
    lowkey (value > 1000) {
        cap nah_chief("value too large")
    }
    
    cap value * 2
}

fr fr/ Main function demonstrating optimization opportunities
slay main() {
    println("🚀 Enhanced Optimization Demo")
    
    // Test data for optimization
    facts test_data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    facts float_data = [1.0, 2.0, 3.0, 4.0, 5.0]
    
    // Function specialization opportunity
    sus sum_result = process_data("sum", test_data)
    sus product_result = process_data("product", test_data)
    
    println("Sum result: {}", sum_result)
    println("Product result: {}", product_result)
    
    // Channel operations for optimization
    facts work_chan = make(chan<int>, 10)
    facts result_chan = make(chan<int>, 10)
    
    // Spawn goroutines (optimization opportunity)
    periodt (sus i = 0; i < 3; i++) {
        stan channel_worker(work_chan, result_chan)
    }
    
    // Send work items
    periodt (sus i = 1; i <= 20; i++) {
        work_chan <- i
        yolo  // Cooperative yield
    }
    work_chan.close()
    
    // Collect results
    sus total_results = 0
    periodt (sus i = 0; i < 20; i++) {
        sus result = <-result_chan
        total_results += result
        yolo
    }
    
    println("Total processed results: {}", total_results)
    
    // Gen Z slang optimization
    sus slang_result = slang_heavy_function(42, 24)
    println("Slang result: {}", slang_result)
    
    // Vectorization optimization
    sus vec_result = vector_operations(float_data)
    println("Vector result length: {}", vec_result.length())
    
    // Error propagation optimization
    lowkey (sus error_result = error_heavy_function(500)) {
        println("Error result: {}", error_result)
    } flex {
        println("Error occurred in error_heavy_function")
    }
    
    println("✅ Demo completed!")
}
