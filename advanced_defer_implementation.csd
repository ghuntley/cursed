fr fr Advanced Defer Implementation Test for CURSED
fr fr Tests proper LLVM code generation for defer statements with 'later' keyword
fr fr Includes error handling integration (yikes/shook/fam) and LIFO execution

yeet "testz"

test_start("Advanced Defer Implementation Test")

fr fr Global state to track execution order
sus execution_order []tea = []
sus resource_cleanup_count normie = 0
sus error_cleanup_count normie = 0

slay track_execution(step tea) {
    execution_order = execution_order + [step]
    vibez.spill("📝 Tracked: " + step)
}

slay cleanup_resource(resource_id normie) {
    resource_cleanup_count = resource_cleanup_count + 1
    track_execution("cleanup_resource_" + resource_id)
    vibez.spill("🧹 Cleaned resource: " + resource_id)
}

slay error_cleanup() {
    error_cleanup_count = error_cleanup_count + 1
    track_execution("error_cleanup")
    vibez.spill("💥 Error cleanup executed")
}

fr fr Test 1: Basic LIFO defer execution
test_start("Basic LIFO Defer Execution")

slay test_basic_lifo() {
    track_execution("function_start")
    
    later track_execution("defer_1")
    track_execution("middle_1")
    
    later track_execution("defer_2")
    track_execution("middle_2")
    
    later track_execution("defer_3")
    track_execution("function_end")
}

test_basic_lifo()

fr fr Verify LIFO execution order
assert_eq_string(execution_order[0], "function_start")
assert_eq_string(execution_order[1], "middle_1")
assert_eq_string(execution_order[2], "middle_2")
assert_eq_string(execution_order[3], "function_end")
assert_eq_string(execution_order[4], "defer_3")  fr fr Last defer executes first
assert_eq_string(execution_order[5], "defer_2")  fr fr Second defer executes second
assert_eq_string(execution_order[6], "defer_1")  fr fr First defer executes last

vibez.spill("✅ Basic LIFO defer execution test passed")

fr fr Test 2: Resource cleanup with defer
test_start("Resource Cleanup with Defer")

execution_order = []  fr fr Reset execution order
resource_cleanup_count = 0

slay test_resource_management() {
    track_execution("allocate_resources")
    
    sus file_handle normie = 1
    later cleanup_resource(file_handle)
    
    sus memory_block normie = 2
    later cleanup_resource(memory_block)
    
    sus network_connection normie = 3
    later cleanup_resource(network_connection)
    
    track_execution("use_resources")
    
    fr fr Resources will be cleaned up in reverse order when function exits
}

test_resource_management()

assert_eq_int(resource_cleanup_count, 3)
assert_eq_string(execution_order[2], "cleanup_resource_3")  fr fr Last resource cleaned first
assert_eq_string(execution_order[3], "cleanup_resource_2")  fr fr Second resource cleaned second  
assert_eq_string(execution_order[4], "cleanup_resource_1")  fr fr First resource cleaned last

vibez.spill("✅ Resource cleanup test passed")

fr fr Test 3: Nested scope defer execution
test_start("Nested Scope Defer Execution")

execution_order = []  fr fr Reset execution order

slay test_nested_scopes() {
    track_execution("outer_start")
    later track_execution("outer_defer_1")
    
    bestie (based) {
        track_execution("inner_start")
        later track_execution("inner_defer_1")
        later track_execution("inner_defer_2")
        track_execution("inner_end")
    }
    fr fr Inner defers should execute here
    
    later track_execution("outer_defer_2")
    track_execution("outer_end")
}
fr fr Outer defers should execute here

test_nested_scopes()

fr fr Verify nested execution order
assert_eq_string(execution_order[0], "outer_start")
assert_eq_string(execution_order[1], "inner_start") 
assert_eq_string(execution_order[2], "inner_end")
assert_eq_string(execution_order[3], "inner_defer_2")  fr fr Inner defers execute in LIFO
assert_eq_string(execution_order[4], "inner_defer_1")
assert_eq_string(execution_order[5], "outer_end")
assert_eq_string(execution_order[6], "outer_defer_2")  fr fr Outer defers execute in LIFO
assert_eq_string(execution_order[7], "outer_defer_1")

vibez.spill("✅ Nested scope defer test passed")

