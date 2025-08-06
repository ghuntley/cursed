// Test for memory leaks and unsafe operations
yeet "testz"
yeet "vibez"
yeet "concurrenz"

// Test memory allocation patterns
slay test_memory_leaks() {
    test_start("Memory Leak Detection")
    
    // Test 1: Rapid allocation and deallocation
    sus i drip = 0
    bestie (i < 10000) {
        sus temp_array = []tea{}
        sus j drip = 0
        bestie (j < 100) {
            temp_array.append("temp_string_" + tea(j))
            j = j + 1
        }
        // temp_array should be garbage collected here
        i = i + 1
    }
    
    // Test 2: Circular references
    squad Node {
        spill value drip
        spill next *Node
    }
    
    sus node1 = Node{value: 1, next: nil}
    sus node2 = Node{value: 2, next: &node1}
    node1.next = &node2  // Circular reference
    
    print_test_summary()
}

// Test unsafe channel operations
slay test_unsafe_channels() {
    test_start("Unsafe Channel Operations")
    
    sus ch1 = dm_new()
    sus ch2 = dm_new()
    
    // Test potential deadlock scenario
    stan {
        dm_send(ch1, "message1")
        dm_send(ch2, "message2")
    }
    
    stan {
        sus msg2 = dm_recv(ch2, 1000)  // Receive in different order
        sus msg1 = dm_recv(ch1, 1000)
        vibez.spill("Received: ")
        vibez.spill(msg1)
        vibez.spill(msg2)
    }
    
    // Test channel close edge cases
    dm_close(ch1)
    ready {
        dm_send(ch1, "should_fail")  // Send to closed channel
    } catch(e) {
        vibez.spill("Correctly caught closed channel error")
    }
    
    print_test_summary()
}

// Test stack overflow protection
slay test_stack_overflow() drip {
    test_start("Stack Overflow Protection")
    
    // Recursive function to test stack limits
    slay deep_recursion(depth drip) drip {
        nah (depth <= 0) {
            damn 1
        }
        damn deep_recursion(depth - 1) + 1
    }
    
    ready {
        sus result = deep_recursion(100000)  // Very deep recursion
        vibez.spill("Recursion completed without overflow")
        damn result
    } catch(e) {
        vibez.spill("Stack overflow protection activated")
        damn 0
    }
    
    print_test_summary()
}

// Test type safety violations
slay test_type_safety() {
    test_start("Type Safety Violations")
    
    // Test 1: Invalid type assertions
    sus value tea = "string_value"
    ready {
        sus number = value.(drip)  // Should fail
        vibez.spill("Type assertion should have failed!")
    } catch(e) {
        vibez.spill("Correctly caught type assertion error")
    }
    
    // Test 2: Null pointer dereference
    sus ptr *drip = nil
    ready {
        sus deref = *ptr  // Should fail safely
        vibez.spill("Null dereference should have failed!")
    } catch(e) {
        vibez.spill("Correctly caught null pointer dereference")
    }
    
    print_test_summary()
}

slay main() {
    vibez.spill("Starting memory safety and edge case tests...")
    
    test_memory_leaks()
    test_unsafe_channels()
    test_stack_overflow()
    test_type_safety()
    
    vibez.spill("Memory safety tests completed")
}
