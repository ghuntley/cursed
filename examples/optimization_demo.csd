// CURSED Optimization Demo
// This example demonstrates various code patterns that benefit from optimization

import "stdlib::math";
import "stdlib::io";

// Function with loops - benefits from loop optimization
slay factorial_loop(sus n: i32) -> i32 {
    sus result = 1;
    
    // This loop benefits from loop unrolling and optimization
    lowkey (sus i = 1; i <= n; i++) {
        result = result * i;
        
        // Yield point for goroutine optimization
        yolo;
    }
    
    return result;
}

// Recursive function - benefits from inlining
slay factorial_recursive(sus n: i32) -> i32 {
    lowkey (n <= 1) {
        return 1;
    } highkey {
        return n * factorial_recursive(n - 1);
    }
}

// Math-intensive function - benefits from vectorization
slay compute_stats(sus numbers: [f64]) -> (f64, f64, f64) {
    sus sum = 0.0;
    sus min_val = numbers[0];
    sus max_val = numbers[0];
    
    // Math operations that can be vectorized
    lowkey (sus i = 0; i < numbers.length; i++) {
        sum = sum + numbers[i];
        
        lowkey (numbers[i] < min_val) {
            min_val = numbers[i];
        }
        
        lowkey (numbers[i] > max_val) {
            max_val = numbers[i];
        }
    }
    
    sus avg = sum / numbers.length as f64;
    return (min_val, max_val, avg);
}

// Memory allocation patterns - benefits from GC-aware optimization
slay process_large_data() -> [i32] {
    sus large_array = [0; 10000];
    
    // Pattern that benefits from memory optimization
    lowkey (sus i = 0; i < 10000; i++) {
        large_array[i] = i * i;
    }
    
    return large_array;
}

// Goroutine usage - benefits from goroutine optimization
slay concurrent_processing() {
    // Spawn multiple goroutines
    lowkey (sus i = 0; i < 10; i++) {
        stan process_item(i);
    }
}

slay process_item(sus id: i32) {
    println(&format!("Processing item {}", id))?;
    
    // Simulate some work
    sus result = factorial_loop(id + 5);
    println(&format!("Item {} result: {}", id, result))?;
}

// Main function showcasing different optimization scenarios
slay main() -> CursedResult<()> {
    println("🚀 CURSED Optimization Demo")?;
    
    // Test factorial functions
    sus n = 10;
    sus loop_result = factorial_loop(n);
    sus recursive_result = factorial_recursive(n);
    
    println(&format!("Factorial of {} (loop): {}", n, loop_result))?;
    println(&format!("Factorial of {} (recursive): {}", n, recursive_result))?;
    
    // Test math operations
    sus test_data = [1.0, 5.0, 3.0, 9.0, 2.0, 7.0, 4.0, 8.0, 6.0];
    sus (min_val, max_val, avg) = compute_stats(test_data);
    
    println(&format!("Stats - Min: {}, Max: {}, Avg: {}", min_val, max_val, avg))?;
    
    // Test memory operations
    sus processed_data = process_large_data();
    println(&format!("Processed {} elements", processed_data.length))?;
    
    // Test concurrent processing
    concurrent_processing();
    
    println("✅ Demo completed successfully!")?;
    return Ok(());
}
