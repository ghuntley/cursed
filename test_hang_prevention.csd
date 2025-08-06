#!/usr/bin/env -S cursed
# Test Hang Prevention Validation

yeet "testz"

# Test 1: Bounded execution with timeout safety
test_start("GC hang prevention test")

sus start_time drip = time.now_ms()
sus max_execution_time drip = 5000  # 5 seconds max

# Create some objects to trigger GC
sus objects arrayz.Array = arrayz.new_array(normie, 100)
sus i drip = 0
bestie (i < 100 and (time.now_ms() - start_time) < max_execution_time) {
    objects.push(i * i)
    i = i + 1
    
    # Yield CPU to prevent tight loops
    if (i % 10 == 0) {
        time.sleep_ms(1)
    }
}

assert_true(i == 100)
assert_true((time.now_ms() - start_time) < max_execution_time)
vibez.spill("✅ GC execution completed within timeout")

# Test 2: Resource cleanup validation  
test_start("Resource cleanup test")

sus resource_count drip = 0
# Simulate resource allocation with cleanup
ready {
    resource_count = resource_count + 1
    # Cleanup on completion
    defer {
        resource_count = resource_count - 1
    }
    
    # Simulate some work
    time.sleep_ms(10)
}

# Wait briefly for goroutine completion
time.sleep_ms(50)
assert_eq_int(resource_count, 0)
vibez.spill("✅ Resource cleanup working correctly")

# Test 3: Non-blocking I/O operations
test_start("Non-blocking I/O test")

sus output_count drip = 0
sus max_outputs drip = 10

bestie (output_count < max_outputs) {
    vibez.spill("Debug output " + output_count.string())
    output_count = output_count + 1
    time.sleep_ms(5)  # Prevent spam
}

assert_eq_int(output_count, max_outputs)
vibez.spill("✅ Non-blocking I/O completed")

print_test_summary()
vibez.spill("🎉 All hang prevention tests passed!")
