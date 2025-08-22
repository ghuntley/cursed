# CURSED Memory Safety Validation Test
# Designed to test memory allocation, deallocation, and leak detection

yeet "vibez"
yeet "arrayz"
yeet "stringz"
yeet "concurrenz"

# ===== MEMORY ALLOCATION STRESS TESTS =====

slay test_large_array_allocation() {
    vibez.spill("Testing large array allocation...")
    
    # Allocate multiple large arrays
    sus arrays [][]drip = []
    
    bestie (sus i drip = 0; i < 50; i = i + 1) {
        sus large_array []drip = []
        
        # Fill with data
        bestie (sus j drip = 0; j < 1000; j = j + 1) {
            arrayz.push(large_array, i * j + j * j)
        }
        
        arrayz.push(arrays, large_array)
    }
    
    vibez.spill("Allocated", arrayz.len(arrays), "large arrays")
    
    # Access elements to ensure memory is valid
    sus total drip = 0
    bestie (sus arr []drip in arrays) {
        total = total + arrayz.len(arr)
    }
    
    vibez.spill("Total elements:", total)
}

slay test_string_memory_management() {
    vibez.spill("Testing string memory management...")
    
    # String concatenation stress test
    sus strings []tea = []
    
    bestie (sus i drip = 0; i < 100; i = i + 1) {
        sus base_string tea = "String number " 
        sus number_string tea = int_to_string(i)
        sus full_string tea = base_string + number_string + " with extra data"
        
        arrayz.push(strings, full_string)
    }
    
    vibez.spill("Created", arrayz.len(strings), "strings")
    
    # Process all strings
    bestie (sus s tea in strings) {
        sus length drip = stringz.len(s)
        # Use the length to prevent optimization
        ready (length > 20) {
            # String is long enough
        }
    }
}

# ===== CONCURRENCY MEMORY TESTS =====

slay test_concurrent_memory_allocation() {
    vibez.spill("Testing concurrent memory allocation...")
    
    sus ch chan<drip> = concurrenz.make_channel()
    
    # Start multiple goroutines that allocate memory
    bestie (sus i drip = 0; i < 5; i = i + 1) {
        go {
            # Each goroutine allocates arrays
            sus local_arrays [][]drip = []
            
            bestie (sus j drip = 0; j < 10; j = j + 1) {
                sus temp_array []drip = []
                
                bestie (sus k drip = 0; k < 100; k = k + 1) {
                    arrayz.push(temp_array, i * j * k)
                }
                
                arrayz.push(local_arrays, temp_array)
            }
            
            # Send completion signal
            ch <- arrayz.len(local_arrays)
        }
    }
    
    # Wait for all goroutines to complete
    sus completed drip = 0
    bestie (completed < 5) {
        sus count drip = <-ch
        vibez.spill("Goroutine completed with", count, "arrays")
        completed = completed + 1
    }
    
    concurrenz.close_channel(ch)
    vibez.spill("All concurrent allocations completed")
}

# ===== RECURSIVE MEMORY TESTS =====

slay create_nested_structure(depth drip) [][]drip {
    ready (depth <= 0) {
        damn []
    }
    
    sus nested [][]drip = []
    
    bestie (sus i drip = 0; i < depth; i = i + 1) {
        sus sub_structure [][]drip = create_nested_structure(depth - 1)
        sus number_array []drip = []
        
        bestie (sus j drip = 0; j < i + 1; j = j + 1) {
            arrayz.push(number_array, i * j)
        }
        
        arrayz.push(sub_structure, number_array)
        arrayz.push(nested, sub_structure)
    }
    
    damn nested
}

slay test_recursive_memory_allocation() {
    vibez.spill("Testing recursive memory allocation...")
    
    sus nested_structure [][]drip = create_nested_structure(5)
    
    # Count total elements recursively
    slay count_elements(structure [][]drip) drip {
        sus total drip = 0
        bestie (sus sub_array []drip in structure) {
            total = total + arrayz.len(sub_array)
        }
        damn total
    }
    
    sus total_elements drip = count_elements(nested_structure)
    vibez.spill("Total elements in nested structure:", total_elements)
}

# ===== CLEANUP AND EDGE CASES =====

slay test_cleanup_edge_cases() {
    vibez.spill("Testing cleanup edge cases...")
    
    # Test empty arrays
    sus empty_arrays [][]drip = []
    bestie (sus i drip = 0; i < 10; i = i + 1) {
        arrayz.push(empty_arrays, [])
    }
    
    # Test single element arrays
    sus single_element_arrays [][]drip = []
    bestie (sus i drip = 0; i < 10; i = i + 1) {
        arrayz.push(single_element_arrays, [i])
    }
    
    # Test string edge cases
    sus edge_strings []tea = []
    arrayz.push(edge_strings, "")  # Empty string
    arrayz.push(edge_strings, "a") # Single character
    arrayz.push(edge_strings, "This is a very long string that should test memory allocation for larger string buffers and ensure proper cleanup")
    
    vibez.spill("Tested edge cases:", arrayz.len(edge_strings), "strings")
}

# ===== MAIN MEMORY SAFETY TEST =====

slay main() {
    vibez.spill("🛡️ Starting CURSED Memory Safety Validation")
    vibez.spill("==========================================")
    
    test_large_array_allocation()
    test_string_memory_management() 
    test_concurrent_memory_allocation()
    test_recursive_memory_allocation()
    test_cleanup_edge_cases()
    
    vibez.spill("==========================================")
    vibez.spill("✅ Memory safety validation completed!")
    vibez.spill("🔍 Run with Valgrind for leak detection")
}

# Helper functions
slay int_to_string(n drip) tea {
    # Placeholder for integer to string conversion
    damn "num"
}