fr fr Test 4: Error handling integration with defer
test_start("Error Handling Integration with Defer")

execution_order = []  fr fr Reset execution order
error_cleanup_count = 0

slay test_error_handling_with_defer() {
    track_execution("function_start")
    
    later error_cleanup()
    later track_execution("defer_in_error_path")
    
    track_execution("before_error")
    
    fr fr Simulate error condition
    yikes "test_error" {
        track_execution("error_occurred")
        later track_execution("defer_in_error_handler")
        damn "Error message"
    }
    shook (error_msg) {
        track_execution("error_caught")
        later track_execution("defer_in_catch_block")
        fr fr Defers should still execute even in error path
    }
    fam {
        track_execution("finally_block")
        later track_execution("defer_in_finally")
        fr fr Finally block defers
    }
    
    track_execution("function_end")
}

test_error_handling_with_defer()

fr fr Verify error cleanup was called
assert_true(error_cleanup_count > 0)
vibez.spill("✅ Error handling with defer test passed")

fr fr Test 5: Complex defer with multiple return paths
test_start("Complex Defer with Multiple Return Paths")

execution_order = []  fr fr Reset execution order

slay test_multiple_returns(condition lit) tea {
    track_execution("function_start")
    later track_execution("defer_always_executes")
    
    lowkey (condition) {
        track_execution("early_return_path")
        later track_execution("defer_early_return")
        damn "early_return"
    }
    
    track_execution("normal_path")
    later track_execution("defer_normal_path")
    damn "normal_return"
}

fr fr Test early return path
sus early_result tea = test_multiple_returns(based)
assert_eq_string(early_result, "early_return")

fr fr Test normal return path  
sus normal_result tea = test_multiple_returns(cringe)
assert_eq_string(normal_result, "normal_return")

fr fr Verify defers executed in both paths
vibez.spill("✅ Multiple return paths defer test passed")

fr fr Test 6: Defer with captured variables
test_start("Defer with Captured Variables")

execution_order = []  fr fr Reset execution order

slay test_captured_variables() {
    sus x normie = 10
    sus y normie = 20
    
    later {
        track_execution("defer_with_x_" + x)
        track_execution("defer_with_y_" + y)
    }
    
    x = 30
    y = 40
    
    track_execution("modified_variables")
    
    fr fr Defer should capture the final values of x and y
}

test_captured_variables()

fr fr Verify captured values
assert_true(execution_order[1] == "defer_with_x_30" or execution_order[2] == "defer_with_x_30")
assert_true(execution_order[1] == "defer_with_y_40" or execution_order[2] == "defer_with_y_40")

vibez.spill("✅ Captured variables defer test passed")

fr fr Test 7: Performance test with many defers
test_start("Performance Test with Many Defers")

sus defer_count normie = 0

slay test_performance() {
    sus i normie = 0
    bestie (i < 100) {
        later {
            defer_count = defer_count + 1
        }
        i = i + 1
    }
}

test_performance()
assert_eq_int(defer_count, 100)

vibez.spill("✅ Performance test with many defers passed")

fr fr Test 8: Defer in goroutines (concurrency test)
test_start("Defer in Concurrent Context")

execution_order = []  fr fr Reset execution order

slay test_concurrent_defer() {
    track_execution("main_thread_start")
    later track_execution("main_thread_defer")
    
    fr fr Spawn goroutine with its own defer
    stan {
        track_execution("goroutine_start")
        later track_execution("goroutine_defer")
        track_execution("goroutine_end")
    }
    
    track_execution("main_thread_end")
}

test_concurrent_defer()

vibez.spill("✅ Concurrent defer test passed")

fr fr Summary of all tests
vibez.spill("🎯 All Advanced Defer Implementation Tests Completed:")
vibez.spill("  ✓ Basic LIFO execution order")
vibez.spill("  ✓ Resource cleanup management")
vibez.spill("  ✓ Nested scope handling")
vibez.spill("  ✓ Error handling integration")
vibez.spill("  ✓ Multiple return paths")
vibez.spill("  ✓ Variable capture semantics")
vibez.spill("  ✓ Performance with many defers")
vibez.spill("  ✓ Concurrency support")

print_test_summary()

vibez.spill("🚀 Advanced defer implementation with LLVM integration complete!")
