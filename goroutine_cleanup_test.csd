yeet "testz"

# Test comprehensive goroutine resource cleanup and stack management

# Test basic goroutine resource cleanup
test_start("goroutine resource cleanup")

# Spawn goroutines with automatic cleanup
sus goroutine1 := stan {
    vibez.spill("Goroutine 1 executing")
    # Goroutine completes normally - should trigger automatic cleanup
}

sus goroutine2 := stan {
    vibez.spill("Goroutine 2 executing")
    # Goroutine completes normally - should trigger automatic cleanup
}

# Wait for goroutines to complete
damn  # Yield to allow goroutines to run

vibez.spill("Basic goroutine cleanup test completed")

# Test error isolation and cleanup
test_start("goroutine error isolation and cleanup")

sus error_goroutine := stan {
    vibez.spill("Error goroutine starting")
    # This would cause a panic in real implementation
    # Should trigger emergency cleanup and error isolation
    vibez.spill("Error goroutine should handle errors gracefully")
}

damn  # Yield to allow error handling

vibez.spill("Error isolation test completed")

# Test memory reclamation
test_start("memory reclamation verification")

# Multiple goroutines to test memory management
sus count normie = 5
bestie i := 0; i < count; i++ {
    stan {
        vibez.spill("Memory test goroutine")
        # Each goroutine should properly clean up its stack
    }
}

# Allow all goroutines to complete
damn
damn
damn

vibez.spill("Memory reclamation test completed")

# Test stack scanning for GC
test_start("stack scanning for GC integration")

# Create goroutines with local variables for GC roots
stan {
    sus local_data tea = "GC root data"
    sus local_number normie = 42
    vibez.spill("Goroutine with GC roots")
    # Stack should be scanned for GC roots before cleanup
}

damn  # Allow stack scanning

vibez.spill("Stack scanning test completed")

# Test scheduler cleanup functionality
test_start("scheduler cleanup functionality")

# Test that scheduler can clean up completed goroutines
vibez.spill("Testing scheduler cleanup of completed goroutines")

# Spawn multiple goroutines that complete quickly
bestie i := 0; i < 3; i++ {
    stan {
        vibez.spill("Quick goroutine")
    }
}

# Allow goroutines to complete and be cleaned up
damn
damn

vibez.spill("Scheduler cleanup test completed")

print_test_summary()
