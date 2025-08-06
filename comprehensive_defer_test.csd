fr fr Comprehensive defer statement test with LLVM compilation
yeet "testz"

test_start("Defer Statement LLVM Compilation Test")

fr fr Test basic defer execution order with global state
sus execution_log []tea = []

slay add_to_log(message tea) {
    execution_log = execution_log + [message]
}

slay test_basic_defer_order() {
    add_to_log("start")
    later add_to_log("defer_1")
    add_to_log("middle")
    later add_to_log("defer_2")
    add_to_log("end")
}

fr fr Execute function and check LIFO order
test_basic_defer_order()

assert_eq_string(execution_log[0], "start")
assert_eq_string(execution_log[1], "middle")
assert_eq_string(execution_log[2], "end")
assert_eq_string(execution_log[3], "defer_2")  fr fr Last defer executes first
assert_eq_string(execution_log[4], "defer_1")  fr fr First defer executes last

test_start("Defer with Resource Management")

sus resource_count normie = 0
sus cleanup_count normie = 0

slay allocate_resource() normie {
    resource_count = resource_count + 1
    damn resource_count
}

slay cleanup_resource(id normie) {
    cleanup_count = cleanup_count + 1
    vibez.spill("Cleaning up resource: " + id)
}

slay test_resource_lifecycle() {
    sus res1 normie = allocate_resource()
    later cleanup_resource(res1)
    
    sus res2 normie = allocate_resource() 
    later cleanup_resource(res2)
    
    sus res3 normie = allocate_resource()
    later cleanup_resource(res3)
    
    fr fr Resources allocated, defers registered
    vibez.spill("All resources allocated")
}

test_resource_lifecycle()

fr fr All resources should be cleaned up in reverse order
assert_eq_int(resource_count, 3)   fr fr 3 resources allocated
assert_eq_int(cleanup_count, 3)    fr fr 3 resources cleaned up

test_start("Defer with Early Return")

sus early_return_log []tea = []

slay early_return_function(should_return_early lit) {
    early_return_log = early_return_log + ["function_start"]
    later { early_return_log = early_return_log + ["defer_executed"] }
    
    shook should_return_early {
        early_return_log = early_return_log + ["early_return"]
        damn
    }
    
    early_return_log = early_return_log + ["function_end"]
}

fr fr Test early return - defer should still execute
early_return_function(based)

assert_eq_string(early_return_log[0], "function_start")
assert_eq_string(early_return_log[1], "early_return")
assert_eq_string(early_return_log[2], "defer_executed")  fr fr Defer executes even with early return

fr fr Clear log and test normal execution
early_return_log = []
early_return_function(cringe)

assert_eq_string(early_return_log[0], "function_start")
assert_eq_string(early_return_log[1], "function_end")
assert_eq_string(early_return_log[2], "defer_executed")

test_start("Nested Defer Scopes")

sus nested_log []tea = []

slay outer_scope() {
    nested_log = nested_log + ["outer_start"]
    later { nested_log = nested_log + ["outer_defer"] }
    
    slay inner_scope() {
        nested_log = nested_log + ["inner_start"]
        later { nested_log = nested_log + ["inner_defer_1"] }
        later { nested_log = nested_log + ["inner_defer_2"] }
        nested_log = nested_log + ["inner_end"]
    }
    
    inner_scope()
    nested_log = nested_log + ["outer_end"]
}

outer_scope()

fr fr Verify nested scope execution order
assert_eq_string(nested_log[0], "outer_start")
assert_eq_string(nested_log[1], "inner_start")
assert_eq_string(nested_log[2], "inner_end")
assert_eq_string(nested_log[3], "inner_defer_2")  fr fr Inner defers in LIFO
assert_eq_string(nested_log[4], "inner_defer_1")
assert_eq_string(nested_log[5], "outer_end")
assert_eq_string(nested_log[6], "outer_defer")     fr fr Outer defer last

test_start("Defer with Variable Capture")

sus captured_values []normie = []

slay test_variable_capture() {
    sus x normie = 10
    later { captured_values = captured_values + [x] }
    
    x = 20
    later { captured_values = captured_values + [x] }
    
    x = 30
    later { captured_values = captured_values + [x] }
}

test_variable_capture()

fr fr Variables should be captured at their final values
assert_eq_int(captured_values[0], 30)  fr fr Last defer first (captures final x=30)
assert_eq_int(captured_values[1], 30)  fr fr Second defer (captures final x=30)
assert_eq_int(captured_values[2], 30)  fr fr First defer last (captures final x=30)

test_start("Defer with Function Calls")

sus call_order []tea = []

slay cleanup_function_a() {
    call_order = call_order + ["cleanup_a"]
}

slay cleanup_function_b() {
    call_order = call_order + ["cleanup_b"]
}

slay cleanup_function_c() {
    call_order = call_order + ["cleanup_c"]
}

slay test_function_defers() {
    call_order = call_order + ["start"]
    later cleanup_function_a()
    later cleanup_function_b()
    later cleanup_function_c()
    call_order = call_order + ["end"]
}

test_function_defers()

assert_eq_string(call_order[0], "start")
assert_eq_string(call_order[1], "end")
assert_eq_string(call_order[2], "cleanup_c")  fr fr Last registered defer executes first
assert_eq_string(call_order[3], "cleanup_b")
assert_eq_string(call_order[4], "cleanup_a")  fr fr First registered defer executes last

test_start("Complex Defer with Error Handling")

sus error_log []tea = []

slay risky_operation(should_fail lit) {
    error_log = error_log + ["operation_start"]
    later { error_log = error_log + ["cleanup_always"] }
    
    shook should_fail {
        error_log = error_log + ["operation_failed"]
        damn  fr fr Early return due to error
    }
    
    error_log = error_log + ["operation_success"]
}

fr fr Test with error condition
risky_operation(based)  fr fr This should fail

assert_eq_string(error_log[0], "operation_start")
assert_eq_string(error_log[1], "operation_failed")
assert_eq_string(error_log[2], "cleanup_always")  fr fr Cleanup happens even on error

fr fr Test with success condition  
error_log = []
risky_operation(cringe)  fr fr This should succeed

assert_eq_string(error_log[0], "operation_start")
assert_eq_string(error_log[1], "operation_success")
assert_eq_string(error_log[2], "cleanup_always")  fr fr Cleanup happens on success too

print_test_summary()
