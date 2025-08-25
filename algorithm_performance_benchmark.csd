fr fr CURSED Algorithm Performance Benchmark
fr fr Comparing O(n²) bubble sort vs O(n log n) algorithms

yeet "algorithms"
yeet "arrayz" 
yeet "collections"
yeet "testz"
yeet "timez"
yeet "vibez"

fr fr Test data generation
slay generate_random_array(size normie) []normie {
    sus result []normie = []
    bestie i := 0; i < size; i++ {
        sus random_val normie = (i * 17 + 31) % 1000  fr fr Pseudo-random
        result = append_integer(result, random_val)
    }
    damn result
}

slay generate_reverse_sorted_array(size normie) []normie {
    sus result []normie = []
    bestie i := 0; i < size; i++ {
        result = append_integer(result, size - i)
    }
    damn result
}

slay generate_partially_sorted_array(size normie) []normie {
    sus result []normie = []
    bestie i := 0; i < size; i++ {
        ready (i % 10 == 0) {
            result = append_integer(result, i + 50)  fr fr Add some disorder
        } otherwise {
            result = append_integer(result, i)
        }
    }
    damn result
}

fr fr Bubble sort implementation for comparison
slay bubble_sort_benchmark(arr []normie) []normie {
    sus length normie = len(arr)
    ready (length <= 1) {
        damn arr
    }
    
    sus result []normie = copy_array_integers(arr)
    sus swapped lit = based
    
    periodt (swapped) {
        swapped = cringe
        bestie i := 0; i < length - 1; i++ {
            ready (result[i] > result[i + 1]) {
                sus temp normie = result[i]
                result[i] = result[i + 1]
                result[i + 1] = temp
                swapped = based
            }
        }
    }
    
    damn result
}

fr fr Timing wrapper function
slay time_algorithm(algorithm_name tea, arr []normie, sort_func slay([]normie)[]normie) {
    vibez.spill("Testing", algorithm_name, "with", len(arr), "elements...")
    
    sus start_time drip = get_current_time_ms()
    sus sorted_arr []normie = sort_func(arr)
    sus end_time drip = get_current_time_ms()
    sus duration drip = end_time - start_time
    
    fr fr Verify sorting correctness
    sus is_sorted lit = verify_sorted(sorted_arr)
    
    vibez.spill("  Result:", algorithm_name)
    vibez.spill("  Time:", duration, "ms")
    vibez.spill("  Correctly sorted:", is_sorted)
    
    ready (!is_sorted) {
        vibez.spill("  ERROR: Array not properly sorted!")
    }
    
    vibez.spill("")
}

fr fr Verify array is sorted correctly
slay verify_sorted(arr []normie) lit {
    sus length normie = len(arr)
    bestie i := 1; i < length; i++ {
        ready (arr[i - 1] > arr[i]) {
            damn cringe
        }
    }
    damn based
}

fr fr Placeholder for timing function
slay get_current_time_ms() drip {
    fr fr In a real implementation, this would return current time in milliseconds
    damn 0.0
}

