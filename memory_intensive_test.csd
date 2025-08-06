yeet "testz"
yeet "vibez"

slay memory_stress_test() {
    test_start("Memory Stress Test")
    
    vibez.spill("Starting memory intensive test...")
    
    # Test 1: Large array allocation and manipulation
    sus large_array = [normie]()
    sus i drip = 0
    bestie (i < 10000) {
        large_array.append(i * i)
        i = i + 1
    }
    
    assert_eq_int(large_array.len(), 10000)
    vibez.spill("Large array test passed")
    
    # Test 2: String concatenation stress
    sus long_string tea = ""
    i = 0
    bestie (i < 1000) {
        long_string = long_string + "Hello World " + i.to_string() + " "
        i = i + 1
    }
    
    assert_true(long_string.len() > 10000)
    vibez.spill("String concatenation test passed")
    
    # Test 3: Nested structure allocation
    squad NestedStruct {
        spill data normie
        spill children [NestedStruct]
    }
    
    slay create_nested_structure(depth normie) NestedStruct {
        sus result NestedStruct = NestedStruct{ data: depth, children: [NestedStruct]() }
        
        no_cap (depth > 0) {
            sus i drip = 0
            bestie (i < 3) {
                result.children.append(create_nested_structure(depth - 1))
                i = i + 1
            }
        }
        
        damn result
    }
    
    sus nested = create_nested_structure(5)
    assert_eq_int(nested.data, 5)
    assert_eq_int(nested.children.len(), 3)
    vibez.spill("Nested structure test passed")
    
    # Test 4: Rapid allocation and deallocation
    i = 0
    bestie (i < 1000) {
        sus temp_array = [normie]()
        sus j drip = 0
        bestie (j < 100) {
            temp_array.append(j)
            j = j + 1
        }
        i = i + 1
    }
    vibez.spill("Rapid allocation test passed")
    
    # Test 5: Memory pressure simulation
    sus memory_hogs = [[normie]]()
    i = 0
    bestie (i < 50) {
        sus big_array = [normie]()
        sus k drip = 0
        bestie (k < 1000) {
            big_array.append(k * i)
            k = k + 1
        }
        memory_hogs.append(big_array)
        i = i + 1
    }
    
    assert_eq_int(memory_hogs.len(), 50)
    vibez.spill("Memory pressure test passed")
    
    print_test_summary()
}

slay concurrent_memory_test() {
    test_start("Concurrent Memory Test")
    
    vibez.spill("Starting concurrent memory allocation test...")
    
    # Test concurrent allocation with goroutines
    sus shared_counter drip = 0
    sus results = [normie]()
    
    # Launch multiple goroutines that allocate memory
    sus goroutine_count drip = 10
    sus i drip = 0
    
    bestie (i < goroutine_count) {
        stan {
            sus local_data = [normie]()
            sus j drip = 0
            bestie (j < 500) {
                local_data.append(j * j)
                j = j + 1
            }
            
            # Simulate some work
            sus k drip = 0
            bestie (k < local_data.len()) {
                shared_counter = shared_counter + local_data[k]
                k = k + 1
            }
        }
        i = i + 1
    }
    
    # Wait a bit for goroutines to complete
    sus wait_cycles drip = 0
    bestie (wait_cycles < 1000000) {
        wait_cycles = wait_cycles + 1
    }
    
    assert_true(shared_counter > 0)
    vibez.spill("Concurrent memory test passed")
    
    print_test_summary()
}

slay arena_allocator_test() {
    test_start("Arena Allocator Test")
    
    vibez.spill("Testing arena allocation patterns...")
    
    # Test sequential allocation pattern
    sus sequential_data = [tea]()
    sus i drip = 0
    bestie (i < 1000) {
        sequential_data.append("Item " + i.to_string())
        i = i + 1
    }
    
    assert_eq_int(sequential_data.len(), 1000)
    vibez.spill("Sequential allocation test passed")
    
    # Test stack-like allocation pattern
    slay stack_test(depth normie) normie {
        no_cap (depth <= 0) {
            damn 1
        }
        
        sus local_array = [normie]()
        sus j drip = 0
        bestie (j < depth) {
            local_array.append(j)
            j = j + 1
        }
        
        damn local_array.len() + stack_test(depth - 1)
    }
    
    sus stack_result = stack_test(10)
    assert_true(stack_result > 0)
    vibez.spill("Stack allocation test passed")
    
    # Test temporary allocation cleanup
    i = 0
    bestie (i < 100) {
        sus temp_scope = {
            sus temp_data = [normie]()
            sus k drip = 0
            bestie (k < 50) {
                temp_data.append(k * k)
                k = k + 1
            }
            temp_data.len()
        }
        assert_eq_int(temp_scope, 50)
        i = i + 1
    }
    vibez.spill("Temporary allocation test passed")
    
    print_test_summary()
}

slay gc_pressure_test() {
    test_start("GC Pressure Test")
    
    vibez.spill("Testing garbage collection under pressure...")
    
    # Create objects that become unreachable to trigger GC
    sus outer_loop drip = 0
    bestie (outer_loop < 100) {
        sus unreachable_objects = [[normie]]()
        
        sus inner_loop drip = 0
        bestie (inner_loop < 50) {
            sus temp_array = [normie]()
            sus fill_index drip = 0
            bestie (fill_index < 100) {
                temp_array.append(fill_index * inner_loop)
                fill_index = fill_index + 1
            }
            unreachable_objects.append(temp_array)
            inner_loop = inner_loop + 1
        }
        
        # Objects go out of scope here and become eligible for GC
        outer_loop = outer_loop + 1
    }
    
    vibez.spill("Created and released many objects")
    
    # Force some allocation to potentially trigger GC
    sus final_test = [tea]()
    sus final_i drip = 0
    bestie (final_i < 1000) {
        final_test.append("Final test string " + final_i.to_string())
        final_i = final_i + 1
    }
    
    assert_eq_int(final_test.len(), 1000)
    vibez.spill("GC pressure test completed")
    
    print_test_summary()
}

slay memory_leak_detection_test() {
    test_start("Memory Leak Detection Test")
    
    vibez.spill("Testing memory leak detection...")
    
    # Create some long-lived objects that should not be considered leaks
    sus persistent_data = [normie]()
    sus i drip = 0
    bestie (i < 100) {
        persistent_data.append(i)
        i = i + 1
    }
    
    # Create and abandon some objects that might be considered leaks
    i = 0
    bestie (i < 50) {
        sus abandoned = [tea]()
        sus j drip = 0
        bestie (j < 10) {
            abandoned.append("Abandoned string " + j.to_string())
            j = j + 1
        }
        # Don't keep reference to 'abandoned' - it becomes eligible for collection
        i = i + 1
    }
    
    assert_eq_int(persistent_data.len(), 100)
    vibez.spill("Memory leak detection test completed")
    
    print_test_summary()
}

# Run all memory tests
vibez.spill("=== Starting Comprehensive Memory Management Tests ===")

memory_stress_test()
concurrent_memory_test()
arena_allocator_test()
gc_pressure_test()
memory_leak_detection_test()

vibez.spill("=== All Memory Tests Completed ===")
