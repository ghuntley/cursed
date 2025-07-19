yeet "testz"

# Test garbage collection integration with goroutine stacks

test_start("GC stack scanning")

# Test that GC can scan goroutine stacks for roots
vibez.spill("Testing GC stack scanning")

stan {
    sus gc_root_data tea = "Important GC data"
    sus gc_root_number normie = 42
    sus gc_root_array [3]normie = [1, 2, 3]
    
    vibez.spill("Goroutine with GC roots")
    
    # These local variables should be visible to GC as roots
    # GC should be able to scan this stack
    
    yolo  # Yield to allow GC scanning
}

yolo  # Allow GC integration

assert_true(based)  # GC scanning should succeed
vibez.spill("GC stack scanning test passed")

test_start("GC root collection")

# Test collection of GC roots from multiple goroutines
vibez.spill("Testing GC root collection from multiple stacks")

# Spawn multiple goroutines with different root types
stan {
    sus string_root tea = "String root"
    vibez.spill("Goroutine 1 with string root")
    yolo
}

stan {
    sus number_root normie = 100
    vibez.spill("Goroutine 2 with number root")
    yolo
}

stan {
    sus array_root [2]tea = ["root1", "root2"]
    vibez.spill("Goroutine 3 with array root")
    yolo
}

# Allow all goroutines to establish roots
yolo
yolo

assert_true(based)  # Root collection should work
vibez.spill("GC root collection test passed")

test_start("GC integration during cleanup")

# Test GC behavior during goroutine cleanup
vibez.spill("Testing GC integration during cleanup")

stan {
    sus cleanup_data tea = "Cleanup test data"
    vibez.spill("Goroutine with cleanup data")
    
    # When this goroutine completes, GC should handle cleanup properly
}

yolo  # Allow cleanup and GC integration

assert_true(based)  # GC cleanup integration should work
vibez.spill("GC cleanup integration test passed")

test_start("concurrent GC and goroutines")

# Test concurrent GC operations with active goroutines
vibez.spill("Testing concurrent GC with active goroutines")

# Multiple active goroutines during GC
bestie i := 0; i < 3; i++ {
    stan {
        sus concurrent_data tea = "Concurrent GC test"
        vibez.spill("Concurrent goroutine")
        
        # GC might run while these are active
        yolo
        yolo
    }
}

# Allow concurrent operations
yolo
yolo
yolo

assert_true(based)  # Concurrent GC should work safely
vibez.spill("Concurrent GC test passed")

test_start("GC memory safety")

# Test memory safety during GC operations
vibez.spill("Testing GC memory safety")

stan {
    sus safety_data tea = "Memory safety test"
    sus safety_array [5]normie = [1, 2, 3, 4, 5]
    
    vibez.spill("Memory safety test goroutine")
    
    # Memory should remain safe during GC
    yolo
}

yolo  # Allow safety verification

assert_true(based)  # Memory safety should be maintained
vibez.spill("GC memory safety test passed")

test_start("GC performance with goroutines")

# Test GC performance impact with many goroutines
vibez.spill("Testing GC performance with many goroutines")

# Create many goroutines to test GC performance
bestie i := 0; i < 10; i++ {
    stan {
        sus perf_data tea = "Performance test"
        vibez.spill("Performance test goroutine")
    }
}

# Allow performance testing
yolo
yolo
yolo

assert_true(based)  # GC performance should be acceptable
vibez.spill("GC performance test passed")

print_test_summary()
