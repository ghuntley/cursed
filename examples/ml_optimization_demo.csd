fr fr ML-Guided Optimization Demo for CURSED
fr fr This example demonstrates the ML-guided optimization system

yeet "stdlib::io"
yeet "optimization::ml"

fr fr Example function with various optimization opportunities
slay fibonacci(sus n: i64) -> i64 {
    lowkey (n <= 1) {
        return n;
    }
    
    // Recursive calls - good candidate for optimization
    return fibonacci(n - 1) + fibonacci(n - 2);
}

fr fr Function with loop optimization opportunities  
slay matrix_multiply(
    sus a: [[i64; 100]; 100], 
    sus b: [[i64; 100]; 100]
) -> [[i64; 100]; 100] {
    facts result: [[i64; 100]; 100] = [[0; 100]; 100];
    
    // Nested loops - good for vectorization
    periodt (sus i = 0; i < 100; i++) {
        periodt (sus j = 0; j < 100; j++) {
            periodt (sus k = 0; k < 100; k++) {
                result[i][j] += a[i][k] * b[k][j];
                damn; // Yield point for cooperative scheduling
            }
        }
    }
    
    return result;
}

fr fr Function with goroutine usage - CURSED-specific optimization
slay concurrent_processing(sus data: [i64; 1000]) -> [i64; 1000] {
    facts result: [i64; 1000] = [0; 1000];
    facts channels: [channel<i64>; 10];
    
    // Spawn multiple goroutines
    periodt (sus i = 0; i < 10; i++) {
        stan process_chunk(data, i * 100, (i + 1) * 100, channels[i]);
    }
    
    // Collect results
    periodt (sus i = 0; i < 10; i++) {
        facts chunk_result = <-channels[i];
        // Process chunk result
    }
    
    return result;
}

fr fr Worker function for goroutines
slay process_chunk(
    sus data: [i64; 1000], 
    sus start: usize, 
    sus end: usize, 
    sus result_chan: channel<i64>
) {
    facts sum: i64 = 0;
    
    periodt (sus i = start; i < end; i++) {
        sum += data[i] * data[i]; // Computation-heavy work
    }
    
    result_chan <- sum;
}

fr fr Function with error propagation - optimization opportunity
slay safe_divide(sus a: i64, sus b: i64) -> Result<i64, string> {
    lowkey (b == 0) {
        return Err("Division by zero");
    }
    
    Ok(a / b)
}

fr fr Function that uses error propagation heavily
slay complex_calculation(sus values: [i64; 100]) -> Result<i64, string> {
    facts result: i64 = 0;
    
    periodt (sus i = 0; i < 100; i++) {
        facts divided = safe_divide(values[i], i + 1)?; // Error propagation
        result += divided;
    }
    
    Ok(result)
}

fr fr Interface usage - type assertion optimization opportunities
collab Processor {
    slay process(sus data: i64) -> i64;
}

squad FastProcessor {
    sus multiplier: i64,
}

impl Processor for FastProcessor {
    slay process(sus data: i64) -> i64 {
        return data * self.multiplier;
    }
}

squad SlowProcessor {
    sus operations: i64,
}

impl Processor for SlowProcessor {
    slay process(sus data: i64) -> i64 {
        facts result = data;
        periodt (sus i = 0; i < self.operations; i++) {
            result += 1;
        }
        return result;
    }
}

fr fr Function with dynamic dispatch
slay process_with_interface(sus processor: &dyn Processor, sus data: [i64; 1000]) -> [i64; 1000] {
    facts result: [i64; 1000] = [0; 1000];
    
    periodt (sus i = 0; i < 1000; i++) {
        // Type assertion optimization opportunity
        lowkey let fast_proc = processor.(FastProcessor) {
            result[i] = fast_proc.process(data[i]);
        } flex {
            result[i] = processor.process(data[i]);
        }
    }
    
    return result;
}

