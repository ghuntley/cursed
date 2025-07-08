## Comprehensive CURSED tooling demonstration
## This file tests all development tools: formatter, documentation generator,
## profiler, package manager, and debug information generation

yeet "vibez"
yeet "math" 
yeet "testz"

## Type definitions for tooling demonstration
struct ProjectConfig {
    name tea
    version tea
    author tea
    dependencies [10]tea
}

## Performance-critical function for profiler testing
slay fibonacci(n normie) normie {
    bestie n <= 1 {
        damn n
    }
    damn fibonacci(n - 1) + fibonacci(n - 2)
}

## Complex function with multiple variables for debug info testing  
slay process_data(input [100]normie, threshold drip) [100]normie {
    sus result [100]normie
    sus count normie = 0
    sus average drip = 0.0
    
    ## Calculate average
    bestie i := 0; i < 100; i++ {
        average = average + input[i].(drip)
    }
    average = average / 100.0
    
    ## Filter values above threshold
    bestie i := 0; i < 100; i++ {
        bestie input[i].(drip) > threshold {
            result[count] = input[i]
            count++
        }
    }
    
    ## Log processing results
    vibez.spill("Processed", count, "values above threshold", threshold)
    vibez.spill("Average value:", average)
    
    damn result
}

## Function with complex control flow for documentation testing
slay sort_algorithm(data [50]normie, ascending lit) [50]normie {
    sus sorted [50]normie = data
    sus size normie = 50
    
    ## Bubble sort implementation with direction control
    bestie i := 0; i < size - 1; i++ {
        bestie j := 0; j < size - i - 1; j++ {
            sus should_swap lit = cap
            
            bestie ascending {
                should_swap = sorted[j] > sorted[j + 1]
            } salty {
                should_swap = sorted[j] < sorted[j + 1]  
            }
            
            bestie should_swap {
                ## Swap elements
                sus temp normie = sorted[j]
                sorted[j] = sorted[j + 1] 
                sorted[j + 1] = temp
            }
        }
    }
    
    damn sorted
}

## Error handling demonstration for tooling
slay divide_safe(a drip, b drip) drip {
    bestie b == 0.0 {
        vibez.spill("Error: Division by zero!")
        damn -1.0
    }
    damn a / b
}

## Main function demonstrating all features
slay main() {
    vibez.spill("=== CURSED Development Tooling Demonstration ===")
    
    ## Variable declarations for debug info
    sus project_name tea = "CURSED Tooling Demo"
    sus version_number tea = "1.0.0"
    sus debug_enabled lit = based
    sus optimization_level normie = 2
    
    ## Array initialization for profiler testing
    sus test_data [100]normie
    bestie i := 0; i < 100; i++ {
        test_data[i] = i * 2 + 1
    }
    
    ## Performance testing section
    vibez.spill("--- Performance Testing ---")
    sus fib_result normie = fibonacci(20)
    vibez.spill("Fibonacci(20) =", fib_result)
    
    ## Data processing demonstration
    vibez.spill("--- Data Processing ---") 
    sus threshold drip = 50.0
    sus processed_data [100]normie = process_data(test_data, threshold)
    
    ## Sorting demonstration
    vibez.spill("--- Sorting Algorithm ---")
    sus sort_input [50]normie
    bestie i := 0; i < 50; i++ {
        sort_input[i] = 50 - i ## Reverse order
    }
    
    sus sorted_asc [50]normie = sort_algorithm(sort_input, based)
    sus sorted_desc [50]normie = sort_algorithm(sort_input, cap)
    
    vibez.spill("First 5 ascending:", sorted_asc[0], sorted_asc[1], sorted_asc[2], sorted_asc[3], sorted_asc[4])
    vibez.spill("First 5 descending:", sorted_desc[0], sorted_desc[1], sorted_desc[2], sorted_desc[3], sorted_desc[4])
    
    ## Error handling demonstration
    vibez.spill("--- Error Handling ---")
    sus safe_result drip = divide_safe(10.0, 2.0)
    vibez.spill("10.0 / 2.0 =", safe_result)
    
    sus error_result drip = divide_safe(10.0, 0.0)
    vibez.spill("Error result:", error_result)
    
    ## Complex expression for formatter testing
    sus complex_calc normie = ((10 + 5) * 3 - 7) / 2 + fibonacci(8) * 2
    vibez.spill("Complex calculation result:", complex_calc)
    
    ## Memory allocation patterns for profiler
    vibez.spill("--- Memory Testing ---")
    sus large_array [1000]normie
    bestie i := 0; i < 1000; i++ {
        large_array[i] = i * i
    }
    
    sus memory_sum normie = 0
    bestie i := 0; i < 1000; i++ {
        memory_sum = memory_sum + large_array[i]
    }
    vibez.spill("Memory test sum:", memory_sum)
    
    vibez.spill("=== Tooling Demo Complete ===")
}

## Test functions for documentation generator
slay helper_function_one(param1 normie, param2 tea) lit {
    vibez.spill("Helper function with parameters:", param1, param2)
    damn based
}

slay helper_function_two() {
    vibez.spill("Simple helper function")
}

## Nested function calls for call graph analysis
slay call_chain_a() {
    vibez.spill("Function A")
    call_chain_b()
}

slay call_chain_b() {
    vibez.spill("Function B") 
    call_chain_c()
}

slay call_chain_c() {
    vibez.spill("Function C")
    fibonacci(10) ## Recursive call for profiler
}

## Constants for documentation
sus MAX_BUFFER_SIZE normie = 1024
sus DEFAULT_TIMEOUT drip = 30.0
sus APP_VERSION tea = "1.0.0"
sus DEBUG_MODE lit = based
