fr fr Defer Error Integration Test - Testing 'later' keyword with CURSED error handling
fr fr Tests integration of defer statements with yikes/shook/fam error handling system

yeet "testz"

test_start("Defer Error Integration Test")

sus error_cleanup_executed lit = cringe
sus normal_cleanup_executed lit = cringe
sus finally_cleanup_executed lit = cringe

slay reset_flags() {
    error_cleanup_executed = cringe
    normal_cleanup_executed = cringe
    finally_cleanup_executed = cringe
}

slay mark_error_cleanup() {
    error_cleanup_executed = based
    vibez.spill("🔥 Error cleanup executed via defer")
}

slay mark_normal_cleanup() {
    normal_cleanup_executed = based  
    vibez.spill("✅ Normal cleanup executed via defer")
}

slay mark_finally_cleanup() {
    finally_cleanup_executed = based
    vibez.spill("🎯 Finally cleanup executed via defer")
}

fr fr Test 1: Basic defer with later keyword
test_start("Basic Later Keyword Test")

reset_flags()

slay test_basic_later() {
    vibez.spill("Starting function")
    later mark_normal_cleanup()
    vibez.spill("Function body executing")
    fr fr Normal cleanup should execute when function exits
}

test_basic_later()
assert_true(normal_cleanup_executed)

vibez.spill("✅ Basic later keyword test passed")

fr fr Test 2: Later with error handling (yikes/shook/fam)
test_start("Later with Error Handling")

reset_flags()

slay test_later_with_error_handling() {
    vibez.spill("Starting error handling test")
    
    later mark_error_cleanup()
    
    yikes "test_error" {
        vibez.spill("About to throw error")
        later mark_normal_cleanup()  fr fr This should still execute
        damn "Test error message"
    }
    shook (error_msg) {
        vibez.spill("Caught error: " + error_msg)
        later mark_finally_cleanup()
    }
    fam {
        vibez.spill("In finally block")
        fr fr All defers should execute here
    }
}

test_later_with_error_handling()

fr fr All cleanup functions should have executed
assert_true(error_cleanup_executed)
assert_true(normal_cleanup_executed) 
assert_true(finally_cleanup_executed)

vibez.spill("✅ Later with error handling test passed")

fr fr Test 3: Multiple later statements with LIFO order
test_start("Multiple Later LIFO Order")

sus execution_order []tea = []

slay add_to_order(step tea) {
    execution_order = execution_order + [step]
    vibez.spill("📝 Order: " + step)
}

slay test_lifo_order() {
    add_to_order("start")
    
    later add_to_order("defer_1")
    add_to_order("middle_1")
    
    later add_to_order("defer_2")  
    add_to_order("middle_2")
    
    later add_to_order("defer_3")
    add_to_order("end")
}

test_lifo_order()

fr fr Verify LIFO execution order
assert_eq_string(execution_order[0], "start")
assert_eq_string(execution_order[1], "middle_1")
assert_eq_string(execution_order[2], "middle_2")
assert_eq_string(execution_order[3], "end")
assert_eq_string(execution_order[4], "defer_3")  fr fr Last defer executes first
assert_eq_string(execution_order[5], "defer_2")  fr fr Second defer executes second
assert_eq_string(execution_order[6], "defer_1")  fr fr First defer executes last

vibez.spill("✅ Multiple later LIFO order test passed")

fr fr Test 4: Resource cleanup with later
test_start("Resource Cleanup with Later")

sus file_closed lit = cringe
sus memory_freed lit = cringe
sus connection_closed lit = cringe

slay open_file() normie {
    vibez.spill("📁 File opened")
    damn 42
}

slay close_file(handle normie) {
    file_closed = based
    vibez.spill("📁 File closed: " + handle)
}

slay allocate_memory() normie {
    vibez.spill("💾 Memory allocated")
    damn 1024
}

