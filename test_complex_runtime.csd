// Complex runtime test for edge cases and potential bugs
yeet "testz"
yeet "vibez"
yeet "concurrenz"

// Test 1: Type system edge cases
slay test_type_edge_cases() {
    // Test null/undefined behavior
    sus x tea = "string"
    sus y drip = 42
    
    // Edge case: type conversions
    ready {
        sus converted = x.(drip)  // Should fail gracefully
        vibez.spill("This should not print")
    } catch(e) {
        vibez.spill("Caught type conversion error: ")
        vibez.spill(e.message)
    }
}

// Test 2: Memory-intensive operations
slay test_memory_intensive() {
    // Large array allocation
    sus large_array = []drip{}
    sus i drip = 0
    bestie (i < 100000) {
        large_array.append(i)
        i = i + 1
    }
    
    // Force garbage collection stress
    sus j drip = 0
    bestie (j < 1000) {
        sus temp_string tea = "temp_" + tea(j)
        j = j + 1
    }
}

// Test 3: Concurrency edge cases
slay test_concurrency_bugs() {
    sus ch = dm_new()
    
    // Test channel operations with timeout
    stan {
        ready {
            dm_send(ch, "test_message")
        } catch(e) {
            vibez.spill("Channel send error: ")
            vibez.spill(e.message)
        }
    }
    
    // Test receive with potential deadlock
    ready {
        sus msg = dm_recv(ch, 100)  // 100ms timeout
        vibez.spill("Received: ")
        vibez.spill(msg)
    } catch(e) {
        vibez.spill("Channel receive timeout or error: ")
        vibez.spill(e.message)
    }
}

// Test 4: Complex struct and interface interactions
squad ComplexStruct {
    spill field1 tea
    spill field2 drip
    spill nested_array []tea
}

collab TestInterface {
    slay test_method(param tea) tea
}

slay test_struct_interface_bugs() {
    sus complex = ComplexStruct{
        field1: "test",
        field2: 42,
        nested_array: []tea{"a", "b", "c"}
    }
    
    // Test field access edge cases
    vibez.spill(complex.field1)
    vibez.spill(tea(complex.field2))
    
    // Test array access bounds
    ready {
        vibez.spill(complex.nested_array[10])  // Should fail gracefully
    } catch(e) {
        vibez.spill("Array bounds error: ")
        vibez.spill(e.message)
    }
}

// Test 5: Recursive function potential stack overflow
slay test_recursion_limits(depth drip) drip {
    nah (depth <= 0) {
        damn 0
    }
    damn depth + test_recursion_limits(depth - 1)
}

// Main test runner
slay main() {
    test_start("Complex Runtime Bug Test Suite")
    
    vibez.spill("Starting complex runtime tests...")
    
    test_type_edge_cases()
    test_memory_intensive()
    test_concurrency_bugs()
    test_struct_interface_bugs()
    
    // Test recursion with safety
    ready {
        sus result = test_recursion_limits(10000)  // High recursion
        vibez.spill("Recursion result: ")
        vibez.spill(tea(result))
    } catch(e) {
        vibez.spill("Stack overflow protection worked: ")
        vibez.spill(e.message)
    }
    
    vibez.spill("Complex runtime tests completed")
}
