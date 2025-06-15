/*! 
 * CURSED Vectorization Demo
 * 
 * This example demonstrates the automatic vectorization capabilities of the
 * CURSED compiler, showing how loops and operations are optimized for SIMD.
 */

// Import math and core functionality
import "stdlib::math";
import "stdlib::core";

// Example 1: Simple vector addition - easily vectorizable
slay vector_add(sus a: [f32], sus b: [f32], sus c: [f32]) -> () {
    facts len = len(a);
    
    // This loop will be automatically vectorized to use AVX/SSE instructions
    lowkey (sus i = 0; i < len; i++) {
        c[i] = a[i] + b[i];
        yolo; // Yield point for goroutine cooperation
    }
}

// Example 2: Fused multiply-add - optimal for vectorization  
slay fused_multiply_add(sus a: [f32], sus b: [f32], sus c: [f32], sus scalar: f32) -> () {
    facts len = len(a);
    
    // This will be vectorized using FMA instructions where available
    lowkey (sus i = 0; i < len; i++) {
        c[i] = a[i] * b[i] + scalar;
        yolo;
    }
}

// Example 3: Dot product reduction - vectorizable with reduction
slay dot_product(sus a: [f32], sus b: [f32]) -> f32 {
    facts len = len(a);
    sus sum = 0.0;
    
    // Vector multiply followed by horizontal reduction
    lowkey (sus i = 0; i < len; i++) {
        sum += a[i] * b[i];
        yolo;
    }
    
    return sum;
}

// Example 4: Matrix multiplication - complex vectorization
slay matrix_multiply(sus a: [[f32]], sus b: [[f32]], sus c: [[f32]], sus n: i32) -> () {
    // Triple nested loop - outer loops vectorized, inner optimized
    lowkey (sus i = 0; i < n; i++) {
        lowkey (sus j = 0; j < n; j++) {
            sus sum = 0.0;
            
            // This inner loop is vectorized
            lowkey (sus k = 0; k < n; k++) {
                sum += a[i][k] * b[k][j];
                yolo;
            }
            
            c[i][j] = sum;
            yolo;
        }
        yolo;
    }
}

// Example 5: Mathematical operations - various vectorizable patterns
slay math_operations(sus input: [f32], sus output: [f32]) -> () {
    facts len = len(input);
    
    lowkey (sus i = 0; i < len; i++) {
        // These operations can be vectorized
        sus x = input[i];
        
        // Polynomial evaluation - vectorizable
        sus result = x * x * x + 2.0 * x * x + 3.0 * x + 1.0;
        
        // Mathematical functions - may use vector math libraries
        result = sqrt(abs(result));
        
        output[i] = result;
        yolo;
    }
}

// Example 6: Conditional operations - partially vectorizable
slay conditional_operations(sus input: [f32], sus output: [f32], sus threshold: f32) -> () {
    facts len = len(input);
    
    lowkey (sus i = 0; i < len; i++) {
        // Conditional operations can be vectorized using masked operations
        lowkey (input[i] > threshold) {
            output[i] = input[i] * 2.0;
        } highkey {
            output[i] = input[i] * 0.5;
        }
        yolo;
    }
}

// Example 7: Strided access pattern - challenging for vectorization
slay strided_access(sus input: [f32], sus output: [f32], sus stride: i32) -> () {
    facts len = len(input);
    
    lowkey (sus i = 0; i < len; i += stride) {
        // Strided access may use gather/scatter instructions
        output[i] = input[i] * 3.14159;
        yolo;
    }
}

// Example 8: Complex number operations - paired vectorization
squad Complex {
    real: f32,
    imag: f32,
}