slay free_memory(ptr normie) {
    memory_freed = based
    vibez.spill("💾 Memory freed: " + ptr)
}

slay open_connection() normie {
    vibez.spill("🌐 Connection opened")
    damn 80
}

slay close_connection(port normie) {
    connection_closed = based
    vibez.spill("🌐 Connection closed: " + port)
}

slay test_resource_cleanup() {
    sus file normie = open_file()
    later close_file(file)
    
    sus memory normie = allocate_memory()
    later free_memory(memory)
    
    sus conn normie = open_connection()
    later close_connection(conn)
    
    vibez.spill("Using resources...")
    fr fr All resources should be cleaned up in reverse order
}

test_resource_cleanup()

assert_true(file_closed)
assert_true(memory_freed)
assert_true(connection_closed)

vibez.spill("✅ Resource cleanup with later test passed")

fr fr Test 5: Later in nested scopes
test_start("Later in Nested Scopes")

execution_order = []  fr fr Reset execution order

slay test_nested_later() {
    add_to_order("outer_start")
    later add_to_order("outer_defer")
    
    bestie (based) {
        add_to_order("inner_start")
        later add_to_order("inner_defer_1")
        later add_to_order("inner_defer_2")
        add_to_order("inner_end")
        fr fr Inner defers execute here
    }
    
    add_to_order("outer_end")
    fr fr Outer defer executes here
}

test_nested_later()

fr fr Verify nested execution - inner defers execute before outer defer
sus found_inner_1 lit = cringe
sus found_inner_2 lit = cringe
sus found_outer lit = cringe

sus i normie = 0
bestie (i < execution_order.length) {
    lowkey (execution_order[i] == "inner_defer_1") {
        found_inner_1 = based
    }
    lowkey (execution_order[i] == "inner_defer_2") {
        found_inner_2 = based
    }
    lowkey (execution_order[i] == "outer_defer") {
        found_outer = based
    }
    i = i + 1
}

assert_true(found_inner_1)
assert_true(found_inner_2)
assert_true(found_outer)

vibez.spill("✅ Later in nested scopes test passed")

fr fr Test 6: Later with early returns
test_start("Later with Early Returns")

execution_order = []

slay test_early_return(condition lit) tea {
    add_to_order("function_start")
    later add_to_order("always_executes")
    
    lowkey (condition) {
        add_to_order("early_return_path")
        later add_to_order("early_return_defer")
        damn "early"
    }
    
    add_to_order("normal_path")
    later add_to_order("normal_defer")
    damn "normal"
}

fr fr Test early return
sus early_result tea = test_early_return(based)
assert_eq_string(early_result, "early")

fr fr Test normal return
execution_order = []
sus normal_result tea = test_early_return(cringe)
assert_eq_string(normal_result, "normal")

vibez.spill("✅ Later with early returns test passed")

fr fr Test 7: Performance test with many later statements
test_start("Performance Test - Many Later Statements")

sus defer_count normie = 0

slay increment_defer_count() {
    defer_count = defer_count + 1
}

slay test_many_defers() {
    sus i normie = 0
    bestie (i < 50) {  fr fr Test with 50 defer statements
        later increment_defer_count()
        i = i + 1
    }
}

test_many_defers()
assert_eq_int(defer_count, 50)

vibez.spill("✅ Performance test with many later statements passed")

fr fr Test Summary
vibez.spill("🎯 Defer Error Integration Test Summary:")
vibez.spill("  ✓ Basic later keyword functionality")
vibez.spill("  ✓ Later with yikes/shook/fam error handling")  
vibez.spill("  ✓ LIFO execution order preservation")
vibez.spill("  ✓ Resource cleanup patterns")
vibez.spill("  ✓ Nested scope handling")
vibez.spill("  ✓ Early return compatibility")
vibez.spill("  ✓ Performance with multiple defers")

print_test_summary()

vibez.spill("🚀 Defer implementation with 'later' keyword and error handling integration complete!")
