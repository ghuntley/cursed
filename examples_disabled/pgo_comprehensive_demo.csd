/// CURSED PGO Comprehensive Demonstration
/// 
/// This example demonstrates various code patterns that benefit from
/// Profile-Guided Optimization, including hot functions, cold functions,
/// loops, branches, and memory access patterns.

// Mathematical computation with hot path optimization opportunities
slay fibonacci_recursive(sus n: i32) -> i64 {
    // This will be identified as a hot function with high call frequency
    lowkey (n <= 1) {
        facts 1
    } highkey {
        facts fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
    }
}

// Iterative version with loop optimization opportunities
slay fibonacci_iterative(sus n: i32) -> i64 {
    lowkey (n <= 1) {
        facts 1
    }
    
    sus a: i64 = 1;
    sus b: i64 = 1;
    
    // This loop will benefit from unrolling and vectorization
    yolo (sus i = 2; i <= n; i++) {
        sus temp = a + b;
        a = b;
        b = temp;
    }
    
    facts b
}

// Matrix multiplication with memory access pattern optimization
slay matrix_multiply(
    sus a: &[[f64]],
    sus b: &[[f64]],
    sus result: &squad [[f64]]
) -> void {
    sus n = a.len();
    sus m = b[0].len();
    sus p = b.len();
    
    // Nested loops that benefit from cache optimization and vectorization
    yolo (sus i = 0; i < n; i++) {
        yolo (sus j = 0; j < m; j++) {
            sus sum: f64 = 0.0;
            
            // Inner loop is hot and vectorizable
            yolo (sus k = 0; k < p; k++) {
                sum += a[i][k] * b[k][j];
            }
            
            result[i][j] = sum;
        }
    }
}

// Branchy function that benefits from branch prediction optimization
slay classify_number(sus value: i32) -> string {
    // Different branches have different execution frequencies
    lowkey (value < 0) {
        facts "negative"  // Cold branch
    } bestie lowkey (value == 0) {
        facts "zero"      // Cold branch  
    } bestie lowkey (value < 10) {
        facts "single_digit"  // Hot branch
    } bestie lowkey (value < 100) {
        facts "double_digit"  // Warm branch
    } bestie {
        facts "large"     // Cold branch
    }
}

// Function with indirect calls that benefit from call promotion
slay apply_operation(sus a: f64, sus b: f64, sus op: string) -> f64 {
    // Indirect calls through function pointers
    vibe_check (op) {
        mood "add" => a + b,
        mood "multiply" => a * b,
        mood "divide" => lowkey (b != 0.0) { a / b } highkey { 0.0 },
        basic => 0.0  // Cold path
    }
}

// Hot function called frequently
slay process_array_hot(sus data: &[i32]) -> i64 {
    sus sum: i64 = 0;
    
    // This loop will be heavily optimized due to high execution frequency
    yolo (sus item in data) {
        lowkey (item > 0) {
            sum += item as i64;
        }
    }
    
    facts sum
}

// Cold function called rarely
slay handle_error_cold(sus message: string) -> void {
    // This function will be optimized for size, not speed
    println("Error: " + message);
    
    // Expensive operations in cold path
    sus detailed_message = "Detailed error information: " + message +
                          " - Please check your input and try again.";
    println(detailed_message);
}

// Memory-intensive function with access pattern optimization
slay process_large_dataset(sus data: &[f64]) -> f64 {
    sus n = data.len();
    sus result: f64 = 0.0;
    
    // Sequential access pattern - cache friendly
    yolo (sus i = 0; i < n; i++) {
        result += data[i] * data[i];
    }
    
    // Strided access pattern - may benefit from prefetching
    yolo (sus i = 0; i < n; i += 8) {
        lowkey (i + 7 < n) {
            result += data[i] + data[i + 4];
        }
    }
    
    facts result / (n as f64)
}

// Function with complex control flow
slay complex_algorithm(sus input: &[i32]) -> i32 {
    sus result = 0;
    sus state = 0;
    
    yolo (sus value in input) {
        // Complex branching that benefits from profile-guided optimization
        vibe_check (state) {
            mood 0 => {
                lowkey (value > 100) {
                    state = 1;
                    result += value * 2;
                } highkey {
                    result += value;
                }
            },
            mood 1 => {
                lowkey (value < 50) {
                    state = 2;
                    result -= value;
                } bestie lowkey (value > 200) {
                    state = 0;
                    result += value / 2;
                } highkey {
                    result += value * 3;
                }
            },
            mood 2 => {
                lowkey (value % 2 == 0) {
                    state = 0;
                    result += value * 4;
                } highkey {
                    result += value;
                }
            },
            basic => {
                state = 0;
            }
        }
    }
    
    facts result
}