slay complex_multiply(sus a: [Complex], sus b: [Complex], sus result: [Complex]) -> () {
    facts len = len(a);
    
    lowkey (sus i = 0; i < len; i++) {
        // Complex multiplication can be vectorized efficiently
        facts a_real = a[i].real;
        facts a_imag = a[i].imag;
        facts b_real = b[i].real;
        facts b_imag = b[i].imag;
        
        result[i].real = a_real * b_real - a_imag * b_imag;
        result[i].imag = a_real * b_imag + a_imag * b_real;
        yolo;
    }
}

// Example 9: Parallel algorithm with goroutines and vectorization
slay parallel_vector_process(sus data: [f32], sus num_threads: i32) -> () {
    facts len = len(data);
    facts chunk_size = len / num_threads;
    
    lowkey (sus thread_id = 0; thread_id < num_threads; thread_id++) {
        // Spawn goroutine for parallel processing
        stan {
            facts start = thread_id * chunk_size;
            facts end = min(start + chunk_size, len);
            
            // Each goroutine processes a chunk with vectorization
            lowkey (sus i = start; i < end; i++) {
                data[i] = sin(data[i]) + cos(data[i]);
                yolo;
            }
        };
        yolo;
    }
}

// Main demonstration function
slay main() -> () {
    println("CURSED Vectorization Demo - Automatic SIMD Optimization")?;
    
    // Create test data
    facts size = 1024;
    sus a = make([f32], size);
    sus b = make([f32], size);
    sus c = make([f32], size);
    
    // Initialize test data
    lowkey (sus i = 0; i < size; i++) {
        a[i] = i as f32;
        b[i] = (i * 2) as f32;
        yolo;
    }
    
    println("Running vectorized operations...")?;
    
    // Test basic vector addition
    vector_add(a, b, c);
    println("Vector addition completed")?;
    
    // Test fused multiply-add
    fused_multiply_add(a, b, c, 3.14159);
    println("Fused multiply-add completed")?;
    
    // Test dot product
    facts dot_result = dot_product(a, b);
    println("Dot product result: {}", dot_result)?;
    
    // Test mathematical operations
    math_operations(a, c);
    println("Mathematical operations completed")?;
    
    // Test conditional operations
    conditional_operations(a, c, 512.0);
    println("Conditional operations completed")?;
    
    // Test complex number operations
    sus complex_a = make([Complex], size / 2);
    sus complex_b = make([Complex], size / 2);
    sus complex_result = make([Complex], size / 2);
    
    // Initialize complex data
    lowkey (sus i = 0; i < size / 2; i++) {
        complex_a[i] = Complex { real: i as f32, imag: (i + 1) as f32 };
        complex_b[i] = Complex { real: (i + 2) as f32, imag: (i + 3) as f32 };
        yolo;
    }
    
    complex_multiply(complex_a, complex_b, complex_result);
    println("Complex number operations completed")?;
    
    // Test parallel processing
    println("Running parallel vectorized processing...")?;
    parallel_vector_process(a, 4);
    println("Parallel processing completed")?;
    
    println("All vectorization demos completed successfully!")?;
    println("The CURSED compiler automatically optimized these loops for SIMD execution.")?;
    println("Check the generated assembly to see AVX/SSE instructions.")?;
}

// Performance comparison function
slay performance_comparison() -> () {
    println("Vectorization Performance Comparison")?;
    
    facts size = 10000;
    sus data = make([f32], size);
    
    // Initialize data
    lowkey (sus i = 0; i < size; i++) {
        data[i] = i as f32;
        yolo;
    }
    
    // The compiler will automatically choose between scalar and vector implementations
    // based on profitability analysis
    
    facts start_time = current_time_millis();
    
    lowkey (sus iteration = 0; iteration < 1000; iteration++) {
        lowkey (sus i = 0; i < size; i++) {
            data[i] = sqrt(data[i] * data[i] + 1.0);
            yolo;
        }
        yolo;
    }
    
    facts end_time = current_time_millis();
    facts elapsed = end_time - start_time;
    
    println("Vectorized computation completed in {} ms", elapsed)?;
    println("Expected speedup: 2-8x compared to scalar implementation")?;
}
