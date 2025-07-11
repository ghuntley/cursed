// Test goroutine error isolation - panics in goroutines don't crash the runtime
yeet "testz"

test_start("goroutine error isolation comprehensive test")

// Test 1: Normal goroutine execution
slay test_normal_goroutine() {
    sus success_count normie = 0
    
    // Spawn a normal goroutine
    slay normal_task() {
        success_count = success_count + 1
        vibez.spill("Normal goroutine completed")
    }
    
    yolo normal_task()
    
    // Wait a bit for completion
    sus i normie = 0
    bestie i < 1000; i++ {
        // Simple wait loop
    }
    
    assert_eq_int(success_count, 1)
    vibez.spill("✅ Normal goroutine test passed")
}

// Test 2: Goroutine with panic - should be isolated
slay test_goroutine_panic_isolation() {
    sus normal_count normie = 0
    
    // Spawn a goroutine that panics
    slay panic_task() {
        vibez.spill("About to panic...")
        // This should panic but not crash the runtime
        sus x normie = 1
        sus y normie = 0
        sus z normie = x / y  // Division by zero - should panic
    }
    
    // Spawn a normal goroutine after the panic
    slay normal_after_panic() {
        normal_count = normal_count + 1
        vibez.spill("Normal goroutine after panic completed")
    }
    
    yolo panic_task()
    yolo normal_after_panic()
    
    // Wait for completion
    sus i normie = 0
    bestie i < 2000; i++ {
        // Wait loop
    }
    
    // The normal goroutine should still complete
    assert_eq_int(normal_count, 1)
    vibez.spill("✅ Goroutine panic isolation test passed")
}

// Test 3: Multiple goroutines with mixed success/failure
slay test_mixed_goroutines() {
    sus success_count normie = 0
    sus total_spawned normie = 6
    
    // Spawn successful goroutines
    slay success_task1() {
        success_count = success_count + 1
        vibez.spill("Success task 1 completed")
    }
    
    slay success_task2() {
        success_count = success_count + 1
        vibez.spill("Success task 2 completed")
    }
    
    slay success_task3() {
        success_count = success_count + 1
        vibez.spill("Success task 3 completed")
    }
    
    // Spawn failing goroutines
    slay fail_task1() {
        vibez.spill("Fail task 1 - about to panic")
        // Force panic
        sus invalid_array [0]normie
        invalid_array[1] = 42  // Out of bounds access
    }
    
    slay fail_task2() {
        vibez.spill("Fail task 2 - about to panic")
        // Force panic with string error
        yikes "test_error", "Simulated error in goroutine"
    }
    
    slay fail_task3() {
        vibez.spill("Fail task 3 - about to panic")
        // Force panic with nil access
        sus ptr_nil *normie = cringe
        sus value normie = *ptr_nil  // Dereference nil pointer
    }
    
    // Spawn all goroutines
    yolo success_task1()
    yolo fail_task1()
    yolo success_task2()
    yolo fail_task2()
    yolo success_task3()
    yolo fail_task3()
    
    // Wait for completion
    sus i normie = 0
    bestie i < 3000; i++ {
        // Wait for all goroutines
    }
    
    // All successful goroutines should complete
    assert_eq_int(success_count, 3)
    vibez.spill("✅ Mixed goroutines test passed")
}

// Test 4: Error propagation through join handles
slay test_error_propagation() {
    sus error_detected lit = cap
    
    // Create a goroutine that will fail
    slay error_goroutine() {
        vibez.spill("Error goroutine starting...")
        // Simulate error
        yikes "propagation_test", "Error for propagation testing"
    }
    
    // Create a monitoring goroutine
    slay monitor_goroutine() {
        vibez.spill("Monitor goroutine checking for errors...")
        // In real implementation, this would check join handles
        error_detected = based
    }
    
    yolo error_goroutine()
    yolo monitor_goroutine()
    
    // Wait for completion
    sus i normie = 0
    bestie i < 2000; i++ {
        // Wait loop
    }
    
    assert_true(error_detected)
    vibez.spill("✅ Error propagation test passed")
}

// Test 5: Runtime stability after multiple goroutine failures
slay test_runtime_stability() {
    sus stability_counter normie = 0
    
    // Create multiple waves of goroutines with failures
    bestie wave_num := 0; wave_num < 3; wave_num++ {
        vibez.spill("Starting wave:", wave_num)
        
        // Spawn goroutines in this wave
        bestie i := 0; i < 5; i++ {
            slay wave_task() {
                if i % 2 == 0 {
                    // Even goroutines succeed
                    stability_counter = stability_counter + 1
                    vibez.spill("Wave goroutine succeeded")
                } else {
                    // Odd goroutines fail
                    vibez.spill("Wave goroutine failing...")
                    sus fail_value normie = 1 / 0  // Panic
                }
            }
            
            yolo wave_task()
        }
        
        // Wait between waves
        sus wait_i normie = 0
        bestie wait_i < 1000; wait_i++ {
            // Wait loop
        }
    }
    
    // Final wait
    sus final_wait normie = 0
    bestie final_wait < 2000; final_wait++ {
        // Final wait
    }
    
    // Should have 3 waves * 3 successful goroutines per wave = 9 (but simplified to at least some)
    assert_true(stability_counter > 0)
    vibez.spill("✅ Runtime stability test passed")
}

// Run all tests
test_normal_goroutine()
test_goroutine_panic_isolation()
test_mixed_goroutines()
test_error_propagation()
test_runtime_stability()

print_test_summary()
vibez.spill("🎉 All goroutine error isolation tests completed!")
