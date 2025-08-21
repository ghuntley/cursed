# Baseline Benchmark 3: Array Operations and Loops
# Tests: Array creation, access, iteration, bounds checking

yeet "arrayz"
yeet "mathz"
yeet "timez"

slay create_array(size drip) []drip {
    sus arr []drip = []
    bestie (sus i drip = 0; i < size; i++) {
        arr = arrayz.push(arr, i * 2 + 1)
    }
    damn arr
}

slay process_array(arr []drip) drip {
    sus sum drip = 0
    sus len drip = arrayz.len(arr)
    
    bestie (sus i drip = 0; i < len; i++) {
        sus val drip = arr[i]
        sum = sum + mathz.pow(val, 2)
    }
    
    damn sum
}

slay sort_array(arr []drip) []drip {
    sus len drip = arrayz.len(arr)
    sus sorted []drip = arr  # Copy array
    
    # Simple bubble sort for consistent workload
    bestie (sus i drip = 0; i < len - 1; i++) {
        bestie (sus j drip = 0; j < len - i - 1; j++) {
            ready (sorted[j] > sorted[j + 1]) {
                sus temp drip = sorted[j]
                sorted[j] = sorted[j + 1]
                sorted[j + 1] = temp
            }
        }
    }
    
    damn sorted
}

slay benchmark_arrays() drip {
    sus iterations drip = 1000
    sus array_size drip = 100
    sus total drip = 0
    
    bestie (sus i drip = 0; i < iterations; i++) {
        sus arr []drip = create_array(array_size)
        total = total + process_array(arr)
        sus sorted []drip = sort_array(arr)
        total = total + sorted[0] + sorted[array_size - 1]
    }
    
    damn total
}

slay main() drip {
    sus start drip = timez.now_microseconds()
    sus result drip = benchmark_arrays()
    sus end drip = timez.now_microseconds()
    
    vibez.spill("Arrays benchmark result:", result)
    vibez.spill("Execution time (μs):", end - start)
    
    damn 0
}
