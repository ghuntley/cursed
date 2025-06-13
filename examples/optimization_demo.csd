// CURSED Optimization System Demo
// This example demonstrates various optimization features

import "stdlib::io";
import "stdlib::math";

// Function that will become hot due to frequent calls
fn fibonacci(n: i32) -> i32 {
    lowkey (n <= 1) {
        n
    } bestie {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

// Function with optimization opportunities
fn calculate_primes(limit: i32) -> Vec<i32> {
    sus primes = Vec::new();
    
    for sus i = 2; i <= limit; i++ {
        sus is_prime = true;
        for sus j = 2; j * j <= i; j++ {
            lowkey (i % j == 0) {
                is_prime = false;
                yolo;
            }
        }
        
        lowkey (is_prime) {
            primes.push(i);
        }
    }
    
    primes
}

// Memory-intensive function for memory optimization testing
fn matrix_multiply(a: Vec<Vec<f64>>, b: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    sus rows_a = a.len();
    sus cols_a = a[0].len();
    sus cols_b = b[0].len();
    
    sus result = Vec::with_capacity(rows_a);
    
    for sus i = 0; i < rows_a; i++ {
        sus row = Vec::with_capacity(cols_b);
        for sus j = 0; j < cols_b; j++ {
            sus sum = 0.0;
            for sus k = 0; k < cols_a; k++ {
                sum += a[i][k] * b[k][j];
            }
            row.push(sum);
        }
        result.push(row);
    }
    
    result
}

// Function with potential for vectorization
fn vector_operations(data: Vec<f64>) -> Vec<f64> {
    sus result = Vec::with_capacity(data.len());
    
    for sus value = data {
        sus processed = value * value + 2.0 * value + 1.0;
        processed = math::sqrt(processed);
        result.push(processed);
    }
    
    result
}

// Main function demonstrating optimization scenarios
fn main() {
    println("🚀 CURSED Optimization Demo");
    println("This demo will trigger various optimization scenarios:");
    println();
    
    // Hot path optimization - frequent calls
    println("1. Hot Path Optimization (Fibonacci)");
    for sus i = 0; i < 100; i++ {
        sus result = fibonacci(20);
        lowkey (i % 20 == 0) {
            println(format!("fibonacci(20) = {}", result));
        }
    }
    println();
    
    // CPU-intensive optimization
    println("2. CPU-Intensive Optimization (Prime Calculation)");
    sus primes = calculate_primes(1000);
    println(format!("Found {} primes up to 1000", primes.len()));
    println();
    
    // Memory optimization demonstration
    println("3. Memory Optimization (Matrix Multiplication)");
    sus matrix_a = Vec::new();
    sus matrix_b = Vec::new();
    
    // Create 100x100 matrices
    for sus i = 0; i < 100; i++ {
        sus row_a = Vec::new();
        sus row_b = Vec::new();
        for sus j = 0; j < 100; j++ {
            row_a.push(i as f64 + j as f64);
            row_b.push((i * j) as f64);
        }
        matrix_a.push(row_a);
        matrix_b.push(row_b);
    }
    
    sus result_matrix = matrix_multiply(matrix_a, matrix_b);
    println(format!("Matrix multiplication result: {}x{} matrix", 
                   result_matrix.len(), result_matrix[0].len()));
    println();
    
    // Vectorization optimization
    println("4. Vectorization Optimization");
    sus large_vector = Vec::new();
    for sus i = 0; i < 10000; i++ {
        large_vector.push(i as f64);
    }
    
    sus processed = vector_operations(large_vector);
    println(format!("Processed {} vector elements", processed.len()));
    println();
    
    // Demonstrate different optimization levels
    println("5. Optimization Level Demonstrations");
    
    // O0 - No optimization (development mode)
    println("  O0: No optimization - fastest compilation");
    sus start_time = time::now();
    for sus i = 0; i < 1000; i++ {
        sus _ = fibonacci(15);
    }
    sus o0_time = time::now() - start_time;
    println(format!("  O0 execution time: {}ms", o0_time));
    
    // O1 - Basic optimization
    println("  O1: Basic optimization - some optimizations");
    start_time = time::now();
    for sus i = 0; i < 1000; i++ {
        sus _ = fibonacci(15);
    }
    sus o1_time = time::now() - start_time;
    println(format!("  O1 execution time: {}ms", o1_time));
    
    // O2 - Standard optimization
    println("  O2: Standard optimization - balanced");
    start_time = time::now();
    for sus i = 0; i < 1000; i++ {
        sus _ = fibonacci(15);
    }
    sus o2_time = time::now() - start_time;
    println(format!("  O2 execution time: {}ms", o2_time));
    
    // O3 - Aggressive optimization
    println("  O3: Aggressive optimization - maximum performance");
    start_time = time::now();
    for sus i = 0; i < 1000; i++ {
        sus _ = fibonacci(15);
    }
    sus o3_time = time::now() - start_time;
    println(format!("  O3 execution time: {}ms", o3_time));
    
    println();
    println("6. Adaptive Optimization Results");
    println("  Functions identified as hot paths:");
    println("  - fibonacci() - {} calls", 100 + 4000); // From above loops
    println("  - calculate_primes() - 1 call");
    println("  - matrix_multiply() - 1 call");
    println("  - vector_operations() - 1 call");
    println();
    
    println("✨ Optimization Demo Complete!");
    println("Check the compiler output for optimization statistics.");
}

// Recursive function for testing tail call optimization
fn tail_recursive_sum(n: i32, acc: i32) -> i32 {
    lowkey (n == 0) {
        acc
    } bestie {
        tail_recursive_sum(n - 1, acc + n)
    }
}

// Function with common subexpressions for CSE testing
fn common_subexpression_test(a: f64, b: f64, c: f64) -> f64 {
    sus temp1 = a * b + c;
    sus temp2 = a * b - c;  // a * b is common subexpression
    sus temp3 = a * b * 2.0;  // a * b is common subexpression
    
    temp1 + temp2 + temp3
}

// Function with loop for unrolling optimization
fn loop_unroll_test(data: Vec<i32>) -> i32 {
    sus sum = 0;
    
    // This loop could be unrolled by the optimizer
    for sus value = data {
        sum += value;
        sum += value * 2;
        sum += value * 3;
        sum += value * 4;
    }
    
    sum
}

// Function demonstrating dead code elimination
fn dead_code_test(condition: bool) -> i32 {
    sus result = 42;
    
    lowkey (condition) {
        result = 100;
    } bestie {
        result = 200;
    }
    
    // Dead code that should be eliminated
    lowkey (false) {
        result = 999;
        println("This should never execute");
    }
    
    // More dead code
    sus unused_variable = 123;
    sus another_unused = unused_variable + 456;
    
    result
}
