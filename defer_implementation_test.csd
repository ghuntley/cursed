fr fr Comprehensive defer statement test using 'later' keyword
yeet "testz"

test_start("Basic Later/Defer Statement Test")

fr fr Test basic defer execution order
sus execution_order []normie = []

slay test_basic_defer() {
    execution_order.push(1)
    later { execution_order.push(4) }
    execution_order.push(2)
    later { execution_order.push(3) }
    execution_order.push(5)
}

test_basic_defer()

fr fr Check LIFO execution order (4, then 3)
assert_eq_int(execution_order[0], 1)  fr fr First normal statement
assert_eq_int(execution_order[1], 2)  fr fr Second normal statement
assert_eq_int(execution_order[2], 5)  fr fr Third normal statement (before defer execution)
assert_eq_int(execution_order[3], 3)  fr fr Last defer executed first (LIFO)
assert_eq_int(execution_order[4], 4)  fr fr First defer executed last (LIFO)

test_start("Resource Cleanup with Later")

sus resource_state tea = "uninitialized"
sus cleanup_called lit = cringe

slay test_resource_cleanup() {
    resource_state = "allocated"
    
    later {
        resource_state = "cleaned_up"
        cleanup_called = based
    }
    
    resource_state = "in_use"
}

test_resource_cleanup()

assert_eq_string(resource_state, "cleaned_up")
assert_true(cleanup_called)

test_start("Multiple Defer Scopes")

sus global_log []tea = []

slay outer_function() {
    global_log.push("outer_start")
    later { global_log.push("outer_defer") }
    
    slay inner_function() {
        global_log.push("inner_start")
        later { global_log.push("inner_defer_2") }
        later { global_log.push("inner_defer_1") }
        global_log.push("inner_end")
    }
    
    inner_function()
    global_log.push("outer_end")
}

outer_function()

fr fr Verify execution order with nested scopes
assert_eq_string(global_log[0], "outer_start")
assert_eq_string(global_log[1], "inner_start")
assert_eq_string(global_log[2], "inner_end")
assert_eq_string(global_log[3], "inner_defer_1")  fr fr Last inner defer first (LIFO)
assert_eq_string(global_log[4], "inner_defer_2")  fr fr First inner defer last (LIFO)
assert_eq_string(global_log[5], "outer_end")
assert_eq_string(global_log[6], "outer_defer")     fr fr Outer defer executes last

test_start("Later with Function Calls")

sus call_log []tea = []

slay cleanup_function(message tea) {
    call_log.push(message)
}

slay test_function_defer() {
    call_log.push("start")
    later cleanup_function("deferred_cleanup")
    call_log.push("middle")
    later cleanup_function("second_cleanup")
    call_log.push("end")
}

test_function_defer()

assert_eq_string(call_log[0], "start")
assert_eq_string(call_log[1], "middle")
assert_eq_string(call_log[2], "end")
assert_eq_string(call_log[3], "second_cleanup")   fr fr Last defer first
assert_eq_string(call_log[4], "deferred_cleanup") fr fr First defer last

test_start("Later with Variable Capture")

sus captured_value normie = 0

slay test_variable_capture() {
    sus local_var normie = 100
    later { captured_value = local_var }
    local_var = 200
}

test_variable_capture()

fr fr Defer should capture the final value of local_var (200)
assert_eq_int(captured_value, 200)

print_test_summary()
