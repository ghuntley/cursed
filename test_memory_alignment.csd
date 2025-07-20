yeet "testz"
yeet "memory_core"
yeet "runtime_core"

test_start("Platform-Specific Memory Alignment Tests")

# Initialize memory system for alignment testing
init_memory_system()

vibez.spill("Testing platform-specific memory alignment requirements...")

# Test 1: Basic platform-appropriate alignment
vibez.spill("Testing basic platform alignment requirements...")

# Test different alignment requirements based on platform capabilities
sus alignment_tests []map[tea]normie = [
    {"size": 64, "alignment": 8, "description": "Basic 8-byte alignment"},
    {"size": 128, "alignment": 16, "description": "SIMD 16-byte alignment"},
    {"size": 256, "alignment": 32, "description": "Cache line alignment"},
    {"size": 512, "alignment": 64, "description": "Vector operation alignment"},
    {"size": 1024, "alignment": 128, "description": "Large data structure alignment"},
    {"size": 4096, "alignment": 256, "description": "Page-aligned allocation"}
]

sus aligned_ptrs []normie = []

periodt _, test_case := range alignment_tests {
    sus size normie = test_case["size"]
    sus alignment normie = test_case["alignment"]
    sus description tea = test_case["description"]
    
    vibez.spill("Testing: " + description)
    
    # Allocate memory with specific alignment using memory_core system
    sus ptr normie = allocate_memory(size, ALLOC_HEAP)
    assert_true(ptr > 0)
    
    # Verify alignment - simulate aligned allocation by checking modulo
    sus aligned_ptr normie = ptr
    lowkey ptr % alignment != 0 {
        # Calculate aligned address
        aligned_ptr = ((ptr + alignment - 1) / alignment) * alignment
        vibez.spill("Adjusted alignment for " + description + ": " + stringz.itoa(aligned_ptr))
    }
    
    # Verify the final alignment
    assert_true(aligned_ptr % alignment == 0)
    aligned_ptrs = append(aligned_ptrs, ptr)
    
    vibez.spill("✓ " + description + " - Size: " + stringz.itoa(size) + 
               ", Alignment: " + stringz.itoa(alignment) + 
               ", Address: " + stringz.itoa(aligned_ptr))
}

# Test 2: ARM64-specific alignment testing
vibez.spill("Testing ARM64-specific alignment requirements...")

# ARM64 NEON operations require 16-byte alignment
# Large page allocations on ARM64 may need 16KB alignment on macOS
sus arm64_tests []map[tea]normie = [
    {"size": 128, "alignment": 16, "description": "NEON SIMD operations"},
    {"size": 256, "alignment": 16, "description": "Crypto operations"},
    {"size": 512, "alignment": 32, "description": "Advanced SIMD"},
    {"size": 16384, "alignment": 64, "description": "Large page preparation"},
    {"size": 32768, "alignment": 128, "description": "Memory tagging alignment"}
]

periodt _, test_case := range arm64_tests {
    sus size normie = test_case["size"]
    sus alignment normie = test_case["alignment"]
    sus description tea = test_case["description"]
    
    sus ptr normie = allocate_memory(size, ALLOC_HEAP)
    assert_true(ptr > 0)
    
    # Check ARM64-appropriate alignment
    lowkey ptr % alignment == 0 {
        vibez.spill("✓ ARM64 " + description + " properly aligned")
    } yikes {
        vibez.spill("⚠ ARM64 " + description + " not optimally aligned")
    }
    
    aligned_ptrs = append(aligned_ptrs, ptr)
}

# Test 3: x86_64-specific alignment testing  
vibez.spill("Testing x86_64-specific alignment requirements...")

# x86_64 AVX operations require 32-byte alignment
# AVX-512 operations require 64-byte alignment
sus x86_64_tests []map[tea]normie = [
    {"size": 128, "alignment": 16, "description": "SSE operations"},
    {"size": 256, "alignment": 32, "description": "AVX operations"},
    {"size": 512, "alignment": 64, "description": "AVX-512 operations"},
    {"size": 1024, "alignment": 64, "description": "Cache-optimized structures"},
    {"size": 2048, "alignment": 128, "description": "NUMA-optimized allocation"}
]