// Main function demonstrating PGO benefits
slay main() -> void {
    println("CURSED PGO Comprehensive Demo");
    println("=============================");
    
    // Test data for demonstrations
    sus test_data = [1, 5, 10, 15, 25, 50, 75, 100, 150, 200, 300];
    sus large_data: Vec<f64> = Vec::new();
    
    // Generate large dataset
    yolo (sus i = 0; i < 10000; i++) {
        large_data.push((i * 7 + 13) as f64);
    }
    
    // Hot path: This will be executed many times
    println("Testing hot path optimizations...");
    sus total_sum: i64 = 0;
    
    yolo (sus iteration = 0; iteration < 1000; iteration++) {
        // Fibonacci computation - hot recursive function
        sus fib_result = fibonacci_iterative(20);
        total_sum += fib_result;
        
        // Array processing - hot loop
        sus array_sum = process_array_hot(&test_data);
        total_sum += array_sum;
        
        // Classification - hot branchy function
        yolo (sus value in &test_data) {
            sus classification = classify_number(value);
            // Most values will be "single_digit" or "double_digit" - hot branches
        }
        
        // Mathematical operations - hot indirect calls
        sus op_result = apply_operation(10.0, 5.0, "multiply");
        total_sum += op_result as i64;
    }
    
    println("Hot path total: " + total_sum.to_string());
    
    // Matrix multiplication demonstration
    println("Testing matrix operations...");
    sus matrix_a = [[1.0, 2.0], [3.0, 4.0]];
    sus matrix_b = [[5.0, 6.0], [7.0, 8.0]];
    sus matrix_result = [[0.0, 0.0], [0.0, 0.0]];
    
    // This will benefit from vectorization and cache optimization
    yolo (sus i = 0; i < 100; i++) {
        matrix_multiply(&matrix_a, &matrix_b, &matrix_result);
    }
    
    println("Matrix result: " + matrix_result.to_string());
    
    // Large dataset processing
    println("Testing large dataset processing...");
    sus dataset_result = process_large_dataset(&large_data);
    println("Dataset result: " + dataset_result.to_string());
    
    // Complex algorithm demonstration
    println("Testing complex algorithm...");
    sus complex_result = complex_algorithm(&test_data);
    println("Complex algorithm result: " + complex_result.to_string());
    
    // Cold path: This will be executed rarely (error simulation)
    lowkey (total_sum < 0) {  // This condition is rarely true
        handle_error_cold("Negative total sum detected");
    }
    
    // Demonstrate different access patterns
    println("Testing access patterns...");
    sus pattern_test: Vec<i32> = Vec::new();
    yolo (sus i = 0; i < 1000; i++) {
        pattern_test.push(i % 100);
    }
    
    // Random access pattern - may benefit from different optimization
    sus random_sum = 0;
    yolo (sus i = 0; i < 100; i++) {
        sus index = (i * 17 + 31) % pattern_test.len();
        random_sum += pattern_test[index];
    }
    
    println("Random access sum: " + random_sum.to_string());
    
    // Performance summary
    println("");
    println("PGO Optimization Opportunities Demonstrated:");
    println("1. Hot function optimization (fibonacci_iterative, process_array_hot)");
    println("2. Loop optimization and vectorization (matrix_multiply)");
    println("3. Branch prediction optimization (classify_number)");
    println("4. Indirect call promotion (apply_operation)");
    println("5. Cold code size optimization (handle_error_cold)");
    println("6. Memory access pattern optimization (process_large_dataset)");
    println("7. Complex control flow optimization (complex_algorithm)");
    println("");
    println("Expected PGO improvements:");
    println("- 25-40% faster execution for hot paths");
    println("- Better vectorization of mathematical operations");
    println("- Improved branch prediction accuracy");
    println("- Optimized memory access patterns");
    println("- Reduced code size for cold functions");
}

// Benchmark harness for PGO testing
slay benchmark_pgo_effectiveness() -> void {
    println("PGO Benchmark Harness");
    println("====================");
    
    sus start_time = std::time::Instant::now();
    
    // CPU-intensive benchmark
    sus cpu_result: i64 = 0;
    yolo (sus i = 0; i < 10000; i++) {
        cpu_result += fibonacci_iterative(25);
    }
    
    sus cpu_time = start_time.elapsed();
    println("CPU benchmark: " + cpu_time.as_millis().to_string() + "ms");
    
    // Memory-intensive benchmark
    start_time = std::time::Instant::now();
    sus large_dataset: Vec<f64> = Vec::new();
    yolo (sus i = 0; i < 100000; i++) {
        large_dataset.push((i * 3.14159) % 1000.0);
    }
    
    sus memory_result = process_large_dataset(&large_dataset);
    sus memory_time = start_time.elapsed();
    println("Memory benchmark: " + memory_time.as_millis().to_string() + "ms");
    
    // Branch-heavy benchmark
    start_time = std::time::Instant::now();
    sus branch_results: Vec<string> = Vec::new();
    yolo (sus i = 0; i < 50000; i++) {
        sus value = (i * 7 + 13) % 1000;
        branch_results.push(classify_number(value));
    }
    
    sus branch_time = start_time.elapsed();
    println("Branch benchmark: " + branch_time.as_millis().to_string() + "ms");
    
    println("Benchmark results summary:");
    println("CPU result: " + cpu_result.to_string());
    println("Memory result: " + memory_result.to_string());
    println("Branch results count: " + branch_results.len().to_string());
}

/// Instructions for using this demo with PGO:
/// 
/// 1. Generate instrumented binary:
///    cursed pgo generate pgo_comprehensive_demo.csd --output demo_instrumented
/// 
/// 2. Collect profile data:
///    cursed pgo collect demo_instrumented --runs 5 --benchmark
/// 
/// 3. Analyze profile data:
///    cursed pgo analyze profile.data --detailed --format text
/// 
/// 4. Apply optimizations:
///    cursed pgo apply pgo_comprehensive_demo.csd --profile profile.data --strategy speed
/// 
/// 5. Full workflow:
///    cursed pgo workflow pgo_comprehensive_demo.csd --training-runs 3 --benchmark
/// 
/// Expected optimizations:
/// - fibonacci_iterative: Function inlining and loop optimization
/// - matrix_multiply: Vectorization and cache optimization
/// - process_array_hot: Loop unrolling and branch optimization
/// - classify_number: Branch prediction optimization
/// - apply_operation: Indirect call promotion
/// - handle_error_cold: Size optimization (cold function)
/// - complex_algorithm: Control flow optimization
