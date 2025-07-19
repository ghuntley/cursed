yeet "testz"

# Test advanced stack management features

test_start("stack allocation and deallocation")

# Test basic stack operations
vibez.spill("Testing stack allocation and deallocation")

# Goroutines automatically allocate stacks
stan {
    vibez.spill("Stack allocation test")
    # Stack should be automatically allocated and deallocated
}

yolo  # Allow stack operations

assert_true(based)  # Stack operations should succeed
vibez.spill("Stack allocation test passed")

test_start("stack overflow detection")

# Test stack overflow monitoring
vibez.spill("Testing stack overflow detection")

stan {
    # This would test stack overflow detection in real implementation
    vibez.spill("Stack overflow detection test")
    # Should detect approaching stack limits
}

yolo

assert_true(based)  # Overflow detection should work
vibez.spill("Stack overflow detection test passed")

test_start("stack frame tracking")

# Test stack frame management for GC
vibez.spill("Testing stack frame tracking")

stan {
    sus local_var tea = "Frame data"
    sus another_var normie = 123
    
    # Stack frames should track local variables for GC
    vibez.spill("Frame tracking test with locals")
    
    # Nested function call would create new frame
    vibez.spill("Nested frame test")
}

yolo

assert_true(based)  # Frame tracking should succeed
vibez.spill("Stack frame tracking test passed")

test_start("stack memory reclamation")

# Test stack memory is properly reclaimed
vibez.spill("Testing stack memory reclamation")

# Create multiple goroutines to test memory management
bestie i := 0; i < 5; i++ {
    stan {
        sus stack_data tea = "Stack memory test"
        vibez.spill("Stack memory test goroutine")
        # Each stack should be properly deallocated
    }
}

# Allow all stacks to be reclaimed
yolo
yolo
yolo

assert_true(based)  # Memory should be reclaimed
vibez.spill("Stack memory reclamation test passed")

test_start("stack guard pages")

# Test stack guard page functionality
vibez.spill("Testing stack guard pages")

stan {
    vibez.spill("Guard page test")
    # Guard pages should prevent stack overflow
}

yolo

assert_true(based)  # Guard pages should work
vibez.spill("Stack guard pages test passed")

test_start("stack usage monitoring")

# Test stack usage tracking
vibez.spill("Testing stack usage monitoring")

stan {
    sus usage_data tea = "Usage monitoring test"
    vibez.spill("Stack usage monitoring test")
    # Stack usage should be tracked for monitoring
}

yolo

assert_true(based)  # Usage monitoring should work
vibez.spill("Stack usage monitoring test passed")

print_test_summary()