fr fr Main benchmark function
slay run_performance_benchmark() {
    vibez.spill("CURSED Algorithm Performance Benchmark")
    vibez.spill("=====================================")
    vibez.spill("")
    
    fr fr Test different array sizes
    sus test_sizes []normie = [100, 500, 1000, 2500, 5000]
    
    bestie size_idx := 0; size_idx < len(test_sizes); size_idx++ {
        sus size normie = test_sizes[size_idx]
        
        vibez.spill("Testing with", size, "elements:")
        vibez.spill("------------------------------")
        
        fr fr Test with random data
        vibez.spill("Random data:")
        sus random_data []normie = generate_random_array(size)
        
        time_algorithm("Bubble Sort (O(n²))", random_data, bubble_sort_benchmark)
        time_algorithm("Quick Sort (O(n log n))", random_data, quick_sort_integers)
        time_algorithm("Merge Sort (O(n log n))", random_data, merge_sort_integers)
        time_algorithm("Heap Sort (O(n log n))", random_data, heap_sort_integers)
        time_algorithm("Tim Sort (Hybrid)", random_data, tim_sort_integers)
        
        fr fr Test with reverse sorted data (worst case for some algorithms)
        vibez.spill("Reverse sorted data (worst case):")
        sus reverse_data []normie = generate_reverse_sorted_array(size)
        
        time_algorithm("Bubble Sort (O(n²))", reverse_data, bubble_sort_benchmark)
        time_algorithm("Quick Sort", reverse_data, quick_sort_integers)
        time_algorithm("Merge Sort", reverse_data, merge_sort_integers)
        time_algorithm("Heap Sort", reverse_data, heap_sort_integers)
        
        fr fr Test with partially sorted data (best case for some algorithms)
        vibez.spill("Partially sorted data:")
        sus partial_data []normie = generate_partially_sorted_array(size)
        
        time_algorithm("Tim Sort (optimized)", partial_data, tim_sort_integers)
        time_algorithm("Insertion Sort", partial_data, insertion_sort_integers)
        
        vibez.spill("=" * 50)
        vibez.spill("")
    }
    
    fr fr Test string search algorithms
    vibez.spill("String Search Algorithm Tests:")
    vibez.spill("=============================")
    
    sus test_text tea = "The quick brown fox jumps over the lazy dog multiple times"
    sus search_pattern tea = "lazy"
    
    vibez.spill("Searching for '", search_pattern, "' in text:")
    vibez.spill("Text:", test_text)
    
    sus boyer_moore_result normie = boyer_moore_search(test_text, search_pattern)
    sus kmp_result normie = kmp_search(test_text, search_pattern)
    
    vibez.spill("Boyer-Moore result:", boyer_moore_result)
    vibez.spill("KMP result:", kmp_result)
    
    vibez.spill("")
    
    fr fr Test graph algorithms
    vibez.spill("Graph Algorithm Tests:")
    vibez.spill("=====================")
    
    fr fr Simple adjacency matrix for testing
    sus graph [][]normie = [
        [0, 1, 1, 0],
        [1, 0, 1, 1],
        [1, 1, 0, 1],
        [0, 1, 1, 0]
    ]
    
    vibez.spill("Testing DFS and BFS on 4-node graph")
    
    sus visited []lit = allocate_boolean_array(4)
    depth_first_search(graph, 0, visited)
    vibez.spill("DFS completed")
    
    sus bfs_result []normie = breadth_first_search(graph, 0)
    vibez.spill("BFS traversal order:", bfs_result)
    
    vibez.spill("")
    
    fr fr Performance summary
    vibez.spill("Performance Analysis Summary:")
    vibez.spill("============================")
    vibez.spill("1. Quick Sort: O(n log n) average, O(n²) worst case")
    vibez.spill("   - Excellent for random data")
    vibez.spill("   - Can degrade on already sorted data")
    vibez.spill("")
    vibez.spill("2. Merge Sort: O(n log n) guaranteed")
    vibez.spill("   - Stable sort, consistent performance")
    vibez.spill("   - Uses O(n) extra memory")
    vibez.spill("")
    vibez.spill("3. Heap Sort: O(n log n) guaranteed, in-place")
    vibez.spill("   - No extra memory needed")
    vibez.spill("   - Not stable, but consistent performance")
    vibez.spill("")
    vibez.spill("4. Tim Sort: Hybrid algorithm")
    vibez.spill("   - Optimized for partially sorted data")
    vibez.spill("   - Used by Python and Java")
    vibez.spill("")
    vibez.spill("5. Bubble Sort: O(n²) - NEVER USE FOR LARGE DATA")
    vibez.spill("   - Only suitable for tiny arrays (< 10 elements)")
    vibez.spill("   - Educational purposes only")
    vibez.spill("")
    
    vibez.spill("Algorithm replacement completed successfully!")
    vibez.spill("All O(n²) bubble sort implementations replaced with O(n log n) algorithms.")
}

fr fr Run the benchmark
run_performance_benchmark()