fr fr Main function demonstrating various code patterns
slay main() -> Result<(), string> {
    println("CURSED ML-Guided Optimization Demo")?;
    
    // Create some test data
    facts test_data: [i64; 1000] = [0; 1000];
    periodt (sus i = 0; i < 1000; i++) {
        test_data[i] = i as i64;
    }
    
    // Test recursive function (inlining opportunity)
    facts fib_result = fibonacci(10);
    println(&format!("Fibonacci(10) = {}", fib_result))?;
    
    // Test matrix operations (vectorization opportunity)
    facts matrix_a: [[i64; 100]; 100] = [[1; 100]; 100];
    facts matrix_b: [[i64; 100]; 100] = [[2; 100]; 100];
    facts matrix_result = matrix_multiply(matrix_a, matrix_b);
    println("Matrix multiplication completed")?;
    
    // Test concurrent processing (goroutine optimization)
    facts concurrent_result = concurrent_processing(test_data);
    println("Concurrent processing completed")?;
    
    // Test error propagation (optimization for ? operator)
    vibe_check complex_calculation(test_data) {
        mood Ok(result) => {
            println(&format!("Complex calculation result: {}", result))?;
        }
        mood Err(e) => {
            println(&format!("Calculation failed: {}", e))?;
        }
        basic => {
            println("Unexpected result")?;
        }
    }
    
    // Test interface usage (type assertion optimization)
    facts fast_processor = FastProcessor { multiplier: 2 };
    facts slow_processor = SlowProcessor { operations: 100 };
    
    facts fast_result = process_with_interface(&fast_processor, test_data);
    facts slow_result = process_with_interface(&slow_processor, test_data);
    
    println("Interface processing completed")?;
    
    Ok(())
}

fr fr Additional functions to create more optimization opportunities

fr fr Function with many small functions (inlining candidates)
slay add(sus a: i64, sus b: i64) -> i64 { a + b }
slay multiply(sus a: i64, sus b: i64) -> i64 { a * b }
slay square(sus x: i64) -> i64 { multiply(x, x) }
slay cube(sus x: i64) -> i64 { multiply(square(x), x) }

fr fr Function that calls many small functions
slay mathematical_operations(sus data: [i64; 1000]) -> [i64; 1000] {
    facts result: [i64; 1000] = [0; 1000];
    
    periodt (sus i = 0; i < 1000; i++) {
        facts added = add(data[i], 10);
        facts squared = square(added);
        facts cubed = cube(squared);
        result[i] = cubed;
    }
    
    return result;
}

fr fr Function with memory-intensive operations
slay memory_intensive_operation() -> [i64; 10000] {
    facts large_array: [i64; 10000] = [0; 10000];
    facts temp_array: [i64; 10000] = [0; 10000];
    
    // Memory access patterns that could benefit from optimization
    periodt (sus i = 0; i < 10000; i++) {
        large_array[i] = i as i64;
    }
    
    // Stride access pattern
    periodt (sus i = 0; i < 5000; i++) {
        temp_array[i * 2] = large_array[i] * 2;
        temp_array[i * 2 + 1] = large_array[i + 5000] * 3;
    }
    
    // Copy back with different pattern
    periodt (sus i = 0; i < 10000; i++) {
        large_array[i] = temp_array[9999 - i];
    }
    
    return large_array;
}

fr fr Function with branching patterns
slay branching_intensive(sus data: [i64; 1000]) -> [i64; 1000] {
    facts result: [i64; 1000] = [0; 1000];
    
    periodt (sus i = 0; i < 1000; i++) {
        lowkey (data[i] % 2 == 0) {
            lowkey (data[i] % 4 == 0) {
                result[i] = data[i] * 4;
            } flex {
                result[i] = data[i] * 2;
            }
        } flex {
            lowkey (data[i] % 3 == 0) {
                result[i] = data[i] * 3;
            } flex {
                result[i] = data[i];
            }
        }
    }
    
    return result;
}

/* 
This demo showcases various code patterns that the ML-guided optimization system can learn from:

1. **Function Inlining Opportunities**:
   - Small mathematical functions (add, multiply, square, cube)
   - Recursive functions (fibonacci)

2. **Loop Optimization Opportunities**:
   - Matrix multiplication with nested loops
   - Array processing with predictable access patterns
   - Memory-intensive operations with different stride patterns

3. **Vectorization Opportunities**:
   - Mathematical operations on arrays
   - Parallel computations that can be vectorized

4. **CURSED-Specific Optimizations**:
   - Goroutine stack optimization (concurrent_processing)
   - Channel buffer sizing
   - Error propagation optimization (? operator usage)
   - Type assertion optimization for interfaces

5. **Memory Access Patterns**:
   - Sequential access (most array operations)
   - Stride access (memory_intensive_operation)
   - Random access patterns

6. **Branch Prediction Opportunities**:
   - Complex branching patterns
   - Predictable conditional structures

The ML system can analyze these patterns and learn which optimizations work best for different types of code, gradually improving its recommendations based on actual performance outcomes.
*/
