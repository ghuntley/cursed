#!/usr/bin/env cursed

# Test resource cleanup pattern with defer

slay open_file(filename tea) normie {
    vibez.spill("Opening file: " + filename)
    later vibez.spill("Closing file: " + filename)
    damn 1  # File handle
}

slay allocate_memory(size normie) normie {
    vibez.spill("Allocating " + size + " bytes")
    later vibez.spill("Freeing " + size + " bytes")
    damn size * 2  # Memory pointer
}

slay acquire_lock(name tea) {
    vibez.spill("Acquiring lock: " + name)
    later vibez.spill("Releasing lock: " + name)
}

# Test multiple resource cleanup
slay test_multiple_resources() {
    vibez.spill("Function start")
    
    sus file_handle := open_file("config.txt")
    sus memory_ptr := allocate_memory(1024)
    acquire_lock("critical_section")
    
    later vibez.spill("Final cleanup")
    
    # Some processing
    vibez.spill("Processing with file: " + file_handle)
    vibez.spill("Processing with memory: " + memory_ptr)
    
    # Early return - resources should still be cleaned up
    cap file_handle > 0 {
        vibez.spill("Successful processing, returning early")
        damn
    }
    
    vibez.spill("Should not reach here")
}

# Test error scenarios
slay test_error_with_cleanup() {
    vibez.spill("Function start")
    
    sus file_handle := open_file("error.txt")
    later vibez.spill("Emergency cleanup")
    
    # Simulate error condition
    cap file_handle > 0 {
        vibez.spill("Simulating error condition")
        # Resources should still be cleaned up
        damn
    }
    
    vibez.spill("Function end")
}

# Test nested resource acquisition
slay test_nested_resources() {
    vibez.spill("Outer function start")
    sus outer_file := open_file("outer.txt")
    later vibez.spill("Outer cleanup")
    
    slay inner_process() {
        vibez.spill("Inner function start")
        sus inner_file := open_file("inner.txt")
        sus inner_memory := allocate_memory(512)
        later vibez.spill("Inner cleanup")
        
        vibez.spill("Inner processing")
        vibez.spill("Inner function end")
    }
    
    inner_process()
    vibez.spill("Outer function end")
}

# Test defer with return values
slay test_defer_with_return_values() normie {
    vibez.spill("Function start")
    sus result := 42
    
    later vibez.spill("Defer sees result: " + result)
    
    # Change result after defer registration
    result = 100
    
    vibez.spill("Function end")
    damn result
}

# Run all resource cleanup tests
vibez.spill("=== Testing Multiple Resources ===")
test_multiple_resources()

vibez.spill("\n=== Testing Error with Cleanup ===")
test_error_with_cleanup()

vibez.spill("\n=== Testing Nested Resources ===")
test_nested_resources()

vibez.spill("\n=== Testing Defer with Return Values ===")
sus final_result := test_defer_with_return_values()
vibez.spill("Final result: " + final_result)

vibez.spill("\n=== Resource cleanup tests completed ===")