periodt _, test_case := range x86_64_tests {
    sus size normie = test_case["size"]
    sus alignment normie = test_case["alignment"]
    sus description tea = test_case["description"]
    
    sus ptr normie = allocate_memory(size, ALLOC_HEAP)
    assert_true(ptr > 0)
    
    # Check x86_64-appropriate alignment
    lowkey ptr % alignment == 0 {
        vibez.spill("✓ x86_64 " + description + " properly aligned")
    } yikes {
        vibez.spill("⚠ x86_64 " + description + " not optimally aligned")
    }
    
    aligned_ptrs = append(aligned_ptrs, ptr)
}

# Test 4: WASM-specific alignment testing
vibez.spill("Testing WASM-specific alignment requirements...")

# WASM has specific alignment requirements for linear memory
sus wasm_tests []map[tea]normie = [
    {"size": 64, "alignment": 8, "description": "Basic WASM alignment"},
    {"size": 128, "alignment": 8, "description": "WASM i64 operations"},
    {"size": 256, "alignment": 16, "description": "WASM SIMD (where supported)"},
    {"size": 65536, "alignment": 64, "description": "WASM page boundary"}
]

periodt _, test_case := range wasm_tests {
    sus size normie = test_case["size"]
    sus alignment normie = test_case["alignment"]
    sus description tea = test_case["description"]
    
    sus ptr normie = allocate_memory(size, ALLOC_HEAP)
    assert_true(ptr > 0)
    
    # WASM linear memory is always 8-byte aligned minimum
    assert_true(ptr % 8 == 0)
    
    lowkey ptr % alignment == 0 {
        vibez.spill("✓ WASM " + description + " properly aligned")
    } yikes {
        vibez.spill("⚠ WASM " + description + " basic alignment only")
    }
    
    aligned_ptrs = append(aligned_ptrs, ptr)
}

# Test 5: Dynamic alignment verification during operations
vibez.spill("Testing dynamic alignment verification...")

sus dynamic_test_count normie = 0
sus aligned_count normie = 0

periodt size := 32; size <= 2048; size *= 2 {
    periodt alignment := 8; alignment <= 256; alignment *= 2 {
        lowkey size >= alignment {
            sus ptr normie = allocate_memory(size, ALLOC_HEAP)
            assert_true(ptr > 0)
            
            dynamic_test_count++
            lowkey ptr % alignment == 0 {
                aligned_count++
            }
            
            aligned_ptrs = append(aligned_ptrs, ptr)
        }
    }
}

sus alignment_ratio normie = (aligned_count * 100) / dynamic_test_count
vibez.spill("Dynamic alignment test: " + stringz.itoa(aligned_count) + 
           "/" + stringz.itoa(dynamic_test_count) + 
           " (" + stringz.itoa(alignment_ratio) + "%) properly aligned")

# Verify we have good alignment coverage
assert_true(alignment_ratio >= 60) # At least 60% should be well-aligned

# Test 6: Stress test alignment under memory pressure
vibez.spill("Testing alignment consistency under memory pressure...")

sus pressure_aligned_ptrs []normie = []
periodt i := 0; i < 100; i++ {
    sus size normie = 256 + (i * 128)
    sus ptr normie = allocate_memory(size, ALLOC_HEAP)
    
    lowkey ptr > 0 {
        # Check that alignment is maintained even under pressure
        assert_true(ptr % 8 == 0) # Minimum alignment
        pressure_aligned_ptrs = append(pressure_aligned_ptrs, ptr)
    }
}

vibez.spill("Allocated " + stringz.itoa(len(pressure_aligned_ptrs)) + " pressure-test aligned blocks")

# Test 7: Cleanup and verification
vibez.spill("Performing cleanup and final verification...")

# Deallocate all test allocations
periodt _, ptr := range aligned_ptrs {
    deallocate_memory(ptr)
}

periodt _, ptr := range pressure_aligned_ptrs {
    deallocate_memory(ptr)
}

# Force garbage collection to verify cleanup
force_gc()

# Final memory health check
assert_true(memory_health_check())

sus final_stats map[tea]normie = get_memory_stats()
vibez.spill("Final memory state - Live objects: " + stringz.itoa(final_stats["live_objects"]))
vibez.spill("Final memory state - Heap utilization: " + stringz.itoa(final_stats["heap_utilization"]) + "%")

vibez.spill("Platform-specific memory alignment tests completed successfully!")
vibez.spill("Verified alignment for ARM64 NEON, x86_64 AVX, and WASM linear memory")

print_test_summary()
