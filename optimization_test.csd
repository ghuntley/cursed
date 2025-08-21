// CURSED Optimization Test Program
// Tests different optimization levels and performance

yeet "vibez"
yeet "mathz" 
yeet "timez"
yeet "testz"

// CPU-intensive function for optimization testing
slay fibonacci(n drip) drip {
    ready (n <= 1) {
        damn n
    }
    damn fibonacci(n - 1) + fibonacci(n - 2)
}

// Loop-intensive function for vectorization testing
slay vectorizable_sum(arr []drip) drip {
    sus sum drip = 0
    bestie (i drip = 0; i < len(arr); i++) {
        sum = sum + arr[i] * arr[i] // Vectorizable operation
    }
    damn sum
}

// Branch-heavy function for branch prediction testing  
slay branch_heavy_search(arr []drip, target drip) lit {
    bestie (i drip = 0; i < len(arr); i++) {
        ready (arr[i] == target) {
            damn based
        }
        otherwise ready (arr[i] > target) {
            damn cap
        }
    }
    damn cap
}

// Memory-intensive function for cache optimization testing
slay matrix_multiply(a [][]drip, b [][]drip) [][]drip {
    sus rows drip = len(a)
    sus cols drip = len(b[0])
    sus result [][]drip = make_2d_array(rows, cols)
    
    bestie (i drip = 0; i < rows; i++) {
        bestie (j drip = 0; j < cols; j++) {
            sus sum drip = 0
            bestie (k drip = 0; k < len(b); k++) {
                sum = sum + a[i][k] * b[k][j]
            }
            result[i][j] = sum
        }
    }
    damn result
}

// Helper function to create 2D array
slay make_2d_array(rows drip, cols drip) [][]drip {
    sus result [][]drip = []
    bestie (i drip = 0; i < rows; i++) {
        sus row []drip = []
        bestie (j drip = 0; j < cols; j++) {
            row = append(row, 0)
        }
        result = append(result, row)
    }
    damn result
}

// Benchmark function
slay benchmark_function(name tea, iterations drip) {
    vibez.spill("🔄 Benchmarking:", name)
    
    sus start_time drip = timez.now_ms()
    
    ready (name == "fibonacci") {
        bestie (i drip = 0; i < iterations; i++) {
            fibonacci(30)
        }
    } otherwise ready (name == "vectorization") {
        sus test_array []drip = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
        bestie (i drip = 0; i < iterations * 1000; i++) {
            vectorizable_sum(test_array)
        }
    } otherwise ready (name == "branch_prediction") {
        sus test_array []drip = [5, 2, 8, 1, 9, 3, 7, 4, 6]
        bestie (i drip = 0; i < iterations * 1000; i++) {
            branch_heavy_search(test_array, 7)
        }
    } otherwise ready (name == "memory_access") {
        sus matrix_a [][]drip = [[1, 2], [3, 4]]
        sus matrix_b [][]drip = [[5, 6], [7, 8]]
        bestie (i drip = 0; i < iterations * 10; i++) {
            matrix_multiply(matrix_a, matrix_b)
        }
    }
    
    sus end_time drip = timez.now_ms()
    sus duration drip = end_time - start_time
    
    vibez.spill("  ✅ Completed in", duration, "ms")
    vibez.spill("  📊 Average per iteration:", duration / iterations, "ms")
}

// Main optimization test
vibez.spill("🚀 CURSED Optimization Performance Test")
vibez.spill("=====================================")

// Test different workload types
benchmark_function("fibonacci", 5)
benchmark_function("vectorization", 100)  
benchmark_function("branch_prediction", 100)
benchmark_function("memory_access", 100)

vibez.spill("")
vibez.spill("📈 Optimization Test Completed!")
vibez.spill("💡 Compare results across different optimization levels:")
vibez.spill("   O0: No optimization (baseline)")
vibez.spill("   O1: Basic optimization")
vibez.spill("   O2: Standard optimization") 
vibez.spill("   O3: Aggressive optimization")
